# Archiv ändern (Ansehen)

<!-- source: https://amic.de/hilfe/archivvolltextrec.htm -->

In diesem Dialog besteht die Möglichkeit folgende Daten zu ändern bzw. einzusehen:

| Felder | |
| --- | --- |
| ***Belegreferenz*** | Die Kern-Identität des Archives.<br>Die Beleg-Referenz verknüpft u.a. den Archiv – Eintrag mit einem Archiv-Fakt (siehe auch [Archiv Fakt-Tabellen](../../technisches_zum_formulararchiv/archivdatenbank_extern_einrichten/archiv_fakt_tabellen.md))<br>Die Beleg-Referenz kann eine Art Klammer für „gleiche“ Archiv-Belege darstellen. Dieses Vorgehen wird auch empfohlen.<br>Für die Archiv – Fakten stehen Beleg-Generatoren (privatisierbare Datenbank-Funktionen) zur Verfügung (siehe [Referenz](../../archiv_manager/referenz/index.md)).<br>Obwohl die Beleg-Referenz im Einzelfall frei wählbar ist, ist aber auch zum Beispiel in Hinblick auf die Funktion ***Archiv anzeigen*** (siehe [Archiv Ansehen](../../archiv_ansehen/index.md)) eine gewisse Organisation der Referenzen anzuraten. Auch nehmen einige Programm-Module sehr wohl auf die konkrete Beleg-Referenz Bezug, so dass im Einzelfall von einer zu freizügigen Abänderung der Beleg-Referenz abgeraten wird. |
| ***Belegnummer*** | Beim Vorgangsdruck und anderen Archivierungen wo eine Zuordnung der Belegnummer möglich ist die Beleg-Nr. |
| ***Kundennummer*** | Zugeordnete Kundennummer |
| ***Belegklasse*** | Klassifizierung des Archiv-Eintrages nach Herkunft bzw. Wesen.<br>Beim Vorgangsdruck findet sich hier die Vorgangsklasse (400=Auftrag, 600=Lieferschein, 700=Rechnung, usw.).<br>Importe hinterlegen hier meist spezielle Belegklassen, z.B. 9000.<br>Für eigene Klassifizierungen stehen innerhalb der Belegklassen die Nummern 10000 bis 10003 zur Verfügung (User1 - User4). |
| ***Kontierung*** | Kennzeichen ob ein Beleg schon kontiert und das Ergebnis akzeptiert wurde. |
| ***Klassifizierung*** | Bietet neben der Belegklasse erweiterte private Klassifizierungsmöglichkeiten. |
| ***Barcode*** | Ähnlich der Beleg-Referenz bietet der Barcode eine Gruppierung des Archivs. Der Barcode hat aber spezialisierte Anwendungen, siehe hierzu [Archiv Barcode](../../archiv_barcode.md) |
| ***Mandant*** | Der Mandant des Archiv-Eintrages.<br>Wird gemäß der Regeln unter Archivieren (Mandant-Herkunft) vergeben. Das Feld ist nicht pflegbar. |
| ***Bedienerklasse*** | Zugeordnete Bedienerklasse |
| ***Datei*** | Beim „Dokument hinzufügen“ lässt sich hier der zu importierende Dateiname einpflegen.<br>Eine Dateiauswahlbox steht mit F3 zur Verfügung. |
| ***Belegfluss*** | |
| ***Belegtyp*** | Belegtyp |
| ***Betreff,***<br>***Kategorie,***<br>***Stichwörter,***<br>***Kommentar,***<br>***Titel,***<br>***Autor*** | Merkmale, die zur Verschlagwortung eines Archiv-Eintrages dienen können.<br> |
| ***Gruppe*** | [Formulararchiv Gruppe](../formulararchiv_gruppe.md) |
| ***Gruppentyp*** | [Formulararchiv Gruppe](../formulararchiv_gruppe.md) |
| ***Gruppentext*** | [Formulararchiv Gruppe](../formulararchiv_gruppe.md) |
| ***Gruppennr*** | [Formulararchiv Gruppe](../formulararchiv_gruppe.md) |
| ***Sortierung*** | [Formulararchiv Gruppe](../formulararchiv_gruppe.md) |
| ***Volltext*** | Hier befinden sich relevante Daten die sich auf den Volltext beziehen. |
| ***Zeitpunkt*** | Der Erstellungszeitpunkt des Volltextes. |

Im „Ändern“-Fall wird bei Archiv-Einträgen, die weder Kundennummer noch Beleg-Referenz haben, die Kundennummer bzw. Beleg-Referenz des aktuellen A.eins-Kontextes „vorbelegt“. Somit besteht eine einfache Möglichkeit der Zuordnung innerhalb eines „aufgesuchten“ Kontextes.

| Funktionen | |
| --- | --- |
| ***Archiv anzeigen*** **CF12** | Das Dokument des Archiv-Eintrages anzeigen |
| <strong><em><span style="FONT-SIZE: 11pt; COLOR: black">Beleg-Referenz erzeugen F10</span></em></strong><em><span style="FONT-SIZE: 11pt"></span></em> | Sind Belegnummer und Beleg-Klasse erfasst, bietet diese Funktion die Möglichkeit gemäß den Vorschriften aus [Archiv Fakte](../../archiv_fakte.md) (*fam_ref_vorg*) die Beleg-Referenz zu generieren. |

Das A.eins-Archiv unterstützt die Volltext-Recherche mit dem Modul „Volltextrecherche“ (Lizenz SPA Volltextrecherche, 914)

Es ist möglich die Volltext-Recherche auf jedem System zu testen. Ohne Lizenz sind so bis zu maximal 10 Archivtext-Einträge möglich.

Mit Installation dieser A.eins-Version befinden sich alle Relationen und technischen Voraussetzungen im System.

Hinweise zum u.a. Status der Lizenz erhält man unter [Archiv Volltext-Recherche](../../archiv_manager/referenz/archiv_volltext_recherche.md)

Das Volltextrecherche Relationsmodell stellt sich wie folgt dar:

| **Relation** | **Formulararchiv** | Hält Archiv-Rahmendaten vor |
| --- | --- | --- |
| Fa_id | Int | Schlüssel |
| FA_Mndnr | Int | Schlüssel |
| Fa_Guid | UniqueIdentifier (Guid) | Verweis auf Archiv-Dokument |

| **Relation** | **Archiv** | Hält Archiv-Dokumente vor |
| --- | --- | --- |
| Archiv_Guid | UniqueIdentifier (Guid) | Schlüssel |
| Archiv_Blob | Long binary | Dokument-Träger |

| **Relation** | **Archivtext** | Hält Archiv-Texte vor |
| --- | --- | --- |
| Archiv_Guid | UniqueIdentifier (Guid) | Schlüssel |
| Archiv_Text | Long varchar | Archivtext-Träger |
| Archiv_Time | TimeStamp | Zeitpunkt der Archiv_Text-Erstellung |

| **Relation** | **ArchivtextBad** | Hält Hinweise bei aufgetretenen Aktualisierungs-Problemen vor und unterstützt automatische Aktualisierungsprozesse bei der geeigneten Kandidatenauswahl |
| --- | --- | --- |
| Archiv_Guid | UniqueIdentifier (Guid) | Schlüssel |
| Fehlprotident | UniqueIdentifier (Guid) | Fehlerprotokoll/Systemhinweise-Verweis |

Um den administrativen Aufwand hinsichtlich konkreter Probleme im Rahmen der Aktualisierung zu erleichtern wurde die Variante „Archivtext“ in der Anwendung „Fehlerprotokoll/Systemhinweise“ [**FEHLP**] eingeführt.

Der Volltext-Index „ArchivtextIndex“ operiert auf Relation/Spalte : „Archivtext“, „Archiv_Text“

Hinweise zum u.a. Status einiger Relationen erhält man unter [Archiv Volltext-Recherche](../../archiv_manager/referenz/archiv_volltext_recherche.md)

| ATF | |
| --- | --- |
| 630705 | Anlage Archivtext |
| 630791 | Initialisierung ArchivText-Volltext-Index im System-Datenspeicher mit Default-Werten und manueller Aktualisierung |
| 630932,630942 | Administratives ArchivTextBad |

Folgende Funktionen stehen zur neben der eigentlichen Recherche zur Verfügung:

| Archivtext-Funktion | | |
| --- | --- | --- |
| Volltext anzeigen | Anzeige des Volltextes im Notepad | Dient zur Einsicht und Überprüfung des erzeugten Archivtextes. |
| Volltext aktualisieren | Initiiert die Volltext-Aktualisierung für den gewählten Archiveintrag.<br>Dabei können in aller Regel auch mehrere verarbeitet werden. (\*) | Bietet die Möglichkeit neben oder statt der automatischen Aktualisierung Archivtexte zu aktualisieren. |
| Volltext editieren | Anzeige und Änderungsmöglichkeit des Archivtextes im Notepad. (\*) | Es könnte von Nöten sein das Ergebnis der Aktualisierung zu ändern bzw. zu korrigieren. |
| Volltext löschen | Löscht den Archivtext und ggf. Hinweise in „ArchivtextBad“ | Damit wird u.a. dieser Eintrag für die Neubearbeitung durch die automatische Aktualisierung verfügbar |

(\*) hierbei erfolgt auch eine automatische Aktualisierung des Archivtext-Indexes. Der Archivtext-Index kann ebenso mittels Direktsprung [**VTRU**] aktualisiert werden.

Die Aktualisierung des Archivvolltextes erfolgt nicht vollautomatisch.

Das bedeutet insbesondere dass beim Erzeugen eines neuen Archiveintrages nicht automatisch auch der Archivtext erzeugt wird.

Es bedeutet vielmehr das ein Prozess ins System integriert werden muss der die Aktualisierung der Archivtexte durch A.eins veranlasst.

Eine Möglichkeit ist die Integration der Aktualisierung im Mandantenserver mit Hilfe der Relation „MandserProzesse“:

| Feld | Vorgeschlagener Wert |
| --- | --- |
| Mp_Aktiv | 1 |
| Mp_Controlstring | ^jpl volltextrecherche progress |
| Mp_Hidden | 0 |
| Mp_Id | (Die muss eruiert werden) |
| Mp_Intervallsekunden | 10 |
| Mp_Name | Volltextaktualisierung |
| Mp_Nurwartezyklus | 1 |

Hinweise bzgl. Möglicher Privatisierungen der Datenbank-Funktion die den nächsten zu bearbeitenden Kandidaten für die Aktualisierung heraussucht erhält man unter [Archiv Volltext-Recherche](../../archiv_manager/referenz/archiv_volltext_recherche.md)

[Was ist eine Volltextsuche?](http://dcx.sybase.com/index.html#1201/de/dbusage/full-text-search-what-is-it.html)

Die Volltextsuche stellt eine erweiterte Methode zum Durchsuchen einer Datenbank dar. Die Volltextsuche findet schnell alle Instanzen eines Begriffs (Worts) in einer Tabelle, ohne die Zeilen durchsuchen zu müssen und ohne wissen zu müssen, in welcher Spalte ein Begriff gespeichert ist. Bei der Volltextsuche werden **Textindizes** verwendet. Ein Textindex speichert Positionsinformationen für alle in den Spalten, für die Sie den Textindex erstellt haben, gefundenen Begriffe. Die Verwendung eines Textindexes kann bei der Suche nach Zeilen, die einen bestimmten Wert enthalten, gegenüber der Verwendung eines normalen Indexes schneller sein.

Die Funktion der Volltextsuche in SQL Anywhere unterscheidet sich von einer Suche mit Prädikaten wie LIKE, REGEXP und SIMILAR TO, weil die Übereinstimmung begriffsbasiert und nicht musterbasiert ist.

Zeichenfolgenvergleiche in der Volltextsuche verwenden alle üblichen Kollatierungseinstellungen für die Datenbank. Wenn die Datenbank beispielsweise dahingehend konfiguriert ist, die Groß-/Kleinschreibung nicht zu berücksichtigen, wird sie auch von der Volltextsuche nicht berücksichtigt.

Abgesehen von den angeführten Ausnahmen nutzt eine Volltextsuche alle internationalen Funktionen, die von SQL Anywhere unterstützt werden.

Die Volltextrecherche steht in der **Dokumentenverwaltung** ([**ODRNER**], **Strg-F12** und [**MYAR**]) und den Anwendungen „**Formulararchiv**“ [**FA**] und „**Formularchiv Administration**“ [**FAAD**] zur Verfügung.

Sie folgt den durch die [Sybase-Volltextreche-Regeln](http://dcx.sybase.com/index.html#1201/de/dbusage/ug-queries-s-5898110.html), siehe auch [hier](http://dcx.sybase.com/index.html#1201/de/dbreference/contains-search-condition.html):

| Einige ausgewählte Beispiele | |
| --- | --- |
| Beispiel | Sucht nach „Beispiel“ |
| Beispiel1 &#124; Beispiel2 | Sucht nach „Beispiel1“ oder „Bespiel2“ |
| Beispiel1 –Beispiel2 | Sucht nach Zeilen die den „Bespiel1“ enthalten, aber nicht „Beispiel2“ |
| Bei\* | Sucht nach allen Begriffen die mit Bei beginnen |

Die angebenen Links geben sehr viele weitere – teils hochkomplexe – Abfrage-Beispiele an bzw. vor.
