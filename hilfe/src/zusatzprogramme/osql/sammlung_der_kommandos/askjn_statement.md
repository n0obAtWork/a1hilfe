# ASKJN Statement

<!-- source: https://amic.de/hilfe/askjnstatement.htm -->

#### Syntax

ASKJN Beschreibungstext;

#### Purpose

Interaktive Ja/Nein Abfrage.

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[ASK](./alter_struct_statement.md)

#### Beschreibung

Mit dem ASKJN Statement kann die Standartdialogbox aufgerufen werden, um eine Benutzerabfrage durchzuführen. Die Variable LDB_JN enthält je nach Auswahl den Wert 1 für JA oder 2 für NEIN. Drück man den Abbruchbutton beim ASKJN-Statement innerhalb einer Kommandodatei, wird diese beendet.

#### Beispiel

ASKJN Drück aufs Knöpfchen, Max;

if(val(LDB_JN)==1)

{

 PAUSE Max hat Ja gedrückt!;

};

if(val(LDB_JN)==2)

{

 PAUSE Max hat Nein gedrückt!;

};

PAUSE Abbruch kommt hier nie an;
