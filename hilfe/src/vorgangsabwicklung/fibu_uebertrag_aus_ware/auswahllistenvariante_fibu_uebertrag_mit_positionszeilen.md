# Auswahllistenvariante Fibu Übertrag mit Positionszeilen

<!-- source: https://amic.de/hilfe/_fibueb_mitPos.htm -->

Hauptmenü > Warenverkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag mit Positionszeilen

Oder

Hauptmenü > Wareneinkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag mit Positionszeilen

Oder

Direktsprung **[FIB]**

In dieser Variante werden Rechnungen, Stornorechnungen, Gutschriften, Stornogutschriften, Rohware-Einzelabrechnungen, Rohware-Einzelstornoabrechnungen und Inventurbelege mit Waren-Positionszeilen dargestellt

<details>
<summary>Felder der Auswahlliste</summary>

| Feld | Beschreibung |
| --- | --- |
| Kontonr. | Kunden-/Lieferantennummer = Kontonummer |
| Kunde | Bezeichnung des Kunden/Lieferanten |
| Belegnr. | Nummer des Vorgangs |
| Typ | Vorgangsklassenkürzel des Belegs |
| Unterklasse | Nummer der Vorgangsunterklasse |
| Datum | Belegdatum |
| Dru | --: Beleg ist nicht gedruckt<br>Ja: Beleg ist gedruckt |
| Fib | --: Beleg ist noch nicht übertragen<br>i.B.: Übertragungsauftrag an Mandantenserver erteilt<br>Ja: Beleg ist an Fibu übertragen<br>NN: Beleg ist nicht übertragbar (z.B. Stornobeleg zu nicht übertragenem Originalbeleg) |
| RAB | --: Beleg ist noch nicht im Rechnungsaugangsbuch<br>i.B.: Übertragungsauftrag an Mandantenserver erteilt<br>Ja: Beleg ist im Rechnungsausgangsbuch |
| Verarb. | --: Beleg ist nicht weiterverarbeitet<br>teilweise: Beleg ist teildisponiert<br>ganz: Beleg ist weiterverarbeitet |
| Warenwert | Netto-Warenwert des Vorgangs |
| Netto. | Netto-Betrag des Vorgangs |
| Steuer | Steuer-Betrag des Vorgangs |
| Periode. | Warenwirtschaftsperiode des Vorgangs |
| Jahr | Warenwirtschaftsjahr des Vorgangs |
| Übertragungsdatum. | Datum des Fibu Übertrags |
| PeriodeFibu | Fibuperiode des Vorgangs |
| JahrFibu | Fibujahr des Vorgangs |
| Erstbediener | Erster Bediener |
| Zustimmungsbediener | Zweiter Bediener beim Vieraugenprinzip |
| Erfasser | Erfasser/Ersteller des Vorgangs |

</details>

<details>
<summary>Bereichsauswahl</summary>

| Filter | Beschreibung |
| --- | --- |
| Belegnummer | Selektion der Belege mit Belegnummer (von/bis) |
| Datum | Selektion der Belege mit Belegdatum (von/bis) |
| Kunde | Selektion der Belege mit Kunden-/Lieferantennummer (von/bis) |
| Vorgangsklasse | Selektion der Belege mit Vorgangsklassen:<br>Einkauf & Verkauf:<br> (700,790,800,890,1700,1790,1800,1890)<br>Verkauf:<br> (700,790,800,890)<br>Einkauf:<br> (1700,1790,1800,1890)<br>Rechnungen EK/VK:<br> (700,790,1700,1790)<br>Gutschriften EK/VK<br> (800,890,1800,1890)<br>Inventurvorgänge:<br> (5052,5055) |
| Übertragungsdatum | Selektion der Belege nach Datum des Fibu Übertrags (von/bis), wenn dieser bereits erfolgt ist. Bei aktiviertem Filterkriterium werden nicht übertragene Belege nicht dargestellt! |
| Abteilung | Selektion der Belege die der Abteilung zugeordnet sind. |
| Fibu Status | Selektion der Belege bezüglich des FibuStatus<br>alle:<br> keine Filterung<br>Noch nicht übertragen ohne NN:<br> Belege mit FibuStatus = ‚—'<br>Noch nicht übertragen mit NN<br> Belege mit FibuStatus = ‚—‘ oder ‚NN‘<br>Übertragen:<br> Belege mit FibuStatus = ‚i.B.‘ oder ‚Ja‘ |

</details>

<details>
<summary>Funktionen der Auswahlliste</summary>

| Funktion | Beschreibung |
| --- | --- |
| Fibu-Übertrag | Die Funktion erstellt für jeden ausgewählten Vorgang einen Eintrag im Datenstrom für den Mandantenserver zur Durchführung des Übertrags in die Finanzbuchhaltung. Das Kennzeichen *Fib* (FibuStatus) im Vorgang wechselt auf *‚i.B.‘* (in Bearbeitung) |
| Formulardruck | Mit dieser Funktion können die ausgewählten Vorgänge gedruckt werden. |
| Vorschau | Die Funktion erlaubt die Ansicht des mittels des Vorschau-Formulars dargestellten Vorgangs. |
| Bereichsauswahl/Filter | Öffnet die Bereichsauswahl-Maske zur Änderung der Filterkriterien. |
| Periode ändern | Mit dieser Funktion kann für noch nicht übertragene Vorgänge die Fibu-Periode geändert werden. |

</details>
