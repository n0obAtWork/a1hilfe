# Steuerungsparameter Währung

<!-- source: https://amic.de/hilfe/steuerungsparameterwhrung.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

Direktsprung **[SPA]**

| Parameter | Nummer | Beschreibung |
| --- | --- | --- |
| Währungsumrechnung aktiv | 35 | Dieser SPA muss auf "aktiv" (**Ja**) stehen, damit in der Finanzbuchhaltung die Währungsumrechnung und OP-Führung in Fremdwährung aktiv ist.<br> |
| Anzeige Fremdwährung in Auswahllisten | 673 | Wird in der Finanzbuchhaltung Fremdwährung geführt, so werden im Beleg diverse Informationen (z.B. Währungskurs, Betrag in Fremdwährung, Steuer in Fremdwährung usw.) geführt. Diese werden in den Auswahllisten dargestellt, wenn hier **Ja** eingetragen wurde. In der Konteninfo steht dann auch eine weitere Variante „Konteninfo mit Währungsauflösung“ zu Verfügung.<br>Dieser Steuerparameter steuert gleichzeitig, ob beim Jahreswechsel zusätzlich ein Übertrag für Fremdwährung erstellt wird. Dieser Übertrag wird u.a. für die Variante „Konteninfo mit Währungsauflösung“ benötigt. Um nach der Umstellung dieses Steuerparameters die korrekten Daten angezeigt zu bekommen, muss einmalig eine „Reorganisation Währung“ ausgeführt werden.<br> |
| Anzeige des Fremdwährungssaldo in der Fibu | 794 | Grundsätzlich wird der Kontosaldo nur in der Buchwährung angezeigt. Will man zusätzlich den Saldo in Fremdwährung sehen, so muss dieser Steuerungsparameter auf **Ja** stehe. Es werden dann bei allen Personenkonten mit Währungstyp ungleich Euro und allen Bilanzkonten mit „Vorbelegung Buchwährung“ gleich **Nein** und Währung ungleich Buchwährung der Saldo in der dort eingetragenen Währung angezeigt.<br>Gleichzeitig bewirkt dieser Parameter, dass beim Jahreswechsel auch der Jahreswechsel für Fremdwährung durchgeführt wird. Um nach der Umstellung dieses Steuerparameters die korrekten Daten angezeigt zu bekommen, muss einmalig eine „Reorganisation Währung“ ausgeführt werden.<br> |
| Aktuell Buchwährung | 353 | Hier wird festgelegt, welche Währung A.eins als Buchwährung zugrunde legt. Diese wird für Auswertungen und Währungsumrechnungen benötigt.<br> |
| Kurbezugswährung | 360 | Auf diese Währung beziehen sich die in den Währungskursen angegebenen Kurse. Dies muss nicht die Buchwährung sein. Vor der offiziellen Einführung des Euros als Zahlungsmittel gab es eine Phase, in der die Kurse sich bereits auf den Euro bezogen, die Buchwährung jedoch noch die damalige Landeswährung (in Deutschland DM) war.<br> |
| Nummer der Währung DM | 494 | Aus DM Zeiten und wurde unter anderem für den Eurotransformer benötig.<br> |
| Fibuübertrag ohne Währungsinformation | 545 | Steht dieser Parameter auf **Ja**, so werden die in der Warenwirtschaft Fremdwährungsbelege ohne Währungsinformation übertragen. Dadurch kann die Währungsverarbeitung in der Finanzbuchhaltung umgangen werden.<br> |
