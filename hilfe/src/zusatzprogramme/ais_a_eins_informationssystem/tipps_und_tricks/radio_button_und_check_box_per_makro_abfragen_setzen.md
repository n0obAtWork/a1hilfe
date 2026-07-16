# Radio-Button und Check-Box per Makro abfragen/setzen

<!-- source: https://amic.de/hilfe/radiobuttonundcheckboxpermakro.htm -->

Der Zugriff auf den Inhalt von Radio-Button und Check-Boxen funktioniert nicht so, wie bei einfachen Eingabefeldern. Um von einem Makro aus auf den Inhalt von Radio-Buttons bzw. von Check-Boxen zugreifen zu können, muss man mit den Funktionen SM_PROP_ID und SM_PROP_GET_X_INT arbeiten. Es folgen zwei Hilfsfunktionen zum Abfragen und Setzen der Haken:

```text
function BoxStatus(NameBox : String; Zeile : integer):integer;
var
  CheckBox_ObjId: integer;
begin
  CheckBox_ObjId := sm_prop_id(NameBox );
  BoxStatus      := sm_prop_get_x_int(CheckBox_ObjId, zeile, 125);
end;
```

Die Hilfsfunktionen „BoxStatus“ – gültig für Check-Box und Radio-Button – liefert für den Name und die Zeile 0 für nicht ausgewählt und 1 für ausgewählt zurück. 

```text
Procedure SetBoxStatus(NameBox : String; Zeile : integer; Status : integer);
var
  CheckBox_ObjId: integer;
begin
  CheckBox_ObjId := sm_prop_id(NameBox );
  sm_prop_set_x_int(CheckBox_ObjId, zeile, 125, Status);
end;
```

Die Hilfsfunktion „SetBoxStatus“ setzt für Check-Box und Radio-Button die entsprechende Zeile auf den angegebenen Wert (0=Aus, 1=An). Dabei ist bei Radio-Buttons zu beachten, dass alle anderen Zeilen sofort ausgeschaltet werden, wenn man eine neue Zeile aktiviert.
