# Eventprozeduren

<!-- source: https://amic.de/hilfe/eventprozeduren.htm -->

Wenn ein Event einen unerwartet langen Lauf hat, so dass gleich nach Beendigung das nächste Event startet, dann kann dies zu großer Last auf der Datenbank führen. Unglücklicherweise lassen sich laufende Events nicht deaktivieren.

Deshalb gibt es eine Sollbruch-Stelle. In Eventprozeduren wird zu Beginn eine Abfrage eingebaut, die bestätigt, ob die Prozedur überhaupt ausgeführt werden soll. So kann sichergestellt werden, dass der nächste Lauf des Events nur kurz ist und eine Abbruchmöglichkeit vorliegt.

Der Code in der Prozedur sollte in etwa so aussehen:

```sql
DECLARE NOGO char(255);
    --Pruefen
ob diese Prozedur laufen darf –
    --wenn
dieser Prozedurname nicht in der AMIC_EVT_STOP-Tabelle steht
    select
Eventprocedurename INTO NOGO from AMIC_EVT_STOP
      where Eventprocedurename= 'AMIC_EVT_Backup_Database' ;
    if
(SQLSTATE = err_notfound) THEN
--eigentliche Bearbeitung
    END IF;
```

In der Eventeinrichtung kann eine Eventprozedur in der Registerkarte „Sonstiges“ getoppt werden. Diese Funktion nimmt den Eintrag in die verwendete Tabelle AMIC_EVT_STOP vor.
