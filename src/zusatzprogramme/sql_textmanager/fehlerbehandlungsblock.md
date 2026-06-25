# Fehlerbehandlungsblock

<!-- source: https://amic.de/hilfe/_sql_textmanager_whenothers.htm -->

Bei der Neuanlage von privaten Triggern und privaten Prozeduren, wird automatisch ein Fehlerbehandlungsblock eingetragen. Dieser sollte nicht entfernt werden, da ansonsten Totalabstürze der Prozedur oder des Triggers an den Kunden weitergeben werden.

Lässt man den Behandlungsblock in der Prozedur / Trigger, dann sollte beim Testen beachtet werden, dass jeglicher Fehler ins Fehlerprotokoll eingetragen wird.

Standardmäßig werden keine Parameter mit ins Fehlerprotokoll geschrieben. Man sollte diese hinzufügen, damit man bei Fehlern anhand der übergebenen Parameter das Problem erneut provozieren kann.

Beispiel für zusätzlichen Parameter:

```text
|| amic_func_sprachtexte ('a','b','Parameter (%s): %s', -1, 'Param1', Param1)
|| '\n'
```

Ein fertiger Block würde dann wie folgt aussehen

```sql
EXCEPTION
  when others then
    Select  ERRORMSG(),SQLCODE,SQLSTATE
    into    dc_ErrorMsg,dc_SQLCODE,dc_SQLSTATE;
call AMIC_FEHLERPROT( 20
  ,amic_func_sprachtexte('a', 'b', 'Prozedur', -1)
  ,amic_func_sprachtexte('a','b','Beim Ausführen der Prozedur "%s" ist ein Fehler aufgetreten.', -1, 'p_TestProzedur')
  || '\n\n'
  || amic_func_sprachtexte('a','b','Parameter (%s): %s', -1, 'in_Param1', in_Param1)
  || '\n'
  || amic_func_sprachtexte ('a','b','Parameter (%s): %s', -1, 'in_ Param2', in_Param2)
  || '\n'
  || amic_func_sprachtexte('a','b','Parameter (%s): %s', -1, 'in_ Param3', in_Param3)
  || '\n'
  || 'SQLCODE: ' || dc_SQLCODE
  || ' [' || dc_SQLSTATE || ']'
  || '\n'
  || dc_ErrorMsg
  ,-10171);
End;
```
