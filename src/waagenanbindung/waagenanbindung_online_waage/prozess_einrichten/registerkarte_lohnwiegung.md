# Registerkarte Lohnwiegung

<!-- source: https://amic.de/hilfe/_porzess_lohnwiegung.htm -->

Dieser Punkt ist für **Fremdwiegungen(Lohnwiegungen)** interessant bei denen z.B. Ware für eine andere Firma gewogen wird die selbst keine Waage besitzt.

| Feld | Bedeutung |
| --- | --- |
| Dienstleistungsartikel | Wird hier ein Dienstleistungsartikel hinterlegt, so wird der Dienstleistungsartikel und nicht die Ware in Rechnung gestellt.<br>Bei der Vorgangserzeugung wird dann die Warenposition des Lieferscheines oder der Rechnung als **Wertartikel** gebucht. Des Weiteren erhält der Beleg eine neue Position, die den Dienstleistungsartikel enthält.<br> <br>Bei der Erzeugung des Vorgangs wird das Lager des Dienstleistungsartikels in folgender Reihenfolge gesucht.<br>1. Lager des Waagebeleges<br>2. Lager aus dem Profil<br>3. Lager des Benutzers (VKONS)<br>4. Lager des Artikels, der im Profil eingetragen wurde<br> <br>Entsprechend wird eine Stufe tiefer gesucht, wenn es kein Artikel in dem entsprechenden Lager gibt.<br> |
| Wertartikel | Default ist Nein.<br>Stellt man Ja ein, dann wird der Dienstleistungsartikel als Wertartikel behandelt.<br> <br> |
| Anzahl Wiegungen | Standard ist ‚zwei Wiegungen’.<br>Hier wird festgelegt nach wie vielen Wiegungen aus der [Vorgangserzeugung](../../online_wiegen_in_der_vorgangserzeugung/index.md) heraus der Waagedatensatz auf den Status ‚mit Vorgang’ gesetzt werden soll.<br>Soll beim Wiegetyp Lohnwägung (auch Schüttwiegung oder Fremdwiegung) nur eine Wiegung durchgeführt werden, dann muss dieses Feld auf ‚eine Wiegung’ gesetzt werden.<br> |
