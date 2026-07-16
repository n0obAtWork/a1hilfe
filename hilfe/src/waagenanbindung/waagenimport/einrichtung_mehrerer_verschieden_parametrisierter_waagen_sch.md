# Einrichtung mehrerer verschieden parametrisierter Waagen-Schnittstellen auf einer Kundendatenbank

<!-- source: https://amic.de/hilfe/einrichtungmehrererverschieden.htm -->

Dies ist grundsätzlich möglich. Hierfür ist generell folgende Vorgehensweise erforderlich:

Für jede Schnittstelle wird in der Relation ScriptParam ein Datensatz mit einer bestimmten ScriptPId angelegt. Die ScriptPId muss mit ‚p_‘ beginnen und der ScriptPBesitzer auf 1 gesetzt werden, damit bei einem Update nichts zufällig versehentlich gelöscht wird.

Für jede Schnittstelle werden die Datensätze aus der Relation ScriptParamPar dupliziert und mit einer neuen ScriptPId gemäß 1. versehen. (Entladen der Relation und manuelle Umschlüsselung des Attributes ScriptPId durch Suchen und Ersetzen).

 Für den Aufruf des Pascal-Scriptes ist für jede Schnittstelle eine private Funktion zu schaffen, die als 3. Aufrufparameter die betreffende ScriptPId enthält.
