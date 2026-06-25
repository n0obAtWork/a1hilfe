# Automatisierung der Erstellung von Bestandsbuchungen

<!-- source: https://amic.de/hilfe/automatisierungdererstellungvo.htm -->

Da es vorkommen kann, dass die letzten Ein-/Ausgangsbuchungen des Tages auf einen Zeitpunkt fallen, zu dem die Buchhaltung nicht mehr besetzt ist, um die Bestandsbuchungen vorzunehmen kann der Wunsch danach bestehen, dies zu automatisieren.

Mit Hilfe eines Eintrags in die Tabelle „Datenstrom“ ist die Erstellung dieser Belege durch den Mandantenserver möglich. Dieser Eintrag kann wiederum in einem Event geschehen.

```sql
INSERT INTO
DATENSTROM
        (
ds_status,
BedienerId,
DS_DSC,
DS_Id,
DS_Parameter,
ds_RefText
        )
        VALUES
        (
0,
(select first Bedienerid from bedienerstamm where bedienername like
'%MAND%'),
12,
amic_func_dbxident('Datenstrom', 0),
'^CS LVSPermInv_BelegCreator',
'Erstellung PIV-Belege'
);
```
