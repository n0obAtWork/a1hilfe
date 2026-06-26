# Beschreibung der Relationen

<!-- source: https://amic.de/hilfe/beschreibungderrelationen.htm -->

**Relation ScriptParam**

ScriptParam ist die Kopfrelation, die eine ganze Gruppe von Parameter unter einer Id zusammenhält. In der Relation ScriptParamPar sind die Details, also die einzelnen Parameter selbst abgelegt.

ScriptPBedKorr

Bedienerkennzeichen, wird auf die UserId des letzten Bearbeiters gesetzt.

ScriptPBesitzer

0: allgemeine öffentliche Parametergruppe, 1: private Parametergruppe

Private Parametergruppen dürfen bei einem Datenbank-Update nicht verändert werden.

Das Recht, private Parameter anzulegen und zu bearbeiten wird über die **optionalen Parameter** (Direktsprung [OPT] eingestellt. Hierzu muss für den betreffenden Bediener ein Satz mit dem Namen **SKRIPTPARAMETER_PRIVAT** angelegt sein. Ein Wert ist nicht erforderlich.

Öffentliche Parameter dürfen kundenseitig nur unter Entwicklerhoheit angelegt oder verändert werden.

**Wichtig:** Öffentliche Parametersätze werden bei einem Update **nicht** gelöscht! Dies geschieht lediglich mit Systemparametern.

ScriptPBezeich

Eine allgemeinverständliche Klartextbezeichnung für die Parametergruppe .

ScriptPId

Die Id, die einen Satz von Skriptparameter zusammenhält (Primary Key) Mit dieser ScriptPId wird in einem Pascal-Skript gekennzeichnet, welche Gruppe von Parametern gewählt werden soll.

ScriptSystem

System-Flag. 0: nicht gesetzt, 1: gesetzt: Die Bearbeitungshoheit für derartige Parametersätze liegt allein im Hause AMIC. Bei einem Update werden alle Datensätze mit ScriptSystem =1 gelöscht und neu angelegt.

**Relation ScriptParamPar**

ScriptParamPar ist die Detailrelation, die durch das Attribut ScriptPId per FOREIGN KEY an die Relation ScriptParam gebunden ist.

Jeder Datensatz ist durch eine ScriptPPId gekennzeichnet, die zusammen mit der ScriptPId eindeutig ist.

Ein Datensatz kann unter einer Id bis zu 3 verschiedene Werte enthalten.

ScriptPId

Bindeglied zwischen Kopfsatz und Detailsätzen.

ScriptPPAktiv

0: Der Parameter ist nicht aktiv, 1: Der Parameter ist aktiv. Nicht zu verwechseln mit einem Löschkennzeichen!

ScriptPPBedKorr

Bedienerkennzeichen, wird auf die UserId des letzten Bearbeiters gesetzt.

ScriptPPBezeich

Eine allgemeinverständliche Klartextbezeichnung für den Parameter.

ScriptPPId

Id zur Kennzeichnung eines Parameters. In Pascal-Skripten werden Parameter anhand dieser ScriptPPId und der ScriptPId aus der Datenbank herausgesucht.

ScriptPPTyp

Informationshalber und als Auswahlkriterium in der zugehörigen AW-Liste **Skript-Parameter** ist der Typ eines Parameter anzugeben. Derzeit sind vorgesehen:

0: *allgemeiner Parameter* (keine besondere Spezifikation)

1: *Konvertierungsparameter* (dient zur Datenkonvertierung)

In ScriptPPWert1 steht die laufende Nummerierung der Parameter zu einer bestimmten Konvertierung.

ScriptPPWert2 enthält den Wert vor der Konvertierung.

In ScriptPPWert3 steht das Ergebnis der Konvertierung.

2: *Positionsparameter Import / Export* (legt Positionen und Längen von Import / Export-Daten fest)

In ScriptPPWert1 steht die Position in der ASCII-Datei.

ScriptPPWert2 enthält die Anzahl der einzulesenden Stellen.

ScriptPPWert3 könnte für die Festlegung von Nachkommastellen verwendet werden, ist im allgemeinen jedoch nicht erforderlich.

In der Formatliste ScriptParTyp sind diese Belegungen hinterlegt.

ScriptPPWert1

Der erste Wert (NOT NULL)

ScriptPPWert2

Der zweite Wert (NOT NULL)

ScriptPPWert3

Der dritte Wert (NOT NULL)

ScriptSystem

System-Flag. 0: nicht gesetzt, 1: gesetzt: Die Bearbeitungshoheit für derartige Parameter liegt allein im Hause AMIC. Systemparameter kann es innerhalb jeder Parametergruppe geben, nicht jedoch in privaten Parametergruppen!

Bei einem Update werden alle Datensätze mit ScriptSystem =1 gelöscht und neu angelegt.
