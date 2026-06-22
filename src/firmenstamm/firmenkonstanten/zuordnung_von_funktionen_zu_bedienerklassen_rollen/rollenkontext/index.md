# Rollenkontext

<!-- source: https://amic.de/hilfe/_rollenkontext.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung [ROLLE]

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung [ZUGF]

Einführende Erläuterungen finden sich unter [Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md).

<details>
<summary>Felder des Rollenkontext</summary>

| Felder |
| --- |
| Rolle | Zuordnung des Rollenkontextes zur [Rolle](../rollenstamm/index.md). |
| Funktion | Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).  
Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen **leeren Kontext** signalisiert. |
| Kontext | Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).  
Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen **leeren Kontext** signalisiert. |
| Beschriftung | Die textuelle Repräsentation einer Funktion in der Benutzeroberfläche. |
| Funktionsart | Funktionen senden Botschaften an das A.eins-System. Die Botschaften lassen sich in den Funktionsarten nach ihrem Wesen klassifizieren.  
Siehe Funktionsarten. |
| Direktsprung | Ausgewählte Funktionen sind per Direktsprung erreichbar. Es kann mehrere Direktsprünge für die gleiche Funktion geben, diese werden hier angelistet. |
| Bezeichnung | Eine kurze Erläuterung zu der Funktion durch den Entwickler. Es handelt sich meist um die Beschriftung, es kann aber hilfreiche Abweichungen geben, um besser eine Funktion „einschätzen“ zu helfen. |
| Anmerkung | Anmerkung, siehe auch Bezeichnung. |
| Steupa | Steuerparameter |
| Pulldown | Gibt den technischen Bezug zum verwendeten Pulldown-Menü an. |
| Controlstring | Die Botschaft die die Funktion beim Ausführen an das A.eins-System versendet.  
Controlstrings werden mit den Funktionsart klassifiziert, so dass man leichter erkennen kann um welchen Typus Funktion es sich handeln könnte. So gibt es Funktionen die „nur etwas zur Ansicht“ bereitstellen, aber auch Funktionen die Datenbestände löschen bzw. verändern. |
| Besitzer | AMIC/Privat  
Funktionen werden größtenteils von AMIC bereitgestellt. Aber auch A.eins-Anwender können private Funktionen bereitstellen.  
Siehe in dem Zusammenhang auch [Controllerklasse](../rollenstamm/index.md#Controllerklasse). |
| Reservierung | Otto Normalbenutzer/Entwicklung  
Eine mit „Otto Normalbenutzer“ ausgewiesene Funktion ist von AMIC freigegeben. |
| Wann zuerst | Wird eine Funktion ins System eingeführt (Privat oder Update), dann wird hier der Zeitpunkt vermerkt wann das geschehen ist. |
| Toolbar | Gibt an ob sich um einen Toolbar-Kontext handelt oder nicht. |

</details>

<details>
<summary>Suchmöglichkeiten des Rollenkontext</summary>

| Suchkriterien |
| --- |
| Suchen | Like  
Sucht in den Feldern  
• Rolle  
• Funktion  
• Kontext  
• Beschriftung  
• Direktsprung  
• Bezeichnung  
• Pulldown  
• Controlstring |
| Anzahl Datensätze | Anzahl, Standard 500  
Zur Sicherheit aus Performance-Gründen. Die Auswahlliste tätigt umfangreiche Kalkulationen. Da man mit ZUGF in aller Regel als nächstes was suchen wird, ist hier die Zeit des Ersteinstiegs regelbar. |
| Direktsprung? | JA/NEIN  
Unterstützt noch besser das gezielte Suchen nach Direktsprüngen.  
Des Weiteren sind somit Listen mit Klassifizierungen von Direktsprüngen möglich. |
| Funktionsart | Funktionsart  
Siehe [Funktionsarten](./funktionsarten.md). |
| Besitzer | AMIC/privat |
| Reservierung | Otto Normalbenutzer/Entwicklung |
| Pulldown? | JA/NEIN  
Unterstützt noch besser das gezielte Suchen nach „Pulldown“-Menüs.  
Des Weiteren sind somit Listen mit Klassifizierungen „Pulldown“-Menüs möglich. |
| Rolle enthält Bedienerklasse | von..bis (1) |
| Rolle enthält nicht Bedienerklasse | von..bis (2) |
| Toolbar? | JA/NEIN/EGAL  
Unterstützt das Auffinden von Toolbar-Funktionen. |

Zu (1): Das bedeutet im Falle das ein Kontext aufgelistet wird nicht zwangsläufig das die Funktion in der Funktionsgruppe auch tatsächlich der Bedienerklasse zugänglich ist. Es könnte nämlich sein, das eine vorherige Funktion die tatsächliche Erreichbarkeit im Programm rollentechnisch ablehnt.

Zu (2): Die so gelisteten Funktionen sind im jeweils zugehörigen Kontext garantiert nicht für die Bedienerklassen erreichbar.

</details>

<details>
<summary>Funktionen des Rollenkontext</summary>

| Funktionen |
| --- |
| Kontext … (F10) | Entwickler-Funktion  
Aufruf des Kontext-Pflegers.  
Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben … |
| Kontext (shift + F11) | Aufruf der Verbindungsübersicht des gewählten Kontextes |
| Private Sortierung/Tasten | Aufruf des Pflegers der die „private Sortierung“ und die „privaten Tastenzuordnung“ innerhalb eines Kontextes betreut.  
Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben … |
| Funktion Information (F9) | Aufruf eines [Informationsdialoges zur Funktion](./rollenkontext_pfleger.md). |
| Funktion ansehen/bearbeiten (F11) | Aufruf des Anwendfunktions-Pflegers. |
| Ändern (F5) | Ändern der Rollenzuordnung des Kontextes.  
Für Details siehe [Rollenklassenpfleger](../rollenklasse/rollenklassen_pfleger.md). |
| Ansehen (F6) | Ansehen der Rollenzuordnung des Kontextes.  
Für Details siehe [Rollenklassenpfleger](../rollenklasse/rollenklassen_pfleger.md). |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Rollenkontext: Pfleger](./rollenkontext_pfleger.md)
- [Informationen](./informationen.md)
- [Funktionsarten](./funktionsarten.md)
- [Dieses Menü](./dieses_menue.md)
- [Dieses Hauptmenü](./dieses_hauptmenue.md)
- [Diese Verbindungen](./diese_verbindungen.md)
- [Rollenantrag](./rollenantrag.md)
