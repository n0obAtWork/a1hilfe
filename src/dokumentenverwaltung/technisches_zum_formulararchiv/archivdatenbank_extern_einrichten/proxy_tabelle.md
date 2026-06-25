# Proxy-Tabelle

<!-- source: https://amic.de/hilfe/_proxytabelle.htm -->

Das externe Archiv bedient sich des Konzepts sogenannter Proxy-Tabellen. Dadurch wird durch einen logischen Verweis eine Tabelle einer anderen Datenbank eingebunden. Diese Einbindung geschieht mittels Angabe einer ODBC-Verbindung.

In seltenen Fällen kann die händische Auffrischung einer solchen Verbindung von Nöten sein.

Hierzu führe man in OSQL folgendes auf der **Nicht-Archiv-Datenbank** aus:

1. select remote_location from sys.systable where table_name = 'formulararchiv'

Man merke sich den Wert der unter remote_location angegeben ist!

2. Drop table formulararchiv

3. Create existing table admin.formulararchiv at ‘GEMERKTER WERT’

Dies veranlasst das System die Verbindung zu refreshen.

Wie stellt man die externen Server eines Systems fest?

```sql
select srvid from sys.sysservers
```
