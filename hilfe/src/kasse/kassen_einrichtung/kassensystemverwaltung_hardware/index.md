# Kassensystemverwaltung (Hardware)

<!-- source: https://amic.de/hilfe/_kass_einr_hardw.htm -->

Jeder logischen Kasse ist ein Kassensystem zugeordnet. Dieses Kassensystem beschreibt die Hardwareeinheit.

Durch diese Trennung ist es möglich eine komplette Hardwareeinheit (Kassensystem) mit ihrer hardwarespezifischen Einrichtung auszutauschen und an einem definierten Arbeitsplatz (logische Kasse) mit seinen Regeln und Spezifikationen einzusetzen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-Kopfdaten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kassensystemnummer</p>
        </td>
        <td>
          <p>Nummer des Hardware-Systems</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung des Kassensystems</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anlagedatum</p>
        </td>
        <td>
          <p>Anzeige des Datums der Anlage dieses Kassensystems</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Änderung am</p>
        </td>
        <td>
          <p>Anzeige des Datums der letzten Änderung</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-Drucker</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rechnungsdrucker</p>
        </td>
        <td>
          <p>Drucker, auf dem Rechnungsbelege (keine Bons) gedruckt werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-Schublade.</strong></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung Schubladentyp</p>
        </td>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <p>Anschlusstyp</p>
        </td>
        <td colspan="2">
          <p>Eine Schublade kann am Drucker angeschlossen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anschluss ist</p>
        </td>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Port</li>
          </ul>
        </td>
        <td colspan="2">
          <p>z.B. COM1 oder LPT1</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Verbindungsparameter</li>
          </ul>
        </td>
        <td colspan="2">
          <p>z.B. 9600,n,8,1 (Baud, Parity, Data-, Stopbits)</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Puffergröße Eingang</li>
          </ul>
        </td>
        <td colspan="2">
          <p>Ist 1024</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Puffergröße Ausgang</li>
          </ul>
        </td>
        <td colspan="2">
          <p>Ist 1024</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Druckertyp (enth. Steuerseq)</p>
        </td>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Typ</li>
          </ul>
        </td>
        <td colspan="2">
          <p>Normal oder Win7/2008</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Druckertyp</li>
          </ul>
        </td>
        <td colspan="2">
          <p>Druckertyp aus den Druckertypen, die mit dem Direktsprung [DRT] gepflegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Druckertyp-Bezeichnung</li>
          </ul>
        </td>
        <td colspan="2">
          <p>Anzeige der Bezeichnung des Druckertyps</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuersequenz</p>
        </td>
        <td colspan="2">
          <p>Hier wird, wenn vorhanden eine Steuersequenz zu diesem Druckertyp angezeigt. Ist noch keine Steuersequenz zu dem Druckertyp hinterlegt, an dem Sie die Schublade anschließen wollen, so muss diese hier eingetragen werden. Diese gilt dann für alle Kassensysteme, die für den Anschluss der Schublade diesen Druckertyp verwenden.</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-Display</strong></p>
          <p>Anzeige auf einem zweizeiligen Kundendisplay</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Displaytyp</p>
        </td>
        <td>
          <p>Texteintrag um welches Kassendisplay es sich handelt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuersequenz</p>
        </td>
        <td>
          <p>Das Display verfügt typerweise über 20 Zeichen pro Zeile. Eine längere Zeichenkette würde in der 2. Zeile fortgesetzt, die Daten nach einer kürzeren Zeichenkette an der aktuellen Stelle fortgesetzt werden. Um zu steuern, welche Zeile die Daten darstellen soll, werden Steuersequenzen aus nicht druckbaren Zeichen verwendet. Diese werden hier hexadezimal angegeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>1. Zeile</li>
          </ul>
        </td>
        <td>
          <p>Steuersequenz, die dem Display angibt, dass die nachfolgenden Zeichen in der 1. Zeile dargestellt werden sollen</p>
          <p>z.B. 1B3D020C0B</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>2. Zeile</li>
          </ul>
        </td>
        <td>
          <p>Steuersequenz, die dem Display angibt, dass die nachfolgenden Zeichen in der 1. Zeile dargestellt werden sollen</p>
          <p>z.B. 1F4218</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anschlusstyp</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Generic</li>
          </ul>
        </td>
        <td>
          <p>Die bisherigen Einstellungen erlaubten COM-LPT und Dateisystem in einem Feld einzugeben. (Diese Einstellung wird als Generic bezeichnet).</p>
          <p>Beginnt die Zielbezeichnung mit „c“ oder „C“, so wird angenommen, dass es sich um einen COM-Port handelt</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>COM</li>
          </ul>
        </td>
        <td>
          <p>Für spätere Verwendungen geplant</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>LPT</li>
          </ul>
        </td>
        <td>
          <p>Für spätere Verwendungen geplant</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Dateisystem</li>
          </ul>
        </td>
        <td>
          <p>Für spätere Verwendungen geplant</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>TCPIP</li>
          </ul>
        </td>
        <td>
          <p>Bei TCPIP kann ein „Serial-Device-Server“ zum Einsatz kommen, um ein Display mit einer seriellen Schnittstelle an diesem im Netzwerk an einer Ethernet-Schnittstelle zu betreiben. Hier sind die IP-Adresse und der Port des Servers einzugeben. Die Verbindungsparameter für die Kommunikation zwischen Server und Display müssen in diesem selbst konfiguriert werden.</p>
          <p>z.B. „192.168.1.12:950“</p>
          <p>siehe auch <a href="../unterstuetze_hardware/serial_device_server.md">unterstützte Hardware</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Display Device</p>
        </td>
        <td>
          <p>Angaben zum Anschluss</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Port</li>
          </ul>
        </td>
        <td>
          <p>COM-Port</p>
          <p>z.B. COM1</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Einstellungen</li>
          </ul>
        </td>
        <td>
          <p>Baudrate 9600, 8 Datenbits, keine Parität und 1 Stopbit</p>
          <p>z.B. 9600,8,n,1</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Puffergröße Eingang</li>
          </ul>
        </td>
        <td>
          <p>Ist 1024</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Puffergrüße Ausgang</li>
          </ul>
        </td>
        <td>
          <p>Ist 1024</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-EC-Cash-Verfahren</strong></p>
          <p>Bargeldlose Zahlung mit EC-Karte mit nachfolgend beschriebenen Optionen</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Lastschrift</li>
          </ul>
        </td>
        <td>
          <p>Hier werden die Daten für die Lastschrift aus dem Kartenleser gewonnen. Eine manuelle Eingabe der Daten ist nicht vorgesehen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Lastschrift auch manuelle Eingabe</li>
          </ul>
        </td>
        <td>
          <p>Hier können neben der Erfassung der Daten für die Lastschrift aus dem Kartenleser auch manuelle Eingaben gemacht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Bezahlterminal verwenden</li>
          </ul>
        </td>
        <td>
          <p>Aktiviert die nachfolgenden Einstellungen zum Bezahlterminal</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kassensystem-EC-Cash-Einstellungen Bezahlterminal</strong></p>
          <p>Verwendung eines Bezahlterminals mit den nachfolgenden Optionen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Manuelle Bedienung</li>
          </ul>
        </td>
        <td>
          <p>Hier wird der Betrag manuell in das Terminal eingegeben, es erfolgt keine Kommunikation mit A.eins. In A.eins wird lediglich die Abwicklung der Bezahlung bestätigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <ul>
            <li>Ansteuerung über Makro</li>
          </ul>
        </td>
        <td>
          <p>Ein Makro steuert das Bezahlterminal über die ZVT-700-Schnittstelle an. A.eins wartet auf eine Rückmeldung vom Bezahlterminal, ob der Bezahlvorgang erfolgreich war.</p>
          <p>Siehe hierzu auch Liste der unterstützten Geräte. <a href="../vorbelegung_kassennummer.md">Bezahlterminals</a></p>
          <p>Siehe auch <a href="../../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/bezahlterminal_metis/index.md">Bezahlterminal-Ansteuerung</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerungsmakro</p>
        </td>
        <td>
          <p>Name des Makros, das von AMIC_BZT_MUSTER abgeleitet ist und die Ansteuerung des Bezahlterminals vornimmt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Geräte einrichten](./geraete_einrichten/index.md)
- [Druckereinrichtung](./druckereinrichtung/index.md)
