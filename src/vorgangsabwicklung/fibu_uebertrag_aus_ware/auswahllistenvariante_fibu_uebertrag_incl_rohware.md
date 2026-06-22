# Auswahllistenvariante Fibu Übertrag incl. Rohware

<!-- source: https://amic.de/hilfe/_fibueb_mitRW.htm -->

Hauptmenü > Warenverkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag incl. Rohware

Oder

Hauptmenü > Wareneinkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag incl. Rohware

Oder

Direktsprung **[FIB]**

In dieser Variante werden Rechnungen, Stornorechnungen, Gutschriften, Stornogutschriften, Rohware-Einzelabrechnungen, Rohware-Einzelstornoabrechnungen und Inventurbelege ohne Produktion und Umbuchungen dargestellt

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
| Druckkennzeichen | --: Beleg ist nicht gedruckt  
Ja: Beleg ist gedruckt |
| Fib | --: Beleg ist noch nicht übertragen  
i.B.: Übertragungsauftrag an Mandantenserver erteilt  
Ja: Beleg ist an Fibu übertragen  
NN: Beleg ist nicht übertragbar (z.B. Stornobeleg zu nicht übertragenem Originalbeleg) |
| RAB | --: Beleg ist noch nicht im Rechnungsaugangsbuch  
i.B.: Übertragungsauftrag an Mandantenserver erteilt  
Ja: Beleg ist im Rechnungsausgangsbuch |
| Verarb. | --: Beleg ist nicht weiterverarbeitet  
teilweise: Beleg ist teildisponiert  
ganz: Beleg ist weiterverarbeitet |
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
| Id | ID des zugehörigen bereits erzeugten Fibu-Belegs |

</details>

<details>
<summary>Bereichsauswahl</summary>

| Filter | Beschreibung |
| --- | --- |
| Belegnummer | Selektion der Belege mit Belegnummer (von/bis) |
| Datum | Selektion der Belege mit Belegdatum (von/bis) |
| Kunde | Selektion der Belege mit Kunden-/Lieferantennummer (von/bis) |
| Vorgangsklasse | Selektion der Belege mit Vorgangsklassen:  
Einkauf & Verkauf:  
 (700,790,800,890,1700,1790,1800,1890)  
Verkauf:  
 (700,790,800,890)  
Einkauf:  
 (1700,1790,1800,1890)  
Rechnungen EK/VK:  
 (700,790,1700,1790)  
Gutschriften EK/VK  
 (800,890,1800,1890)  
Inventurvorgänge:  
 (5052,5055) |
| Übertragungsdatum | Selektion der Belege nach Datum des Fibu Übertrags (von/bis), wenn dieser bereits erfolgt ist. Bei aktiviertem Filterkriterium werden nicht übertragene Belege nicht dargestellt! |
| Abteilung | Selektion der Belege die der Abteilung zugeordnet sind. |
| Fibu Status | Selektion der Belege bezüglich des FibuStatus  
alle:  
 keine Filterung  
Noch nicht übertragen ohne NN:  
 Belege mit FibuStatus = ‚—'  
Noch nicht übertragen mit NN  
 Belege mit FibuStatus = ‚—‘ oder ‚NN‘  
Übertragen:  
 Belege mit FibuStatus = ‚i.B.‘ oder ‚Ja‘ |
| Erstunterschrift | Selektion nach dem Kürzel des Erstbedieners bei Vieraugenprinzip |
| Ersteller | Selektion der Belege mit dem Kürzel des Erfassers/Erstellers der Belege |

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
