# Bewegungsübersicht

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_ausw_bewegungsueber.htm -->

Hauptmenü > Wareneinkauf > Nachhaltigkeit > Bewegungsübersicht > Variante Bewegungsübersicht

Direktsprung **[NAART]**

In dieser Auswahlliste werden alle Bewegungen im Ein - und Verkauf angezeigt. Diese kann man zum Beispiel nach nachhaltig und nicht nachhaltiger Ware filtern.

Bei Rohwarebewegungen wird jeweils nur die für die Massebilanz relevante Bewegung einer Rohwarenkette angezeigt.

Die Bewegungen kann man, falls diese in keiner festgeschriebenen Massebilanz sind und die Bewegungen nachhaltig sind, einer Massebilanz zuordnen.

Durch das Zuordnen einer Massebilanz zur letzten für die Massebilanz relevanten Bewegung werden die zugehörigen Bewegungen der Rohwarebelegkette (Lieferschein - > Abschlag -> Folgeabschlag - > Finale) auch intern zugeordnet.

Die Spalte der Auswahlliste Nachhaltig gibt den Status zurück, nachfolgend die Bedeutung der Farben dazu.

| Farbe | Beschreibung |
| --- | --- |
| **grün** | Bewegung ist nachhaltig |
| **rot** | Bewegung ist nicht nachhaltig |
| **Weiß** | Bewegung hat keine Nachhaltigkeitsinformationen |

Erläuterung speziellerer Selektionskriterien.

| Kriterium | Beschreibung |
| --- | --- |
| Massebilanz zulässig | Einstellung ob nur Warenbewegungen angezeigt werden sollen, die zulässig für eine Massebilanz wären oder nicht. |
| Ohne Nachhaltigkeitsbewegung | Einstellung ob nur Warenbewegungen angezeigt werden sollen, die keine Nachhaltigkeitsinformationen enthalten. (*gelöschte Positionen*) |
| Schwellenwertprobleme | Hiermit kann man einstellen, ob Positionen mit oder ohne Schwellenwertprobleme angezeigt werden sollen. |

#### THG / Massebilanz ändern

Auf dieser Maske lassen sich für die Warenbewegungen die Nachhaltigkeitswerte pflegen. Einzelne Werte lassen sich dadurch einfach ändern, wenn diese nicht bereits einer Massebilanz zugeordnet sind. Falls eine Massebilanz schon zugeordnet wurde, kann nur noch die Massebilanzzuordnung geändert/entfernt werden.

Möchte man die normalerweise nicht mehr änderbaren Werte ändern, dann muss man die zugeordnete Massebilanz zunächst entfernen.

Über die Funktion

 Zusätzlich stehen noch folgende Funktionen zur Verfügung. Nur nicht in einer Massebilanz festgeschriebenen Warenbewegungen werden in die Maske geladen.

| Funktion | Beschreibung |
| --- | --- |
| Initialisieren | Die Werte der aktuellen Zeile werden erneut initialisiert. Dabei werden die manuellen Werte entsprechend des [Steuerparameters 844](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/nachhaltigkeitseinstellungen_spa_844.md) beachtet. |
| Alle initialisieren | Die Werte aller Zeilen werden erneut initialisiert. (*siehe Funktion „Initialisieren“*) |
| Initialisieren (auch manuelle) | Die Werte der aktuellen Zeile werden erneut initialisiert. Dadurch werden auch manuell geänderte Werte zurückgesetzt und mit den aktuell gültigen Werten überschrieben. |
| Alle initialisieren (auch manuelle) | Die Werte aller Zeilen werden erneut initialisiert. (*siehe Funktion „Initialisieren (auch manuelle)“*) |
| Position löschen | Die Werte der aktuellen Zeile werden gelöscht. |
| Alle Positionen löschen | Die Werte aller Zeilen werden gelöscht. (*siehe Funktion „Position löschen“*) |
| Feldwert für alle übernehmen | Der Wert des aktuellen Feldes wird in alle anderen Zeilen der Spalte übernommen. |

#### Massebilanz ändern

Auf dieser Maske lässt sich die Massebilanz für die aktuell markierten Datensätze ändern. Dafür stehen die folgenden Funktionen zur Verfügung.

| Funktion | Beschreibung |
| --- | --- |
| Massebilanz zuordnen | An den markierten Datensätzen wird die ausgewählte Massebilanz zugeordnet, wenn möglich. |
| Massebilanz entfernen | An den markierten Datensätzen wird die Massebilanz entfernt, soweit dies möglich ist. |

Beim Massebilanz entfernen muss keine Massebilanznummer mit angegeben werden, da mehrere unterschiedliche Massebilanznummern von markierten Belegen entfernt werden können. Sollte das Entfernen einiger oder aller markierter Belege nicht möglich sein, erscheint eine Warnung und im Fehlerprotokoll wird eine ausführliche Meldung und Begründung geschrieben.

Sollte das Zuordnen einer Massebilanz zu einem oder mehrerer Belege nicht möglich sein, erscheint eine Warnung und im Fehlerprotokoll wird eine ausführliche Meldung und Begründung geschrieben. Außerdem werden teils benötigte Einrichtungen automatisch durchgeführt und ins Fehlerprotokoll geschrieben.
