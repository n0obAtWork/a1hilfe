# pdf_pipeline

Erzeugt das Gesamt-PDF aus dem mdbook-HTML-Output (`book/print.html`) und umgeht das
**Chrome-`printToPDF`-Kapazitätslimit**: Bei sehr großen Büchern rendert `mdbook-pdf` nur bis zu
einer (speicherabhängigen) Grenze und lässt den Rest leer (bei diesem Buch ab ~Seite 3238). Hier wird
`print.html` in Kapitel-Chunks zerlegt, jeder Chunk **einzeln** gerendert und alles zusammengeführt.

Self-contained: dieser Ordner enthält die Dependencies in `node_modules` (Einchecken optional — sonst
einmal `npm install` in diesem Ordner).

## Voraussetzungen
- `book/` ist gebaut (enthält `print.html`) — d. h. der mdbook-**HTML**-Output liegt vor.
- Chrome/Chromium installiert.
- Node.

## Verwendung

```sh
# Vom Repo-Root. Rendert alle Chunks + merged via pdf-lib -> book/pdf/output.pdf
node pdf_pipeline/build_pdf.js

# Optionen (ENV):
CHROME="C:/Program Files/Google/Chrome/Application/chrome.exe"   # Chrome-Pfad (Default gesetzt)
BOOK="../book"        # mdbook-Output (Default: ../book relativ zum Skript)
PDF_PER=600           # Kapitel pro Chunk (Default 600; bei schwacher Maschine kleiner)
PDF_MAX_CHUNKS=2      # nur n Chunks (Schnelltest)
PDF_FORMAT=A4         # Seitenformat (Default A4; z. B. Letter, Legal, A3)

# Beispiel Schnelltest:
PDF_MAX_CHUNKS=2 PDF_PER=40 node pdf_pipeline/build_pdf.js
```

## Chrome-Erkennung / CI
Das Skript findet Chrome in dieser Reihenfolge: ENV `CHROME` / `CHROME_PATH` /
`PUPPETEER_EXECUTABLE_PATH` → `PATH` → übliche OS-Installationspfade.

Mit `browser-actions/setup-chrome@v2` (legt Chrome in den `PATH`) genügt i. d. R. der nackte Aufruf.
Robuster (unabhängig vom PATH) ist es, den Action-Output zu setzen:

```yaml
- uses: browser-actions/setup-chrome@v2
  id: setup-chrome
- run: node pdf_pipeline/build_pdf.js
  env:
    CHROME: ${{ steps.setup-chrome.outputs.chrome-path }}
```

(Auf Linux-Runnern erkennt die Auto-Suche `google-chrome`/`chromium` im PATH bzw. unter `/usr/bin`.)

## Render-Settings
An **mdbook-pdf 0.1.13** Defaults angelehnt (da `book.toml` → `[output.pdf]` leer): Ränder je 1",
`printBackground=false`, `scale=1`, kein Header/Footer, `preferCSSPageSize=false`.
Seitenformat: **A4** (Default; mdbook-pdf-Default wäre Letter 8.5×11") — via `PDF_FORMAT` änderbar.

## Lesezeichen / Outline
Der `pdf-lib`-Merge in `build_pdf.js` überträgt die pro-Chunk-Outline **nicht** → das Gesamt-PDF hat
keine Lesezeichen. Wer Lesezeichen will: `split_print.js` nutzen (erzeugt nur die Chunk-HTMLs),
extern mit Chrome rendern und mit **qpdf**/**pdftk** mergen (outline-erhaltend):

```sh
node pdf_pipeline/split_print.js 600
# ... jeden book/print_part*.html mit Chrome zu PDF rendern ...
qpdf --empty --pages book/print_part*.pdf -- book/pdf/output.pdf
```

## Hinweis zur Chunk-Größe
Das Limit ist **speicher-/inhaltsabhängig**, keine feste Seitenzahl (leichte Seiten: 8000+ ok; schwere
Buchseiten dieses Projekts: ~3238). Daher mit Sicherheitsabstand unter der beobachteten Grenze bleiben.
`PDF_PER=600` lief auf der Testmaschine für alle Chunks sauber durch (Gesamt-PDF ~7600 Seiten).
