# Excelimport [EXCELI]

<!-- source: https://amic.de/hilfe/_83_32891.htm -->

Seit der Version 8.3.2202.7 wurde beim Excelimport [EXCELI] von .xlsx - und .xlsm- Dateien im SQL-Text der erstellten Variante ein "SELECT :FIELDS" verwendet. Diese Änderung wurde zurückgebaut. Jetzt werden im SQL-Text wieder alle Spalten einzeln selektiert. Des Weiteren werden "long-varchar" - Spalten im SQL-Text als char(255) zurückgegeben. 

### Releasenote Kategorie:

Ticket: 714258[32891]

Version: 8.3.2210.20

Datum: 20.10.2022

Anwendung: Excel-Import

Variante: -

Funktion/Report: Variante aktualisieren

[Weitere Informationen](http://www.amic.de/hilfe/excelimport.htm)

#### Tags:

Releasenote, 8.3.2210.20, 32891, 714258
