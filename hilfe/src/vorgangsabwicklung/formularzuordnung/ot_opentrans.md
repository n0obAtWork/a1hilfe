# OT – openTRANS

<!-- source: https://amic.de/hilfe/otopentrans.htm -->

Einstellungen für die Verarbeitung für openTRAN.

openTRANS ist ein Datenformat, das es erlaubt Informationen zu einem Vorgang in einem technischen Standardformat (xml) zu speichern.

### openTRANS aktiv

Gibt an, ob openTRANS-Export aktiv ist. Ist der Kunde dieses Vorgangs ein openTRANS-Kunde, so wird beim Druck des Dokuments ein Export gemacht.

### Pfad für Dateiablage

Pfad für die Ablage eines openTRANS-Exports als Datei. Bei jedem Export wird eine Datei erstellt, die in diesem Pfad abgelegt wird. Der Dateiname kann von A.eins oder über eine optionale Prozedur erstellt werden.

### Prozedur für Dateinamen

Hier kann der Name einer Prozedur angegeben werden, die den Dateinamen für den openTRANS-Export eines Vorgangs bestimmt. Wird kein Name angegeben, so wird der Dateiname des Exports von A.eins bestimmt.

```sql
create procedure p_otdateiname(in in_v_id integer)
result ("V_UKlassOTPath" varchar(1000) )
```

### Profil für Export

Hier wird das Profil angegeben das beim openTRANS-Export verwendet wird. Das Profil können Sie unter dem Direktsprung [OT] definieren.

### OT->EDI-Konverter

Hier kann der Name einer Funktion angegeben werden, die als Eingabeparameter die Vorgangs-ID und ein XML vom Typ long varchar bekommt und dieses verändert im gleichen Typ long varchar wieder zurückgibt. Diese Funktion kann verwendet werden, um nachträglich Informationen in das XML einzubetten, zu verändern oder zu entfernen, die für den Empfänger über den Standard hinaus notwendig oder nützlich oder nicht relevant sind.

```text
create function p_otediconvert(in in_v_id integer, in in_otxml long varchar)
returns  long varchar
```

### ME-Umschlüsselungsprozedur

Hier kann der Name einer privaten Prozedur angegeben werden, die eine Mengeneinheitsnummer aus A.eins übergeben bekommt und eigenständig die dazu gehörige UN-Mengeneinheit und den Umrechnungsfaktor ermittelt. Der Faktor gibt an, in welchem Verhältnis die A.eins-Mengeneinheit zu der UM-Mengeneinheit steht. Werden beispielsweise A.eins-Tonnen (t) in KGM angegeben, so ist der Faktor 1000.

```sql
create procedure ottestmasme( in in_menummer integer )
RESULT (UNME char(3), Faktor decimal(15,4))
begin
  select 'C62', 100 from dummy
end
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_menummer</p>
        </td>
        <td>
          <p>Mengeneinheitsnummer aus A.eins</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UNME</p>
        </td>
        <td>
          <p>UN-Mengeneinheit</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Faktor</p>
        </td>
        <td>
          <p>Umrechnungsfaktor</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### XML-Codierung

Hier wird die Codierung des openTRANS-XML festgelegt. Standardeinstellung ist UTF-16. Diese Einstellung kann jedoch auch gegen andere Codierungen wie z.B. „UTF-8“ oder „ISO-8859-1“ ausgetauscht werden.

### Artikelumschlüsselung

Hier wird eine Prozedur angegeben, die die Umschlüsselung von Artikelnummern zu denen des Rechnungsempfängers übernimmt.

```sql
create procedure p_artiums(in in_herkunft char(8), in in_Artikelid integer, in in_Rechnungskunde integer)
result ("type" varchar(50), value varchar(40) )
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_herkunft</p>
        </td>
        <td>
          <p>Ist „SUPPLIER“ oder „BUYER“ je nachdem für welchen Ausgabetyp diese Prozedur aufgerufen wird</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_ArtikelId</p>
        </td>
        <td>
          <p>ArtikelId, die umgeschlüsselt werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_Rechnungskunde</p>
        </td>
        <td>
          <p>ID des Kunden, für den die Rechnung erstellt werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>type</p>
        </td>
        <td>
          <p>Typbezeichnung der ausgegebenen Artikelnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>value</p>
        </td>
        <td>
          <p>Umgeschlüsselte Artikelnummer</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### Kundenumschlüsselung

Hier wird eine Prozedur angegeben, die die Umschlüsselung von Kundennummern zu denen des Rechnungsempfängers übernimmt. 

```sql
create procedure p_kundums(in in_kundnummer integer, in in_Rechnungskunde integer, in in_partyrole char(255), in in_adressId integer default 0)
result ("type" varchar(50), value varchar(40) )
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_kundnummer</p>
        </td>
        <td>
          <p>Kundennummer, die umgeschlüsselt werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_Rechnungskunde</p>
        </td>
        <td>
          <p>ID des Kunden, für den die Rechnung erstellt werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_partyrole</p>
        </td>
        <td>
          <p>Rolle des Kunden (openTRANS-Notation)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_adressid</p>
        </td>
        <td>
          <p>AdressId der ermittelten Adresse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>type</p>
        </td>
        <td>
          <p>Typbezeichnung der ausgegebenen Kundennummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>value</p>
        </td>
        <td>
          <p>Umgeschlüsselte Kundennummer</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### Vorgang

Hier wird der openTRANS-Vorgangs-Typ angegeben, der beim Export dieses Vorgangs erstellt werden soll.

Zur Verfügung stehen derzeit:

- Angebot
- Bestellung
- Lieferschein
- Rechnung

### Prozedur für UDX-Felder

Es ist in openTRANS möglich, Userdefinierte Felder einzufügen. Gemöß dem openTRANS-Standard finden sich diese im UDX-Header bzw. im UDX-Item. Alle Feldnamen beginnen mit „UDX“.

Hier wird eine Prozedur angegeben, die UDX-Felder für das openTRANS füllt. 

```sql
create procedure p_UDXProc (in in_v_id integer, in in_v_posizaehler integer, in in_otzaehler integer)
result (path varchar(50))
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_id</p>
        </td>
        <td>
          <p>ID des Vorgangs, für den UDX-Felder angegeben werden sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_posizaehler</p>
        </td>
        <td>
          <p>Zähler der Warenposition im Vorgang</p>
          <p>Beim Aufruf für einen Header steht hier eine 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_otzaehler</p>
        </td>
        <td>
          <p>Zähler der Position im openTRANS</p>
          <p>Beim Aufruf für einen Header steht hier eine 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Path</p>
        </td>
        <td>
          <p>Ausgabe eines Pfades zur Ausgabe von UDX-Elementen</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

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
    <UDX.HTML xmlns="http://www.amic.de/openTRANS/UDX/2013">
      <UDX.TEST>
        <UDX.MASTEST h4="45"/>
        <UDX.G4/>
        <UDX.F3>huhu</UDX.F3>
      </UDX.TEST>
    </UDX.HTML>
  </UDX.AEINS.DOCUMENT>
</HEADER_UDX>
```

### Prozedur für Item-Features

In openTRANS® gibt es in den Items der Itemliste der Vorgänge Features, also Eigenschaften, die das Produkt genauer beschreiben sollen.

Um diese in einen openTRANS-Export zu integrieren, kann eine Prozedur angesprochen werden, die den Namen der Eigenschaft und den Wert zurückgeben soll.

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure p_FeatureProc (in in_v_id integer, in in_v_posizaehler integer)
result (bezeichner varchar(1000), wert varchar(1000))
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_id</p>
        </td>
        <td>
          <p>ID des Vorgangs, für den UDX-Felder angegeben werden sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_posizaehler</p>
        </td>
        <td>
          <p>Zähler der Warenposition im Vorgang</p>
          <p>Beim Aufruf für einen Header steht hier eine 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>bezeichner</p>
        </td>
        <td>
          <p>Ausgabe des Names der Eigenschaft</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wert</p>
        </td>
        <td>
          <p>Ausgabe des Wertes der Eigenschaft</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

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

### Prozedur für Partie-Details

In openTRANS® gibt es in den Komponenten der Positionen der Vorgänge Features, also Eigenschaften, die das Komponenten-Produkt genauer beschreiben sollen.

Um diese in einen openTRANS-Export zu integrieren, kann eine Prozedur angesprochen werden, die den Namen der Eigenschaft und den Wert zurückgeben soll.

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure p_PartieFeatureProc (in in_partieid integer)
result (bezeichner varchar(1000), wert varchar(1000))
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_partieid</p>
        </td>
        <td>
          <p>ID der Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>bezeichner</p>
        </td>
        <td>
          <p>Ausgabe des Names der Eigenschaft</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wert</p>
        </td>
        <td>
          <p>Ausgabe des Wertes der Eigenschaft</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

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

### Transformation Stylesheet

Hier kann ein Stylesheet angegeben werden, dass beim openTRANS-Export verwendet wird, um das ausgehende openTRANS in ein anderes XML-Format zu konvertieren.

### Prozedur für MIME-Info

In openTRANS® gibt es in den

Hier wird eine Prozedur angegeben, die MIME-Info für das openTRANS füllt. 

```sql
create procedure p_MIMEProc (in in_v_id integer)
result (filename varchar(1024), type varchar(1024), description varchar(1024), extension varchar(255))
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_id</p>
        </td>
        <td>
          <p>ID des Vorgangs, für den die MIME-Info angegeben werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Filename</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Type</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>decription</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>extension</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

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

### Prozedur für Order_History

In openTRANS® gibt es in nur in den Rechnungen (Invoice) die Kopfinformation ORDER_HISTORY. Diese enthält Informationen über den zugrunde liegenden Auftrag und Lieferschein.

Die Beispiel-Prozedur AMIC_DEMO_OT_ORDER_HISTORY zeigt, wie diese Daten ermittelt werden können.

openTRANS sieht nur eine direkte Linie von Auftrag über Lieferschein zu Rechnung vor, weshalb hier auch nur ein Wert ermittelt werden kann. 

Hier wird eine Prozedur angegeben, die Features-Felder für das openTRANS füllt. 

```sql
create procedure p_OT_OrdHistProc (in in_v_id integer)
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

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Parameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>in_v_id</p>
        </td>
        <td>
          <p>ID des Vorgangs, für den UDX-Felder angegeben werden sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>order_id</p>
        </td>
        <td>
          <p>Nummer des Auftrags aus Sicht des Bestellers</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>alt_customer_id</p>
        </td>
        <td>
          <p>Alternative Auftraggebernummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>supplier_order_id</p>
        </td>
        <td>
          <p>Auftragsnummer des Auftragnehmers (eigenes System)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>order_date</p>
        </td>
        <td>
          <p>Auftragsdatum</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>order_descr</p>
        </td>
        <td>
          <p>Auftragsbeschreibung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>deliverynote_id</p>
        </td>
        <td>
          <p>Nummer des Lieferscheins des Auftragnehmers (eigenes System)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>deliverynote_date</p>
        </td>
        <td>
          <p>Lieferscheindatum</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Das Ergebnis im XML sieht dann so aus:

```text
<ORDER_HISTORY>
  <ORDER_ID>40136667</ORDER_ID>
  <SUPPLIER_ORDER_ID>74413</SUPPLIER_ORDER_ID>
  <ORDER_DESCRIPTION>n/a</ORDER_ DESCRIPTION >
  <DELIVERYNOTE_ID>1122334455</ DELIVERYNOTE_ID>
  < DELIVERYNOTE_DATE>2018-04-16</ DELIVERYNOTE _DATE>
</ORDER_HISTORY>
```
