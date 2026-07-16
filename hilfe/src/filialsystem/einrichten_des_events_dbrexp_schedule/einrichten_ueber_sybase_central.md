# Einrichten über Sybase Central

<!-- source: https://amic.de/hilfe/einrichtenbersybasecentral.htm -->

1. Nach dem Sie sich mit der gewünschten Datenbank verbunden haben, wählen Sie in der Ordneransicht den Punkt „Ereignisse“

2. Klicken Sie diesen mit der RECHTEN Maustaste und wählen „Neu → Ereignis“

3. Im Assistenten zum Erstellen von Ereignissen geben Sie bitte zunächst den Namen „**dbrexp_schedule**“ ein und klicken auf „Weiter“

4. Dieses Ereignis wird „Geplant“ ausgelöst. Auswählen und „Weiter“

5. Geben Sie nun einen Namen des Zeitplans an und klicken auf „Weiter“

6. Geben Sie an, wann das Event ausgeführt werden soll

   a. …auslösen um:

   b. …auslösen zwischen

   c. Optionales Datum wird NICHT benötigt

7. Geben Sie nun den gewünschten Wiederholungsintervall an

   a. Haken bei: Dieses Ereignis auslösen alle z.B. 1 Minuten

8. Klicken Sie auf „Weiter“

9. Je nachdem in welcher Datenbank Sie sich aktuell befinden geben Sie die Option:

   a. Nur in der konsolidierten Datenbank ausführen

   b. Nur in entfernten Datenbanken ausführen

an und klicken auf „Fertig stellen“

10. Geben Sie in der Registerkarte „SQL“ des Ereignisses „dbrexp_schedule“ folgendes ein:

```sql
begin
  call amic_remote_schedule()
exception
  when others then
    call amic_exception(ERRORMSG() || '\\x0A' || TRACEBACK(),sqlcode,sqlstate,'EVENT dbrexp_schedule',-19004)
end
```

11. Speichern Sie die Änderungen über „Datei → Speichern“
