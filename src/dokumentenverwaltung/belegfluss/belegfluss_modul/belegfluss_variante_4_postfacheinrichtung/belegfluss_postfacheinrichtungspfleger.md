# Belegfluss Postfacheinrichtungspfleger

<!-- source: https://amic.de/hilfe/belegflusspostfacheinrichtungs.htm -->

| Name | Beschreibung |
| --- | --- |
| Postfach | ID + Name des Postfachs  
 |
| Nummernkreis | Der Nummernkreis für die Finanzbelegerfassung  
 |
| Finanzbelegerfassung | Bestimmt, ob die OB-Funktion Finanzbelegerfassung zur Verfügung steht.  
 |
| Anzahl Direktbuchungen | Gibt die Anzahl der Buchungen bei Direkt- Finanzbelegerfassung an. Die Funktion wird ausgeblendet, wenn die Anzahl gleich null ist.  
 |
| Beleg-Freigabe erlaubt?  
 | Steht das Feld „Beleg-Freigabe erlaubt?“ auf „Ja“, so kann im Belegfluss die Zuordnung eines Beleges zum Belegfluss entfernt werden (siehe [Belegzuordnung entfernen](../belegfluss_variante_1_meine_postfaecher/belegfluss_pfleger.md#Belegzuordnung_entfernen))  
. |
| Eingangslieferscheinklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)  
Sollte auf 1600 gestellt werden.  
 |
| Eingangsrechnungsklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)  
Sollte auf 1700 gestellt werden.  
 |
| Eingangsgutschriftsklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)  
Sollte auf 1800 gestellt werden.  
 |
| Belegflusshistorie | Prozedur. Standard: BelegflussHistorie  
 |
| Belegflussbemerkung | Prozedur. Standard: BelegflussBemerkung  
 |
| Anforderung | Prozedur. Standard: BelegflussZeigeAnforderung  
Wenn keine Prozedur angegeben ist, wird der Bereich ausgeblendet. Es wird empfohlen entweder Anforderung oder Genehmigung in einem Postfach zu nutzen.  
 |
| Genehmigung | Prozedur. Standard: BelegflussZeigeGenehmigung  
Wenn keine Prozedur angegeben ist, wird der Bereich ausgeblendet. Es wird empfohlen entweder Anforderung oder Genehmigung in einem Postfach zu nutzen.  
 |
| Verarbeitung | Prozedur. Standard: BelegflussGenehmigung  
In dieser Prozedur wird die Logik des Systems definiert.  
(Unter welchen Bedingungen bewegt sich ein Dokument in welches Postfach weiter)  
 |
| Direkt-Finanzbelegerfassung | Prozedur für die Direkt-Finanzbelegerfassung.  
   
Standardprozedur: BelegFluss_Direktbuchung  
   
Hinweis: Um sicherzustellen, dass die Funktion „Direkt-Finanzbelegerfassung“ im Belegfluss nicht mehrfach ausgeführt werden kann, ist in der Prozedur die übergebene „in_paginiernr“ als „fibuv_paginiernr“ zu selektieren (siehe Prozedur-Ausschnitt).  
 |
| Prüfung vor Belegerfassung | Prozedur zum Prüfen, ob eine Belegerfassung erffolgen werden darf.  
Ist hier eine Prozedur eingetragen, so wird diese vor jeder Belegerfassung (Direkt-Finanzbelegerfassung, Finanzbelegerfassung, Warenbelegerfassung) ausgeführt. Wird eine nicht-leere Meldung von der Prozedur zurückgeben, so wird diese Meldung im Belegfluss ausgegeben und es wird keine Belegerfassung durchgeführt.  
   
Standardprozedur: BelegFlussPruefeBelegerfassung  
 |
| eRechnung ArtikelInfo | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung eine Artikelnummer und eine Artikelbezeichnung für den Belegfluss. Diese Prozedur wird nur verwendet, wenn im Kundenstamm keine andere Prozedur eingetragen wurde. |
| eRechnung ArtikelDetails | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung weitere Inforationen zum Abgleich z.B. Mengeneinheiten, Umrechnungsfaktoren etc. und stellt diese den Werten aus der erfassten Quell-Position im Belegfluss gegenüber.  
Diese Prozedur wird nur verwendet, wenn im Kundenstamm keine andere Prozedur eingetragen wurde. |
| Excel Import | Hier kann ein Makro hinterlegt werden. Wenn dies geschieht, wird die Funktion ExcelImport in der OB freigeschaltet  
 |
| Weiterleitung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.  
 |
| Vorverteilung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.  
 |
| Kontierung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.  
 |
| Tabelle | Hier kann für jedes Feld einzeln angegeben werden, ob es pflegbar sein soll. Die Felder sind im Anwenderformat „af_bfLocked“ hinterlegt. Somit kann das System um AIS-Felder erweitert werden.  
Zugleich kann hier festgelegt werden, ob Welche der Felder bei welchem Belegtyp angezeigt werden. Die Bearbeitungsreihgenfolge der Felder und ihr Zugriff kann hier auch definiert werden. Bitte beachten Sie, dass einige Felder vom System gegen Eingabe gesperrt werden.  
 |
