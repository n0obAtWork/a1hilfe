# Zinsabrechnung erstellen (EPA FIZINSV)

<!-- source: https://amic.de/hilfe/_EPA_FIZINSV.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Auch gelöschte Personenkonten verarbeiten? | Nein | Wird für Kunden, die als gelöscht gekennzeichnet sind, eine Zinsabrechnung benötigt, so muss man diesen Einrichterparameter auf **Ja** stellen |
| Alte Zinsrechnungen überprüfen? | Nein | Steht diese Option auf **Ja**, so werden beim Zinslauf automatisch alle alten Zinsabrechnungen dieses Kalenderjahres, die der Auswahl entsprechen nachgerechnet. Dabei wird der Eröffnungssaldo der ersten Zinsabrechnung inklusive aller Nachbuchungen als Eröffnung herangezogen und anschließend alle Zinsabrechnungen nachgerechnet. Nachträgliche Buchungen, die bisher nur in der folgenden Zinsabrechnung berücksichtigt wurden, werden beim „Nachrechnen“ der korrekten Periode zugewiesen. Das Ergebnis wird in den Feldern ZINSABRSOLLZRECALC, ZINSABRHABENZRECALC, ZINSABRSTARTSALDORECALC, ZINSABRSALDORECALC festgehalten.  
 Es steht auf dieser Maske dann auch eine weitere Funktion „***Nachrechnen*** **SF9**“ zur Verfügung, die die Zinsabrechnungen nachrechnet, ohne eine neue Zinsabrechnung zu erstellen.  
 |
| Zinssaldo vor Neuerstellung testen | Warnung | Stellt man hier ein **Ja** ein, so wird der Saldo der letzten Zinsabrechnung mit dem fälligen Saldo überprüft und es erschein ggf. eine Fehlermeldung.  
 |
