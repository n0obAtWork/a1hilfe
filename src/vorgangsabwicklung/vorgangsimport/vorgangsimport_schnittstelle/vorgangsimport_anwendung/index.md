# Vorgangsimport Anwendung

<!-- source: https://amic.de/hilfe/_vimp_anwendung.htm -->

Folgende Standard Varianten stehen zur Auswahl

1. Vorgangsimport

2. Importierte Positionen bearbeiten

3. Vorgangsimportstatistik

Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“

Da sich die beiden Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“ sich nicht wesentlich unterscheiden werden diese beiden Varianten unter diesem Punkt beschrieben.

Der Unterschied zwischen der Variante „Vorgansimport“ und „Importiert Vorgänge bearbeiten“ liegt im wesentliche darin, dass bestimmte Felder in der Auswahlliste farblich markiert werden, wenn diese zu einem nicht erfolgreichem Anlegen des Vorgangs führen würden. Des Weiteren besteht die Möglichkeit in dieser Variante einzelne Position zu Bearbeiten.

Automatik

In der [Automatikschnittstelle](http://www.amic.de/ihilfe/#!XMLDocuments/AeinsInterface/html/M_JPL_JFunktionen_VimpErzeugeBelege_5_2981a6cf.htm) könne automatisierte Importprozesse realisiert werden, wie z.B. nächtliche Übernahmen von Tankbelegen oder automatisierte Übernahmen von EDI Ordersätzen.

Auswahlliste

In der Auswahlliste der Variante „Importierte Positionen bearbeiten“ können bestimmte Felder farblich markiert, wenn diese zu einem nicht erfolgreichen Anlegen des Vorgangs führen würden.

Folgende Bedeutung haben die farblich markierten Felder

| Feld | Farben Bedeutung |
| --- | --- |
| Status Stamm | 1. Weiß Bereit Vorgang kann erzeugt werden  
2. Grün Belegerzeugung oder Einspielung läuft  
3. Rot Es sind Fehler bei der Einspielung, beim Erzeugen aufgetreten oder der Stammsatz ist gelöscht worden.  
 |
| Kundnr./ Kunde | 1. Weiß Kunde kann gewählt werden  
2. Rot der Lieferant hat eine Bestellsperre für diesen Artikel |
| Artikel / Artikelnummer | 1. Artikel kann Bestellt werden  
2. Rot der Artikel hat eine Bestellsperre |
| Artikellieferant / Artikellieferant | |
| Status Position | 1. Weiß Bereit Vorgang kann erzeugt werden  
2. Grün Belegerzeugung oder Einspielung läuft  
3. Rot Es sind Fehler bei der Einspielung, beim Erzeugen aufgetreten oder der Positionssatz ist gelöscht worden.  
 |

Gleiche Funktionen in den Varianten „Vorgangsimport“ und „Importierte Positionen bearbeiten“

Daten einlesen(Vorg.Import)

Mit dieser Funktion können Vorgangsdaten die in einer Datei gespeichert sind eingelesen werden. Für das Einlesen und Speichern der Daten ist das Makro Vorgangeinlesen zuständig.

VIMP (Positionen)

Diese Funktion ruft eine weitere Auswahlliste auf, welche nur die Positionen der Vorgangsimport anzeigt.

VIMP (Text-Positionen)

Diese Funktion ruft eine weitere Auswahlliste auf, welche nur die Textpositionen zur dazugehörigen Position anzeigt.

Importumsetzer

Mit dieser Funktion wird der [Importumsetzer](../../../../externe_kommunikation/importumsetzer.md) aufgerufen.

Übernahmelöschen

Eine komplette Übergabe wird gelöscht unabhängig vom Status der Übergabe. Wenn zu dieser Übergabe schon ein umgewandelter Vorgang existiert, wird eine Warnung ausgegeben.

Fehlerrücksetzen

Ausgewählte Datensätze aus den Relationen [ImportVorgstamm](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgstamm.htm) und [ImportVorgPosition](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgPosition.htm)

mit dem Status 6 und 7 (Belegerzeugung läuft bzw. fehlerhaft bei Belegerzeugung)

werden auf den Status 5 (ist erfolgreich umgeschlüsselt) zurückgesetzt, damit sie z. B.

nach Anpassung der Stammdaten erneut in Belege umgewandelt werden können.

Belege mit Status 3 und 4 (Konvertierung läuft bzw. Fehler bei Konvertierung) werden auf

den Status 2 zurückgesetzt

Erledigte löschen

Hierbei wird das Löschkennzeichen für erledigte Vorgänge(Status 9) gesetzt.

Endgültig löschen

Alle Vorgänge aller betroffenen Relationen mit Status 9 (erledigt) werden gelöscht

Sonderfunktionen in der Variante „Importierte Positionen bearbeiten“

Standard Vorgänge erzeugen

Mit dieser Funktion können Vorgänge aus den Importierten Daten erzeugt werden. Welche Vorgänge erzeugt werden und mit welchen Inhalten ist [an dieser Stelle](./index.md#StandardVorgangErzeugen) beschrieben.

Ändern / Ansehen /Löschen

Mit dieser Funktion könne Daten auf der [Positionsebene](./funktionen_des_vimp_pflegers.md#PositionBearbeiten) bearbeitet oder gelöscht werden.

Richtlinien fürs Schreiben der Hilfe

Variante Vorgangsimportstatistik

 

<p class="siehe-auch">Siehe auch:</p>

- [Funktionen des VIMP-Pflegers](./funktionen_des_vimp_pflegers.md)
