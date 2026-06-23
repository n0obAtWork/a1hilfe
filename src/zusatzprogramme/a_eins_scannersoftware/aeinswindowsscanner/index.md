# AeinsWindowsScanner

<!-- source: https://amic.de/hilfe/_cescannerwindows.htm -->

In dem Aeins\\bin Verzeichnis finden Sie die A.eins.Scanner Software. Dieses Programm ist ein Abbild von der Software, die auf den MDE Geräten benutzt wird.

Die AeinsWindowsScanner Software hat noch ein paar Besonderheiten. Da Desktop Rechner im normalen Fall keine Scaneinheit besitzen, so wird der der Scancode über die Tastatur in das Eingabefeld gegeben eingetragen z.B. AU 4711. Ein Scanvorgang wird mit F2 bestätigt. Das Eingeben der Mengeneinheit wird wie auf dem MDE Gerät mit ENTER bestätigt.

Es besteht die Möglichkeit einen Scanner an den Rechner anzuschließen, um mit diesem die Barcodes zu lesen. Der Scanner muss so eingerichtet werden, dass als Präfix das Enterzeichen mit übermittelt wird.

Des Weiteren müssen auf jeden Fall die Steuerparameter [727](../../../firmenstamm/steuerparameter/scanner/ean8_code_wird_als_solcher_erkannt_auch_wenn_dieser_nicht_gu.md) und [728](../../../firmenstamm/steuerparameter/scanner/ean13_code_wird_als_solcher_erkannt_auch_wenn_dieser_nicht_g.md) auf ja gestellt werden, da es keine Scaneinheit gibt welche mir den erkannten Scancode mitteilt.

<p class="just-emphasize">Starten der Software aus dem Bin Verzeichniss</p>

Wird die Scanner Software zum ersten Mal aus dem Bin Verzeichnis gestartet, so müssen dem Scanner die [Verbindungsparameter](../private_prozeduren/index.md) mitgeteilt werden.

<p class="just-emphasize">Starten der Software aus dem Hauptmenü</p>

Hauptmenü > Externe Kommunikation > Scanner Lösungen > Scannerprogramm öffnen

<p class="just-emphasize">Besonderheit</p>

Beim starten des Scanners aus dem Hauptmenü wird als logische IP-Adresse des Scanners der aktuelle Bediener aus dem A.eins System genommen, damit wird die Möglichkeit geschaffen mehrere Scanner gleichzeitig auf einem Rechner zu starten.

<p class="just-emphasize">XML Datei</p>

Da diese Abarbeitung sehr langwierig sein kann, haben wir eine Technologie bereitgestellt, mit dem Sie die Möglichkeit haben einen Auftrag, Inventur etc. in eine XML Struktur zu verpacken. Diese XML Struktur kann dann der Scanner einlesen und verarbeiten.

<p class="siehe-auch">Siehe auch:</p>

- [Kommandozeile](./kommandozeile.md)
- [Beispiel XML Datei](./beispiel_xml_datei.md)
