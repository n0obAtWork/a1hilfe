# Aufruf ohne Auswahlliste

<!-- source: https://amic.de/hilfe/aufrufohneauswahlliste.htm -->

Die JVAR mit dem Owner 1977 kann mit einer Liste kommaseparierter Adressid gefüttert werden. Dann wird die Anzeige des Browsers aufgerufen:

Mit diesem JPL-Code

```text
call JVARS_SET(1977, "AdressIds", "1478,1480,1482,1484,1486")
call CS ("GoogleMapsPoints")
```

wird der Browser mit den markierten Adressen geöffnet.

(1478,1480 usw. stehen für die AdressIds)

Im Pascal-Makro wird der Controlstring aufgerufen

```text
StrCpy(Adress,"294,299,300");
JVarsSet( 1977, "AdressIds", Adress );
if( JPPNEW ( "PFF" , "JExec" ) = 1 ) then
  begin
  sprintf(buf,"^CS GoogleMapsPoints" );
  JPPIN ( "PFF" ,"ctrl"       , buf       );
  JPPDO ( "PFF", "CtrlString" , " " ,2048 );
  JPPDELETE ( "PFF" )
End
```
