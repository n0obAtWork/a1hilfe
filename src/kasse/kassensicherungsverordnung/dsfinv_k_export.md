# DSFinV-K Export

<!-- source: https://amic.de/hilfe/_dsfinvkexport.htm -->

Hauptmenü > Barvorgänge > Stammdaten > DSFinV-K Export

Allgemein

DSFinV-K ist die Digitale Schnittstelle der Finanzverwaltung für Kassensysteme.

Dies ist die Taxonomie, nach der die Transaktionsdaten der Kassen und Aufzeichnungssysteme einheitlich gespeichert werden müssen. Die einheitliche Speicherung ermöglicht den Finanzbehörden eine tiefergehende und strukturierte Prüfung der Kassenvorgänge, als dies in der Vergangenheit der Fall war. Dies impliziert, dass das Finanzamt nicht lediglich die manipulationsfreie Nutzung der Registrierkasse überprüfen kann, sondern durch die im DSFinV-K Format strukturierten Daten auch die korrekte Verbuchung von Geschäftsvorfällen, wie z. B. Trinkgeld, überprüfen kann. Insofern geht die Kassensicherungsverordnung weit über die Absicherung von Bargeldumsätzen hinaus. +

Der Steuerpflichtige muss einen DSFinV-K Export jederzeit für eine Prüfung durch die Finanzbehörde zur Verfügung stellen. Der DSFinV-K Export knüpft an den GoBD-Export an, ist jedoch einheitlich strukturiert und deutlich umfangreicher. Der GoBD-Export reicht also ab dem 1.1.2020 nicht mehr aus, um die steuerlichen Anforderungen zu erfüllen.

Ziele der DSFinV-K

Ziel der Standardisierung ist die Definition einer Struktur für Daten aus Kassensystemen, für die ab dem 01.01.2020 die Nutzung der gesetzlich geforderten einheitlichen digitalen Schnittstelle (§ 146a Abs. 1 S. 4 AO) gilt. Durch die Standardisierung sollen folgende Ziele abgedeckt werden:

• Einheitliche Datenbereitstellung für die Außenprüfung sowie für Kassen-Nachschauen durch definierte Kasseneinzelbewegungen, Stammdaten und Kassenabschlüsse, so dass eine progressive und retrograde Prüfbarkeit zwischen den Grundaufzeichnungen und der Erfassung im Hauptbuch (Finanzbuchhaltung) gewährleistet ist.

• Ermöglicht die Auslagerung aller im jeweiligen System erfassten Daten in ein Archivsystem.

• Ermöglicht eine vereinfachte Überprüfung der in die Finanzbuchhaltung übertragenen strukturierten Kassendaten.

Hierfür liefert die DSFinV-K eine fachliche und technische Beschreibung. Die DSFinV-K entspricht inhaltlich der „DFKA-Taxonomie Kassendaten“. Sofern der Standard „DFKA-Taxonomie Kassendaten“ (Datensatzbeschreibung im json-Format, der u. a. vom Deutschen Fachverband für Kassen- und Abrechnungssystem-technik e.V. entwickelt wurde) zur Übermittlung der Kassendaten an die Finanzbuchhaltung genutzt wird, ist eine Konvertierung der Daten für Zwecke der Außenprüfung oder der Kassen-Nachschau zwingend erforderlich (vom originären json-Format in csv-Dateien mit beschreibender ***index.xml***; vgl. Anhang **G**).

Mittels dieses Datenformates ist ein Import der Kassendaten in IDEA einheitlich möglich.

Die Bereitstellung dieser konvertierten Daten ist Aufgabe des jeweiligen Steuerpflichtigen. Umfang der DSFinV-K. Der nachfolgende Datenkranz beschreibt den Mindestumfang für eine standardisierte Datenaufbereitung und einen möglichen Prüfungseinstieg für die Finanzverwaltung. Eine abschließende Aufzählung der für Zwecke der Außenprüfung oder der Kassen-Nachschau vorzuhaltenden Daten aus elektronischen Kassensystemen kann z. B. schon deshalb nicht erfolgen, weil nicht im Datenkranz enthaltene systemspezifische Datenfelder dazu führen könnten, dass sich Geschäftsvorfälle nicht sinnvoll und nachvollziehbar abbilden lassen.

Für die Nachvollziehbarkeit der Daten notwendige systemspezifische Zusatzinformationen sind in der jeweiligen CSV-Datei als zusätzliches Datenfeld am Zeilenende anzufügen.

Dabei ist zu beachten, dass die erforderliche Anpassung in der index.xml vorgenommen werden muss (Definition zusätzlicher Felder). Darüber hinaus enthält die DSFinV-K die für eine Prüfung notwendigen, sich aus der Kommunikation mit der TSE ergebenden Daten.

Die DSFinV-K nutzt zur besseren Lesbarkeit Begrifflichkeiten aus dem Handel. Dienstleistungen sind innerhalb der vorgegebenen Struktur entsprechend abzubilden.

DSFinV-K

<details>
<summary>Register Export</summary>

| Feld | Beschreibung |
| --- | --- |
| Kassennummer | Die A.eins-Kassennummer mit Bezeichnung |
| Filialnummer | Die Kassennummer ist immer die Nummer des aktuellen Mandanten.<br>Die Filialnummer gibt in Replikationsumgebungen die aktuelle Filialnummer an. Diese ist durch den Mandanten vorgegeben und an dieser Stelle auch nicht frei auswählbar.<br>Die Kassennummer- und die Filialnummer-Datenherkunft stammt aus der replizierten bzw. einzelgepflegten Tabelle ***AcshStammDatenLink.***<br>Die Tabelle ***AcashStammdatenLink*** ist notwendig, da der Kassenstamm ***AcashStdKsse*** noch nie in einer Replikation war. |
| Kassenseriennummer | Die Kassenseriennummer, die für die Kasse verwendet wird.<br>Die Kassenseriennummer wird von AMIC in Abhängigkeit der gekauften Kassenlizenzen generiert.<br>Bei Meldung der Kasse an das Finanzamt, wird diese Seriennummer benötigt. |
| Kassensitzungsnummer | Die zur Kassennummer passende und gewählte Kassensitzungsnummer. |
| Filialnummer | Die hinterlegte Filialnummer der Kassensitzungsnummer. |
| Kassenabschluss | Der Kassenabschluss-Zeitpunkt der Kassensitzungsnummer. |
| Export-Verzeichnis | Das Verzeichnis in den die amtlichen Daten des DSFin-V_K hinterlegt werden. |

Hinweis:

Zurzeit ist die Kassennummer das Kriterium, dass die TSE vorgibt.

Somit ist dringend geraten bei TSE-Ersetzungen notwendig mit ***Speichern unter…*** in der TSE-Einrichtung zu arbeiten.

Nur das gewährleistet, dass auch alle Kassenvorgänge zum Export gefunden werden können. Sobald uns nähere Informationen zum konkreten Ablauf einer Kassenprüfung vor Ort vorliegen, ist eine Änderung auf die eindeutige Kassenfinanznummer denkbar.

</details>

<details>
<summary>Aufbereitung</summary>

Beschreibt Parameter zur Aufbereitung der DSFinV-K-Daten.

Die Daten in den Tabellen DSFinV-K[CSV-Dateiname] werden durch die passende Aufbereitungs-Prozedure DSFinV_K_[CSV-Dateiname]__Load aufbereitet.

Jede einzelne der Standard-Auslieferungsprozeduren kann privatisiert werden.

| Feld | Beschreibung |
| --- | --- |
| CSV-Dateiname | Nach DSFin-V_K sind die CSV-Dateinamen vorgegeben. |
| Aufbereitungsprozedure | Der Name der zugehörigen Aufbereitungsprozedure. Diese Prozeduren können jeweils privatisiert werden. Der Name der Privatisierung ist auch fest vorgegeben. |

Hinweis:

Die ***Privatisierung*** ist für eine Übergangszeit (der Produkteinführung) hauptsächlich für den AMIC-Support und – Entwicklung ein Mittel vor Ort bei bestimmten Datenzusammenstellungen schnell und direkt reagieren zu können.

Zu beachten ist, dass bei einem Programmupdate die Privatisierung Vorrang hat (das ist nicht immer gewollt, und deswegen wird das Feature in kommenden Versionen abgekündigt).

</details>

<details>
<summary>Infodaten des Exports</summary>

| Feld | Beschreibung |
| --- | --- |
| Anzahl Privatisierungen | Auf der Registerkarte **Aufbereitung** sind die möglichen Privatisierungen der Aufbereitungsprozeduren der CSV-Tabellen pflegbar.<br>Auf dieser Registerkarte haben Sie schnell eine Übersicht, ob und wie viele Privatisierungen vorliegen. |
| Taxonomie-Version | Die offizielle Taxonomie-Version, die durch den in A.eins implementierten SDFin-V_K-Export realisiert wird.<br>Diese Nummer wird durch das jeweilige A.eins-Release bestimmt, lässt sich durch die private Aufbereitungsprozedure ***p_DSFinV_K_cashpoint_closing_Load*** anpassen, wenn notwendig. |

</details>

<details>
<summary>Funktionen des Exports</summary>

| Kopfdaten | Beschreibung |
| --- | --- |
| Export Erzeugen **(F10)** | Generiert die Dateien und öffnet den Explorer.<br> <br>**Das Export-Verzeichnis ist fest vorgegeben. Vor jedem Export wird das Verzeichnis komplett gelöscht.** |
| Export TAR Zeitraum … | Hinweis:<br>Diese Funktion sollte nach der ***Export Erzeugen***\-Funktion aufgerufen werden, da das Verzeichnis gelöscht wird.<br><br>Die Vorbelegung für die Zeiträume ist wie folgt:<br>**Von:** 1 Minute vor der zugehörigen Kasseneröffnung<br>**Bis:** 1 Minute nach dem zugehörigen Kassenabschluss.<br><br>Die erzeugte Tar-Datei enthält dann die zum Export-Zeitraum der Buchungen zugehörigen Transaktionen.<br>In welchen Umfang das Programm bei TSE unterstützen kann, die nicht mehr im Zugriff sind (**Speichern unter**\-**Historie**) steht noch nicht fest, dafür werden u. U. organisatorische Maßnahmen notwendig sein.<br><br>Das Verzeichnis ist mit dem des DSFinV-K-Exports identisch (..\\export\\dsfinv-k).<br><br> |

</details>
