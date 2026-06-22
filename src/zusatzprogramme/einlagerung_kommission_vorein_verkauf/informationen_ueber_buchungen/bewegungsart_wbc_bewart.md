# Bewegungsart (wbc_BewArt)

<!-- source: https://amic.de/hilfe/bewegungsartwbcbewart.htm -->

Das Feld wbc_BewArt findet sich in der View [AMIC_V_Warenbewegung_info](./view_amic_v_warenbewegung_info.md).

| Bewegungsart<br>Ist wbc_BewArt gleich 0, so handelt es sich um eine reine buchhalterische Buchung, bei der keine Ware physisch bewegt wird (Ausnahme reiner Einkauf, reiner Verkauf). Dies kann das bisherige Kennzeichen WaBewBestTyp der Warenbewegung ersetzen. |
| --- |
| 0 | Alle Buchungen, die buchhalterisch relevante Bestände berühren (EK, VK, Vereinnahmung und Kommissionsverkauf)<br>Diese Bewegungsart wurde in früheren Versionen auch als Eigenbewegung bezeichnet. |
| 1 | Buchung, die nur einen physikalischen Bestand berührt – Vorverkauf Abholung<br>Diese Bewegungsart wurde in früheren Versionen auch als FremdwareVerkauf bezeichnet |
| 2 | Buchung, die nur einen physikalischen Bestand berührt – Voreinkauf Anlieferung<br>Diese Bewegungsart wurde in früheren Versionen auch als Fremdlager Einkauf bezeichnet |
| 3 | Buchung, die nur einen physikalischen Bestand berührt – Einlagerung und Abholung |
| 4 | Buchung, die nur einen physikalischen Bestand berührt – Kommission und Rücknahme |
