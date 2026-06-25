# Aufbau der Datenbank-Tracedatei

<!-- source: https://amic.de/hilfe/aufbauderdatenbanktracedatei.htm -->

Die erzeugte Datenbank-Tracedatei ist als OSQL-Einspielscript formuliert.

```sql
LOADTUETTEL;
insert into amic_tracefile (TraceZeit,TraceCursorNo,TraceMaske,TraceVerbrauch,     TraceError,TraceCursor,TracePlan,TraceSelect,TraceUser,TraceStatus,TraceTrace) values (%s)
```

| Felder der Datenbank-Tracedatei |
| --- |
| Tracezeit | Zeitstempel |
| TraceCursorNo | |
| TraceMaske | Diese Maske war zum Zeitpunkt aktiv |
| TraceVerbrauch | Zeitverbrauch in Millisekunden |
| TraceError | Rückgabe-Status des Datenbank-Servers auf die Datenbank-Anweisung |
| TraceCursor | |
| TraceSelect | Datenbank-Anweisung |
| TraceUser | |
| TraceStatus | Stati die das technische Umfeld beschreiben |
| TraceTrace | |
