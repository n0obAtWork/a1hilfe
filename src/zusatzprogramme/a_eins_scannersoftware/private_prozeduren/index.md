# Private Prozeduren

<!-- source: https://amic.de/hilfe/_cescannerprozeduren.htm -->

Eine Private Prozedur erhält diese Übergabe Parameter. Der Kopf der Prozedur sieht so aus:

```sql
Create procedure
p_meine_procedure( in in_Aktionstyp integer,
in in_aktionswert char(255),
in in_ident integer,
in in_positionsIdent integer,
in in_scannernummer char(40),
in in_kommando_scanident integer,
in in_AnzahlImBlock integer,
in in_Blockzaehler integer,
in in_letzte_aktion integer,
in in_Aktionstext char(100),
in in_Kopftext1 char(100),
in in_Kopftext2 char(100),
in in_reaktionstyp char(5),
in in_lagernummer integer,
in in_bedienerid integer,
in in_protokoll char(100),
in in_feldid integer,
in in_scanident integer,
in in_klassnummer integer,
in in_nummer integer,
in in_testflag integer,
in in_diese_positionsnummer integer)
```

| Parameter | Erklärung |
| --- | --- |
| in_Aktionstyp | Enthält dem von Scannerzurückgegeben AI Code |
| in_aktionswert | Enthält den gescannten Wert |
| in_ident | Enthält den Aktuellen Ident der Realtion DatenStromScanner |
| in_positionsident | |
| in_scannernummer | Enthält die Aktuelle Scannernummer des Bearbeiters |
| in_kommando_scanident | in_kommando_scanident |
| in_AnzahlImBlock: | |
| in_Blockzaehler: | |
| in_letzte_aktion: | |
| in_Aktionstext. | Enthält die erste Zeile die auf dem Scannerdisplayangezeigt wird |
| in_Kopftext1: | Enthält die zweite Zeile die auf dem Scannerdisplayangezeigt wird. |
| in_Kopftext2: | Enthält die dritte Zeile die auf dem Scannerdisplayangezeigt wird |
| in_reaktionstyp | |
| in_lagernummer: | Enthält die Lagernummer des Bedieners |
| in_bedienerid | Enthält die BedienerId des Bedieners |
| in_protokoll | |
| in_feldid | |
| in_scanident | |
| in_klassnummer | Enthält die Klassennumer des Vorgangs |
| in_nummer | Enthält die Belegnummer |
| in_testflag | Tesflag |
| in_diese_positionsnummer | Enthält die Aktuelle Zeilennummer der Angezeigten Daten in der Anzeige des Scanners |

<p class="siehe-auch">Siehe auch:</p>

- [Beispiel Partiesperre](./beispiel_partiesperre.md)
