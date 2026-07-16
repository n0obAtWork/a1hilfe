# Nachhaltigkeitsprüfung (SPA 1157)

<!-- source: https://amic.de/hilfe/_SPA_1157.htm -->

Standard ist „keine Prüfung“

| SPA-Einstellung | Zertifikat für Lieferant (bei Verkauf Mandantkunde) für Artikel | Anzeige einer Warnung | Die Warenposition kann abgeschlossen werden. |
| --- | --- | --- | --- |
| Keine Prüfung | Abgelaufen, ungültig oder nicht nachhaltig | Nein | Ja |
| keine Prüfung | Gültig und nachhaltig | Nein | Ja |
| weiche Prüfung mit Warnhinweis | Abgelaufen, ungültig oder nicht nachhaltig | Ja | Ja |
| weiche Prüfung mit Warnhinweis | Gültig und nachhaltig | Nein | Ja |
| harte Prüfung | Abgelaufen, ungültig oder nicht nachhaltig | Ja | Nein |
| harte Prüfung | Gültig und nachhaltig | Nein | Ja |

Wird dieser SPA auf „keine Prüfung“ gestellt, dann wird nichts überprüft.

Wird dieser SPA auf „weiche Prüfung mit Warnhinweis“ gestellt, dann wird beim Verlassen der Warenpositionsmaske geprüft, ob der für den Beleg relevante Kunde für den Artikel im Beleg ein gültiges Nachhaltigkeitszertifikat besitzt. Dies bedeutet, dass man auf dem Kundenstammpfleger im Tabreiter Zertifikate ein Zertifikat größer gleich 10 hat. Siehe dem Anwenderformat [AF_NACHHSTAT](../../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md).

Wenn kein Nachhaltigkeitszertifikat vorhanden ist, das Nachhaltigkeitszertifikat den Status 1-9 hat, oder das Gültigkeitsdatum des Belegs außerhalb des Nachhaltigkeitszertifikates liegt, dann wird eine Warnung generiert.

Wird dieser SPA auf „harte Prüfung“ gestellt, dann wird beim Verlassen der Warenpositionsmaske geprüft, ob der für den Beleg relevante Kunde für den Artikel im Beleg ein gültiges Nachhaltigkeitszertifikat besitzt.  
Neben der Warnung wird dann auch das Verlassen der Warenpositionsmaske unterbunden und man muss einen anderen Artikel angeben oder die Warenpositionsaufnahme abbrechen.
