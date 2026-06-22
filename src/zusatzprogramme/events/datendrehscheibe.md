# Datendrehscheibe

<!-- source: https://amic.de/hilfe/_event_datendrehscheibe.htm -->

Will man den Artikelimport für die Datendrehscheibe automatisieren, kann man auf dem Register Vorlagen hinter Datendrehscheibe ein Ja eintragen. Es wird dann eine von AMIC entwickelte Prozedur in die Verarbeitungsroutine eingetragen. Mit dem Schalter „Mit Artikelimport“ kann entschieden werden, ob beim Einspielen der Terres Dateien die neuen Artikel oder Änderungen mit in den A.eins Artikelstamm übernommen werden sollen. Wird der Schalter auf „Ja“ gestellt, so wird dem Parameter „in_Artikelimport“ eine 1 zugewiesen, bei „Nein“ wird dem Parameter die 0 zugewiesen. Der Standardwert ist 0 .

```sql
begin
  call Fehlerprotokoll(in_text = 'Start
Datendrehscheibe');
  call amic_evt_datendrehscheibe(in_Artikelimport=0);
  call Fehlerprotokoll(in_text = 'Ende
Datendrehscheibe');
exception
  when others
then call fehlerprotokoll(in_text = 'FEHLER
DatenDrehscheibe!')
end
```

Neben einigen Systemprüfungen, geschieht hier folgendes:

Die Dateien, die sich in dem Verzeichnis befinden, das man unter Datendrehscheibe angegeben hat, werden eingelesen.

Wurden Dateien erfolgreich eingespielt, so werden sie - wie unter Datendrehscheibe definiert - weiter verarbeitet.

Eventuell auftretende Probleme findet man im Fehlerprotokoll.
