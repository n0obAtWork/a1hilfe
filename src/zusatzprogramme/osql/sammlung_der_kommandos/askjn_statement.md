# ASKJN Statement

<!-- source: https://amic.de/hilfe/askjnstatement.htm -->

<p class="just-emphasize">Syntax</p>

ASKJN Beschreibungstext;

<p class="just-emphasize">Purpose</p>

Interaktive Ja/Nein Abfrage.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[ASK](./alter_struct_statement.md)

<p class="just-emphasize">Beschreibung</p>

Mit dem ASKJN Statement kann die Standartdialogbox aufgerufen werden, um eine Benutzerabfrage durchzuführen. Die Variable LDB_JN enthält je nach Auswahl den Wert 1 für JA oder 2 für NEIN. Drück man den Abbruchbutton beim ASKJN-Statement innerhalb einer Kommandodatei, wird diese beendet.

<p class="just-emphasize">Beispiel</p>

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
