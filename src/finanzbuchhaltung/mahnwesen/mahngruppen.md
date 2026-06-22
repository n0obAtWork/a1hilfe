# Mahngruppen

<!-- source: https://amic.de/hilfe/mahngruppen.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Mahnwesen einrichten > Funktion Mahngruppen **F9**

Direktsprung **[FIMSG]**.

Hierbei handelt es sich um eine Klassifizierung der Mahnungsbehandlung. Die Mahngruppe wird im Kundenstamm hinterlegt.

| | Beschreibung |
| --- | --- |
| Mahngruppe | Laufende Nummer der Mahngruppe<br> |
| Bezeichnung | Textbeschreibung der Mahngruppe<br>Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 [sprachabhängige Bezeichnungen](../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md) zu pflegen.<br> |
| Steuerschlüssel | Welcher Steuerschlüssel soll beim Erstellen der Fibu-Belege für Zinsen bzw. Mahngebühren herangezogen werden. Bei der Festlegung des Steuerschlüssels ist daran zu denken, dass Mahngebühren und Mahnkosten nicht das Entgelt besonderer Leistungen und somit nicht steuerbar sind! Aus diesem Grund sollte hier ein Steuerschlüssel mit dem Steuersatz 0.0 % hinterlegt werden!<br> |
| Betrag ist Netto oder Brutto<br><br> | Hier kann man hinterlegen, ob die Gebühren Netto oder Brutto gebucht werden.<br> |
| Wie mahnen | Hier wird festgelegt, welche Belege auf der Mahnung erscheinen sollen:<br>0 alle Posten<br>Es werden alle offenen Posten, sowohl fällige als auch nicht fällige, in einer Mahnung gedruckt. Beim Berechnen der Summen erfolgt eine Verrechnung von Soll und Haben.<br>1 Mahnbar getrennt nach Mahnstufe<br>Für alle Mahnstufen wird eine separate Mahnung erstellt. Es erfolgt keine Verrechnung von Soll und Haben.<br>2 Mahnbar + weitere Sollposten.<br>Es werden alle Sollposten, egal ob fällig oder nicht, in der Mahnung aufgeführt. Es erfolgt keine Verrechnung von Soll und Haben. Für alle Mahnstufen wird nur eine Mahnung erstellt.<br>3 Mahnbar + fällige Habenposten <br>Es erscheinen alle fälligen Belege auf der Mahnung. Beim Berechnen der Summen erfolgt eine Verrechnung von Soll und Haben. Für alle Mahnstufen wird nur eine Mahnung erstellt.<br>4\. Mahnbar nicht getrennt nach Mahnstufen<br>Alle fälligen Belege im Soll werden auf der Mahnung aufgeführt. Für alle Mahnstufen wird nur eine Mahnung erstellt.<br> |
| Verrechnung ER/EG/ZA<br><br> | Grundsätzlich kann es aus organisatorischen Gründen nötig sein, dass Belege aus dem Einkauf nicht mit verrechnet werden sollen. Dazu dient dieser Schalter. Steht er auf „Nein“, so werden die Belegarten Eingangsrechnung (ER), Eingangsgutschrift (EG) und Zahlungsausgang nicht mit verrechnet, egal wie die Einstellung unter „Wie mahnen“ vorgenommen wurde.<br> |
| Mahnsaldo 0 mahnen | Im Normalfall werden keine Mahnvorschläge erstellt, wenn der zu mahnende Saldo 0 oder kleiner ist. Jetzt kann es jedoch gewünscht sein, die Mahnzinsen mit auszuweisen. Dann muss bei Teilzahlungen auch die letzte Zahlung mit auf die Mahnung. Der Saldo wäre dann 0 und es würde kein Mahnvorschlag erstellt werden. Setzt man diesen Schalter auf „Ja“, wird der Mahnvorschlag erstellt. Zu beachten ist jedoch hierbei, dass nur OP’s gemahnt werden. Sind diese OP’s jedoch abgeschlossen, also ausgeziffert, dann werden auch trotz dieses Schalters keine Mahnvorschläge erstellt.<br> |
| Höchster Text | Ist dort der Haken gesetzt, wird das Formular aus dem Mahnstamm gewählt, das zur höchsten Mahnstufe dieser Mahnung gehört. Ansonsten das Formular zur kleinsten Mahnstufe. Auch die Mahntexte werden nach dieser Einstellung bestimmt.<br> |
| Höchste Gebühr<br><br> | Ist dort der Haken gesetzt, wird die Gebühr genommen, die in den Mahnsätzen bei der höchsten Mahnstufe hinterlegt ist.<br> |
| Zinsen immer ab Fälligkeit | Es gibt zwei Möglichkeiten, wie die Mahnzinsen behandelt werden können. Entweder man bucht bei jeder Mahnung die Zinsen, dann dürfen die Zinsen nur von einer Mahnung bis zur nächsten Mahnung berechnet werden. Es darf dann kein Haken gesetzt werden.<br>Oder man bucht die Zinsen erst, wenn die Mahnung inklusive Zinsen gezahlt bzw. die Forderung dem Anwalt übergeben wurde. Dann muss die Mahnung den gesamten Zinsbetrag ausweisen, also Berechnung ab Fälligkeitsdatum.<br> |
| Zinsgutschrift zulassen | Bei der Berechnung von [Mahnzinsen](./mahnzinsen.md) kann es dazu kommen, dass Guthabenzinsen entstehen. Für gewöhnlich möchte man einem säumigen Zahler jedoch keine Zinsgutschrift erteilen. Dies ist die Standardeinstellung **Nein**. Ist der Zinssaldo also kleiner 0, so werden die Zinsen in sämtlichen Positionen und der Zinssaldo mit 0 ausgewiesen.<br> |
