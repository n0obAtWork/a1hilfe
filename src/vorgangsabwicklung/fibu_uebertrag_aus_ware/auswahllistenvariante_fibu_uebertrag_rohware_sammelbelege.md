# Auswahllistenvariante Fibu Übertrag Rohware-Sammelbelege

<!-- source: https://amic.de/hilfe/_fibueb_RWSammel.htm -->

Hauptmenü > Warenverkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag Rohware-Sammelbelege

Oder

Hauptmenü > Wareneinkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag Rohware-Sammelbelege

Oder

Direktsprung **[FIB]**

In dieser Variante werden Rohware-Sammeldruck-Abrechnungen und Rohware-Sammeldruck-Stornoabrechnungen dargestellt.  
Aufgrund der notwendigen Trennkriterien für den Fibu-Übertrag kann es vorkommen, das ein Sammeldruckbeleg hier für die Übertragung in mehrere Teilbelege zergliedert wird (z.B. bei unterschiedlichem Valutadatum der Einzelbelege).

<details>
<summary>Felder der Auswahlliste</summary>

| Feld | Beschreibung |
| --- | --- |
| Kl. | Vorgangsklassenkürzel des Belegs |
| SBel.Datum | Druckdatum (Belegdatum) des Sammeldruckbelegs |
| Drucknr. | Sammeldrucknummer (Belegnummer) des Sammeldruckbelegs |
| Dru | \--: Beleg ist nicht gedruckt  
Ja: Beleg ist gedruckt |
| Fib | --: Beleg ist noch nicht übertragen  
i.B.: Übertragungsauftrag an Mandantenserver erteilt  
Ja: Beleg ist an Fibu übertragen  
NN: Beleg ist nicht übertragbar (z.B. Stornobeleg zu nicht übertragenem Originalbeleg) |
| Kontonr. | Kunden-/Lieferantennummer = Kontonummer |
| Kunde/Lieferant | Bezeichnung des Kunden/Lieferanten |
| Kontrakt | Hier wird eine Liste der Kontraktnummern aller Anlieferungspositionen der Einzelbelege des Sammeldruckbelegs dargestellt |
| Filiale | Nummer der Filiale des Sammeldruckbelegs |
| Status | Abrechnungsstufe:  
Abschlag, F-Abschlag, Finale |
| Belege | Anzahl der zugehörigen Einzelabrechnungen |
| VFKtr. | Anzahl der beteiligten Vorverkaufs-/Voreinkaufs-Kontrakte |
| Währung | Währungsnummer zum Sammeldruckbeleg |
| Netto | Nettobetrag des Sammeldruckbelegs |
| Valuta | Valutadatum des Sammeldruckbelegs |
| StGrp. | Steuergruppe zum Sammeldruckbeleg |
| Jahr | Warenwirtschaftsjahr des Sammeldruckbelegs |
| Übertragungsdatum. | Datum des Fibu Übertrags |
| PeriodeFibu | Fibuperiode des Vorgangs |
| JahrFibu | Fibujahr des Vorgangs |
| Erstbediener | Nur bei aktiviertem Vieraugenprinzip:  
Erster Bediener |
| Zustimmungsbediener | Nur bei aktiviertem Vieraugenprinzip:  
Zweiter Bediener |
| Erfasser | Erfasser/Ersteller des Vorgangs |
| Abrplan über Jahrwechsel | Kennzeichen für geplanter jahresübergreifender Abrechnung |
| Ist Proforma | Kennzeichen für Proforma-Abrechnung bei jahresübergreifender Abrechnung |

</details>

<details>
<summary>Bereichsauswahl</summary>

| Filter | Beschreibung |
| --- | --- |
| Lieferanten-/Kundennummer | Selektion der Sammeldruck-Belege mit Kunden-/Lieferantennummer (von/bis) |
| Namensanfang | Selektion der Sammeldruck-Belege mit Namen der Lieferanten/Kunden beginnend mit dem angegebenen Text |
| Filiale | Selektion der Sammeldruck-Belege mit Filialnummer (von/bis) |
| Druckdatum | Selektion der Sammeldruck-Belege mit Druckdatum (Sammeldruck-Belegdatum) (von/bis) |
| Drucknummer | Selektion der Sammeldruck-Belege mit Sammeldrucknummer (Sammeldruck-Belegnummer) (von/bis) |
| Fibustatus | Selektion der Belege bezüglich des FibuStatus  
nicht in Fibu:  
 Belege mit FibuStatus = ‚—' oder ‚NN‘  
in Fibu:  
 Belege mit FibuStatus = ‚Ja‘  
egal:  
 keine Filterung  
nicht in Fibu ohne nn::  
 Belege mit FibuStatus = ‚__‘‘ |
| Einkauf/Verkauf | Selektion der Belege mit Vorgangsklassen:  
Einkauf & Verkauf:  
 (700,790,1700,1790)  
Verkauf:  
 (700,790)  
Einkauf:  
 (1700,1790)  
Rechnungen EK/VK:  
 (700,790,1700,1790) |
| Bearbeitungsstatus | Selektion der Sammeldruck-Belege nach Abrechnungsstufe:  
alle  
Abschlag  
Folgeabschlag  
Finale |
| Druck-Bearbeiterwahl | Selektion der Sammeldruck-Belege mit Bediener der Sammeldruckbeleg-Erstellung:  
\- alle Belege  
\- nur selbst gedruckte Belege |
| Jahrnummer | Selektion der Sammeldruck-Belege mit dem angegebenem Warenwirtschaftsjahr |
| Erstunterschrift | Selektion nach dem Kürzel des Erstbedieners bei Vieraugenprinzip |
| Ersteller | Selektion der Belege mit dem Kürzel des Erfassers/Erstellers der Belege |

</details>

<details>
<summary>Funktionen der Auswahlliste</summary>

| Funktion | Beschreibung |
| --- | --- |
| Fibu-Übertrag | Die Funktion erstellt für jeden ausgewählten Vorgang einen Eintrag im Datenstrom für den Mandantenserver zur Durchführung des Übertrags in die Finanzbuchhaltung. Das Kennzeichen *Fib* (FibuStatus) in den zugehörigen Einzel-Vorgängen wechselt auf *‚i.B.‘* (in Bearbeitung) |
| Periode ändern | Mit dieser Funktion kann für noch nicht übertragene Vorgänge die Fibu-Periode geändert werden. |

</details>
