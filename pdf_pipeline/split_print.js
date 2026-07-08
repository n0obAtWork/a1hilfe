// split_print.js — NUR die Split-Stufe (keine Dependencies). Zerlegt book/print.html strukturtreu in
// Kapitel-Chunk-HTMLs im book/-Root. Danach extern rendern (Chrome) + mit qpdf/pdfunite mergen.
// Alternative zu build_pdf.js (das alles automatisch via puppeteer-core + pdf-lib macht).
//
// Aufruf:  node pdf_pipeline/split_print.js [chaptersPerChunk]    (Default 600)
// ENV:     BOOK=<pfad zum mdbook-Output>   (Default ../book relativ zu diesem Skript)

const fs = require("fs"), path = require("path");
const BOOK = process.env.BOOK ? path.resolve(process.env.BOOK) : path.resolve(__dirname, "..", "book");
const PRINT = path.join(BOOK, "print.html");
const PER = Math.max(1, parseInt(process.argv[2] || "600", 10));
const BREAK = '<div style="break-before: page; page-break-before: always;"></div>';

if (!fs.existsSync(PRINT)) { console.error(`print.html fehlt unter ${PRINT}.`); process.exit(1); }
// Meta-Refresh-Weiterleitungen der Redirect-Seiten entfernen — sonst navigiert Chrome beim
// Rendern eines Chunks weg (siehe build_pdf.js). Im linearen PDF ohnehin sinnlos.
const h = fs.readFileSync(PRINT, "utf8")
  .replace(/<meta\b[^>]*http-equiv\s*=\s*["']?refresh["']?[^>]*>/gi, "");
const mo = h.match(/<main\b[^>]*>/); const start = mo.index + mo[0].length;
const end = h.lastIndexOf("</main>");
const prefix = h.slice(0, start), suffix = h.slice(end);
const chapters = h.slice(start, end).split(BREAK);

for (const f of fs.readdirSync(BOOK)) if (/^print_part\d+\.html$/.test(f)) fs.unlinkSync(path.join(BOOK, f));
const nChunks = Math.ceil(chapters.length / PER);
const pad = String(nChunks).length;
for (let i = 0; i < nChunks; i++) {
  const name = `print_part${String(i + 1).padStart(pad, "0")}.html`;
  fs.writeFileSync(path.join(BOOK, name), prefix + chapters.slice(i * PER, (i + 1) * PER).join(BREAK) + suffix);
  console.log(`  ${name}: Kapitel ${i * PER + 1}..${Math.min((i + 1) * PER, chapters.length)}`);
}
console.log(`\n${nChunks} Chunks in ${BOOK} geschrieben.`);
console.log("\n--- Rendern (Chrome headless) ---");
console.log('for f in book/print_part*.html; do "<chrome>" --headless=new --disable-gpu \\');
console.log('  --no-pdf-header-footer --print-to-pdf="$(pwd)/${f%.html}.pdf" "file://$(pwd)/$f"; done');
console.log("--- Mergen (outline-erhaltend) ---");
console.log("qpdf --empty --pages book/print_part*.pdf -- book/pdf/output.pdf");
