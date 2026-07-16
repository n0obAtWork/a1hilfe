# LVS / MDE-Workflow

<!-- source: https://amic.de/hilfe/_mde_workflow.htm -->

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > XXXXXXXXXXXXXXXXXX

Direktsprung **[LVSWF]**

Mit dem LVS-Workflow kann ein Arbeitsablauf beschrieben werden, bei dem der Scanner Daten sammelt. Bedingung ist die Einrichtung der Prozedur „AMIC_LVS_WORKFLOW_SCANNER“ im Steuerparameter [801 – private Scanenerprozedur](./steuerparameter/scanner/private_scannerprozedur_spa_801.md) für die IP des Scanners.

Ausgehend vom Startzustand hat der Scanner den Status „0“. In diesem Status kann ein Barcode gescannt werden. Diesen kategorisiert die Prozedur „AMIC_LVS_GETSCANTYPE“ in eine Kategorie und schreibt Werte in die Tabelle „TCPIP_SCANNER“.

Analog dazu kann in Steuerparameter [1029 – LVS Workflow Prozeduren](./steuerparameter/allgemeine_programmsteuerung/lvs_workflow_prozeduren_spa_1029.md) in der Option „LVS-Kommandos“ eine private Workflow-Funktion hinterlegt werden, die weitere Schlüsselwörter erkennt.

Wird also nun ein Kommandowort oder ein Zifferntyp erkannt, so wird von der Prozedur ggf. der Wert in die Tabelle „TCPIP_SCANNER“ eingetragen und der Scantyp zurückgegeben.

So kann z.B. v#12345/500/0/2019 als der Ladeschein 12345 identifiziert werden. Der Scantyp ist „VORG“ und in „TCPIP_SCANNER“ steht nun die V_NumNummer, die V_KlassNummer und die V_UKlassnummer und Jahrnummer.

Es kann aber auch ein Kommandowort wie z.B. „WARENEINGANENDE“ vom Scanner als solches erkennt werden.

Im Workflow wird nun für jeden Status der mögliche Satz an Scantypes festgelegt und die Prozedur, die danach aufgerufen werden soll.

So kann z.B. in Status 0 eine NVE (erkennbar an der Länge 20, numerisch, beginnend mit der Ziffernfolge 00) eine Informationsprozedur aufrufen.

Der Folgestatus bestimmt, welcher Status nach dem Scannen dieses Typs gesetzt wird. So kann eine Reihenfolge der Eingabe festgelegt werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="7">
          <p><strong>Workflow Beispiel</strong></p>
          <p>WARENEINGANG – WARENBEWEGUNG – NVE + MENGE</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Status</p>
        </td>
        <td>
          <p>Sort</p>
        </td>
        <td>
          <p>Feldtyp</p>
        </td>
        <td>
          <p>Feldname</p>
        </td>
        <td>
          <p>Feldbezeichnung</p>
        </td>
        <td>
          <p>Prüf-u. Anzeigepr.</p>
        </td>
        <td>
          <p>Folgest.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>WARENEINGANG</p>
        </td>
        <td>
          <p>&lt;null&gt;</p>
        </td>
        <td>
          <p>&lt;null&gt;</p>
        </td>
        <td>
          <p>P_WE</p>
        </td>
        <td>
          <p>100</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>100</p>
        </td>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>WARENBEWEGUNG</p>
        </td>
        <td>
          <p>Letzter_Wert</p>
        </td>
        <td>
          <p>Position</p>
        </td>
        <td>
          <p>P_WE</p>
        </td>
        <td>
          <p>100</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>100</p>
        </td>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>NVE</p>
        </td>
        <td>
          <p>Ladetraegernummer</p>
        </td>
        <td>
          <p>Ladeträgernummer</p>
        </td>
        <td>
          <p>P_WE</p>
        </td>
        <td>
          <p>100</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>100</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>MENGE</p>
        </td>
        <td>
          <p>Gewicht1</p>
        </td>
        <td>
          <p>Menge</p>
        </td>
        <td>
          <p>P_WE</p>
        </td>
        <td>
          <p>0</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Der Status 0 ist der Initialstatus. Wird hier nun „WARENEINGANG“ gescannt, wo wird der Status auf 100 gesetzt.

Am Ende der Ausgabe wird angezeigt „Position eingeben bzw. scannen“ . Das Wort Position findet sich in der Feldbezeichnung des ersten Eintrags im Status 100. Nun wird ein Scan erwartet, der das Feld „letzter_wert“ in „TCPIP_SCANNER“ füllt, damit der Arbeitsschritt weitergeschaltet werden kann.

Sind schließlich alle drei Felder in TCPIP_Scanner gefüllt, so wird der Status auf 0 zurückgesetzt, wenn die Prozedur „P_WE“ den Status „OK“ zurückgegeben hat.

Die Prozedur „P_WE“ kann hier Berechnungen anstellen, Ausgaben machen oder Buchungssätze in den Vorgangsimport schreiben.

Beispiel-Signatur:

```sql
---<summary>LVS Scannerei</summary>
---<returns>default scanner</returns>
---<param name="in_Tcpip_Adresse">Adresse des Scanners</param>
---<param name="in_AktionsWert">gescannter Wert</param>
---<param name="in_scantype">Scantyp aus der Typ-Prozedur</param>
---<param name="in_status">Status</param>
create PROCEDURE P_BUK_INFO
(
  in in_Tcpip_Adresse char(40) default '',
  in in_AktionsWert char(1024) default '',
  in in_scantype char(20) default '',
  in in_status integer default 0
)
--
--
--
  RESULT( okay integer,
          "statustext" long varchar,
          "StatusMelodie" char( 100),
          "Aktionstext" char(1024),
          "Kopftext1" char( 100),
          "Kopftext2" char( 100) )
```
