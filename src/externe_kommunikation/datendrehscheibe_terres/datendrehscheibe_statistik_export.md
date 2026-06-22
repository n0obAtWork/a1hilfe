# Datendrehscheibe Statistik Export

<!-- source: https://amic.de/hilfe/_terres_statistik.htm -->

Hauptmenü > Externe Kommunikation > Datendrehscheibe > Statistikexport [TERRS]  
    

In dieser Anwendung kann die Statistik für die einzelnen Perioden an Terres übermittelt werden.

Dazu wird in der Variante „Terresstatistik Export“ der Statisikexport [F9] aufgerufen. Die Statistik wird als csv exportiert. Der Name der Datei enthält die Periode und das Jahr z.B. TerresStatistikExport_2012_12.csv.

Bevor die Statistik übermittelt werden kann, müssen auf der Registerkarte Optionen folgende Einstellungen vorgenommen werden.

Registerkarte Statistikexport

Wenn die Statistik manuell exportiert werden soll, so kann in das Feld manuell das Jahr und die Periode eingetragen werden. Es ist möglich eine Statistik für eine Periode mehrfach zu übertragen.

In der unteren Tabelle wird angezeigt welche Statistik für welche Periode schon übermittelt worden ist.

Registerkarte Optionen

In dem Feld „Statistik Export“ die Prozedur angegeben, die die Statistik erstellt. Diese Prozedur kann privatisiert werden. Das Feld wird mit der Standardprozedur „TerresdatenexportStatistik“ vorbelegt.

In dem Feld „Pfad zur Ausgabe der Exportdatei“ wird der Pfad angegeben wohin die Datei mit den Statistikdaten gespeichert wird.

Achtung der Pfad muss relativ zum Datenbankserver liegen. Der Pfad wird vorbelegt mit dem Export Verzeichnis von A.eins, wenn dieses sich Anhand der Logdatei der Datenbank ermitteln lässt. Dafür muss die Datenbankeigenschaft consolelogfile gesetzt sein. Kann der Pfad nicht vorbelegt werden, so muss ein Pfad eingetragen werden.

Ablauf

Die Statistik kann manuell übertragen werden oder per Event als automatischer Lauf.

Bei der manuellen Übertragung ist es möglich für eine Periode den Export mehrfach anzustoßen, dazu sind die Felder Periode und Jahr auf der Maske „Datendrehscheibe Statistikexport“ anzugeben. Mit Statistikexport Starten [F9] wird der Export gestartet.

Bei der Übertragung per [Event](../../zusatzprogramme/events/datendrehscheibe_statistikexport.md) wird die letzte geschlossen Periode übermittelt, die noch nicht exportiert worden ist. Dazu wird der Prozedur in den Parametern in_Jahrnummer und in_perinummer jeweils die 0 übergeben.

Prozedur

Folgende Prozedur „TerresdatenexportStatistik“ erzeugt die Statistik, welche exportiert warden kann.

Felder in der CSV des Statistiksexports

| ![\*](../../ImagesExt/image8_1556.jpg "*") Feldnamen | ![\*](../../ImagesExt/image8_1556.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Kd-Nr-AGRAVIS | ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferant des Artikels |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") Terres Artikelnummer. Nicht die A.eins spezifische Artikelnummer |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Jahr | ![\*](../../ImagesExt/image8_1556.jpg "*") Für welches Jahr gilt die Statistik |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Monat | ![\*](../../ImagesExt/image8_1556.jpg "*") Für welche Periode gilt die Statistik |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lagerort | ![\*](../../ImagesExt/image8_1556.jpg "*") Lagernummer |
| ![\*](../../ImagesExt/image8_1556.jpg "*") ArtikelObergruppe | ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelobergruppe von dem Terres Artikel |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelgruppe | ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelgruppe von dem Terres Artikel |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikeluntergruppe | ![\*](../../ImagesExt/image8_1556.jpg "*") Artikeluntergruppe von dem Terres Artikel |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Einzelhandelsartikel Eigener Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") Einzelhandelsartikel |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Einzelhandelsartikel Standardartikel | ![\*](../../ImagesExt/image8_1556.jpg "*") Einzelhandelsartikel |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lose Ware | ![\*](../../ImagesExt/image8_1556.jpg "*") Lose Ware |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Kostenstelle | ![\*](../../ImagesExt/image8_1556.jpg "*") Wird im Moment nicht benötigt und kann mit 0 befüllt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Strecke | ![\*](../../ImagesExt/image8_1556.jpg "*") 0 |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Verkauf (Menge) | ![\*](../../ImagesExt/image8_1556.jpg "*") Verkaufte Menge des Artikels |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Verkauf (MW) | ![\*](../../ImagesExt/image8_1556.jpg "*") Betrag |
| ![\*](../../ImagesExt/image8_1556.jpg "*") DB (MW) | ![\*](../../ImagesExt/image8_1556.jpg "*") Deckungs Beitrag |
| ![\*](../../ImagesExt/image8_1556.jpg "*") W„hrung (MW) | ![\*](../../ImagesExt/image8_1556.jpg "*") EUR |
| ![\*](../../ImagesExt/image8_1556.jpg "*") G-TIN | ![\*](../../ImagesExt/image8_1556.jpg "*") Globale Artikelnummer |
