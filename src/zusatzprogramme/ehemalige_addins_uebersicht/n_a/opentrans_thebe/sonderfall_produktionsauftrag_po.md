# Sonderfall Produktionsauftrag (PO)

<!-- source: https://amic.de/hilfe/_info_sonder_otimport.htm -->

Der Standard openTRANS ist für den Austausch von Handelsvorgängen vorgesehen. Ein Produktionsauftrag gehört nicht zu dem Umfang des Standards.

Da A.eins diesen Prozess jedoch abbilden sollte, ist ein Protokoll zum Austausch von Produktionsaufträgen und Status erschaffen worden, der sich im Wesentlichen an dem bestehenden der openTRANS®-Struktur ORDER orientiert, jedoch zur Unterscheidung „PROD_ORDER“ (kurz PO) genannt wird.

###### Einrichtung Produktionsauftrag

<p class="just-emphasize">SPA 850 und die Formularzuordnung</p>

Da mit Steuerparameter [Steuerparameter 850 – Belegänderungssperre durch Beteiligung von openTRANS](../../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegaenderungssperre_durch_beteiligung_von_opentrans_spa_85.md) die Einstellungsmöglichkeit besteht, Belege gegen Bearbeitung zu sperren, für die bereits ein openTRANS erstellt worden ist, kann in der [Formularzuordnung auf der Registerkarte SPA](../../../../vorgangsabwicklung/formularzuordnung/spa.md) diese Sperre abgeschaltet werden. Dies wird für die Vorgangsklasse 5220 (Produktion) empfohlen.

<p class="just-emphasize">Prozeduren</p>

Auf der [Registerkarte openTRANS (OT) in der Formularzuordnung](../../../../vorgangsabwicklung/formularzuordnung/ot_opentrans.md) sind eine Reihe spezifischer Einstellungen für die Vorgangsklasse 5220 (Produktion) einzurichten.

OpenTRANS ist zu aktivieren und es sind diverse Prozeduren zur Ermittlung Produktionsspezifischer Daten individuell einzurichten. Diese hier aufzuzählen käme der Beschreibung eines Individualfalles gleich, der bei Neueinrichtung sicher nicht vorliegt.

Als Beispiel seien jedoch UDX, Partie-Details, Item-Features, Umschlüsselungen und Dateinamen genannt.

###### Kopfinformationen

Kopfinformationen eines Produktionsauftrages werden im PO-Header eingetragen.

Einige der Informationen lassen sich in das PO-Format einfügen, andere werden in den UDX-Header übernommen.

| Kopfinformationen im UDX-Header |
| --- |
| Bestellnummer KWS | UDX_HEADER/UDX.PROD_ORDER_NO |
| Positionsnummer KWS | UDX_HEADER/UDX.PROD_ORDER_POS |
| Status | UDX_HEADER/UDX.STATE |
| Vorgangstexte | UDX_HEADER/UDX.TEXT |
| Liste der Vorgangstexte | UDX_HEADER/UDX.TEXT/UDX.TEXT Sequence=(1….n) |
| Produktionsart | UDX_OPERATION |
| | |

<p class="just-emphasize">Beispiel:</p>

Eine vom Auftraggeber an den Auftragnehmer eingehende Nachricht im PO-Format wird mit dem Status „PROD_NEW“ gesendet.

z.B. Status im Pfad PROD_ORDER/PROD_ORDER_HEADER/PROD_ORDER_INFO/ HEADER_UDX/UDX.AEINS.DOCUMENT/UDX.STATE

Mögliche Status sind :

| Mögliche Status |
| --- |
| PROD_NEW | Nur eingehende Nachrichten |
| PROD_DTCHANGE | Änderung des Plandatums |
| PROD_BEGIN | Beginn einer Produktion |
| PROD_END | Produktionsfertigstellung |
| PROD_INFO | Eingang von Qualitätsdaten<br>Änderung von Daten (außer Plandatum) |

| Weitere Kopfinformationen |
| --- |
| Bestelldatum | PROD_ORDER_INFO/PROD_ORDER_DATE |
| Produktionsdatum (Plandatum) | PROD_ORDER_INFO/DELIVERY_DATE/DELIVERY_START_DATE |

###### Positionsinformationen

Positionsinformationen eines Produktionsauftrages werden in der PO-Item-List eingetragen.

Komponenten und Produktionsprodukte werden in dieser Itemliste gleichrangig behandelt und nur durch ein Kennzeichen unterschieden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <p>Artikelnummer</p>
        </td>
        <td>
          <p>PRODUCT_ID/SUPPLIER_PID bzw. BUYER_PID</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikeltext</p>
        </td>
        <td>
          <p>PRODUCT_ID/DESCRIPTION_LONG</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kennzeichen Komp./Prod.</p>
        </td>
        <td>
          <p>PRODUCT_ID/ITEM_UDX/UDX.ITEM_TYPE</p>
          <p>Werte „PRODUCT“ oder „COMPONENT“ möglich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menge</p>
        </td>
        <td>
          <p>PRODUCT_ID/QUANTITY</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit (KGM/UN/C62)</p>
        </td>
        <td>
          <p>PRODUCT_ID/ORDER_UNIT</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Qualitätsdaten oder weitere Individuelle Positionsinformationen***</p>
        </td>
        <td>
          <p>PRODUCT_FEATURES/FEATURE/FNAME</p>
          <p>PRODUCT_FEATURES/FEATURE/FVALUE</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

\*\*\* Eine Feature-Definition

```xml
<PRODUCT_FEATURES>
    <FEATURE>
        <bmecat:FNAME>HEUBACHWERT</bmecat:FNAME>
        <bmecat:FVALUE>99999</bmecat:FVALUE>
    </FEATURE>
    <FEATURE>
        <bmecat:FNAME>ANERKENNUNGSNUMMER</bmecat:FNAME>
        <bmecat:FVALUE>DE-0000-00000/00</bmecat:FVALUE>
    </FEATURE>
</PRODUCT_FEATURES>
```

###### Chargeninformationen

Werden in den Komponenten Chargennummern angegeben, die den gleichen Artikel betreffen, so können diese auf zweierlei Weise mitgeteilt werden:

1. Es werden zwei Positionen angegeben und es werden jeweils in deren Features die Chargennummern als Feature genannt.

2. Es wird eine Position mit zwei PRODUCT_COMPONENTS angegeben, in deren Features sich die Chargennummern als Feature finden.

```xml
<PRODUCT_COMPONENTS>
  <PRODUCT_COMPONENT>
<PRODUCT_ID><bmecat:SUPPLIER_PID
type="supplier_specific">9101</bmecat:SUPPLIER_PID>
<LOT_NUMBER>1021005-0</LOT_NUMBER>
<bmecat:DESCRIPTION_SHORT>Produktionskomponente
9101</bmecat:DESCRIPTION_SHORT>
<bmecat:DESCRIPTION_LONG>Produktionskomponente
9101</bmecat:DESCRIPTION_LONG>
    </PRODUCT_ID>
    <PRODUCT_FEATURES>
<FEATURE><bmecat:FNAME>CHARGEN_NUMMER</bmecat:FNAME><bmecat:FVALUE>0815</bmecat:FVALUE>
      </FEATURE>
    </PRODUCT_FEATURES>
<QUANTITY>5.0000</QUANTITY>
<bmecat:ORDER_UNIT>C62</bmecat:ORDER_UNIT>
  </PRODUCT_COMPONENT>
(…)
</PRODUCT_COMPONENTS>
```
