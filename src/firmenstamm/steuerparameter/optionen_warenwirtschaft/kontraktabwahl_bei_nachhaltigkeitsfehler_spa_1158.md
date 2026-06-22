# Kontraktabwahl bei Nachhaltigkeitsfehler (SPA 1158)

<!-- source: https://amic.de/hilfe/_SPA_1158.htm -->

Aktuell ist durch diesen SPA nur folgende Kombination unter Schutz:  
Der Kontraktartikel im Kontrakt hat den Nachhaltigkeitsstatus „Nachhaltig“ und die Warenposition hat den Nachhaltigkeitsstatus „Nicht Nachhaltig“.

Nur in dieser Kombination greift der SPA und achtet auf die folgenden Einrichtungsmöglichkeiten.

Standard ist „keine Kontraktabwahl, kein Hinweis“.

| SPA-Einstellung | Zertifikat für Lieferant (bei Verkauf Mandantkunde) für Artikel | Anzeige einer Warnung | Kontrakt wird abgewählt |
| --- | --- | --- | --- |
| Kontraktabwahl, kein Hinweis | Abgelaufen, ungültig oder nicht nachhaltig | Nein | Nein |
| Kontraktabwahl, kein Hinweis | Gültig und nachhaltig | Nein | Nein |
| nur Hinweis auf notwendige Kontraktabwahl | Abgelaufen, ungültig oder nicht nachhaltig | Ja | Nein |
| nur Hinweis auf notwendige Kontraktabwahl | Gültig und nachhaltig | Nein | Nein |
| Kontraktabwahl und Hinweis | Abgelaufen, ungültig oder nicht nachhaltig | Ja | Ja |
| Kontraktabwahl und Hinweis | Gültig und nachhaltig | Nein | Nein |

Wird dieser SPA auf „Kontraktabwahl, kein Hinweis“ gestellt, dann wird nichts überprüft.

Wird dieser SPA auf „nur Hinweis auf notwendige Kontraktabwahl“ gestellt, dann wird bei der manuellen oder automatischen Kontraktauswahl überprüft, ob der für den Beleg relevante Kunde für den Artikel im Beleg/Kontrakt ein gültiges Nachhaltigkeitszertifikat besitzt. Dies bedeutet, dass man auf dem Kundenstammpfleger im Tabreiter Zertifikate ein Zertifikat größer gleich 10 hat. Siehe dem Anwenderformat [AF_NACHHSTAT](../../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md).  
Wenn kein Nachhaltigkeitszertifikat vorhanden ist, das Nachhaltigkeitszertifikat den Status 1-9 hat, oder das Gültigkeitsdatum des Belegs außerhalb des Nachhaltigkeitszertifikates liegt, dann wird eine Warnung generiert.

Wird dieser SPA auf „Kontraktabwahl und Hinweis“ gestellt, dann wird neben der Warnung auch ein Fehlerprotokolleintrag geschrieben und der Kontrakt wird abgewählt.
