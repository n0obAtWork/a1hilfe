# Systemmeldungspfleger

<!-- source: https://amic.de/hilfe/systemmeldungspfleger.htm -->

| **Felder** | |
| --- | --- |
| Name | Eindeutiger technischer Name der Systemmeldung. |
| Aktiv | Ja/Nein<br>Bestimmt, ob die Systemmeldung überhaupt aktiv sein soll, d.h. ob die Bedingungen für eine Anzeige beim Programmstart überhaupt geprüft werden sollen. |
| Desktop | Bestimmt, ob die Systemmeldung zusätzlich als Benachrichtigung auf dem Windows-Desktop dargestellt werden soll. |
| Funktion | Die Funktion, die ausgeführt werden soll wenn ein User die Systemmeldung klickt.<br>Sie können hier private Funktionen anbinden. |
| Beschriftung | Die explizite Beschriftung der Systemmeldung.<br>Hinweis: Es handelt sich hierbei nicht um die Beschriftung der Funktion. |
| Sortierung | Kriterium für die Reihenfolge der Abarbeitung der Systemmeldungen. |
| Statement | Auf Basis dieses Statements wird entschieden, ob eine Systemmeldung durchgeführt werden soll oder nicht.<br>Das Statement ist so zu formulieren, dass der Ziel-Alias „wert“ heißt.<br>Dann entscheidet der Wert:<br>1 bedeutet Systemmeldung anzeigen.<br>0 bedeutet keine Systemmeldung anzeigen. |
| Exklusiv-User | Gemäß Rollenkontext kann es Bedienerklassen geben, denen die Systemmeldung vorlegt wird. (Rolle)<br>Durch Angabe eines Kurznamens lässt sich die Systemmeldung weiter einschränken. (Es kann auch durch komma-getrennte Liste von Kurznamen angegeben werden) |

Hinweis zur internen Statement-Verarbeitung:

Das angegebene Statement wird ausgeführt. Führt die Ausführung nicht auf einen Fehler, dann wird die Rückgabe ausgewertet. Wie dokumentiert führt das Ergebnis 0 dazu, das keine Systemmeldung angezeigt wird.  
Alle anderen Rückgaben führen zur Ausgabe der Systemmeldung, auch der Umstand das eine Ausführung zum Prüfungszeitpunkt technische Probleme hatte.  
Beispiel hierfür kann sein, dass Ressourcen durch andere Prozesse zum Zeitpunkt der Prüfung blockiert waren.
