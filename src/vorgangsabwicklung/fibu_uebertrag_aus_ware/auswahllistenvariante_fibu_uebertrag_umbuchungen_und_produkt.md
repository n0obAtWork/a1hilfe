# Auswahllistenvariante Fibu Übertrag Umbuchungen und Produktion

<!-- source: https://amic.de/hilfe/_fibueb_umbprod.htm -->

Hauptmenü > Warenverkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag Umbuchungen und Produktion

Oder

Hauptmenü > Wareneinkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag Umbuchungen und Produktion

Oder

Direktsprung **[FIB]**

In dieser Variante werden Produktions- und Umbuchungs-Vorgänge dargestellt

<details>
<summary>Felder der Auswahlliste</summary>

| Feld | Beschreibung |
| --- | --- |
| Belegnr. | Nummer des Vorgangs |
| Typ | Vorgangsklassenkürzel des Belegs |
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
| PeriodeFibu | Fibuperiode des Vorgangs |
| JahrFibu | Fibujahr des Vorgangs |

</details>

<details>
<summary>Bereichsauswahl</summary>

| Filter | Beschreibung |
| --- | --- |
| Belegnummer | Selektion der Belege mit Belegnummer (von/bis) |
| Datum | Selektion der Belege mit Belegdatum (von/bis) |
| Vorgangsklasse | Selektion der Belege mit Vorgangsklassen:<br>alle Umbuchungen:<br> Lagerumbuchungen (5110),<br> Artikelumbuchungen (5120),<br> Produktion (5220)<br>Lagerplatzumbuchungen (5100)<br>Lagerumbuchungen (5110)<br>Artikelumbuchungen (5120)<br>Produktion (5220) |

</details>

<details>
<summary>Funktionen der Auswahlliste</summary>

| Funktion | Beschreibung |
| --- | --- |
| Fibu-Übertrag | Die Funktion erstellt für jeden ausgewählten Vorgang einen Eintrag im Datenstrom für den Mandantenserver zur Durchführung des Übertrags in die Finanzbuchhaltung. Das Kennzeichen *Fib* (FibuStatus) im Vorgang wechselt auf *‚i.B.‘* (in Bearbeitung) |
| Formulardruck | Mit dieser Funktion können die ausgewählten Vorgänge gedruckt werden. |
| Vorschau | Die Funktion erlaubt die Ansicht des mittels des Vorschau-Formulars dargestellten Vorgangs. |
| Bereichsauswahl/Filter | Öffnet die Bereichsauswahl-Maske zur Änderung der Filterkriterien. |
| Archiv anzeigen | Zeigt die zum Vorgang vorhandenen Archiveinträge. |
| Periode ändern | Mit dieser Funktion kann für noch nicht übertragene Vorgänge die Fibu-Periode geändert werden. |

</details>
