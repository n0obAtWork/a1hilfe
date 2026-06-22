# Registerkarte Ladeort-Transp.-Bef.Route in Zollausfuhr

<!-- source: https://amic.de/hilfe/registerkarteladeorttranspbefr.htm -->

Die Registerkarte „Ladeort/Transp./Bef.Route“ ist für die Zollausfuhr ebenfalls auszufüllen.

Der Abschnitt „**Ladeort**“ definiert den Ort des Beladens der zu versendenden Ware sowie die beteiligte Ausfuhrzollstelle und Ausgangszollstelle.

Diese Angaben sind vorbelegt durch die Angaben im Lagerstamm des Lagers, aus dem die letzte in der Auswahlliste angeklickte Position dieser Sammlung geladen wird.

Soll eine alternative Prozedur zur Ermittlung des Lagers verwendet werden, so kann diese im Einrichterparameter „alternative Ladeort Vorbelegungs-Prozedur“ hinterlegt werden.

Ein Beispiel für die Prozedur kann sein:

```sql
create procedure mas_test_proc(V_ID integer, V_POSIZAEHLER integer)
BEGIN
  select 'MEINLager' as
Ladezusatz,
  'MeineStrasse 1'
as AdressStrasse,
  '12345'
as AdressPLZ,
  'Meinestadt'
as AdressOrt,
  'DE002101'
as AusfuhrZollstelle
  from DUMMY
END
```

Auch diese Daten sind bis zum Versenden der Ausfuhr änderbar.

| Parameter | Bedeutung |
| --- | --- |
| Bezeichnung / Zusatz | Zusatz zur Angabe des Ortes, an dem die Beladung der Ware auf das Transportmittel stattfindet |
| Straße | Straße des Ortes, an dem die Beladung der Ware auf das Transportmittel stattfindet |
| Ort | Ort, an dem die Beladung der Ware auf das Transportmittel stattfindet |
| Postleitzahl | Postleitzahl des Ortes, an dem die Beladung der Ware auf das Transportmittel stattfindet |
| Ausfuhrzollstelle | Deutsche Zollstelle, welche den Transport der Ware verwaltet |
| Ausgangszollstelle | Zollstelle, über welche die Ware die EU verlässt  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |

Der Abschnitt „**Transport**“ enthält wichtige Angaben zum Warentransport.

| Parameter | Bedeutung |
| --- | --- |
| Verkehrszweig Inland | Verkehrszweig, welcher für den Transport der Ware im Inland verwendet wird  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |
| Verkehrszweig Grenze | Verkehrszweig, welcher für den Transport der Ware ab dem Überschreiten einer EU-Grenze verwendet wird  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |
| Beförderungsmittel Grenze | Art des Beförderungsmittels, welches für den Transport der Ware ab dem Überschreiten einer EU-Grenze verwendet wird  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |
| Nat. Beförderung Grenze | Nationalität des Beförderungsmittels, welches für den Transport der Ware ab dem Überschreiten einer EU-Grenze verwendet wird  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |
| Beförderungsmittel Grenze Kennz. | Kennzeichen des Beförderungsmittels, welches für den Transport der Ware ab dem Überschreiten einer EU-Grenze verwendet wird  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |

Der Abschnitt „**Beförderungsroute**“ enthält die Angaben zur Route des Warentransports.  
Unter Berücksichtigung von Ausfuhrland, Ausgangszollstelle und Bestimmungsland kann die Beförderungsroute mit der Funktion „Beförderungsroute initialisieren“ berechnet werden.

| Parameter | Bedeutung |
| --- | --- |
| Ausfuhrland | Das Ausfuhrland wird mit DE für Deutschland vorbelegt und kann zurzeit nicht geändert werden. |
| Bestimmungsland | Das Bestimmungsland der Sendung.  
Die Vorbelegung erfolgt aus den Zolldaten zur Anschrift (Versandanschrift oder Hauptanschrift) des Vorgangs/Kunden |
| Tabelle der Beförderungsroute | Liste der Staaten auf der Beförderungsroute |
