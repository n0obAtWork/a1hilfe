# Datendrehscheibe Statistikexport

<!-- source: https://amic.de/hilfe/_event_statistik_export.htm -->

Mit diesem Event kann der Statistikexport automatisiert werden, dazu wird auf der Registerkarte Vorlagen der Schalter Statistikexport auf Ja gesetellt.

```sql
begin
  call Fehlerprotokoll(in_text = 'Start Statistik Export');
  call amic_evt_StatistikExport(0,0);
  call Fehlerprotokoll(in_text = 'Ende Statistik Export')
exception
  when others then call fehlerprotokoll(in_text = 'FEHLER Statistik Export!')
end
```

Der Event erzeugt eine Statistikdatei von der letzten geschlossen Periode die noch nicht Exportiert worden ist.
