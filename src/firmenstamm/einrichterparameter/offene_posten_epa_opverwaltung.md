# Offene Posten (EPA OPVERWALTUNG)

<!-- source: https://amic.de/hilfe/_EPA_OPVERWALTUNG.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Obligokunden in einer Itembox abfragen? | Nein | Bei der Abfrage eines Obligokunden geht eine Auswahlliste auf, in der man den Kunden Auswählt und mit F9 bestätigt. Wenn man jedoch die Auswahl mit ENTER bestätigen möchte, dann kann man hier Ja einstellen.  
 |
| Bei Obligokunden bei ESC in die Auswahl springen? | Nein | Dies ist eine spezielle ESCAPE-Behandlung. Wenn man im OP-Verwaltung Esc drückt, dann wird nicht die OP-Verwaltung verlassen, sondern erst ein weiterer Obligokunde abgefragt. Erst wenn man auch bei der Abfrage des Obligokunden Esc gedrückt hat, wird die OP-Verwaltung verlassen.  
 |
| ¨eClearing¨ und ¨Kasse¨ beim Ausziffern überprüfen? | Warnung | Wenn Belege bereits in den Modulen eClearing bzw. Kasse verrechnet wurden, kommt es zu Konflikten. Wie man mit diesen Konflikten umgehen will, kann man hier einstellen:  
• Ignorieren: Es findet keine Prüfung statt  
• Fehler: Ist der OP bereits in dem Modul eClearing oder Kasse verwendet, kann er hier nicht ausgeziffert werden.  
• Warnung: Es wird geprüft, ob der OP-bereits verrechnet wurde und man wird vor dem Ausziffern gefragt, ob der OP trotzdem verwendet werden soll.  
 |
| Auszifferungsdatum beim Ausziffern mit Belegperioden prüfen? | Warnung | Um auch im Nachhinein feststellen zu können, wann ein OP noch offen war, wird ein Auszifferungsdatum im Beleg hinterlegt. Wenn die ausgewählte Buchungsperiode hinter dem Auszifferungsdatum liegt, kann es zu Unstimmigkeiten in der historischen OP-Liste kommen. Es existieren folgende Einstellungsmöglichkeiten:  
• Ignorieren: Es findet keine Prüfung statt  
• Fehler: Liegt die Periode hinter dem vor dem Belegdatum, kann nicht ausgeziffert werden.  
• Warnung: Man wir bei fehlerhaftem Test gefragt, ob der OP trotzdem verwendet werden soll.  
 |
| Periodenfenster bei Einstieg? | Ja | Beim Einstieg in die OP-Verwaltung werden das Jahr, die Periode und das Datum, auf das sich die Aktionen beziehen in einem separaten Fenster abgefragt. Soll im zum Tagesdatum gearbeitet werden, kann man hier **Nein** eintragen.  
 |
| Wie viele Tage Skontoüberschreitung? | 0 | Wenn OP’s miteinander verrechnet werden, so wird ggf. automatisch der Skonto errechnet. Will man eine Überschreitung des Skontodatums um einige Tage tolerieren, so kann man hier die Anzahl der Tag hinterlegen.  
 |
| Beim Hinzufügen der OP´s zu Zahlungsvorschlägen die Bank testen? | Ignorieren | Beim DTA muss eine Bank beim Kunden hinterlegt sein. Soll dies bereits bei hinzufügen der Zahlungsvorschläge getestet werden, dann kann man dies hier einstellen.  
• Ignorieren: Es findet keine Prüfung statt  
• Fehler: Nur wenn eine Bank zu diesem Kunden hinterlegt wurde, kann er zu den Zahlungsvorschlägen hinzugefügt werden.  
• Warnung: Man wir bei fehlerhaftem Test gefragt, ob der OP trotzdem verwendet werden soll. |
| Beim Hinzufügen der OP´s zu Zahlungsvorschlägen die Bank auswählen?  
 | Nein | Sind bei dem Kunden mehrere aktive Banken hinterlegt, dann kann man hier einstellen, dass man direkt vor dem hinzufügen die Bank auswählen will. |
| Zahlungsliste beim Ausziffern überprüfen? | Warnung | Ein Beleg kann auch bereits zur [Zahlung](../../finanzbuchhaltung/zahlungsverkehr/zahlungen_bearbeiten/index.md) freigegeben, jedoch noch verbucht sein. Dann könnte man ihn auch manuell in der OP einer eingehenden Zahlung zuweisen. Das ist jedoch nicht immer gewollt. Daher kann man hier das Verhalten auswählen:  
• Ignorieren: Es findet keine Prüfung statt  
• Fehler: Nur Beleg, die nicht zur Zahlung freigegeben sind, dürfen ausgeziffert werden.  
• Warnung: Man wir bei fehlerhaftem Test gefragt, ob der OP trotzdem verwendet werden soll.  
 |
| Per DTA oder per Scheck verarbeitetet OP´s für Auszifferung sperren?  
 | Nein | Wurde bereits eine DTA oder ein Scheck mit diesem Beleg erstellt, kann möchte man ggf. diesen Beleg nur über die Automatische Buchung ausziffern lassen. Dann kann man hier verbieten, dass der Beleg hier ausgeziffert wird.  
 |
| Zahlungsvorschläge beim Ausziffern überprüfen? | Warnung | Ist der Belege bereits in den [Zahlungsvorschlägen](../../finanzbuchhaltung/zahlungsverkehr/zahlungsvorschlaege_erstellen.md) enthalten, kann es wünschenswert sein, diesen nicht in der OP-verwaltung ausziffern zu können:  
• Ignorieren: Es findet keine Prüfung statt  
• Fehler: Ist der OP bereits in dem Modul eClearing oder Kasse verwendet, kann er hier nicht ausgeziffert werden.  
• Warnung: Es wird geprüft ob der OP-bereits verrechnet wurde und man wird vor dem Ausziffern gefragt, ob der OP trotzdem verwendet werden soll.  
 |
| Auszifferung bei Kontoeingabe testen? | Nein | Leider kann es durch Abstürze o.ä. dazu kommen, dass Auszifferungen nicht aufgehen. Das ist u.a. daran zu erkennen, dass Saldenliste und OP-Liste nicht übereinstimmen. Neben dem Test in Fibu-Reorganisator gibt es hier die Möglichkeit diesen Test für das angegebene Konto sofort ausführen zu lassen. Dies hat den Vorteil, dass dieses Problem Frühzeitig erkannt wird.  
 |
| Beim Löschen der Auszifferung Rücklastschriftbehandlung aktivieren? | Ja | Wenn Auzifferungen gelöscht werden, denen ein Zahlungsbeleg aus dem automatischen Zahlungsverkehr zugrunde liegt, muss dieser Zahlungsbelege als Rücklastschrift markiert werden, wenn die dieser Zahlung zugeordneten Rechnungen wieder beim Erstellen der Zahlungsvorschläge berücksichtigt werden sollen. Hier gibt es drei Ausprägungen:  
• Nein; Der Zahlungsbeleg wird nicht gekennzeichnet, die Rechnungen können nur manuell einem Zahlungsvorschlag hinzugefügt werden oder man muss den Zahlungsbeleg nachträglich kennzeichnen.  
• Ja: Es wird abgefragt, ob die Auszifferung als Rücklastschrift gekennzeichnet werden soll.  
• Automatisch: Der Zahlungsbeleg wird automatisch ohne Rückfrage als Rücklastschrift markiert.  
 |
