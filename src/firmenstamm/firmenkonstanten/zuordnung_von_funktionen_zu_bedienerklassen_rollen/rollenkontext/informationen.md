# Informationen

<!-- source: https://amic.de/hilfe/_rollenkontext_info.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Dieser Dialog zeigt je nach [Funktionsart](./funktionsarten.md) zusätzliche Informationen zu einer Funktion in den Feldern „Funktions-Detail“ und „Funktions-Information“ an.

<details>
<summary>Funktions-Informationen:</summary>

| Funktions-Informationen |
| --- |
| Funktion | Die Identifikation der Funktion |
| Beschriftung | Textuelle Repräsentation der Funktion in der Benutzeroberfläche (Label) |
| Reservierung | Wenn was anderes als „Otto Normalbenutzer“ |
| Bezeichnung | Wenn sie sich von der „Beschriftung“ unterscheidet. |
| Funktionsart | [Funktionsart](./funktionsarten.md) |
| Controlstring | Die Botschaft die bei Ausführung der Funktion an A.eins gesendet wird. |
| Anmerkung | Wenn vorhanden eine Anmerkung |
| Pulldown | Wenn es um eine Pulldown-Zuweisung der Funktion vorhanden ist. |
| Direktsprung | Wenn Direktsprünge zur Funktion vorhanden sind. |
| Maske | Wenn die Funktion einen Dialog aufruft, der Maskenname |
| Titel | Wenn die Funktion einen Dialog aufruft, der Titel der Maske |
| Menü/Favorit | Wenn die Funktion einem Menü-Favoriten zugeordnet ist. |
| Menü-Überschrift | Wenn es bei der Funktion sich einen Hauptmenü-Eintrag handelt, dann wird hier der „Pfad“ aufgelistet.<br>Zum Beispiel für OSQL:<br>Administration->Werkzeuge->AMIC SQL Zugriff |
| Menü-Aufrufer | Gibt den „Weg“ im Haupt-Menü an, um von ganz links nach rechts zu gelangen.<br>Zum Beispiel für OSQL:<br>MENU_2/Firmenstamm<br>menu_14/MENU_3_9786_AMIC<br>Das bedeutet der Kontext MENU_2 ruft durch die Funktion „Firmenstamm“ den Kontext „menu_14“ auf, wo wiederum die Funktion „MENU_3_9786_AMIC“ den Kontext „menu_41“ aufruft in dem sich die Menü-Funktion „MENU_AMIC_SQL“ befindet. |

</details>
