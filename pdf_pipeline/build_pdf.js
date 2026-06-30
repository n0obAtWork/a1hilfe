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
//
// Render-Optionen an mdbook-pdf 0.1.13 angelehnt (book.toml [output.pdf] leer): Ränder je 1",
// print_background=false, scale=1, kein Header/Footer, prefer_css_page_size=false, tagged+outline=true.
// Seitenformat: Default A4 (mdbook-pdf-Default wäre Letter 8.5x11"); via PDF_FORMAT änderbar.
// Hinweis: der pdf-lib-Merge überträgt die pro-Chunk-Outline NICHT -> das Gesamt-PDF hat keine
// Lesezeichen. Für outline-erhaltenden Merge stattdessen qpdf/pdftk auf book/_parts/part*.pdf nutzen.

const fs = require("fs"), path = require("path");
const puppeteer = require("puppeteer-core");
const { PDFDocument } = require("pdf-lib");

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

async function main() {
  if (!fs.existsSync(PRINT)) { console.error(`print.html fehlt unter ${PRINT} — erst HTML bauen (oder BOOK= setzen).`); process.exit(1); }
  if (!CHROME) { console.error("Chrome nicht gefunden (weder ENV CHROME/CHROME_PATH/PUPPETEER_EXECUTABLE_PATH, noch im PATH, noch an üblichen Pfaden). In CI z. B. CHROME=${{ steps.setup-chrome.outputs.chrome-path }} setzen."); process.exit(1); }
  const h = fs.readFileSync(PRINT, "utf8");
  const mo = h.match(/<main\b[^>]*>/); const start = mo.index + mo[0].length;
  const end = h.lastIndexOf("</main>");
  const prefix = h.slice(0, start), suffix = h.slice(end);
  const chapters = h.slice(start, end).split(BREAK);
  const nChunks = Math.min(MAX, Math.ceil(chapters.length / PER));
  console.log(`HTML: ${HTML_DIR}\nOutput: ${OUT}\nKapitel: ${chapters.length} | ${PER}/Chunk | ${nChunks} Chunk(s) | Chrome: ${CHROME}`);

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
      fs.writeFileSync(htmlFile, prefix + chapters.slice(i * PER, (i + 1) * PER).join(BREAK) + suffix);
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
  let total = 0;
  for (const pf of partPdfs) {
    const doc = await PDFDocument.load(fs.readFileSync(pf));
    const pages = await merged.copyPages(doc, doc.getPageIndices());
    pages.forEach(p => merged.addPage(p));
    total += doc.getPageCount();
  }
  fs.writeFileSync(OUT, await merged.save());
  console.log(`fertig.\n=> ${OUT}\nSeiten gesamt: ${total} | ${(fs.statSync(OUT).size / 1024 / 1024).toFixed(1)} MB`);

  for (const f of chunkHtmls) fs.unlinkSync(f); // book/_parts/part*.pdf bleiben (Kontrolle / externer Merge)
}
main().catch(e => { console.error(e); process.exit(1); });
