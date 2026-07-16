// build_pdf.js — eigenständige PDF-Pipeline (Workaround für das Chrome-printToPDF-Kapazitätslimit:
// mdbook-pdf droppt bei sehr großen Büchern den Tail -> Leerseiten). Zerlegt book/print.html in
// Kapitel-Chunks, rendert jeden EINZELN mit Chrome (puppeteer-core) und merged via pdf-lib zu
// book/pdf/output.pdf.
//
// Dieser Ordner (pdf_pipeline/) ist self-contained: enthält die Dependencies in node_modules.
//   node pdf_pipeline/build_pdf.js                  # alle Chunks, 600 Kapitel/Chunk
//   PDF_PER=800 node pdf_pipeline/build_pdf.js
//   PDF_MAX_CHUNKS=2 PDF_PER=40 node pdf_pipeline/build_pdf.js     # schneller Teiltest
//
// ENV:
//   CHROME=<pfad/zu/chrome>       (optional; sonst Auto-Suche: ENV CHROME_PATH/PUPPETEER_EXECUTABLE_PATH
//                                  -> PATH -> übliche OS-Pfade. CI: Chrome-im-PATH reicht, oder
//                                  CHROME=${{ steps.setup-chrome.outputs.chrome-path }})
//   BOOK=<pfad zum mdbook-Output> (Default: ../book relativ zu diesem Skript)
//   HTML_DIR=<pfad mit print.html> (Default: Auto — book/html falls vorhanden [Mehr-Renderer-Layout],
//                                   sonst book/. Hier liegen print.html + Assets css/ImagesExt/fonts)
//   OUT=<ziel-pdf>               (Default: <BOOK>/pdf/output.pdf)
//   PDF_PER=<kapitel/chunk>       (Default 600)
//   PDF_MAX_CHUNKS=<n>            (nur n Chunks rendern; für Tests)
//   PDF_FORMAT=<A4|Letter|...>    (Seitenformat, Default A4)
//   PDF_BREAKS=<0|N|all|none>     (Seitenumbrüche: 0=nur Top-Level neue Seite [Default], N=bis Ebene N,
//                                  all=vor jedem Kapitel, none=fortlaufend. Ebenen aus SUMMARY.md)
//   SUMMARY=<pfad>                (SUMMARY.md für die Ebenen; Default ../src/SUMMARY.md)
//
// Render-Optionen an mdbook-pdf 0.1.13 angelehnt (book.toml [output.pdf] leer): Ränder je 1",
// print_background=false, scale=1, kein Header/Footer, prefer_css_page_size=false, tagged+outline=true.
// Seitenformat: Default A4 (mdbook-pdf-Default wäre Letter 8.5x11"); via PDF_FORMAT änderbar.
// Hinweis: der pdf-lib-Merge überträgt die pro-Chunk-Outline NICHT -> das Gesamt-PDF hat keine
// Lesezeichen. Für outline-erhaltenden Merge stattdessen qpdf/pdftk auf book/_parts/part*.pdf nutzen.

const fs = require("fs"), path = require("path");
const puppeteer = require("puppeteer-core");
const { PDFDocument, PDFName, PDFDict, PDFArray, PDFNumber, PDFNull, PDFRef, PDFString, PDFHexString } = require("pdf-lib");

const BOOK = process.env.BOOK ? path.resolve(process.env.BOOK) : path.resolve(__dirname, "..", "book");
// HTML-Verzeichnis = wo print.html + Assets (css/ImagesExt/fonts) liegen. Bei MEHREREN mdbook-
// Renderern (z. B. [output.html] + [output.pdf]) packt mdbook den HTML-Output nach book/html/, bei
// nur [output.html] direkt nach book/. Auto-Erkennung (per ENV HTML_DIR überschreibbar):
const HTML_DIR = process.env.HTML_DIR ? path.resolve(process.env.HTML_DIR)
  : fs.existsSync(path.join(BOOK, "html", "print.html")) ? path.join(BOOK, "html") : BOOK;
const PRINT = path.join(HTML_DIR, "print.html");
const PARTS = path.join(BOOK, "_parts");
const OUT = process.env.OUT ? path.resolve(process.env.OUT) : path.join(BOOK, "pdf", "output.pdf");
const BREAK = '<div style="break-before: page; page-break-before: always;"></div>';
const PER = parseInt(process.env.PDF_PER || process.argv[2] || "600", 10);
const MAX = parseInt(process.env.PDF_MAX_CHUNKS || "0", 10) || Infinity;
const FORMAT = process.env.PDF_FORMAT || "A4"; // A4 (Default) | Letter | Legal | A3 | …
// Seitenumbruch-Modus: "0" = nur Top-Level-Kapitel beginnen auf neuer Seite (Default), Unterkapitel
// fortlaufend. "<N>" = Umbruch vor Kapiteln bis Ebene N. "all" = vor JEDEM Kapitel (mdbook-Default).
// "none" = keine erzwungenen Umbrüche. Ebenen kommen aus SUMMARY.md (Einrücktiefe).
const BREAKS = process.env.PDF_BREAKS || "0";
const SUMMARY = process.env.SUMMARY || path.resolve(__dirname, "..", "src", "SUMMARY.md");

// SUMMARY.md -> Ebene je Kapitel (Reihenfolge = print.html-Kapitelreihenfolge, 1:1).
function readLevels() {
  if (!fs.existsSync(SUMMARY)) return null;
  const re = /^(\s*)[-*]\s+\[[^\]]*\]\(([^)]*)\)/;
  const rePrefix = /^\[[^\]]*\]\(([^)]*)\)\s*$/; // Prefix-Kapitel (kein Bullet) = Ebene 0
  const levels = [];
  for (const l of fs.readFileSync(SUMMARY, "utf8").split(/\r?\n/)) {
    const m = l.match(re);
    if (m) { levels.push(Math.round(m[1].replace(/\t/g, "  ").length / 2)); continue; }
    if (rePrefix.test(l)) levels.push(0);
  }
  return levels;
}

// SUMMARY.md -> [{level,title}] je Kapitel (inkl. Prefix-Kapitel), Reihenfolge = print.html.
function readSummaryEntries() {
  if (!fs.existsSync(SUMMARY)) return null;
  const reList = /^(\s*)[-*]\s+\[([^\]]*)\]\(([^)]*)\)/;
  const rePrefix = /^\[([^\]]*)\]\(([^)]*)\)\s*$/;
  const out = [];
  for (const l of fs.readFileSync(SUMMARY, "utf8").split(/\r?\n/)) {
    let m = l.match(reList);
    if (m) { out.push({ level: Math.round(m[1].replace(/\t/g, "  ").length / 2), title: m[2] }); continue; }
    m = l.match(rePrefix);
    if (m) out.push({ level: 0, title: m[1] });
  }
  return out;
}

// erster Überschriften-Anker (id) eines Kapitels aus print.html (roher Slug, unkodiert).
function slugFromChapter(html) {
  const m = html.match(/<h[1-6][^>]*\sid="([^"]+)"/i);
  return m ? m[1] : null;
}

// Chrome-Dest-Name (PDF-Name, #XX-escaped + %XX) -> roher Slug (wie print.html-id).
function decodeChromeName(nameStr) {
  let s = nameStr.replace(/^\//, "").replace(/#([0-9A-Fa-f]{2})/g, (_, h) => String.fromCharCode(parseInt(h, 16)));
  try { return decodeURIComponent(s); } catch { return s; }
}

// Lesezeichen-Baum aus SUMMARY-Hierarchie bauen, Ziele über die (gemergten) Named-Dests.
// slugToName: roher Slug -> PDFName (Schlüssel in katalog /Dests). Alle Items initial zugeklappt.
function buildOutline(merged, entries, chapters, slugToName) {
  const ctx = merged.context;
  const n = Math.min(entries.length, chapters.length);
  const items = [];
  let withDest = 0;
  for (let i = 0; i < n; i++) {
    const dict = ctx.obj({});
    dict.set(PDFName.of("Title"), PDFHexString.fromText(entries[i].title || ""));
    const slug = slugFromChapter(chapters[i]);
    const destName = slug != null ? slugToName.get(slug) : undefined;
    if (destName) { dict.set(PDFName.of("Dest"), destName); withDest++; }
    items.push({ level: entries[i].level, dict, ref: ctx.register(dict), children: [] });
  }
  // Hierarchie per Ebenen-Stack
  const roots = [], stack = [];
  for (const it of items) {
    while (stack.length && stack[stack.length - 1].level >= it.level) stack.pop();
    if (stack.length) stack[stack.length - 1].children.push(it); else roots.push(it);
    stack.push(it);
  }
  const outlines = ctx.obj({}); outlines.set(PDFName.of("Type"), PDFName.of("Outlines"));
  const outlinesRef = ctx.register(outlines);
  (function wire(children, parentRef) {
    for (let i = 0; i < children.length; i++) {
      const it = children[i];
      it.dict.set(PDFName.of("Parent"), parentRef);
      if (i > 0) it.dict.set(PDFName.of("Prev"), children[i - 1].ref);
      if (i < children.length - 1) it.dict.set(PDFName.of("Next"), children[i + 1].ref);
      if (it.children.length) {
        it.dict.set(PDFName.of("First"), it.children[0].ref);
        it.dict.set(PDFName.of("Last"), it.children[it.children.length - 1].ref);
        it.dict.set(PDFName.of("Count"), ctx.obj(-it.children.length)); // zugeklappt
        wire(it.children, it.ref);
      }
    }
  })(roots, outlinesRef);
  if (roots.length) {
    outlines.set(PDFName.of("First"), roots[0].ref);
    outlines.set(PDFName.of("Last"), roots[roots.length - 1].ref);
    outlines.set(PDFName.of("Count"), ctx.obj(roots.length)); // sichtbar: Top-Level (alle zu)
  }
  merged.catalog.set(PDFName.of("Outlines"), outlinesRef);
  return { total: n, withDest };
}

// Chrome-Binary finden: 1) explizite ENV, 2) PATH (so legt es browser-actions/setup-chrome@v2 ab),
// 3) übliche Installationspfade je OS. In CI genügt damit Chrome-im-PATH; alternativ
//    CHROME=${{ steps.setup-chrome.outputs.chrome-path }} setzen.
function resolveChrome() {
  for (const v of ["CHROME", "CHROME_PATH", "PUPPETEER_EXECUTABLE_PATH"]) {
    const p = process.env[v];
    if (p && fs.existsSync(p)) return p;
  }
  const isWin = process.platform === "win32", isMac = process.platform === "darwin";
  const names = isWin
    ? ["chrome.exe", "chromium.exe", "msedge.exe"]
    : ["google-chrome", "google-chrome-stable", "chromium", "chromium-browser", "chrome"];
  for (const dir of (process.env.PATH || "").split(path.delimiter)) {
    for (const n of names) {
      const cand = path.join(dir, n);
      try { if (fs.statSync(cand).isFile()) return cand; } catch { /* weiter */ }
    }
  }
  const common = isWin
    ? ["C:/Program Files/Google/Chrome/Application/chrome.exe",
       "C:/Program Files (x86)/Google/Chrome/Application/chrome.exe"]
    : isMac
      ? ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
         "/Applications/Chromium.app/Contents/MacOS/Chromium"]
      : ["/usr/bin/google-chrome", "/usr/bin/google-chrome-stable",
         "/usr/bin/chromium-browser", "/usr/bin/chromium", "/snap/bin/chromium"];
  for (const p of common) if (fs.existsSync(p)) return p;
  return null;
}
const CHROME = resolveChrome();

const PDF_OPTS = {
  format: FORMAT, // setzt Papiergröße; Default A4 (210x297mm). mdbook-pdf-Default wäre Letter.
  margin: { top: "1in", bottom: "1in", left: "1in", right: "1in" },
  printBackground: false, scale: 1, landscape: false,
  displayHeaderFooter: false, preferCSSPageSize: false,
  tagged: true, outline: true,
};

// Print-Override (nur für die PDF-Generierung, wird in jedes Chunk-HTML injiziert): mdbook setzt
// `.table-wrapper { overflow-x: auto }` -> im PDF wird der weggescrollte Teil überbreiter Tabellen
// ABGESCHNITTEN. Hier: Scroll-Container auflösen + Tabellen auf Seitenbreite umbrechen statt clippen.
const PRINT_CSS = `<style id="pdf-pipeline-print">
@media print {
  .table-wrapper { overflow: visible !important; }
  .table-wrapper > table { max-width: 100% !important; table-layout: auto; }
  .table-wrapper th, .table-wrapper td { overflow-wrap: anywhere; word-break: break-word; }
}
</style>`;

async function main() {
  if (!fs.existsSync(PRINT)) { console.error(`print.html fehlt unter ${PRINT} — erst HTML bauen (oder BOOK= setzen).`); process.exit(1); }
  if (!CHROME) { console.error("Chrome nicht gefunden (weder ENV CHROME/CHROME_PATH/PUPPETEER_EXECUTABLE_PATH, noch im PATH, noch an üblichen Pfaden). In CI z. B. CHROME=${{ steps.setup-chrome.outputs.chrome-path }} setzen."); process.exit(1); }
  // Meta-Refresh-Weiterleitungen der Redirect-Seiten (z. B. src/index.md "Startseite" und die
  // Leer-Index-Knoten) aus dem PDF-Rendering entfernen: sonst navigiert Chrome beim Laden eines
  // Chunks weg -> page.pdf() erfasst die Zielseite statt der Chunk-Kapitel. Im linearen PDF sind
  // Redirects ohnehin sinnlos; der sichtbare "Weiter zu …"-Fallback-Link bleibt erhalten.
  const h = fs.readFileSync(PRINT, "utf8")
    .replace(/<meta\b[^>]*http-equiv\s*=\s*["']?refresh["']?[^>]*>/gi, "");
  const mo = h.match(/<main\b[^>]*>/); const start = mo.index + mo[0].length;
  const end = h.lastIndexOf("</main>");
  const prefix = h.slice(0, start).replace("</head>", PRINT_CSS + "\n</head>"), suffix = h.slice(end);
  const chapters = h.slice(start, end).split(BREAK);
  const nChunks = Math.min(MAX, Math.ceil(chapters.length / PER));

  // Umbruch-Entscheidung je Kapitelgrenze festlegen.
  let mode = BREAKS, levels = null;
  if (BREAKS !== "all" && BREAKS !== "none") {
    const lvl = parseInt(BREAKS, 10);
    if (isNaN(lvl)) { console.warn(`PDF_BREAKS="${BREAKS}" ungültig -> 'all'.`); mode = "all"; }
    else {
      levels = readLevels();
      if (!levels || levels.length !== chapters.length) {
        console.warn(`PDF_BREAKS=${BREAKS}: SUMMARY-Ebenen nicht verfügbar/passend (${levels ? levels.length : "keine"} vs ${chapters.length}) -> Fallback 'all'.`);
        mode = "all";
      } else { mode = lvl; } // numerische Ebene
    }
  }
  // break VOR Kapitel-Index i? (i=0 nie; Chunk-Erste sowieso neue Seite, daher nur i>chunkStart relevant)
  const breakBefore = (i) =>
    i === 0 ? false : mode === "all" ? true : mode === "none" ? false : levels[i] <= mode;

  const modeStr = mode === "all" ? "jedes Kapitel" : mode === "none" ? "keine" : `Top-Level..Ebene ${mode}`;
  console.log(`HTML: ${HTML_DIR}\nOutput: ${OUT}\nKapitel: ${chapters.length} | ${PER}/Chunk | ${nChunks} Chunk(s) | Umbruch: ${modeStr} | Chrome: ${CHROME}`);

  fs.mkdirSync(PARTS, { recursive: true });
  fs.mkdirSync(path.dirname(OUT), { recursive: true });
  const pad = String(nChunks).length;

  const browser = await puppeteer.launch({
    executablePath: CHROME, headless: true,
    args: ["--no-sandbox", "--disable-gpu", "--no-first-run", "--no-default-browser-check"],
  });

  const partPdfs = [], chunkHtmls = [];
  try {
    for (let i = 0; i < nChunks; i++) {
      const id = String(i + 1).padStart(pad, "0");
      // Chunk-HTML MUSS im HTML-Verzeichnis liegen, damit relative CSS-/Font-/Bild-Pfade auflösen.
      const htmlFile = path.join(HTML_DIR, `_chunk_${id}.html`);
      const pdfFile = path.join(PARTS, `part${id}.pdf`);
      const s = i * PER, e = Math.min((i + 1) * PER, chapters.length);
      let body = "";
      for (let g = s; g < e; g++) { if (g > s && breakBefore(g)) body += BREAK; body += chapters[g]; }
      fs.writeFileSync(htmlFile, prefix + body + suffix);
      chunkHtmls.push(htmlFile);
      process.stdout.write(`  Chunk ${id}: rendern… `);
      const page = await browser.newPage();
      await page.goto("file:///" + htmlFile.replace(/\\/g, "/"), { waitUntil: "networkidle0", timeout: 300000 });
      await page.pdf({ path: pdfFile, ...PDF_OPTS });
      await page.close();
      console.log(`ok (${(fs.statSync(pdfFile).size / 1024 / 1024).toFixed(1)} MB)`);
      partPdfs.push(pdfFile);
    }
  } finally { await browser.close(); }

  process.stdout.write("Merge (pdf-lib)… ");
  const merged = await PDFDocument.create();
  const mergedDests = merged.context.obj({}); // katalog-weite /Dests-Tabelle (Name -> Ziel)
  const slugToName = new Map();               // roher Slug -> PDFName-Key (für die Outline)
  let total = 0, destCount = 0, destCollision = 0;
  for (const pf of partPdfs) {
    const doc = await PDFDocument.load(fs.readFileSync(pf));
    const srcPages = doc.getPages();
    const tagToIdx = new Map(srcPages.map((p, i) => [p.ref.tag, i]));
    const copied = await merged.copyPages(doc, doc.getPageIndices());
    copied.forEach(p => merged.addPage(p));
    total += copied.length;
    // Benannte Ziele (Katalog /Dests) übernehmen: Chrome legt interne Sprungziele hier ab
    // (Name = Überschriften-Slug). copyPages kopiert diese Tabelle NICHT -> sonst zeigen alle
    // internen GoTo-Links ins Leere. Seiten-Referenz auf die kopierte Seite umbiegen.
    const dRef = doc.catalog.get(PDFName.of("Dests"));
    if (dRef) {
      const dDict = doc.context.lookup(dRef, PDFDict);
      for (const [nameKey, val] of dDict.dict) {
        let arr = doc.context.lookup(val);
        if (arr instanceof PDFDict) arr = doc.context.lookup(arr.get(PDFName.of("D")));
        if (!(arr instanceof PDFArray)) continue;
        const first = arr.get(0);
        const idx = first instanceof PDFRef ? tagToIdx.get(first.tag) : undefined;
        if (idx === undefined) continue;
        if (mergedDests.has(nameKey)) { destCollision++; continue; }
        const rest = [];
        for (let k = 1; k < arr.size(); k++) {
          const el = arr.get(k);
          if (el instanceof PDFName) rest.push(el);                       // /XYZ, /Fit …
          else if (el instanceof PDFNumber) rest.push(merged.context.obj(el.asNumber()));
          else rest.push(PDFNull);                                        // null-Koordinaten
        }
        mergedDests.set(nameKey, merged.context.obj([copied[idx].ref, ...rest]));
        const slug = decodeChromeName(nameKey.asString());
        if (!slugToName.has(slug)) slugToName.set(slug, nameKey);
        destCount++;
      }
    }
  }
  if (destCount) merged.catalog.set(PDFName.of("Dests"), mergedDests);

  // Outline/Lesezeichen aus SUMMARY-Hierarchie rekonstruieren (pdf-lib überträgt die pro-Chunk-
  // Outline nicht). Ziele über die eben gemergten Named-Dests.
  let outlineInfo = null;
  const entries = readSummaryEntries();
  if (entries && entries.length) {
    if (entries.length !== chapters.length)
      console.warn(`\n  Outline: SUMMARY-Einträge (${entries.length}) != Kapitel (${chapters.length}) -> baue min(${Math.min(entries.length, chapters.length)}).`);
    outlineInfo = buildOutline(merged, entries, chapters, slugToName);
  }

  fs.writeFileSync(OUT, await merged.save());
  console.log(`fertig.\n=> ${OUT}\nSeiten gesamt: ${total} | benannte Ziele: ${destCount}${destCollision ? ` (Kollisionen: ${destCollision})` : ""}` +
    (outlineInfo ? ` | Lesezeichen: ${outlineInfo.total} (mit Ziel: ${outlineInfo.withDest})` : "") +
    ` | ${(fs.statSync(OUT).size / 1024 / 1024).toFixed(1)} MB`);

  for (const f of chunkHtmls) fs.unlinkSync(f); // book/_parts/part*.pdf bleiben (Kontrolle / externer Merge)
}
main().catch(e => { console.error(e); process.exit(1); });
