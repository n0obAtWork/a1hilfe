# Archiv-Ansicht-Definition

<!-- source: https://amic.de/hilfe/_archivansichtdefinit.htm -->

Ausgelöst wird eine Archiv-Ansicht über die Funktion ***Archiv anzeigen*** **CF12** im jeweiligen Programm-Kontext in A.eins. ([Ansichten allgemein](../ansichten_allgemein/index.md))

Eine Archiv-Ansichts-Definition ist im einfachsten Fall eine von AMIC vorkonfektionierte Beschreibung, mit deren Hilfe A.eins im Archiv recherchiert.

Der Programm-Kontext in A.eins stellt automatisch (fest vorgegebene) Kriterien zur Verfügung, die in der Archiv-Ansicht-Definition zur Auswertung und Bestimmung, welche Archiv-Einträge in der Archiv-Ansicht aufgelistet werden sollen, herangezogen werden können.

Alle ausgelieferten Archiv-Ansicht-Definitionen, die von AMIC mitgeliefert und bei Programmupdate aktualisiert werden, finden sich in der Variante Archiv-Ansichten-Variante: Ansichten ( nur AMIC – Auslieferung ).

Tabelle 5 Wichtige Archiv-Ansicht-Definitions-Begriffe

| |
| --- |
| Name | Sammelbegriff für Archiv-Ansichten gleichen Typus.  
Der Name des Ansichtsprofils.  
Eine so zusätzlich angelegte Ansichts-Definition kann nun auf einfache Weise ins A.eins-System integriert werden. Dazu bindet man eine private Funktion in die entsprechende Optionbox ein. Der Controlstring lautet ^jpl fa_view Name, wobei dann für Name der jeweilige Name zu setzen ist.  
Alle AMIC-Standard-Ansichten werden intern auf die Funktionalität fa_view umgesetzt. |
| Bedienerklasse | Bedienerklasse  
Hinweis: Die Bedienerklasse -1 bedeutet stets alle Bedienerklassen.  
Eine Ansichtsdefinition kann für eine einzelne Bedienerklasse bestimmt werden. Diese hat dann zur Laufzeit für ein Mitglied der Bedienerklasse Vorrang vor der speziellen Bedienerklasse -1.  
Somit ist es möglich individuelle Ansichtsdefinitionen für spezielle Bedienerklassen auszuprägen. |
| Besitzer | AMIC: Eine von AMIC ausgelieferte Ansichtsdefinition  
Privat: Eine vom A.eins-Anwender erstellte Ansichtsdefinition |
| Ansichts-Id | Technische Identifikation einer Ansichtsdefinition. |

Über die Pflege-Funktionen in den unter „[Archiv-Ansichten definieren](../archiv_ansichten_definieren/index.md)“ angegebenen Varianten erhält man Zugriff auf folgende weitere allgemeine Merkmale einer Archiv-Ansichten-Definition:

| |
| --- |
| Dokument | JA/NEIN  
Bestimmt ob „persönliche Dokumente“ beim Öffnen/Ausführen der Ansicht über **CF12** dem Kontext der entsprechenden A.eins-Lokalität zugewiesen werden sollen – oder nicht.  
A.eins bietet die Möglichkeit „persönliche/private“ Dokumente, die sogenannten „freien Dokumente“ (Belegklasse 9000) ins System zu importieren. Diese haben die Eigenschaft, dass, wenn man an geeigneter Stelle eine Formulararchiv-Ansicht aufruft, sie eben diesen aufgerufenen Kontext zugeordnet werden. Weitere Erläuterungen/Konfigurierungen bei der Behandlung der „Details“ .  
![](../../../ImagesExt/image8_919.png)  
„Geeignete Stellen“ sind somit Profile/Ansicht – Umgebungen, in denen man eben hier diese Möglichkeit vorsieht.  
So bietet es die Möglichkeit, 9000er-Zuweisungen auf bestimmte Bereiche einzugrenzen. Eben weil man u. U. viel mit **SF4** arbeitet und ansonsten eine „Fehlzuweisung“ passieren könnte. |
| Zusatz | JA/NEIN  
Bestimmt, ob zusätzlich zu den durch die Ansicht-Definition bestimmten Archiv-Einträgen auch alle die Archiv-Einträge aufgelistet werden, die weder eine Kundennummer noch eine Beleg-Referenz haben.  
Unterstützung einer möglichen Organisationsform hinsichtlich des Fehlens von Kundennummer und Belegreferenz von Archiv-Einträgen. Solche Dokumente kommen vornehmlich per Scanner-Importverfahren oder ähnlich gearteten Verfahren ins System. Jedenfalls ist die Absicht, genau diese im Grunde noch nicht zugeordneten Belege an jeder Stelle im Programm, in der in das Formulararchiv gesehen wird, den Sachbearbeitern zur Kenntnis zu bringen.  
Diese haben dann die Möglichkeit den Beleg einzusehen und ggf. durch „Bearbeiten“ eine Kundennummer und/oder Referenznummer zuzuordnen, um somit den Beleg „abzuarbeiten“.  
Unterstützung bezüglich dieser Bearbeitung bietet auch der Punkt „Autoedit“. |
| Autoedit | JA/NEIN  
Bestimmt, ob bei der Funktion **F5** in der Formulararchiv-Anzeige die Felder „Kundennummer“ und „Beleg-Referenz“ automatisch mit dem jeweiligen A.eins-Kontext belegt werden. |
| Ausschluss | Hier kann eine durch Komma getrennte Liste von Beleg-Klassen angegeben werden, deren Archiv-Einträge nicht angelistet werden sollen. |
| Durchstart | JA/NEIN  
Ergibt die durch die Ansichts-Definition veranlasste Archiv-recherche, dass genau ein „Dokument“ gefunden wurde, so wird die Anzeige dieses einen Dokumentes sofort ohne weitere Rückfrage durch das Programm eingeleitet. |
| Hinzufügen | JA/NEIN  
In der Formulararchiv-Anzeige wird die Funktion ***Hinzufügen*** **F8** aktiviert. |
| Bed.Schutz | JA/NEIN  
Bestimmt ob das Schutz-System über die Bedienerklasse des Archives aktiviert wird. |
| Priv. Import aktiviert | JA/NEIN  
Bestimmt ob während der Archiv-Recherche der „Private Import“ durchgeführt wird.  
[Archiv: Privater Import](./archiv_privater_import.md) |
| Profilpfad | Ein hier hinterlegter Pfad wird von A.eins automatisch um den Kurznamen des jeweiligen ausführenden Bedieners erweitert und bestimmt dann den Pfad für den „Privaten Import“.  
[Archiv: Privater Import](./archiv_privater_import.md) |
| Profilfilter | Reguläres Muster für Importfilter  
[Archiv: Privater Import](./archiv_privater_import.md) |
| Einsatz | Beschreibung über den Einsatz bzw. Verwendungen der Ansichts-Definition. |
| Grundlage | Versucht über das Einsatzgebiet der Archiv-Ansichten zu informieren.  
Mögliche Identifizierungen sind:  
0 : Frei  
1 : Auswahlliste  
2 : Dialog  
3 : Extern  
4: Auswahl |
| Variante | Variante, die zur Formulararchiv-Anzeige herangezogen wird.  
 |
| Anwendung | Anwendung, die zur Formulararchiv-Anzeige herangezogen wird.  
Zurzeit ist diese auf „fa_anzeige“ festgelegt und nicht veränderbar. |
| Dialog | JA/NEIN  
Bestimmt, ob die Formulararchiv-Anzeige als Dialog oder Nicht-Dialog gestartet wird. Empfehlung=JA |
| Vorschau | JA/NEIN  
Aktiviert in dieser Ansicht eine erweiterte Archiv-Ansichts-Technologie. |

<p class="siehe-auch">Siehe auch:</p>

- [Archiv: Privater Import](./archiv_privater_import.md)
