# Dauerauftrag

<!-- source: https://amic.de/hilfe/dauerauftrag.htm -->

Hierbei handelt es sich um einen spezielle Form des Auftrages: Im Vorgangskopf wird der Turnus des Dauerauftrages eingestellt.

| Feld | Beschreibung |
| --- | --- |
| DA Anfang | Startdatum für den Dauerauftrag |
| DA nächster Termin | nächste Fälligkeit<br>Nächster Termin an dem der Dauerauftrag ausgeführt werden soll |
| DA Periode | Angabe der Periodizität:<br>1,2..,12 – alle .. Monate<br> (z.B. 1 = monatlich, 3 = vierteljährlich, 12 = jährlich)<br>mehrfach im Monat<br> Angabe der Termintage per Funktion ‚Dauerauftrag-Fälligkeit‘<br>wöchentlich<br> einmal pro Woche am Wochentag des Startdatums<br>alle 2 Wochen<br> am Wochentag des Startdatums<br>alle 3 Wochen<br> am Wochentag des Startdatums<br>alle 4 Wochen<br> am Wochentag des Startdatums<br>mehrfach pro Woche<br> Angabe der Wochentage per Funktion ‚Dauerauftrag-Fälligkeit‘ |
| DA Ende | Datum an dem der Dauerauftrag nicht mehr wirksam sein soll |

| Funktion | Bedeutung |
| --- | --- |
| ***Dauerauftrag erfassen*** **F8** | Erfassung eines neuen Dauerauftrages |
| ***Dauerauftrag starten*** **SF9** | Erstellt eine Rechnung aus dem Dauerauftrag.<br> <br>Es öffnet sich die Maske ‚Rechnung aus Dauerauftrag‘. Zur Erläuterung der Felder auf dieser Maske siehe [Umwandeln und Kopieren](../../umwandeln_und_kopieren/index.md).<br>Umwandlung starten F9 erzeugt dann die Rechnung und setzt die nächste Fälligkeit des Dauerauftrages neu. |
| ***Dauerauftrag ändern*** **F5** | Ändern der markierten von Daueraufträge, z.B. um den als nächstes vorgeschlagenen Fälligkeitstermin zu ändern |
| ***Anschriften aktualisieren*** | [Anschriften im Dauerauftrag manuell aktualisieren](../../vorgangsbearbeitung/dauerauftrag_anschrift_aktualisieren.md). |

<p class="just-emphasize">Dauerauftrag-Termine</p>

Bei den Dauerauftrag-Periodentypen ‚mehrfach im Monat‘ und ‚mehrfach pro Woche‘ können die Termine, an denen der Dauerauftrag ausgeführt werden soll, in einer Liste von Monatstagen beziehungsweise Wochentagen markiert werde.  
Ausgehend von dem aktuellen Fälligkeitsdatum wird bei diesen Periodentypen der jeweils nächste markierte Tag bei der Rechnungserstellung aus dem Dauerauftrag das neue Fälligkeitsdatum berechnet.  
Per Korrektur des Dauerauftrags kann ein zuvor berechneter ‚nächster Termin‘ manuell geändert werden. Dabei ist jedoch zu beachten, dass bei der Berechnung des nächsten Termins bei den (mehr-)monatlichen und (mehr)-wöchentlichen Periodentyp der Tag beziehungsweise Wochentag des Startdatums herangezogen wird. Im letzteren Fall wird der Sonntag als letzter Tag einer Kalenderwoche angesehen. Bei den Periodentyp ‚mehrfach im Monat‘ und ‚mehrfach pro Woche‘ wird hingegen ausgehend vom geänderten Termin der neue nächste Termin entsprechend der Angaben unter ‚Dauerauftrag-Fälligkeit‘ bestimmt.  
Wird ein Termin berechnet, der nach dem End-Datum des Dauerauftrags liegt, so wird dieser auch als nächster Termin in den Dauerauftrag eingetragen und eine Rechnung erzeugt. Zu dem neuen Termin kann dann aber keine Rechnung mehr erstellt werden, da er nach dem Ablaufdatum liegt.
