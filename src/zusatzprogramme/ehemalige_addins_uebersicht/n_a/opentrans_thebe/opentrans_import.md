# openTRANS-Import

<!-- source: https://amic.de/hilfe/_info_otimport.htm -->

Der Import von openTRANS ist vielfältig. Deshalb wird dieser durch Makros individuell gestaltet. Das AddIn Thebe bereitet lediglich die Bearbeitung vor. Im Folgenden ist die Einrichtung und die Verarbeitung beschrieben.

###### Einrichtung

Warenverkauf > openTRANS Import > Variante „Profile für den Importbereich“

| Pfleger des openTRANS-Importprofils |
| --- |
| Profilname | Name des Profile |
| Klasse | Vorgangsklasse |
| Unterklasse | Vorgangsunterklasse |
| Absendertyp | Welchen Typ hat der im Dokument beschriebene Absender |
| Importlager | Lager auf das importiert wird |
| Makro | Makro, das zum Import aufgerufen wird Das Makro wird von einer Funktion in der Variante „Dokumentenverarbeitung“ benutzt. (siehe [Import per Makro](./opentrans_import.md#EAI_OpenTRANS_Thebe_Import_Makro)) |
| Gebinde nicht exportieren | |

Beim Speichern eines neuen Profils wird eine Funktion in der Optionbox der Variante „Dokumentenverarbeitung“ angelegt, die den Import von Dokumenten mit Hilfe dieses Profils startet.

###### Verarbeitung

Quellen

Die openTRANS-Dokumente können aus verschiedenen Quellen gewonnen werden. Je nach Typ können sie erst nach Extrakt oder sofort weiterverarbeitet werden. In jedem Fall werden die Dateien zunächst ins Formulararchiv importiert und mit einer entsprechenden Belegklasse versehen.

| Belegklassen für openTRANS im Formulararchiv |
| --- |
| Typ | Belegklasse | Verarbeitung |
| E-Mail mit Anhang | 8031 – openTRANS unbearbeitet<br> | Mit Extraktion |
| PDF mit openTRANS-Anhang |
| openTRANS-XML-Datei | 8032 – openTRANS extrahiert | |

Warenverkauf > openTRANS Import > Variante „Dokumentenverarbeitung“

Schritt 1 : Extraktion

Beginnen Sie zunächst mit dem Status „extrahierbare“ Dokumente.

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8031 – openTRANS unbearbeitet“. Dies können eMails oder PDF-Dokumente mit eingeschlossenen Anhängen sein. Diese müssen zunächst ins Archiv extrahiert werden.

Verwenden Sie die Funktion „Extrahieren“, um die markierten Einträge zu entpacken.

Schritt 2 : Import per Makro

Wechseln Sie nun in den Status „zur Verarbeitung“ anstehende Dokumente

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8032 – openTRANS extrahiert“.

Dies können extrahierte E-Mails, PDF-Dokumente und deren eingeschlossenen Anhänge sein.

Alle extrahierten Anhänge heben die gleiche Archivreferenz, wie das Ursprungsdokument.

Verwenden Sie die Funktion „Import ….“, um das zum jeweiligen Profil gehörenden Makro zu starten.

Hinweis:

Es werden alle Dokumente der gleichen Belegreferenz importiert !

Ansicht verarbeiteter Dokumente

Wechseln Sie nun in den Status „verarbeitet“ .

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8033 – openTRANS verarbeitet“.

Hier werden alle Einträge aus dem Formulararchiv angezeigt, die mit Hilfe des Importmakros bearbeitet wurden.
