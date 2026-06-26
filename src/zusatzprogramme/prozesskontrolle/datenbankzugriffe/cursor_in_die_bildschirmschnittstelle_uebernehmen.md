# Cursor in die Bildschirmschnittstelle übernehmen

<!-- source: https://amic.de/hilfe/cursorindiebildschirmschnittst.htm -->

Der mit dem DBX_SELECT Befehl angesprochenen Datenbankcursor wird aus der Cursorstruktur in die Maskenstruktur übertragen.

**Anfruf**

call dbx_get_buf ( cursor, Bildschirm_Handle, von_BS_Position, bis_BS_Position )

**Parameter**

t:2 Cursor Name des zu nutzenden Datenbankcursors, hier ist ein eindutiger Text anzugeben. BildschirmHandle alle Bildschirmfelder mit diesem Alias (a.) Namen werden über diesen Befehl angesprochen vonpos Ab welcher Bildschirmposition ( 0=alle ). bispos Bis zu welcher Bildschirmposition ( 0=alle ).

**Returnwert**

keiner, Fehler werden auch nich Reportet

**Umfeld**

Diese Routine ist im JPL und im COM Interface nutzbar.

**Beispiel**

 call dbx_select ( "x", "select db_kundid", "TMP" )

 if ( DBERR != 0 )

 {

 call dbx_get_buf ( "x", "h", 0, 0 )

 }

 call dbx_freecursor ( "x" )
