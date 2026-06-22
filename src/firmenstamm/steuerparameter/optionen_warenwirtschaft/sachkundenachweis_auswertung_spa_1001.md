# Sachkundenachweis Auswertung(SPA 1001)

<!-- source: https://amic.de/hilfe/_SPA_1001.htm -->

Mit diesem Steuerparameter wird festgelegt, wie in der Vorgangserfassung (nicht in der Kasse!) das nicht-Vorliegen eines Verkaufsbeschränkungszertifikats behandelt wird.

| Einstellung | Bedeutung |
| --- | --- |
| Abfrage in der GUI | Wie bisher wird in dem Fall, dass ein Sachkundezertifikat im Artikelstamm eingetragen ist dieses auch im Kunden erwartet. Ist dies dort nicht eingetragen, so wird abgefragt, ob dies zum Zeitpunkt der Erfassung geprüft wurde. |
| Erfassung abweisen (nicht in der Kasse) | Ist im Artikelstamm ein Zertifikatstyp zugewiesen worden, so muss dieses Zertifikat im Kunden zu hinterlegen, sonst kann dieser Artikel im Vorgang nicht erfasst werden.<br>Ausnahme bilden hier nur diejenigen Artikel für die zwar eine Verkaufsbeschränkung vorliegt, aber kein Zertifikatstyp eingetragen ist. Darunter fällt z.B. Alkohol. Für diesen gibt es die Verkaufsbeschränkung ab 16 bzw. ab 18 Jahren, aber keinen Sachkundenachweis. Hier würde auch bei dieser SPA-Einstellung eine Prüfungsabfrage angezeigt werden. |
