# pdf_pipeline

Erzeugt das Gesamt-PDF aus dem mdbook-HTML-Output (`book/print.html`) und umgeht das
**Chrome-`printToPDF`-Kapazitätslimit**: Bei sehr großen Büchern rendert `mdbook-pdf` nur bis zu
einer (speicherabhängigen) Grenze und lässt den Rest leer (bei diesem Buch ab ~Seite 3238). Hier wird
`print.html` in Kapitel-Chunks zerlegt, jeder Chunk **einzeln** gerendert und alles zusammengeführt.

Self-contained: dieser Ordner enthält die Dependencies in `node_modules` (Einchecken optional — sonst
einmal `npm install` in diesem Ordner).

## Voraussetzungen
- Der mdbook-**HTML**-Output liegt vor (enthält `print.html`). Layout wird automatisch erkannt:
  `book/html/print.html` (Mehr-Renderer-Layout, z. B. `[output.html]` + `[output.pdf]`) **oder**
  `book/print.html` (nur `[output.html]`). Sonst per `HTML_DIR=` setzen.
- Chrome/Chromium installiert.
- Node.

## Verwendung

```sh
# Vom Repo-Root. Rendert alle Chunks + merged via pdf-lib -> book/pdf/output.pdf
node pdf_pipeline/build_pdf.js

# Optionen (ENV):
CHROME=...            # Chrome-Pfad (sonst Auto-Suche, s. u.)
BOOK="../book"        # mdbook-Output-Basis (Default: ../book relativ zum Skript)
HTML_DIR=...          # wo print.html + Assets liegen (Default: Auto — book/html falls vorhanden, sonst book)
OUT=...               # Ziel-PDF (Default: <BOOK>/pdf/output.pdf)
PDF_PER=600           # Kapitel pro Chunk (Default 600; bei schwacher Maschine kleiner)
PDF_MAX_CHUNKS=2      # nur n Chunks (Schnelltest)
PDF_FORMAT=A4         # Seitenformat (Default A4; z. B. Letter, Legal, A3)
PDF_BREAKS=0          # Seitenumbrüche: 0=nur Top-Level neue Seite (Default), N=bis Ebene N,
                      #   all=vor jedem Kapitel (mdbook-Default), none=fortlaufend
SUMMARY=...           # SUMMARY.md für die Kapitel-Ebenen (Default: ../src/SUMMARY.md)

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

**Überbreite Tabellen:** mdbook setzt `.table-wrapper { overflow-x: auto }` → im PDF würde der
weggescrollte Teil abgeschnitten. `build_pdf.js` injiziert deshalb pro Chunk eine Print-Override
(`overflow: visible` + Tabellen auf Seitenbreite umbrechen statt clippen). `custom.css` / die HTML-Site
bleiben unverändert.

## Seitenumbrüche (`PDF_BREAKS`)
mdbook setzt vor JEDEM Kapitel einen Seitenumbruch → jede `.md` beginnt auf neuer Seite (sehr viele
Seiten). Steuerbar:
- **`0`** (Default): nur **Top-Level-Kapitel** (Ebene 0, die Hauptsektionen) beginnen auf neuer Seite,
  Unterkapitel laufen fortlaufend weiter → kompaktes PDF.
- **`N`**: Umbruch vor Kapiteln bis Ebene N (z. B. `1` = Sektionen + deren direkte Unterkapitel).
- **`all`**: vor jedem Kapitel (altes mdbook-Verhalten).
- **`none`**: keine erzwungenen Umbrüche.

Die Ebenen stammen aus der Einrücktiefe in `SUMMARY.md` (1:1 zur Kapitelreihenfolge in `print.html`).
Fehlt `SUMMARY.md` oder passt die Anzahl nicht, fällt das Skript mit Warnung auf `all` zurück.
Kleiner Nebeneffekt: An Chunk-Grenzen (alle `PDF_PER` Kapitel) beginnt ohnehin eine neue Seite — das
sind ein paar wenige zusätzliche Umbrüche, in einem mehrtausendseitigen PDF vernachlässigbar.

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
