# Protokollstruktur

<!-- source: https://amic.de/hilfe/protokollstruktur.htm -->

Die Protokollstruktur ist abhängig von der Art wie sie gespeichert wird. Zurzeit gibt es nur die Auswahl von XML-Struktur und keine XML-Struktur. Um zu erkennen ob es sich um eine XML-Struktur handelt, enthält die Spalte „protoXML“ der Tabelle Protokoll den Wert „1“.

[XML-Struktur](./protokollstruktur.md#uebson_xml_struktur)

[Keine XML-Struktur](./protokollstruktur.md#uebson_xml_keine_struktur)

<p class="just-emphasize">XML-Struktur</p>

Bei der XML-Struktur werden die protokollierten Daten in einer XML-Struktur gespeichert. Dadurch kann man später gezielter auf die Daten zugreifen.

Beispiel:

```xml
<?xml version="1.0"
encoding="iso-8859-1" ?>
<root>
  <mode>UPDATE</mode>
  <Felder>
    <Field id="KtrId"
label="Ident">
<alt>473543</alt>
<neu>473543</neu>
    </Field>
    <Field id="KtrAbDatum" label="Ab
Datum">
<alt>12-04-2010</alt>
<neu>12-04-2010</neu>
    </Field>
    <Field id="KtrBisDatum"
label="Bis Datum">
<alt>22-04-2010</alt>
<neu>22-04-2010</neu>
    </Field>
    <Field id="KtrBisDatumFix"
label="Bis Datum (max)">
<alt>22-04-2010</alt>
<neu>22-04-2010</neu>
    </Field>
    <Field id="KtrDatum"
label="Datum">
<alt>12-04-2010</alt>
<neu>12-04-2010</neu>
    </Field>
    <Field id="KtrErlediStatus"
label="Erledigungsstatus">
<alt>0</alt>
<neu>0</neu>
    </Field>
    <Field id="KtrNummer"
label="Nummer">
<alt>24332</alt>
<neu>24332</neu>
    </Field>
    <Field id="KtrStornoStatus"
label="Stornostatus">
<alt>0</alt>
<neu>0</neu>
    </Field>
    <Field id="WaehrNummer"
label="Währungsnummer">
<alt>20</alt>
<neu>20</neu>
    </Field>
  </Felder>
</root>
```

Erklärung der XML-Tags

| XML-Tag | Bedeutung |
| --- | --- |
| &lt;root> | Dieser Tag kennzeichnet nur den Start der XML-Struktur. |
| &lt;mode> | Der „Mode“ sagt aus, wodurch die Daten entstanden sind. In dem Beispiel sind die Daten durch ein „Update“ entstanden. |
| &lt;Felder> | Dies ist der Start-Tag unter dem sich alle mitprotokollierten Felder befinden. |
| &lt;Field> | Der „Field“ Tag ist ein Tag für eine einzelne Spalte. Innerhalb des Tags existieren noch der „id“ und „label“ Parameter.<br>*ID = Der Spaltenname aus der Tabelle.*<br>*LABEL = Die Bezeichnung die in dem Stammdaten hinterlegt werden kann.* |
| &lt;alt> | Der Tag enthält den alten Wert. |
| &lt;neu> | Der Tag enthält den neuen Wert. |

Zum Auswerten eines XML-Protokolleintrags könnte folgendes SQL-Statement verwendet werden.

Die Variable „dc_XML_Protokoll“ würde dabei die XML-Struktur enthalten. Nach ausführen des Statements hätte man für jede Spalte eine Zeile.

```sql
SELECT  TriggerType
,FeldName
,FeldBezeichnung
        ,WertAlt
        ,WertNeu
FROM OPENXML(dc_XML_Protokoll, '/root/Felder/Field'
)
WITH  (
WertAlt          LONG
VARCHAR    'alt'
,WertNeu         LONG
VARCHAR    'neu'
,FeldName
CHAR(255)       '@id'
      ,FeldBezeichnung
char(255)       '@label'
,TriggerType
char(255)       '../../mode'
      )
```

Beispielergebnis:

| TriggerType | Feldname | Feldbezeichnung | WertAlt | WertNeu |
| --- | --- | --- | --- | --- |
| UPDATE | KtrId | Ident | 473543 | 473543 |
| UPDATE | KtrErlediStatus | Erledigungsstatus | 0 | 2 |
| UPDATE | WaehrNummer | Währungsnummer | 20 | 1 |

<p class="just-emphasize">Keine XML-Struktur</p>

Werden die Daten nicht in einer XML-Struktur mitprotokolliert, so werden diese als fortlaufender Text gespeichert. Das weiterverarbeiten der Daten ist hierbei nicht so einfach wie bei der XML-Struktur.

Beispiel:

```text
UPD KtrId
471597->471597,KtrAbDatum 07-02-2009->07-02-2009,KtrBisDatum
07-02-2011->07-02-2011,KtrBisDatumFix 07-02-2011->07-02-2011,KtrDatum
04-02-2009->04-02-2009,KtrErlediStatus 0->0,KtrKlasse 1->1,KtrNummer
12->12,KtrStornoStatus 0->0,ParitaetNummer 0->1,WaehrNummer
20->20
```
