# Vorgangsimport Anwendung

<!-- source: https://amic.de/hilfe/_vimp_anwendung.htm -->

<p class="just-emphasize">Folgende Standard Varianten stehen zur Auswahl</p>

1. Vorgangsimport

2. Importierte Positionen bearbeiten

3. Vorgangsimportstatistik

<p class="just-emphasize">Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“</p>

Da sich die beiden Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“ sich nicht wesentlich unterscheiden werden diese beiden Varianten unter diesem Punkt beschrieben.

Der Unterschied zwischen der Variante „Vorgansimport“ und „Importiert Vorgänge bearbeiten“ liegt im wesentliche darin, dass bestimmte Felder in der Auswahlliste farblich markiert werden, wenn diese zu einem nicht erfolgreichem Anlegen des Vorgangs führen würden. Des Weiteren besteht die Möglichkeit in dieser Variante einzelne Position zu Bearbeiten.

<p class="just-emphasize">Automatik</p>

In der [Automatikschnittstelle](http://www.amic.de/ihilfe/#!XMLDocuments/AeinsInterface/html/M_JPL_JFunktionen_VimpErzeugeBelege_5_2981a6cf.htm) könne automatisierte Importprozesse realisiert werden, wie z.B. nächtliche Übernahmen von Tankbelegen oder automatisierte Übernahmen von EDI Ordersätzen.

<p class="just-emphasize">Auswahlliste</p>

In der Auswahlliste der Variante „Importierte Positionen bearbeiten“ können bestimmte Felder farblich markiert, wenn diese zu einem nicht erfolgreichen Anlegen des Vorgangs führen würden.

Folgende Bedeutung haben die farblich markierten Felder

| Feld | Farben Bedeutung |
| --- | --- |
| Status Stamm | 1. Weiß Bereit Vorgang kann erzeugt werden<br>2. Grün Belegerzeugung oder Einspielung läuft<br>3. Rot Es sind Fehler bei der Einspielung, beim Erzeugen aufgetreten oder der Stammsatz ist gelöscht worden.<br> |
| Kundnr./ Kunde | 1. Weiß Kunde kann gewählt werden<br>2. Rot der Lieferant hat eine Bestellsperre für diesen Artikel |
| Artikel / Artikelnummer | 1. Artikel kann Bestellt werden<br>2. Rot der Artikel hat eine Bestellsperre |
| Artikellieferant / Artikellieferant | |
| Status Position | 1. Weiß Bereit Vorgang kann erzeugt werden<br>2. Grün Belegerzeugung oder Einspielung läuft<br>3. Rot Es sind Fehler bei der Einspielung, beim Erzeugen aufgetreten oder der Positionssatz ist gelöscht worden.<br> |

<p class="just-emphasize">Gleiche Funktionen in den Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“</p>

<p class="just-emphasize">Daten einlesen(Vorg.Import)</p>

Mit dieser Funktion können Vorgangsdaten die in einer Datei gespeichert sind eingelesen werden. Für das Einlesen und Speichern der Daten ist das Makro Vorgangeinlesen zuständig.

<p class="just-emphasize">VIMP (Positionen)</p>

Diese Funktion ruft eine weitere Auswahlliste auf, welche nur die Positionen der Vorgangsimport anzeigt.

<p class="just-emphasize">VIMP (Text-Positionen)</p>

Diese Funktion ruft eine weitere Auswahlliste auf, welche nur die Textpositionen zur dazugehörigen Position anzeigt.

<p class="just-emphasize">Importumsetzer</p>

Mit dieser Funktion wird der [Importumsetzer](../../../../externe_kommunikation/importumsetzer.md) aufgerufen.

<p class="just-emphasize">Übernahmelöschen</p>

Eine komplette Übergabe wird gelöscht unabhängig vom Status der Übergabe. Wenn zu dieser Übergabe schon ein umgewandelter Vorgang existiert, wird eine Warnung ausgegeben.

<p class="just-emphasize">Fehlerrücksetzen</p>

Ausgewählte Datensätze aus den Relationen [ImportVorgstamm](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgstamm.htm) und [ImportVorgPosition](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgPosition.htm)

mit dem Status 6 und 7 (Belegerzeugung läuft bzw. fehlerhaft bei Belegerzeugung)

werden auf den Status 5 (ist erfolgreich umgeschlüsselt) zurückgesetzt, damit sie z. B.

nach Anpassung der Stammdaten erneut in Belege umgewandelt werden können.

Belege mit Status 3 und 4 (Konvertierung läuft bzw. Fehler bei Konvertierung) werden auf

den Status 2 zurückgesetzt

<p class="just-emphasize">Erledigte löschen</p>

Hierbei wird das Löschkennzeichen für erledigte Vorgänge(Status 9) gesetzt.

<p class="just-emphasize">Endgültig löschen</p>

Alle Vorgänge aller betroffenen Relationen mit Status 9 (erledigt) werden gelöscht

<p class="just-emphasize">Sonderfunktionen in der Variante „Importierte Positionen bearbeiten“</p>

<p class="just-emphasize">Standard Vorgänge erzeugen</p>

Mit dieser Funktion können Vorgänge aus den Importierten Daten erzeugt werden. Welche Vorgänge erzeugt werden und mit welchen Inhalten ist [an dieser Stelle](./index.md#StandardVorgangErzeugen) beschrieben.

<p class="just-emphasize">Ändern / Ansehen /Löschen</p>

Mit dieser Funktion könne Daten auf der [Positionsebene](./funktionen_des_vimp_pflegers.md#PositionBearbeiten) bearbeitet oder gelöscht werden.

Richtlinien fürs Schreiben der Hilfe

<p class="just-emphasize">Variante Vorgangsimportstatistik</p>

 

<p class="siehe-auch">Siehe auch:</p>

- [Funktionen des VIMP-Pflegers](./funktionen_des_vimp_pflegers.md)
