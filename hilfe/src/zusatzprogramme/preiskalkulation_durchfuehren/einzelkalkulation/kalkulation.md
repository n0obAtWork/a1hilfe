# Kalkulation

<!-- source: https://amic.de/hilfe/kalkulation.htm -->

Die Kalkulationsformeln der Gruppe 0 des zugeh. Schemas werden aufgerufen:

bei Blättern zu einem neuen Artikel

bei Blättern auf einen anderen Zeitraum der Spalte ‚Aktuelle Preise‘

(Die Werte für ‚alter Preis‘ in Formeln ändern sich)

Bei Änderung der Datumsgrenzen des Neu-Preis-Zeitraumes

(Die Steuersätze könnten sich ändern) 

Wird ein Preis der Spalte ‚Neuer Preis‘ manuell überschrieben, ( dieses ist nur möglich, wenn die Preisliste über keine Korrektursperre verfügt), so werden anschließend die Kalkulationsformeln des Schemas mit der Gruppe ausgeführt (sofern vorhanden), deren Gruppennummer gleich der Preislistennummer des geänderten Preises ist.

In beiden Fällen wird das Ergebnis dem Betrage nach ‚kaufmännisch‘ entsprechend der Angabe im Feld ‚Rundungseinheiten‘ der Preislistenbezeichnung auf ein Vielfaches dieses Wertes gerundet.

Ist dieser z.B. 0,5 so wird

bis x,24 auf x,00

bis x,74 auf x,50

darüber auf x+1,00

gerundet.

Grundsätzlich werden alle Werte für Preisvariablen der Formeln während der Ausführung vor Übergabe an den Formel-Interpreter in die Formel-Zielpreis-Einheiten (Preiseinheit und Preismengeneinheit)

umgerechnet.

Dabei werden die Werte der Formelvariablen Nx (neuer Preis x) den Werten aus ‚Neuer Preis‘, diejenigen der Formelvariablen Ax (alter Preis x) den Werten aus ‚Aktueller Preis‘ des dort gewählten Zeitraumes entnommen.

Kalkuliert werden nur die in der Spalte ‚Neuer Preis‘ sichtbaren Preislisten (Sortierung > 0);

Sind in Formeln Bezüge ‚alter Preis‘ auf Preislisten vorhanden, die zwar nicht auf der Maske erscheinen (Sortierung = 0), aber dennoch mit Preisen für die Listenpreisgruppe existieren, so werden diese gezogen und können somit ‚verdeckt‘ in die Kalkulation einfließen.

Es werden alle VK- und EK.Preise der zum Artikel gehörigen VK- und EK- Listenpreisgruppen eingelesen und ggf. nach Kalkulation auch gespeichert. Daher kann es vorkommen, dass mehr Preise auf der Kalkulationsmaske erscheinen, als aus EK- und VK-Preismatrix des Artikels hervorgehen. Dann existiert entweder ein Artikel gleicher Listenpreisgruppen mit anderer Preismatrix, oder es handelt sich um Staffelpreise.

Beim Preiszugriff gilt grundsätzlich die Regel VK vor EK, d.h.:

Sollten sich in der VK- und EK-Listenpreisgruppe des Artikels Preise zur selben Preislistennummer befinden, so zieht der VK-Preis.

Um Konflikte zu vermeiden wird empfohlen, im EK-Bereich grundsätzlich andere Preislistennummern als im VK zu verwenden. Dieses gilt jedoch nur, wenn der SPA ‚Preiskalk.: EK-Listenpreisgruppen‘ eine der Einstelllungen ‚ Lesen, Kalkul., KEINE SPEICHERUNG‘ oder ‚volle Berücksichtigung‘ aufweist. Im ersten Fall werden EK-Preise während der Kalkulation möglicherweise durch die Formeln oder manuell verändert, können jedoch nicht, wie im zweiten Fall, gespeichert werden. Bei der Einstellung ‚keine Berücksichtigung‘ werden EK-Listenpreisgruppen von der Kalkulation ausgenommen.
