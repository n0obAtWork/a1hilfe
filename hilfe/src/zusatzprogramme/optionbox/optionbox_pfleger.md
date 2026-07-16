# Optionbox-Pfleger

<!-- source: https://amic.de/hilfe/optionboxpfleger.htm -->

| **Felder** | Dialog „Option Box“ |
| --- | --- |
| Option Box | Identifikation der Optionbox |
| Tabelle 1 | Siehe [Übersicht der zugehörigen Funktionen](./optionbox_pfleger.md#uebersicht_der_funktionen) |
| Zu bearbeitende Funktion | Gibt die jeweils zu bearbeitende Funktion an |
| Hilfetitel | Eingabehilfe zur Festlegung der Hilfe |
| Hilfetextmarke | Eingabehilfe zur Festlegung der Hilfe |
| Tabelle 2 | Vorkommen der Optionbox |

| **Tabelle 1** | Übersicht der zugehörigen Funktionen |
| --- | --- |
| Funktion | Identifikation der Funktion |
| Label | Beschriftung |
| Bezeichnung | |
| FT | Feste definierte Funktionstaste, übersteuert Eintrag „Taste“ |
| Taste | Taste die dieser Funktion in dieser Optionbox zugewiesen wird.<br>Mit [**F3**] erhält man die Auswahl der möglichen Zuordnungen. |
| Gruppe | Gruppierungsmerkmal für Funktionen in der Optionbox.<br>Gängige Gruppen sind 0,1,2,3.<br>Die Gruppe 0 und 1 stellen in aller Regel Basisfunktionalitäten zur Verfügung.<br>Gruppe 2 und Gruppe 3 erlangen besondere Bedeutung in Auswahllisten.<br>Gruppen wie z.B. 100 oder 1000 werden beispielsweise oft per Software (OB_ADD, OB_REMOVE) zur Laufzeit dazu addiert bzw. entfernt. |
| EA | Einzelauswahl<br>Nur für „Gruppe 3-Funktionen“ in Auswahllisten<br>(Funktion wird nur aufgeführt, wenn mindestens eine Zeile markiert wurde) |
| Maus | Bestimmt ob die Funktion bei Doppelklick auf einen Auswahllisteneintrag ausgeführt werden soll. |
| Untermenü | Bestimmt das Untermenü in das die Funktion einsortiert werden soll. |
| Sort. | Bestimmt die Sortierung innerhalb der Optionbox. |

| **Tabelle 2** | Vorkommen der Optionbox |
| --- | --- |
| Anwendung | Ist keine Variante angegeben, dann ist die Optionbox dieser Anwendung zugeordnet.<br>Zusätzlich werden hier alle Source-Fundstellen der Optionbox angelistet. |
| Variante | Ist die Variante angegeben dann ist die Optionbox dieser Variante zugeordnet, die wiederum der obigen Anwendung untergeordnet ist |

| **Funktionen** | |
| --- | --- |
| Hilfe zuordnen [**F10**] | Zuweisung einer Hilfe-Funktion mittels der Eingabehilfen „Hilfetitel“ und „Hilfetextmarke“ |
| Funktion bearbeiten [**F11**] | Ruft den Funktionen-Pfleger auf. |
| Template für … [**F8**] | Ermöglicht die Erstellung eines Clones der Optionbox |
| Export Optionbox | Erstellt ein OSQL-Einspielscript für die Optionbox |
| Export Funktion | Erstellt ein OSQL-Einspielscript für die Funktion |
| Umbenennen in … | Umbenennen der Optionbox |
| Ganze OB löschen | Ermöglicht das Löschen der Optionbox |
| Neu | Ermöglicht die Neuanlage einer Optionbox |
| Wechseln | Wechselt zu der anzugebenden Optionbox |
| Funktion Information [**CF9**] | Zeigt Informationen zu der Funktion an. |
