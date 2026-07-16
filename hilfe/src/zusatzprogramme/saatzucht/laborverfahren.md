# Laborverfahren

<!-- source: https://amic.de/hilfe/_laborverfahren.htm -->

Hauptmenü > Saatzucht > Saatenlabor > Verfahren

oder Direktsprung **[LABVE]**

In diesem Stammdatenpfleger werden die Daten über Laborverfahren gepflegt. Der Einrichterparameter „[Erweiterte Einstellungen](../../firmenstamm/einrichterparameter/laborverfahren_epa_laborverfahren.md)“ erlaubt weitere Eingabemöglichkeiten auf der Maske.

### Erfassungsmaske

Es stehen folgende Eingabefelder und Eingabemöglichkeiten zur Verfügung.

| Name | Bedeutung |
| --- | --- |
| Verfahrensnummer | Eindeutige Nummer des Laborverfahrens.<br> |
| Detailprüfung | Art des Verfahrens. Eine Auswahl der möglichen Verfahren ist mit F3 möglich. Bei Eingabe des Verfahrens wird die Karteikarte (Registerkarte des Pflegers für Labordaten) gleich korrekt vorbelegt.<br> |
| Bezeichnung | Bezeichnung des Verfahrens. Dies wird als Überschrift der Box auf dem Pfleger der Labordaten verwendet.<br> |

#### Felder auf der Registerkarte Allgemein

| Name | Erweitere Einstellung | Bedeutung |
| --- | --- | --- |
| Druckoptionen | Ja | Hier werden Etikettennamen, die über den [AMIC-Etikettendruck](../amic_etikettendruck/index.md#Amic_Etikettendruck) definiert werden müssen, und die Anzahl der Kopien eingetragen, die für dieses Verfahren gedruckt werden sollen. Der Druck dieser Etiketten geschieht in der Anwendung Labor über die Funktion „Drucke Untersuchungsetiketten“.<br>Im A.eins-System existieren keine Standardvorlagen für die Etiketten. Diese müssen vor Ort entwickelt werden. Um die Daten zu identifizieren wird die aktuelle Qualitaetsid vor dem Aufrufen des jeweiligen Etikettes der Variable „LDB_TRANSFER$I4“ zugewiesen. LDB_TRANSFER$N0 wird die Nummer des Verfahrens zugewiesen. Diese Variable kann dann beim AMIC Etikettendruck verwendet werden.<br>Beispiel(siehe Prozedur Name):<br>![](../../ImagesExt/image8_1240.png) <br> <br>Die Prüfberichte werden in der Tabelle „Verfahrenetiketten“ gespeichert.<br> |
| Verfahrens Prozedur | Nein | Hier kann der Name einer zu hinterlegenden Prozedur angegeben werden.<br> |
| Kartenbezeichnung | | Die hier eingegebene Bezeichnung wird der Titel der Registerkarte.<br> |
| Kurzbezeichnung | | Die Kurze Bezeichnung des Verfahrens wird u.a. in einigen Varianten dazu verwendet, um zu den Labordaten die Verfahren aufzulisten.<br>![](../../ImagesExt/image8_1241.png)<br> |
| Karteikarte | | Es sind die Registerkarten des Pflegers für Labordaten, die bei der Auswahl der Detailprüfung bereits korrekt vorbelegt werden.<br> |
| Bemerkung | Ja | Hier wird eine Bemerkung für das Verfahren eingetragen.<br> |
| Druck | | Der Druckstatus wird über das Anwenderformat „af_kfdruck“ bestimmt.<br> |
| Firma | Ja | Die Firma kann mit F3 ausgewählt werden.<br> |
| Waageterminal | | Ermöglicht die Zuordnung einer [Waage](../../waagenanbindung/waagenterminals/maske_waagenprofil/index.md) zu den Verfahren „Reinheit“ und „Besatz“. Bei anderen Verfahren wird dieses Feld ausgeblendet.<br> |

#### Felder auf der Registerkarte „Felder“

| Name | Bedeutung |
| --- | --- |
| Benutzte Felder | Hier werden die Felder angezeigt, die in den Labordaten verwendet werden sollen. Mithilfe der Pfeiltasten auf der Maske können sie zu den „vorhandenen“ Feldern verschoben werden.<br> |
| Startfeld | Wird zurzeit nicht verwendet.<br> |
| Restfeld Behandlung | Hier kann angegeben werden, was mit den nicht benutzen Feldern in der Spalte „Vorhandenen Felder“ geschehen soll.<br><ul><li>egal&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Es werden nach wie vor alle Felder angezeigt und sind auch änderbar.</li><li>schützen &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Die Felder werden angezeigt, können jedoch nicht geändert werden.</li><li>verstecken&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Die Felder werden ausgeblendet.<br>&nbsp;</li></ul> |
| Vorhandende Felder | Hier wird eine Auswahl an Feldern angezeigt, die in den Labordaten für das betreffende Verfahren verwendet werden können. Sollen die Felder in den Labordaten verwendet werden, können sie mithilfe der Pfeiltasten zu dem Feld „Benutzte Felder“ verschoben werden.<br><br> |

#### Felder auf der Registerkarte Keimfähigkeit

Das Register **Keimfähigkeit** wird für die Detailprüfungen „Keimfähigkeit“, „Keimfähigkeit gebeizt“, „Keimfähigkeit ungebeizt“, „Triebkraft gebeizt“, „Triebkraft ungebeizt“, „Lufa“, „HLG“ und „Feuchte“ eingeblendet.

| Name | Bedeutung |
| --- | --- |
| Behandlung und Menge | Die Behandlung wird über das Anwenderformat „AF_BEHANDLUN“ erfasst. Im folgenden Feld wird die Menge zur Behandlung eingetragen. Eine Auswahl ist mit F3 über das Anwenderformat „AF_BEHAMENGE“ möglich.<br> |
| Medium | Medium wird über das Anwenderformat „AF_MEDIUM“ gesteuert.<br> |
| Körner | Hier werden die Anzahl der Körner und die Anzahl der Wiederholungen eingetragen.<br>Steht der Einrichterparameter „Erweiterte Einstellungen“ auf „Nein“, so können die Anzahl an Körner mittels Anwenderformat „AF_Koerner“ vorbelegt werden.<br> |
| Vorkühlung | Dauer der Vorkühlung in Tagen.<br> |
| Temperatur | Temperatur der Vorkühlung.<br> |
| Keimtage | Hier werden die Keimtage eingetragen. Vorbelegung über Anwenderformat „AF_KEIMTAGE“.<br> |
| Temperatur | Hier wird die Keimtemperatur eingetragen. Vorbelegung über Anwenderformat „AF_KEIMTEMP“.<br> |
| Abfragen | Hiermit wird gesteuert, ob die oben vorbelegten Daten im Pfleger der Labordaten noch geändert werden können.<br> |

### Feuchte Grunddaten

Die Felder zu „Feuchte Grunddaten“ sind nur verfügbar, wenn der Einrichterparameter „Erweiterte Einstellungen“ auf „Ja“ steht.

| Name | Bedeutung |
| --- | --- |
| Schroten | Folgende Ausprägungen sind möglich.<br><ul><li>Nein</li><li>Grob</li><li>Fein<br>Die Ausprägungen sind im Anwenderformat AF_FESCHROTE hinterlegt und können erweitert werden.<br>&nbsp;</li></ul> |
| Dauer | In dem Feld Dauer wird die Anzahl der Stunden eingetragen. Diese sind in dem Anwenderformat „AF_FEDAUER“ hinterlegt.<br> |
| Temperatur | In diesem Feld wird die Temperatur eingetragen. Folgende Ausprägungen sind möglich<br><ul><li>Niedrig (101-105°C)</li><li>Hoch (130-133)<br>Die Daten sind im Anwenderformat „AF_FETEMP“ hinterlegt und können erweitert werden.<br>&nbsp;</li></ul> |

### Hohlmaß Grunddaten

Die Eingabemöglichkeiten für Hohlmaß sind nur verfügbar, wenn der Einrichterparameter „Erweiterte Einstellungen“ auf „Ja“ steht.

| Name | Bedeutung |
| --- | --- |
| Hohlmaß | In diesem Feld kann das Hohlmaß hinterlegt werden. Das Hohlmaß wird im Anwenderformat „AF_LABHOHLM“ gespeichert.<br> |

### Lufa Grunddaten

Die Eingabemöglichkeiten für Lufa Grunddaten sind nur verfügbar, wenn der Einrichterparameter „Erweiterte Einstellungen“ auf „Ja“ steht

| Name | Bedeutung |
| --- | --- |
| Institut | Hier wird das Auftragslabor aus dem Kundenstamm eingetragen.<br> |
| Menge | Hier wird die zu untersuchende Masse mit Mengeneinheit eingetragen.<br> |
| Beschreibung | Hier werden die Inhaltsstoffe der Untersuchung eingetragen wählbar aus den Artikelbestandteile[ABST.]<br> |
| Vergleich | Größer/Kleiner/-<br> |
| Standartwert | Hier werden die Standardgrenzwerte der Inhaltsstoffe eingetragen. Vorbelegt aus Artikelbestandteile.<br> |
| ME | Mengeneinheit der Inhaltsstoffe. Vorbelegt aus Artikelbestandteile. Auswählbar über das Format „AF_LUFAME“<br> |

### E-Mail Laborleitung

| Name | Bedeutung |
| --- | --- |
| Laborleitung | Hier kann eine Liste von E-Mail Adressen eingetragen werden. Dieses Feld wird im Standard nicht ausgewertet<br> |
| E-Mail Text | Hier kann ein vorgefertigter Text eingetragen werden. Dieses Feld wird im Standard nicht ausgewertet.<br> |

#### Felder auf der Registerkarte Vermehrung

Diese Registerkarte wird für die Detailprüfung „Vermehrungen“ eingeblendet.

| Name | Bedeutung |
| --- | --- |
| Prozedur Schläge | Hier wird die Prozedur für „Schläge“ eingetragen.<br> |
| Prozedur Vermehrer | Hier wird die Prozedur für „Vermehrer“ eingetragen.<br> |

#### Felder auf der Registerkarte Besatzarten

Die Registerkarte „Besatzarten“ ist nur verfügbar, wenn der Einrichterparameter „Erweiterte Einstellungen“ auf „Ja“ steht und die Detailprüfung auf „Besatz“ oder „Reinheit“ steht.

| Name | Bedeutung |
| --- | --- |
| Untersuchungsmenge | Hier kann dem Verfahren eine Untersuchungsmenge für die Besatzarten zugeordnet werden.<br> |
| Besatzart | Angabe der Besatzarten, die in dem Verfahren geprüft werden sollen. [Besatzarten](./besatzarten.md) können unter dem Direktsprung [SAATA] gepflegt werden.<br> |
| Bezeichnung | Bezeichnung der Besatzart.<br> |
| max. % | Hier kann der Grenzwert für die jeweilige Besatzart in Prozent eingetragen werden. Dieser Grenzwert dient als Vorgabe für die Labordaten.<br> |
| max. Anzahl | Grenzwert für die Anzahl an Samen für die betreffende Besatzart. Dieser Grenzwert dient als Vorgabe für die Labordaten.<br> |
| Grp. | Jede Besatzart kann zu einer Besatzartgruppe zugeordnet werden. Es kann zwischen<br><ul><li>Kultur (Kulturart)</li><li>Unkraut (Wildart)<br>unterschieden werden. Die Vorbelegung erfolgt über das Anwenderformat „AF_BESATZART“.<br>&nbsp;</li></ul> |

#### Felder auf der Registerkarte Merkmale

Die Registerkarte „Merkmale“ wird nur angezeigt, wenn der Einrichterparameter „Erweiterte Einstellungen“ den Wert „Ja“ hat und die Detailprüfung auf „Kontrollanbau“ oder „Markeranalyse“ steht

### Feldversuch

| Name | Bedeutung |
| --- | --- |
| Nummer | Hier können das Labor bzw. das Institut für den Feldversuch aus dem Kundenstamm angegeben werden.<br> |
| Anschrift | Die Hauptanschrift des Labors bzw. des Instituts.<br> |
| Menge | Menge für den Feldversuch.<br> |
| Merkmal | Merkmale für die phänotypische Untersuchung. Mit der Taste **F3** kann eine Auswahl über die [Qualitätsmerkmale](./qualitaetsmerkmale.md) (Direktspring **[SAATR]**)abgerufen werden, die in dem betreffenden Verfahren untersucht werden soll. Hier können nur Qualitätsmerkmale ausgewählt werden, die den Merkmalstyp „Phänotyp“ haben.<br> |
| Bezeichnung | Bezeichnung des Merkmals.<br> |

### Markeranalyse

| Name | Bedeutung |
| --- | --- |
| Nummer | Hier kann das Labor bzw. das Institut für die Markeranalyse aus dem Kundenstamm angegeben werden.<br> |
| Anschrift | Die Hauptanschrift des Labors bzw. des Instituts.<br> |
| Anzahl | Anzahl an Datenpunkten mit denen das jeweilige Merkmal mit dem entsprechenden Marker untersucht werden soll.<br> |
| Merkmal | Merkmale für die genotypische Untersuchung. Mit der Taste **F3** kann eine Auswahl über die [Qualitätsmerkmale](./qualitaetsmerkmale.md) (Direktspring **[SAATR]**) abgerufen werden, die in dem betreffenden Verfahren untersucht werden soll. Hier können nur Qualitätsmerkmale ausgewählt werden, die den Merkmalstyp „Genotyp“ haben.<br> |
| Bezeichnung | Bezeichnung des Merkmals.<br> |
| Marker | Hier können die Marker angegeben werden, die für die Untersuchung eines Merkmals eingesetzt werden soll. Die Marker werden im Anwenderformat „AF_ANMARKER“ hinterlegt.<br> |

#### Felder auf der Registerkarte TKM

Die Refisterkarte TKM wird für die Detailprüfungen „TKM“, „TKM Extern“ und „TKM Leguminosen“ eingeblendet.

| Name | Bedeutung |
| --- | --- |
| Kartenbezeich. | Hier kann die Formel für die TKM-Berechnung angegeben werden.<br> |
| Anzeige TKM g<br> | Bestimmt, wie der Wert des Feldes „TKM g“ dargestellt wird:<br><br><br><ul><li>Gerundet:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Beispiel: Der Wert 3,6045 wird als 3,605 dargestellt.</li><li>Abgeschnitten&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Beispiel: Der Wert 3,6045 wird als 3,604 dargestellt.<br>&nbsp;</li></ul> |
