# Steueränderung durch Kundennummernänderung

<!-- source: https://amic.de/hilfe/_KundNummerAenderungSteuer.htm -->

Wird zu einem Kunden gewechselt, der einen abweichenden Steuersatz hat (z.B. sind EU-Bürger von außerhalb Deutschlands von der Steuer befreit), so werden alle Steuersätze von den Positionen heruntergerechnet.

Hierbei gilt folgendes zu beachten:

Preislisten

• Wird eine Netto-Preisliste verwendet, so wird der Brutto-Preis ohnehin durch die Addition eines Steuersatzes berechnet und die Neu-Berechnung eines Bruttopreises mit einem geringeren oder ohne Steuersatz ist simpel.

• Wird eine Brutto-Preisliste verwendet, so hat dieser Brutto-Preis eine bereits einkalkulierte Steuer.

o Ist der verwendete Steuersatz unbekannt, der dieser Preisliste zugrunde liegt. In diesem Fall wird der Steuersatz vom Bruttopreis abgezogen, der sich der Steuergruppe des Kunden und Steuerschlüssel des Artikels ergibt.

o In der Preisliste kann und sollte jedoch der verwendete Steuersatz angegeben werden. Verwenden Sie dazu den Pfleger für [Preislistenbezeichnungen](../../../zusatzprogramme/preiskalkulation_durchfuehren/index.md) (Preise/Konditionen > Konstanten der Preispflege > Preislistenbezeichnungen)

Absolute Rabatte/Zuschläge

Es wird empfohlen, an dieser Stelle mit relativen Rabatten (also prozentualem Anteil) zu arbeiten. Absolute Werte werden nicht mit einem Steuersatz berechnet. Sie sind absolut. Das bedeutet, dass die für den „Normalkunden“ erfassten 10€-Rabatt auch ein Rabatt für den EU-Bürger außerhalb Deutschlands bedeuten.

So wird der Betrag voll vom Endbetrag abgezogen. Das führt u.U. dazu, dass dem Kunden ungewollt ein größerer Rabatt als beabsichtigt gewährt wird.
