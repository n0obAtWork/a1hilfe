# Einrichtung der Inventur in A.eins

<!-- source: https://amic.de/hilfe/_identassenrichtungaeins.htm -->

Hauptmenü > Inventur

Grundsätzliches zur [Inventur](../../../abschluesse_inventur/inventur/index.md).

Als erstes muss die zu erfassende Inventur in A.eins geöffnet werden.

Damit die Inventur richtig durchgeführt werden kann müssen zwei Steuerparameter gesetzt werden und eine Zuordnung der ScannerID zu einem Bediener hergestellt werden.

Es werden die Steuerparameter 809 und 810 ausgewertet.

Mit dem Steuerparameter 809 kann dem Scanner eine Inventurgruppe zugeordnet werden. Wird keine Inventurgruppe zugeordnet so wird die Inventurgruppe 0 als Standard gewählt.

Mit dem Steuerparameter 810 kann ausgewählt werden, in welche Tabelle die erfassten Daten gespeichert werden sollen

1. Nur MDEUebergabe

2. MDEUebergabe und Inventurbeleg

3. Inventurbeleg

Sollen die Daten in MDEUebergabe und Inventurbeleg gespeichert werden, so werden alle Datensätze die in der Tabelle Inventurbeleg geschrieben worden sind in der Tabelle MDEUebergabe als verarbeitet dargestellt.

Datensätze die einen Fehler erzeugt haben und nicht in Inventurbeleg gelandet sind, können aus MDEUebergabe in die Inventur übertragen werden.

Sollen die Daten nur im Inventurbeleg gespeichert werden, werden fehlerhafte Datensätze in die Tabelle MDEUebergabe geschrieben.

Die ScannerIP Adresse des Scanners wird im Bedienerstamm im Feld **Name Extern** hinterlegt. Dadurch kann einem Scanner ein Bediener zugwiesen werden.

Des Weiteren wird beim Einspielen der Daten die Lagernummer aus [VKONS] des jeweiligen Bedieners gelesen. Dies bedeutet, dass beim Lagerwechsel während der Inventur die Lagernummer unter VKONS neu für den Bediener eingestellt werden muss.
