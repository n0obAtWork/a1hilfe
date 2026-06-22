# OT – openTRANS

<!-- source: https://amic.de/hilfe/otopentrans.htm -->

Einstellungen für die Verarbeitung für openTRAN.

openTRANS ist ein Datenformat, das es erlaubt Informationen zu einem Vorgang in einem technischen Standardformat (xml) zu speichern.

<p class="just-emphasize">openTRANS aktiv</p>

Gibt an, ob openTRANS-Export aktiv ist. Ist der Kunde dieses Vorgangs ein openTRANS-Kunde, so wird beim Druck des Dokuments ein Export gemacht.

<p class="just-emphasize">Pfad für Dateiablage</p>

Pfad für die Ablage eines openTRANS-Exports als Datei. Bei jedem Export wird eine Datei erstellt, die in diesem Pfad abgelegt wird. Der Dateiname kann von A.eins oder über eine optionale Prozedur erstellt werden.

<p class="just-emphasize">Prozedur für Dateinamen</p>

Hier kann der Name einer Prozedur angegeben werden, die den Dateinamen für den openTRANS-Export eines Vorgangs bestimmt. Wird kein Name angegeben, so wird der Dateiname des Exports von A.eins bestimmt.

```sql
create procedure p_otdateiname(in in_v_id
integer)
result ("V_UKlassOTPath" varchar(1000)
)
```

<p class="just-emphasize">Profil für Export</p>

Hier wird das Profil angegeben das beim openTRANS-Export verwendet wird. Das Profil können Sie unter dem Direktsprung [OT] definieren.

<p class="just-emphasize">OT->EDI-Konverter</p>

Hier kann der Name einer Funktion angegeben werden, die als Eingabeparameter die Vorgangs-ID und ein XML vom Typ long varchar bekommt und dieses verändert im gleichen Typ long varchar wieder zurückgibt. Diese Funktion kann verwendet werden, um nachträglich Informationen in das XML einzubetten, zu verändern oder zu entfernen, die für den Empfänger über den Standard hinaus notwendig oder nützlich oder nicht relevant sind.

```text
create function p_otediconvert(in in_v_id integer, in
in_otxml long varchar)
returns  long varchar
```

<p class="just-emphasize">ME-Umschlüsselungsprozedur</p>

Hier kann der Name einer privaten Prozedur angegeben werden, die eine Mengeneinheitsnummer aus A.eins übergeben bekommt und eigenständig die dazu gehörige UN-Mengeneinheit und den Umrechnungsfaktor ermittelt. Der Faktor gibt an, in welchem Verhältnis die A.eins-Mengeneinheit zu der UM-Mengeneinheit steht. Werden beispielsweise A.eins-Tonnen (t) in KGM angegeben, so ist der Faktor 1000.

```sql
create procedure ottestmasme(
in in_menummer integer )
RESULT (UNME char(3), Faktor decimal(15,4))
begin
  select 'C62', 100 from dummy
end
```

| Parameter |
| --- |
| in_menummer | Mengeneinheitsnummer aus A.eins |
| UNME | UN-Mengeneinheit |
| Faktor | Umrechnungsfaktor |

<p class="just-emphasize">XML-Codierung</p>

Hier wird die Codierung des openTRANS-XML festgelegt. Standardeinstellung ist UTF-16. Diese Einstellung kann jedoch auch gegen andere Codierungen wie z.B. „UTF-8“ oder „ISO-8859-1“ ausgetauscht werden.

<p class="just-emphasize">Artikelumschlüsselung</p>

Hier wird eine Prozedur angegeben, die die Umschlüsselung von Artikelnummern zu denen des Rechnungsempfängers übernimmt.

```sql
create procedure p_artiums(in in_herkunft char(8), in
in_Artikelid integer, in in_Rechnungskunde integer)
result ("type" varchar(50), value varchar(40)
)
```

| Parameter |
| --- |
| in_herkunft | Ist „SUPPLIER“ oder „BUYER“ je nachdem für welchen Ausgabetyp diese Prozedur aufgerufen wird |
| in_ArtikelId | ArtikelId, die umgeschlüsselt werden soll |
| in_Rechnungskunde | ID des Kunden, für den die Rechnung erstellt werden soll |
| type | Typbezeichnung der ausgegebenen Artikelnummer |
| value | Umgeschlüsselte Artikelnummer |

<p class="just-emphasize">Kundenumschlüsselung</p>

Hier wird eine Prozedur angegeben, die die Umschlüsselung von Kundennummern zu denen des Rechnungsempfängers übernimmt. 

```sql
create procedure p_kundums(in in_kundnummer integer,
in in_Rechnungskunde integer, in in_partyrole char(255), in in_adressId integer
default 0)
result ("type" varchar(50), value varchar(40)
)
```

| Parameter |
| --- |
| in_kundnummer | Kundennummer, die umgeschlüsselt werden soll |
| in_Rechnungskunde | ID des Kunden, für den die Rechnung erstellt werden soll |
| in_partyrole | Rolle des Kunden (openTRANS-Notation) |
| in_adressid | AdressId der ermittelten Adresse |
| type | Typbezeichnung der ausgegebenen Kundennummer |
| value | Umgeschlüsselte Kundennummer |

<p class="just-emphasize">Vorgang</p>

Hier wird der openTRANS-Vorgangs-Typ angegeben, der beim Export dieses Vorgangs erstellt werden soll.

Zur Verfügung stehen derzeit:

• Angebot

• Bestellung

• Lieferschein

• Rechnung

<p class="just-emphasize">Prozedur für UDX-Felder</p>

Es ist in openTRANS möglich, Userdefinierte Felder einzufügen. Gemöß dem openTRANS-Standard finden sich diese im UDX-Header bzw. im UDX-Item. Alle Feldnamen beginnen mit „UDX“.

Hier wird eine Prozedur angegeben, die UDX-Felder für das openTRANS füllt. 

```sql
create procedure
p_UDXProc (in
in_v_id integer, in
in_v_posizaehler integer, in in_otzaehler
integer)
result (path varchar(50))
```

| Parameter |
| --- |
| in_v_id | ID des Vorgangs, für den UDX-Felder angegeben werden sollen |
| in_v_posizaehler | Zähler der Warenposition im Vorgang  
Beim Aufruf für einen Header steht hier eine 0 |
| in_otzaehler | Zähler der Position im openTRANS  
Beim Aufruf für einen Header steht hier eine 0 |
| Path | Ausgabe eines Pfades zur Ausgabe von UDX-Elementen |

Soll ein Wert in dem tag stehen, so ist diesem ein „#“ voranzustellen

Solle in Attribut mit einem Wert dargestellt werden, so ist der Name des Attributs und des Wertes mit einem vorangestellten „§“ stehen.

Die Ergebnisse der Prozedur können z.B. so aussehen:

```text
HTML\TEST\MASTEST\§h4§45
HTML\TEST\G4
HTML\TEST\F3\#huhu
```

Das Ergebnis im XML sieht dann so aus:

```xml
<HEADER_UDX>
  <UDX.AEINS.DOCUMENT>
    <UDX.HTML
xmlns="http://www.amic.de/openTRANS/UDX/2013">
      <UDX.TEST>

<UDX.MASTEST h4="45"/>

<UDX.G4/>

<UDX.F3>huhu</UDX.F3>
      </UDX.TEST>
    </UDX.HTML>
  </UDX.AEINS.DOCUMENT>
</HEADER_UDX>
```

<p class="just-emphasize">Prozedur für Item-Features</p>

In openTRANS® gibt es in den Items der Itemliste der Vorgänge Features, also Eigenschaften, die das Produkt genauer beschreiben sollen.

Um diese in einen openTRANS-Export zu integrieren, kann eine Prozedur angesprochen werden, die den Namen der Eigenschaft und den Wert zurückgeben soll.

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure p_FeatureProc
(in in_v_id integer, in in_v_posizaehler integer)
result (bezeichner varchar(1000), wert
varchar(1000))
```

| Parameter |
| --- |
| in_v_id | ID des Vorgangs, für den UDX-Felder angegeben werden sollen |
| in_v_posizaehler | Zähler der Warenposition im Vorgang  
Beim Aufruf für einen Header steht hier eine 0 |
| bezeichner | Ausgabe des Names der Eigenschaft |
| wert | Ausgabe des Wertes der Eigenschaft |

Das Ergebnis im XML sieht dann so aus:

```xml
<PROD_ORDER_ITEM>
 (…)
  <PRODUCT_FEATURES>
    <FEATURE>

<bmecat:FNAME>Farbe</bmecat:FNAME>

<bmecat:FVALUE>Grau</bmecat:FVALUE>
    </FEATURE>
  </PRODUCT_FEATURES>
(…)
</PROD_ORDER_ITEM>
```

<p class="just-emphasize">Prozedur für Partie-Details</p>

In openTRANS® gibt es in den Komponenten der Positionen der Vorgänge Features, also Eigenschaften, die das Komponenten-Produkt genauer beschreiben sollen.

Um diese in einen openTRANS-Export zu integrieren, kann eine Prozedur angesprochen werden, die den Namen der Eigenschaft und den Wert zurückgeben soll.

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure
p_PartieFeatureProc (in in_partieid
integer)
result (bezeichner varchar(1000), wert
varchar(1000))
```

| Parameter |
| --- |
| in_partieid | ID der Partie |
| bezeichner | Ausgabe des Names der Eigenschaft |
| wert | Ausgabe des Wertes der Eigenschaft |

Das Ergebnis im XML sieht dann so aus:

```xml
<PRODUCT_COMPONENT>
 (…)
  <PRODUCT_FEATURES>
    <FEATURE>

<bmecat:FNAME>Farbe</bmecat:FNAME>

<bmecat:FVALUE>Grau</bmecat:FVALUE>
    </FEATURE>
  </PRODUCT_FEATURES>
(…)
</PRODUCT_COMPONENT>
```

<p class="just-emphasize">Transformation Stylesheet</p>

Hier kann ein Stylesheet angegeben werden, dass beim openTRANS-Export verwendet wird, um das ausgehende openTRANS in ein anderes XML-Format zu konvertieren.

<p class="just-emphasize">Prozedur für MIME-Info</p>

In openTRANS® gibt es in den

Hier wird eine Prozedur angegeben, die MIME-Info für das openTRANS füllt. 

```sql
create procedure p_MIMEProc
(in in_v_id integer)
result (filename varchar(1024), type varchar(1024), description
varchar(1024), extension varchar(255))
```

| Parameter |
| --- |
| in_v_id | ID des Vorgangs, für den die MIME-Info angegeben werden soll |
| Filename | |
| Type | |
| decription | |
| extension | |

Das Ergebnis im XML sieht dann so aus:

```xml
<MIME_INFO>
   <MIME>

<bmecat:MIME_TYPE>application/pdf</bmecat:MIME_TYPE>

<bmecat:MIME_SOURCE>_6213841.pdf</bmecat:MIME_SOURCE>

<bmecat:MIME_DESCR>Beleg</bmecat:MIME_DESCR>
   </MIME>
 </MIME_INFO>
```

<p class="just-emphasize">Prozedur für Order_History</p>

In openTRANS® gibt es in nur in den Rechnungen (Invoice) die Kopfinformation ORDER_HISTORY. Diese enthält Informationen über den zugrunde liegenden Auftrag und Lieferschein.

Die Beispiel-Prozedur AMIC_DEMO_OT_ORDER_HISTORY zeigt, wie diese Daten ermittelt werden können.

openTRANS sieht nur eine direkte Linie von Auftrag über Lieferschein zu Rechnung vor, weshalb hier auch nur ein Wert ermittelt werden kann. 

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure
p_OT_OrdHistProc (in in_v_id integer)
result (

order_id varchar(250),
  alt_customer_id  varchar(250),
  supplier_order_id varchar(250),
  order_date varchar(50),
  order_descr varchar(300),
  deliverynote_id varchar(250),
  deliverynote_date varchar(50)
)
```

| Parameter |
| --- |
| in_v_id | ID des Vorgangs, für den UDX-Felder angegeben werden sollen |
| order_id | Nummer des Auftrags aus Sicht des Bestellers |
| alt_customer_id | Alternative Auftraggebernummer |
| supplier_order_id | Auftragsnummer des Auftragnehmers (eigenes System) |
| order_date | Auftragsdatum |
| order_descr | Auftragsbeschreibung |
| deliverynote_id | Nummer des Lieferscheins des Auftragnehmers (eigenes System) |
| deliverynote_date | Lieferscheindatum |

Das Ergebnis im XML sieht dann so aus:

```text
<ORDER_HISTORY>
  <ORDER_ID>40136667</ORDER_ID>

<SUPPLIER_ORDER_ID>74413</SUPPLIER_ORDER_ID>
  <ORDER_DESCRIPTION>n/a</ORDER_
DESCRIPTION >
  <DELIVERYNOTE_ID>1122334455</
DELIVERYNOTE_ID>
  < DELIVERYNOTE_DATE>2018-04-16</
DELIVERYNOTE _DATE>
</ORDER_HISTORY>
```
