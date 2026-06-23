# Mahnsätze einrichten

<!-- source: https://amic.de/hilfe/mahnstzeeinrichten.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Mahnwesen einrichten

Direktsprung **[FIMSG]**.

Dieser Pfleger fasst alle vorherigen Pfleger für die Mahnstammdaten zusammen. Es werden die einzelnen Mahngruppen untereinander und die Mahnstufen nebeneinander dargestellt.

![](../../ImagesExt/image8_629.jpg) 

Über Anzeige lässt sich einstellen, welcher Wert in der Kreuztabelle angezeigt werden soll.

Der Text der Mahnstufe lässt sich ändern, indem man auf das Feld klickt oder mit ENTER bestätigt: Neue Mahnstufen lassen sich über <strong>(Neu)</strong> eintragen.

Die linke Spalte mit den [Mahngruppen](./mahngruppen.md) funktioniert analog. Man gelang also direkt von der Kreuztabelle in den Stammdatenpfleger.

Wenn man nun in die Kreuztabelle klickt oder mit ENTER ein Feld auswählt, erscheint ein Pfleger, der sowohl den Mahnstamm als auch den Mahnsatz beinhaltet. Es wir dort immer der aktive Datensatz, also der mit dem größten "**Ab Datum**", angezeigt. Will man ab einem bestimmten Datum einen neuen Mahnsatz einrichten, so erreicht man dies über "***Neuer Satz*** **F8**". Die [Texte](./mahntexte.md) für diese Kombination lassen sich über "***Texte*** **F5**" erfassen.

![Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung](../../ImagesExt/image8_630.jpg "Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung")

| | Beschreibung |
| --- | --- |
| Mahngruppe | Angabe der Mahngruppe, für die die Bedingungen gelten<br> |
| Mahnstufe | Angabe der Mahnstufe, für die die Bedingungen gelten sollen, z.B. **"1"** für **"Mahnstufe 1"**<br> |
| Währung<br><br> | Währung, für die die Mahngebühr gilt. |
| Ab Datum<br><br> | Ab wann gelten diese Einstellungen.<br> |
| Buchungstext | Ist hier ein Text eingegeben, so wird dieser bei der Übernahme der Mahngebühren in die Primanota verwendet, sonst der bei „[Übernahme in die Primanota](./mahnungen_bearbeiten.md#MahnungenBuchen)“ als Einrichterparameter hinterlegte Buchungstext „Text Hauptzeile bei Übernahme der Mahnungen in die Primanota“<br> |
| Formular-Id<br><br> | Nummer des Mahnformulars, das ausgedruckt werden soll. Es kann somit für jede Kombination aus Mahngruppe und Mahnstufe ein eigenes Formular mit unterschiedlichem Aufbau bzw. Text hinterlegt werden. Man kann aber auch für jede Stufe dasselbe Formular hinterlegen und die unterschiedlichen Mahnstufen durch die Mahntexte kenntlich machen.<br> |
| Zinsgruppe | Falls Verzugszinsen berechnet werden sollen, wird hier die Zinsgruppe angegeben, deren Werte berücksichtigt werden sollen. Bei der Berechnung der Mahnzinsen wird nur der Soll-Zinssatz herangezogen.<br> |
| Kontonummer | Mahngebühren werden auf dieses Konto gebucht.<br> |
| Kostenstelle | Bei der Übernahme in die Primanota wird diese [Kostenstelle](../kostenrechnung/kostenstellen.md) verwendet.<br> |
| Kostenträger | Bei der Übernahme in die Primanota wird dieser [Kostenträger](../kostenrechnung/kostentraeger.md) verwendet.<br> |
| Kostenobjekt | Bei der Übernahme in die Primanota wird dieses [Kostenobjekt](../kostenrechnung/kostenobjekte/index.md) verwendet.<br> |
| Mahnabstand | Der Mahnabstand zwischen zwei Mahnungen. Häufig wird von der Fälligkeit bis zur ersten Mahnung noch eine Schonfrist gewährt. In diesem Fall muss hier bei Mahnstufe 1 ein Zeitraum von z.B. 14 Tagen eingetragen werden, für Mahnstufe 2 und höher wird dann z.B. 10 Tage eingetragen. Somit sind auch unterschiedliche Intervalle je Stufe möglich.<br> |
| Mahngebühr | Welche Mahngebühr soll gezogen werden? In der Mahngruppe ist hinterlegt, ob die Mahngebühr der kleinsten oder der größten Mahnstufe der Mahnung gezogen werden soll.<br> |
| Kleinste Mahnsumme | Wenn beim automatischen Erstellen einer Mahnung die zu mahnende Summe kleiner als der hier eingetragene Betrag ist, werden für diesen Kunden keine Mahnvorschläge erstellt.<br> |
| **Alle folgenden Felder (Versandprofil, Formular bei Mail-Versand, Formular Mailbody, Fa-Eintrag Mailbody, DB-Funktion Mailbody, DB-Funktion Betreff) erscheinen nur bei aktiver Belegversand-Lizenz. Sie sind unter** [Mahnstamm](./mahnstamm.md#MahnStammMailVersand) **dokumentiert.** |
