# Einrichtung Verbotsliste

<!-- source: https://amic.de/hilfe/_einrichtungverbotsli.htm -->

Auf dem Register „Zusätze“ in den Anschriften ist, für die Verbotslistenprüfung, der Prüfstatus dieser Adresse zu setzen:

| ID | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 0 | nicht testen | Diese Adresse wird nicht automatisch (z.B. durch ein Event) geprüft. Wird diese Adresse ausgewählt und die Funktion ***Verbotslistenprüfung*** gewählt, so wird diese Adresse geprüft. |
| 1 | manuell erlaubt | Obwohl die Prüfung eine Übereinstimmung gefunden hat, darf diese Adresse verwendet werden. z.B. weil die Übereinstimmung zufällig oder die Handelsart nicht vom Embargo betroffen ist. Es ist anzuraten, die Gründe für die Setzung dieses Status zu dokumentieren! |
| 10 | ungeprüft | Diese Adresse ist derzeit nicht geprüft, ist jedoch zur Prüfung vorgesehen. |
| 11 | nicht zulässig | Diese Adresse ist bei der Prüfung auf eine Übereinstimmung gestoßen.<br>Dieser Status kann nicht manuell gesetzt werden! |
| 12 | zulässig | Diese Adresse hat die Prüfung durchlaufen und ist nicht auffällig.<br>Dieser Status kann nicht manuell gesetzt werden! |
| 99 | egal | Dieser Status dient lediglich der Filterung der Auswahlliste.<br>Dieser Status kann nicht manuell gesetzt werden! |

Ist der Status „nicht zulässig“ oder „zulässig“, so wird er bei Änderung der Anschrift automatisch auf „ungeprüft zurückgesetzt, da schließlich die neue Anschrift ein anderes Prüfergebnis zur Folge haben kann.

Ändern Sie eine Anschrift zum Status „nicht testen“ oder „manuell erlaubt“, so werden Sie um die Eingabe einer Begründung gebeten.
