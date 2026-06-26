# Lagerverwaltungssystem

<!-- source: https://amic.de/hilfe/_celagerverwaltungssystem.htm -->

Mit diesem Modul kann per Scanner Ware in das [Lagerverwaltungssystem](../../../firmenstamm/lagerverwaltungssystem/index.md) Ein- und Ausgebucht werden. Das [Lagerverwaltungssystem](../../../firmenstamm/lagerverwaltungssystem/index.md) ist an spezielle Scannerprozesse angeschlossen worden. Das [Lagerverwaltungssystem](../../../firmenstamm/lagerverwaltungssystem/index.md) wird mit dem Steuerparamter 636 eingeschaltet.

Als erstes müssen die [Scancodes](../anwendung_scanner_in_aeins/scanner_scancodes_bearbeiten_modus/standard_einstellungen_scancodes.md) für das [Lagerverwaltungssystem](../../../firmenstamm/lagerverwaltungssystem/index.md) im A.eins System eingerichtet werden.

Nach dem die Scancodes eingerichtet worden sind müssen noch folgenden Einrichtungen vorgenommen werden.

Da die Abarbeitung der Lagerverwaltungsbefehle direkt nach dem Erfassen muss, muss für jeden Scanner ein Worker gestartet werden. Der Worker ist ein A.eins welches aus dem Bin Verzeichnis des A.eins gestartet werden muss. Am besten wird sich zum Starten des Prozesses eine Batch Datei angelegt. Der Aufbau der Batch Datei sieht wie folgt aus:

```text
start aeins welcome „sectionname“ -c eng=“Name des DB Servers“;dbn=“Name der Datenbank“;uid=SCANNER1;pwd=AMIC;links=tcpip pda=lvs_verarbeitungsmodul ScannerNummer=192.168.241.50
```

Wichtig dabei ist, das die Parameter pda auf lvs_verarbeitungsmodul steht und der Parameter Scannernummer hat den Wert der IP-Adresse des Scanners der mit diesem Prozess kommunizieren soll. Bei n Scanner müssen auch n Prozesse gestartet werden.

Als nächste muss auf jeden Fall der Pfad zur Datei Aeins_Programmstart.vbs eingerichtet werden, dies passiert auf der [Registerkarte LVS](../anwendung_scanner_in_aeins/einrichtung_des_scanners_an_der_zentral_datenbank/server_starten.md#ReigsterkarteLVS) unter dem Punkt [Serverstarten](../anwendung_scanner_in_aeins/einrichtung_des_scanners_an_der_zentral_datenbank/server_starten.md).

In folgenden Modulen ist das Lagerverwaltungssystem integriert worden

1. [Eingangslieferschein](../prozesse_des_scanners/eingangslieferscheine_bestellung_lieferschein_erfassen.md)

2. [Auftrag / Bestellung](../prozesse_des_scanners/auftraege_kommissionierung_retoure.md)

3. Produktion

4. [Inventur](../prozesse_des_scanners/inventur_im_scanner/index.md)

5. [Lagerumbuchung](../prozesse_des_scanners/lagerumbuchung.md)

6. [Ladescheinbearbeitung](../prozesse_des_scanners/ladescheinbearbeitung.md)

Damit die Scancodes in einem diesen Modulen zugelassen werden, müssen auf der Registerkarte [Zugelassene Scancodes](../anwendung_scanner_in_aeins/scanner_scancodes_bearbeiten_modus/index.md#ScancodeRegZugelasseneScancodes) in der Variante Scancodes die Lagerverwaltungsmodule zugelassen werden.

Folgende Scancodes sind auf jeden Fall zu erschaffen.

1. Ein Scancode für den Ladeträger. Für den Ladeträger wird der AI-Code 97 aus dem Code 128 genommen. Beispiel 97001 97 ist der Identifier. 001 ist die Ladeträgernummer.

2. Ein Scancode für die Lokalität. Für die Lokalität wird der AI-Code 99 aus dem Code 128 genommen. Beispiel 99001 99 ist der Identifier. 001 ist die Lokalität

#### LVS Scanner Module

#### LPL Ladeträger leeren

Mit diesem Modul werden Volle Ladeträger geleert. Diese können dann Alternativ noch auf eine Lokalität verschoben werden.

| Abarbeitung |
| --- |
| Scannen von LPL |
| Scannen von Ladeträgern. Es können mehrere hintereinander erfasst werden. |
| Optional kann noch eine Lokalität erfasst werden. |
| Scannen von LPLENDE |

#### Leeren von Ladeträgern

Das Leeren von Ladeträgern kann nur durchgeführt werden, wenn ein darüber gelagerter Prozess gestartet worden ist. Der darüber gelagert Prozess startet eine Abarbeitungsmaschine. Die Daten werden dann in die Relationen tcpip_scanner_maschine und produktionleerenfuellen geschrieben. Diese werden dann bei der Bearbeitung des Übergelagert Prozess benutzt.

Des Weiteren kann das Leeren nur in Abhängigkeit einer Lokalität durchgeführt werden. Beim Leeren Prozess können mehrere Kisten geleert werden, wenn diese Kisten komplett entleert werden sollen. Muss beim Leeren eine Menge mit angegeben werden, so kann im Leeren Prozess nur ein Ladeträger erfasst werden.

Wird beim Leeren Prozess nur die Menge angegeben, wird diese Menge als Restmenge in der Kiste gewertet.

Wird beim Leeren Prozess die Menge und der Befehl AUSWIEGEN erfasste, so wird die eingegebene Menge aus der Kiste entnommen.

| Abarbeitung |
| --- |
| Scannen von LEEREN 4711 wobei 4711 die Lokalität ist. |
| Scannen von Ladeträgern |
| Optional Eingabe der Menge |
| Optional Erfassen des Scancodes AUSWIEGEN |
| Scannen von LEERENENDE |

#### Füllen von Ladeträgern

Das Füllen von Ladeträgern kann nur durchgeführt werden, wenn ein darüber gelagerter Prozess gestartet worden ist. Der darüber gelagert Prozess startet eine Abarbeitungsmaschine. Die Daten werden dann in die Relationen tcpip_scanner_maschine und produktionleerenfuellen geschrieben. Diese werden dann bei der Bearbeitung des Übergelagert Prozess benutzt.

Des Weiteren kann das Füllen nur in Abhängigkeit einer Lokalität durchgeführt werden. Beim Füllen Prozess können mehrere Kisten gefüllt werden.

Optional kann im Füllen Vorgang eine Partie und eine Menge erfasst werden. Die erfasste Menge wird dann auf alle gefüllten Kisten aufgeteilt.

Des Weiteren gibt es einen Optionalen Parameter an dem Füllen Kommando. Dieser Parameter beinhaltet, die Position des Produktartikels in einem Rezept, wenn es sich um eine Beleglose Produktion handelt.

Wenn keine Menge erfasste worden ist, kann dem Ladeträger mit dem Wiegekommando ein Gewicht zugewiesen werden.

| Abarbeitung |
| --- |
| Scannen von FUELLEN 4711 1 Wobei 4711 die Lokalität ist und die 1 ist die Position des Produktartikels bei Beleglosen Produktion(Reinigung) |
| Scannen von Ladeträgern |
| Optional Eingabe der Menge |
| Optional Erfassen einer Partie |
| Scannen von FUELLENENDE |

#### Wiegen von Ladeträgern

Das Wiegen von Ladeträgern kann nur durchgeführt werden, wenn ein darüber gelagerter Prozess gestartet worden ist. Wenn mehrere Ladeträger erfasst werden, so wird die Menge auf die Ladträger aufgeteilt

| Abarbeitung |
| --- |
| Scannen von WIEGEN 4711 Wobei 4711 die Lokalität ist. |
| Scannen von Ladeträgern |
| Eingabe der Menge |
| Scannen von WIEGENENDE |

#### Maschineninfo

Beim Scannen der Maschineninformation wird der Zustand der Maschine(Produktion) angezeigt.

| Abarbeitung |
| --- |
| Scannen von MAI |
| Scannen der Lokalität |

#### Ladeträgerinfo

Beim Scanner der Boxinformation wird angezeigt Welcher Artikel mit welcher Partie und welcher Menge sich auf dem Ladeträger befindet.

| Abarbeitung |
| --- |
| Scannen von BOI |
| Scannen des Ladeträgers |

#### Partieinfo

Beim Scanner einer Box werden alle Ladeträger angezeigt, die zu der Partie im Ladeträger gehören.

| Abarbeitung |
| --- |
| Scannen von PA |
| Scannen des Ladeträgers |

#### Lokalitätsinfo

Beim Scanner der Lokalität werden alle Ladeträger angezeigt, dies sich auf dieser Lokalität befinden.

| Abarbeitung |
| --- |
| Scannen von LOI |
| Scannen der Lokalität |

<p class="siehe-auch">Siehe auch:</p>

- [Waagenanlieferung](./waagenanlieferung.md)
