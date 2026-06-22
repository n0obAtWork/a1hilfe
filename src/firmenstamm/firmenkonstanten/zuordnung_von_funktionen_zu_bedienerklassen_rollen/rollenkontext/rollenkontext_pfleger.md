# Rollenkontext: Pfleger

<!-- source: https://amic.de/hilfe/_rollenkontext_pfleger.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Pflege des Rollen-Kontextes der Funktion in der Optionbox.

Ein solcher „Kontext“ beschreibt die Rechtezuordnung der Bedienerklassen an genau dieser Stelle im Programm durch die Zuweisung einer Rolle.

Ändert sich diese Zuweisung, werden also Bedienerklassen Rechte erteilt bzw. entzogen, führt das zu einer neuen Rollendefinition. Das System prüft automatisch, ob es schon eine solche Rechtekonfiguration gibt, und stellt diese zur Verfügung. Im Falle das es die gewünschte Konfiguration als Rolle noch nicht gibt wird diese vom System angelegt.

Sollen Rollen insgesamt geändert werden, also für alle Vorkommen der Rolle in allen Kontexten gleichzeitig, empfiehlt sich der [Rollenpfleger](../rollenstamm/rollenstamm_pfleger.md).

In Umgebungen in denen spezielle Bedienerklasse für die Abarbeitung und Überwachung der Rollenänderungen eingerichtet sind lassen sich sogenannte Rollenanträge erstellen und ggf. vermailen.

<details>
<summary>Felder des Rollenkontext Pfleger</summary>

| Felder |
| --- |
| Kontext | Zuordnung des Rollenkontextes zu dem Kontext. |
| Funktion | Zuordnung des Rollenkontextes zu der Funktion. |
| Rolle | Die zugeordnete [Rolle](../rollenstamm/index.md) des Rollenkontextes.<br>Rolle ist per **F3** aus den vorhandenen Rollen auswählbar. |
| Neue Rolle | Zeigt an, ob die Rolle neu erstellt wird |
| Ist | Status der Bedienerklasse innerhalb der zuordneten Rolle. |
| Bedienerklasse | [Bedienerklasse](../../bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md) |
| Soll | Der gewünschte neue Status der Bedienerklasse.<br>Geänderte Soll-Stati im Vergleich zum Ist-Status werden farblich zur besseren Übersicht abgegrenzt. |
| Bedienerklassen-bezeichnung | Die Bezeichnung der Bedienerklassen.<br>Ein vorangestellter Stern (\*) bedeutet das die Bedienerklasse eine Controller-Klasse ist, somit die Bedienerklasse Mitglied der [Controller-Rolle](../rollenstamm/index.md#Controllerklasse) ist. |
| Bediener | Informatorische auf max. 255 Zeichen begrenzte Liste der Bediener der Bedienerklasse. |

</details>

<details>
<summary>Funktionen des Rollenkontext Pfleger</summary>

| Funktionen |
| --- |
| ***Speichern*** (**F9)** | Speichert ggf. Änderungen. |
| ***Rollenantrag stellen*** (**F10)** | Es wird nicht gespeichert!<br>Es wird ein Rollenantrag auf Änderung der Kontexte gestellt. Im Zuge der Abwicklung(\*) erfolgt eine Abfrage ob sofort eine Antragsmail verschickt werden soll oder nicht. Damit bietet sich über [Rollenantrag](./rollenantrag.md) u.a. die Möglichkeit mehrere Anträge in einer Mail zusammenzufassen.<br>Somit gibt es „Antrag stellen in beiden Modi“: Einmal Änderung der Rolle und zum anderen Neu-Anlage einer Rolle mit zugehöriger Rollenklasse zur späteren Freischaltung.<br>Dafür muss die Rolle aber existieren, deshalb wird eine solche Rolle dann auch hier gespeichert, aber an dieser Stelle nicht irgendwelchen Kontexten zugeordnet. Das geschieht erst im Rollenantrag.<br> <br>(\*) Der Einrichterparameter [Rollenantragsmailabfrage](../../../einrichterparameter/rollenantragsmailabfrage_epa_rollenantragsmailabfrage.md#UEB_EPA_ROLLEANTARGSMAIL_ABFRAGEN) bietet die Möglichkeit diese Abfrage abzuschalten.<br> <br>Sind bei Aufruf des Rollenkontext-Pflegers mehrere Kontexte ausgewählt so erfolgt bei Abarbeitung die Abfrage ob alle markierten mit der aktuellen Rolle beantragt werden sollen. |

</details>
