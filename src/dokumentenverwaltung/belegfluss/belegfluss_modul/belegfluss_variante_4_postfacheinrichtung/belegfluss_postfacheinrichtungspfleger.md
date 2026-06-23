# Belegfluss Postfacheinrichtungspfleger

<!-- source: https://amic.de/hilfe/belegflusspostfacheinrichtungs.htm -->

| Name | Beschreibung |
| --- | --- |
| Postfach | ID + Name des Postfachs<br> |
| Nummernkreis | Der Nummernkreis für die Finanzbelegerfassung<br> |
| Finanzbelegerfassung | Bestimmt, ob die OB-Funktion Finanzbelegerfassung zur Verfügung steht.<br> |
| Anzahl Direktbuchungen | Gibt die Anzahl der Buchungen bei Direkt- Finanzbelegerfassung an. Die Funktion wird ausgeblendet, wenn die Anzahl gleich null ist.<br> |
| Beleg-Freigabe erlaubt?<br> | Steht das Feld „Beleg-Freigabe erlaubt?“ auf „Ja“, so kann im Belegfluss die Zuordnung eines Beleges zum Belegfluss entfernt werden (siehe [Belegzuordnung entfernen](../belegfluss_variante_1_meine_postfaecher/belegfluss_pfleger.md#Belegzuordnung_entfernen))<br>. |
| Eingangslieferscheinklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)<br>Sollte auf 1600 gestellt werden.<br> |
| Eingangsrechnungsklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)<br>Sollte auf 1700 gestellt werden.<br> |
| Eingangsgutschriftsklasse (Unterklasse) | Vorgangsnummer + Unterklasse (optional)<br>Sollte auf 1800 gestellt werden.<br> |
| Belegflusshistorie | Prozedur. Standard: BelegflussHistorie<br> |
| Belegflussbemerkung | Prozedur. Standard: BelegflussBemerkung<br> |
| Anforderung | Prozedur. Standard: BelegflussZeigeAnforderung<br>Wenn keine Prozedur angegeben ist, wird der Bereich ausgeblendet. Es wird empfohlen entweder Anforderung oder Genehmigung in einem Postfach zu nutzen.<br> |
| Genehmigung | Prozedur. Standard: BelegflussZeigeGenehmigung<br>Wenn keine Prozedur angegeben ist, wird der Bereich ausgeblendet. Es wird empfohlen entweder Anforderung oder Genehmigung in einem Postfach zu nutzen.<br> |
| Verarbeitung | Prozedur. Standard: BelegflussGenehmigung<br>In dieser Prozedur wird die Logik des Systems definiert.<br>(Unter welchen Bedingungen bewegt sich ein Dokument in welches Postfach weiter)<br> |
| Direkt-Finanzbelegerfassung | Prozedur für die Direkt-Finanzbelegerfassung.<br> <br>Standardprozedur: BelegFluss_Direktbuchung<br><pre><code>create procedure BelegFluss_Direktbuchung (&#10;&#10; in in_fa_id&#10; integer default 0,&#10; in&#10; in_numkreisnummer integer default 0,&#10; in&#10; in_paginiernr char(255) default '' ,&#10; in in_fa_mndNr&#10; integer default 0,&#10; in in_zaehler&#10; integer default 1)&#10;Result(…)&#10;BEGIN&#10;Select in_paginiernr&#10; as Fibuv_Paginiernr,&#10;.&#10;.&#10;.&#10;END</code></pre><br> <br>Hinweis: Um sicherzustellen, dass die Funktion „Direkt-Finanzbelegerfassung“ im Belegfluss nicht mehrfach ausgeführt werden kann, ist in der Prozedur die übergebene „in_paginiernr“ als „fibuv_paginiernr“ zu selektieren (siehe Prozedur-Ausschnitt).<br> |
| Prüfung vor Belegerfassung | Prozedur zum Prüfen, ob eine Belegerfassung erffolgen werden darf.<br>Ist hier eine Prozedur eingetragen, so wird diese vor jeder Belegerfassung (Direkt-Finanzbelegerfassung, Finanzbelegerfassung, Warenbelegerfassung) ausgeführt. Wird eine nicht-leere Meldung von der Prozedur zurückgeben, so wird diese Meldung im Belegfluss ausgegeben und es wird keine Belegerfassung durchgeführt.<br> <br>Standardprozedur: BelegFlussPruefeBelegerfassung<br><pre><code>create PROCEDURE&#10; "admin"."BelegFlussPruefeBelegerfassung" (&#10; in in_fa_id&#10; integer,&#10; in in_fa_mndNr&#10; integer,&#10; in in_postfach&#10; integer,&#10; in in_BelegTyp&#10; integer)&#10;result (status&#10; char(255))&#10;BEGIN&#10;.&#10;.&#10;.&#10;END</code></pre><br> |
| eRechnung ArtikelInfo | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung eine Artikelnummer und eine Artikelbezeichnung für den Belegfluss. Diese Prozedur wird nur verwendet, wenn im Kundenstamm keine andere Prozedur eingetragen wurde. |
| eRechnung ArtikelDetails | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung weitere Inforationen zum Abgleich z.B. Mengeneinheiten, Umrechnungsfaktoren etc. und stellt diese den Werten aus der erfassten Quell-Position im Belegfluss gegenüber.<br>Diese Prozedur wird nur verwendet, wenn im Kundenstamm keine andere Prozedur eingetragen wurde. |
| Excel Import | Hier kann ein Makro hinterlegt werden. Wenn dies geschieht, wird die Funktion ExcelImport in der OB freigeschaltet<br> |
| Weiterleitung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.<br> |
| Vorverteilung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.<br> |
| Kontierung | Hilfsfeld: Hier kann ein Postfach angegeben werden, welches in den Prozeduren aufgegriffen werden kann.<br> |
| Tabelle | Hier kann für jedes Feld einzeln angegeben werden, ob es pflegbar sein soll. Die Felder sind im Anwenderformat „af_bfLocked“ hinterlegt. Somit kann das System um AIS-Felder erweitert werden.<br>Zugleich kann hier festgelegt werden, ob Welche der Felder bei welchem Belegtyp angezeigt werden. Die Bearbeitungsreihgenfolge der Felder und ihr Zugriff kann hier auch definiert werden. Bitte beachten Sie, dass einige Felder vom System gegen Eingabe gesperrt werden.<br> |
