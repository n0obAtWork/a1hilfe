# SQL Befehl an Datenbank absenden

<!-- source: https://amic.de/hilfe/sqlbefehlandatenbankabsenden.htm -->

Die über den Standardhandle geöffnete Datenbank wird mit diesem Aufruf angesprochen und es wird ein Dynamic SQL Befehl an die Datenbank abgegeben. Es sind alle SQL Befehle erlaubt, der erste parameter dieser Methode legt den ggf. zu nutzenden Cursor fest.

Anfruf

call dbx_select ( cursor, sqlstatement, tempanzeiger, option )

Parameter

t:2 Cursor Name des zu nutzenden Datenbankcursors, hier ist ein eindutiger Text anzugeben. Schon benutzte oder noch in Benutzung befindliche Cursor werden vor dem Einsatz in einem neuen dbx_select geschlossen Ausdruck Es wird hier das SQL Statement erwartet, es sind select, Insert, delete aber auch create Befehle erlaubt Temp optionaler Parameter, der angibt, ob der cursor nur in diesem Befehl gültig sein soll. opt nicht freigegebener Parameter

Returnwert

In der Globalen Variablen DBERR wird der Fehlerzustand dieses Befehls zurückübermittelt 0=OK, 1=Fehler, (in diesem fall ist dann die Globale Variable LDB_SQLERROR zur weiteren Verarbeitung auszuwerten.

Umfeld

Diese Routine ist im JPL und im COM Interface nutzbar.

Beispiel

 call dbx_select ( "x", "insert into a (a) values ('x')", "TMP" )

 if ( DBERR != 0 )

 {

 call smx_warnung ( "SID", "Der Inserbefehl (:LASTDBX_SELECT) ist mit dem Fehler (:LAST_SQLERRORTEXT) schiefgegangen" )

 }

 call dbx_freecursor ( "x" )
