# Erfasserstamm

Hauptmenü  Administration  Firmenkonstanten  Erfasserstamm

oder Direktsprung **[ERF]**

Zusätzlich zu Bedienern lassen sich Erfasser einrichten. Im Gegensatz zu Bedienern können Erfasser ohne weiteres während des Programmbetriebs gewechselt werden. Das kann vor allem im Kassenumfeld genutzt werden, indem ein einzelner Kassen-Bediener als Sammel-Account genutzt wird und die einzelnen Kassierer als Erfasser eingerichtet sind. Die Kassierer können dann per Erfasserwechsel **[ERFW]** / **[SERFW]** gewechselt werden.

Das Abmelden eines Erfassers fungiert gleichzeitig als Sperrung von A.eins. Das Programm kann erst dann weiter benutzt werden, wenn sich wieder ein Erfasser angemeldet hat.

Um sofort zu sehen, welcher Erfasser gerade angemeldet ist, wird sein Kürzel neben dem Bedienerkürzel in der Titelzeile von A.eins angezeigt. Die Erfasser-ID des aktuell angemeldeten Erfassers wird in der LDB-Variable ERFASSERID gespeichert. Außerdem können Informationen zum angemeldeten Erfasser über die Formularpositionen ID_ERFASSERID und ID_ERFASSERKURZ abgerufen werden. Den Erfasser eines neuen Vorgangs erhält man mit ID_ERFASSERNEU oder ID_NAMEERFASSERNEU. Sollten Vorgänge mit und ohne Erfasser existieren, kann man die Formularpositionen ID_ERFASSERNEU_ODER_BEDIENERNEU oder ID_NAME_ERFASSERNEU_ODER_BEDIENERNEU verwenden. Wenn kein Erfasser bestimmt werden kann, zeigen diese Positionen die Daten des Bedieners an, der den Vorgang erstellt hat. Bei allen Positionen, die einen Namen ausgeben, wird der Parameter verarbeitet. Bei einer 0 wird der Kurzname ausgegeben, sonst der volle Name.

Jedem Bediener müssen seine Erfasser explizit zugewiesen werden. Diese Zuordnung kann entweder direkt im Erfasserstamm oder im Register Erfasser des Bedienerstamms eingestellt werden. Dabei kann ein Erfasser aus mehreren Bedienern zugewiesen werden. Zusätzlich kann im Bedienerstamm ein Standarderfasser eingestellt werden, der beim Einloggen des Bedieners automatisch eingeloggt wird.

<details>
  <summary>Felder des Erfasserstamm</summary>

  | Felder | Beschreibung |
  | :----- | :----------- |
  | Nummer | ID des Erfassers |
  | Kurzbeschreibung | Kurzbeschreibung des Erfassers |
  | Name | Name des Erfassers |
</details>

<details>
  <summary>Suchmöglichkeiten des Erfasserstamm</summary>

  |    | Suchkriterien |
  | :--- | :------------ |
  | Nummer | von … bis … |
</details>

<details>
  <summary>Funktionen des Erfasserstamm</summary>

  | Funktionen | Beschreibung |
  | :--------- | :----------- |
  | Ändern **(F5)**, Ansehen **(F6)**, Löschen **(F7)**, Neu **(F8)** | Erfasser Pfleger |
  | Doppelte Passwörter anzeigen | Zeigt an, welche Erfasser ein gleiches Passwort haben. |
</details>

<p class="siehe-auch">Siehe auch:</p>

- [Erfasser: Pfleger](erfasser_pfleger.md)
- [Erfasserwechsel](erfasser_wechsel.md)
