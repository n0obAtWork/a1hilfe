# Textersetzungsmodule

<!-- source: https://amic.de/hilfe/_Textersetzungsmodule.htm -->

Hauptmenü > Systempflege > Individuelle Textersetzung

Direktsprung **[TEXTM]**

Hier lassen sich die Module erstellen, die für die Verwendung der individuellen Textersetzung zuständig sind.

| Feld | Beschreibung |
| --- | --- |
| Modulnummer | Nummer für das Modul |
| Bezeichnung | Bezeichnung des Moduls |
| Verwendung | Ja/Nein Auswahl<br>Entscheidet darüber, ob ein Modul und damit die individuelle Textersetzung verwendet werden soll.<br>Ja:<br>Hierbei werden bei einem Kunden ALLE Bediener mit der Sprachnummer 0 (Standard deutsch) auf die Sprachnummer 1000 umgestellt.<br> <br>Nein:<br>Soll kein Modul mehr verwendet werden, also auch keine individuellen Texte, so werden auch ALLE Bediener wieder zurück gestellt. |

Das Programm prüft bei Verwendung eines Moduls ob die Sprachnummer 1000 bereits existiert. Ist sie noch nicht vorhanden, wird sie automatisch angelegt. Weiterhin werden die alternativen Texte die für dieses Modul gespeichert sind in die Sprache 1000 – Alternativtexte übertragen. Somit stehen diese individuellen Texte für alle Bediener zur Verfügung, die die Sprachnummer 1000 verwenden.

Soll ein Modul nicht mehr verwendet werden, so werden dessen Texte aus der Sprache 1000 – Alternativtexte wieder entfernt. (siehe Tabelle, Feld Verwendung)

Wird kein Modul mehr verwendet, wird bei allen Bedienern, die die Sprachnummer 1000 verwenden, diese wieder auf die Standardsprachnummer 0 zurückgesetzt. Dies geschieht automatisch im Hintergrund bei Einstellung „Nein“ im Feld **Verwendung.**
