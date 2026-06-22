# View AMIC_V_Warenbewegung_Info

<!-- source: https://amic.de/hilfe/viewamicvwarenbewegunginfo.htm -->

Zuweilen wollen Sie sicher zusätzliche Informationen zu den Warenbewegungen bekommen. Diese bietet Ihnen die View Warenbewegung_Info.

Diese View kann mit dem Feld wabew_id an die Tabelle Warenbewegung oder andere Views gejoint werden, die die wabew_id enthalten.

| AMIC_V_Warenbewegung_Info  
Gibt zusätzliche Informationen zu Warenbewegungen |
| --- |
| Feld | Typ | Bezeichnung |
| wabew_id | Integer | ID der Warenbewegung |
| tmp_ist | Integer | temporäre Zwischenergebnise |
| tmp_fremd | Numeric(15,4) | temporäre Zwischenergebnise |
| tmp_ktrdiff | Numeric(15,4) | temporäre Zwischenergebnise |
| tmp_wert | Numeric(15,4) | temporäre Zwischenergebnise |
| wbc_Typ_EKVK | smallint | Einkauf/Verkaufskennzeichen (EK=1, VK=2) |
| wbc_SigniEigenware | Numeric(15,4) | Vorzeichen Eigenware |
| wbc_SigniEigenwareKtrDiff | Numeric(15,4) | Vorzeichen Eigenware Kontraktdifferenz |
| wbc_SigniFremdware_VVK | Numeric(15,4) | Vorzeichen Fremdware Vorverkauf |
| wbc_SigniFremdlager_VEK | Numeric(15,4) | Vorzeichen Eigenware Voreinkauf |
| wbc_SigniFremdware_EINL | Numeric(15,4) | Vorzeichen Fremdware Einlagerung |
| wbc_SigniFremdlager_KOM | Numeric(15,4) | Vorzeichen Fremdlager Kommission |
| wbc_SigniEigenBestand | Numeric(15,4) | Vorzeichen Eigenbestand |
| wbc_SigniLagerBestand | Numeric(15,4) | Vorzeichen Lagerbestand |
| wbc_SigniEinkauf | Numeric(15,4) | Vorzeichen Einkauf |
| wbc_SigniVerkauf | Numeric(15,4) | Vorzeichen Verkauf |
| wbc_Eigenware | Numeric(15,4) | Menge Eigenware |
| wbc_Fremdware_VVK | Numeric(15,4) | Menge Vorverkauf |
| wbc_Fremdlager_VEK | Numeric(15,4) | Menge Voreinkauf |
| wbc_Fremdware_EINL | Numeric(15,4) | Menge Einlagerung |
| wbc_Fremdlager_KOM | Numeric(15,4) | Menge Kommission |
| wbc_EigenBestand | Numeric(15,4) | Menge Eigenbestand |
| wbc_LagerBestand | Numeric(15,4) | Menge Lagerbestand |
| wbc_Einkauf | Numeric(15,4) | Menge Einkauf |
| wbc_Verkauf | Numeric(15,4) | Menge Verkauf |
| wbc_Eigenware_Wert | Numeric(15,4) | Wert Eigenware |
| wbc_Fremdware_VVK_Wert | Numeric(15,4) | Wert Vorverkauf |
| wbc_Fremdlager_VEK_Wert | Numeric(15,4) | Wert Voreinkauf |
| wbc_Fremdware_EINL_Wert | Numeric(15,4) | Wert Einlagerung |
| wbc_Fremdlager_KOM_Wert | Numeric(15,4) | Wert Kommission |
| wbc_EigenBestand_Wert | Numeric(15,4) | Wert Eigenbestand |
| wbc_LagerBestand_Wert | Numeric(15,4) | Wert Lagerbestand |
| wbc_Einkauf_Wert | Numeric(15,4) | Wert Einkauf |
| wbc_Verkauf_Wert | Numeric(15,4) | Wert Verkauf |
| wbc_BewArt | smallint | Siehe unten |
| wbc_BewCode | smallint | Siehe unten |

| Bewegungsart  
Ist wbc_BewArt gleich 0, so handelt es sich um eine reine buchhalterische Buchung, bei der keine Ware physisch bewegt wird (Ausnahme reiner Einkauf, reiner Verkauf). Dies kann das bisherige Kennzeichen WaBewBestTyp der Warenbewegung ersetzen. |
| --- |
| 0 | Alle Buchungen, die buchhalterisch relevante Bestände berühren (EK, VK, Vereinnahmung und Kommissionsverkauf)  
Diese Bewegungsart wurde in früheren Versionen auch als Eigenbewegung bezeichnet. |
| 1 | Buchung, die nur einen physikalischen Bestand berührt – Vorverkauf Abholung  
Diese Bewegungsart wurde in früheren Versionen auch als FremdwareVerkauf bezeichnet |
| 2 | Buchung, die nur einen physikalischen Bestand berührt – Voreinkauf Anlieferung  
Diese Bewegungsart wurde in früheren Versionen auch als Fremdlager Einkauf bezeichnet |
| 3 | Buchung, die nur einen physikalischen Bestand berührt – Einlagerung und Abholung |
| 4 | Buchung, die nur einen physikalischen Bestand berührt – Kommission und Rücknahme |

| Bewegungscode  
Der wbc_BewCode beschreibt die Herkunft der Warenbewegung. Während Codes kleiner 10 die eigentliche Tätigkeit beschreiben, kennzeichnen Codes zwischen 11 und 19 die jeweiligen Folgeschritte. Die Codes zwischen 21 und 29 sind für die Rückabwicklungen reserviert. Die Codes 10 und 20 stehen für Einkauf bzw. Verkauf. |
| --- |
| 1 | Vorverkauf |
| 2 | Voreinkauf |
| 3 | Einlagerung |
| 4 | Kommission |
| 10 | Einkauf |
| 11 | Vorverkauf Abholung |
| 12 | Voreinkauf Anlieferung |
| 13 | Einlagerung Vereinnahmung |
| 14 | Kommission Verkauf |
| 20 | Verkauf |
| 21 | Vorverkauf Rücknahme |
| 22 | Voreinkauf Rückgabe |
| 23 | Einlagerung Abholung |
| 24 | Kommission Rücknahme |
