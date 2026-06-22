# Schnelle Teildisposition

<!-- source: https://amic.de/hilfe/_formularzuordnung_teildisposition.htm -->

Unter dem Register „Schnelle Teildisposition“ können Einstellungen für die „Schnelle Teildisposition“ bei der Artikelerfassung vorgenommen werden. Die Einstellungen teilen sich in die Bereiche „Unterklasse als Dispositionsziel“ und „Unterklasse als Dispositionsquelle“ auf.

Unterklasse als Dispositionsziel

Diese Einstellungen beeinflussen das Verhalten des disponierenden Beleges dieser Klasse/Unterklasse.

| Maskenfeld | Beschreibung |
| --- | --- |
| Quellbeleg abbuchen | Hier kann eingestellt werden, ob dieser Beleg Mengen vom Quellbeleg abbucht. |
| Lagerübergreifende Teildisposition | Bei „Nein“ wird die Auswahl der Artikel bei der „Schnellen Teildisposition“ auf Artikel eingeschränkt, die sich in dem gewählten Lager befinden.<br>Bei „Ja“ werden alle verfügbaren Artikel angezeigt und bei der Wahl eines Artikels, der nicht aus dem gewählten Lager stammt, wird dieser lagerübergreifend umgewandelt, sofern er auch in dem gewählten Lager vorhanden ist. |
| Waagenbelege zulassen | Hier kann eingestellt werden, ob aus der Waage erzeugte Vorgänge mit der „Schnellen Teildisposition“ bearbeitet werden dürfen. Im Standard steht der Schalter auf Nein. Wird der Schalter auf „Ja“ gestellt, so kann auch die Menge bei der „Schnellen Teildisposition“ abgeändert werden.<br>*Achtung: Die „Schnelle Teildisposition“ ändert die Original Menge des Vorganges. Wird der Vorgang komplett per Teildisposition umgewandelt und danach storniert wird der Status der Waage für den Beleg auf abgeschlossen zurück gestellt.* |
| Itembox Quellbelege | Hier kann eine individuelle Itembox für die Auswahl der Belege hinterlegt werden. |
| Disponierbare Belegklassen<br> | Hier kann eingestellt werden aus welcher Belegklasse die Vorgänge stammen, deren Artikel für diese Belegunterklasse bei der „Schnellen Teildisposition“ angezeigt werden.<br>Mit Mausklick wird die gewünschte Zeile ausgewählt. Sie ist dann schwarz unterlegt. Mit einem weiteren Mausklick kann sie wieder abgewählt werden.<br>Bei der „Schnellen Teildisposition“ werden für Kontokorrentkunden in der Itembox nur Belege angezeigt die entsprechend zu EK oder VK gehören. |

Unterklasse als Dispositionsquelle

Diese Einstellungen beeinflussen das Verhalten, wenn ein Beleg dieser Klasse/Unterklasse disponiert wird.

| Maskenfeld | Beschreibung |
| --- | --- |
| Verhalten bei Überdisposition | Diese Einstellung steuert das Verhalten, wenn ein Beleg dieser Belegklasse, der als Quellbeleg verwendet wird, überdisponiert wird.<br>Es gibt folgende Einstellungsmöglichkeiten:<br>Menge auf 0 (Standard): Der Beleg mit der überhöhten Menge kann erstellt werden und die Menge im Quellbeleg wird auf 0 gesetzt.<br>Menge auf 0 mit Warnung: Der Beleg mit der überhöhten Menge kann erstellt werden und die Menge im Quellbeleg wird auf 0 gesetzt, allerdings erfolgt vor Abschluss der Warenposition ein Warnhinweis.<br>Fehler: Die Warenposition kann nicht abgeschlossen werden und es erfolgt eine Fehlermeldung. Die Disposition wird nicht durchgeführt. |
| Stornoprozentsatz | Dieser Prozentsatz gibt an, ab welcher Warenmenge ein Quellbeleg dieser Unterklasse bei der „Schnellen Teildisposition“ storniert wird. Hat eine Warenmenge einer Warenposition aus dem Quellbeleg diesen Prozentsatz unterschritten, so wird sie zu 0 gesetzt. Sind alle Mengen auf 0 gesetzt, wird dieser Beleg storniert. Dies hat den Zweck, dass abgearbeitete Belege oder Belege mit kleinsten Restmengen nicht im Speicher verbleiben. Bei Eingabe einer negativen Prozentzahl wird der Quellbeleg auch nach vollständiger Umwandlung nicht storniert. |
