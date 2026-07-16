# Beispiel XML Datei

<!-- source: https://amic.de/hilfe/_cescannerxml.htm -->

&lt;?xml version="1.0" encoding="iso-8859-1"?>

&lt;Scans xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" Stop = "false"\>

 &lt;ScanListe>

 &lt;Scan Code="-1" Stop = "false">IV&lt;/Scan>

 &lt;Scan Code="-4" Stop = "false">6666000000001&lt;/Scan>

 &lt;Scan Code="-30" Stop = "false" WaitTime = "500">15&lt;/Scan>

 &lt;Scan Code="-1" Stop = "false" Close = "true">IVENDE&lt;/Scan>

 &lt;/ScanListe>

&lt;/Scans>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>XML Elemente</p>
        </td>
        <td>
          <p>Bedeutung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>&lt;Scans&gt;</p>
        </td>
        <td>
          <p>Kann mehrere Scanlisten enthalten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>&lt;ScanListe&gt;</p>
        </td>
        <td>
          <p>Die ScanListe enthält alle Scanns die für eine komplette Erfassung eines Scannvorgangs nötig sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>&nbsp;&nbsp;&nbsp;&nbsp; &lt;Scan&gt;</p>
        </td>
        <td>
          <p>Enthält die einzelen Scannpositionen</p>
          <p>Attribute</p>
          <table>
            <tbody>
              <tr>
                <th>Attribut</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>Code</td>
                <td>Hier wird der Scanncode angegeben.<br>Folegende Scanncodes sind offiziell möglich.<br>-1<br>Steht für nicht Identfizierten Code. Dieser wird für die Start und Endcodes eines Vorganges benutzt. Dieser kann auch verwendet werden, wenn ein ein EAN 128 Code erfasst werden und per Prozedur SPLIT AI ausgewertet werden soll<br>-4<br>Steht für ein EAN 13 Code<br>-8<br>Steht für ein EAN 8 Code<br>-30 Steht für die Mengeneingabe</td>
              </tr>
              <tr>
                <td>
                  Stop
                  <br>
                  <ul>
                    <li>True</li>
                    <li>False</li>
                  </ul>
                </td>
                <td>Wird das Attribut auf true gesetzt, so wird der ScannCode nicht abgearbeitet</td>
              </tr>
              <tr>
                <td>
                  Close
                  <br>
                  <ul>
                    <li>True</li>
                    <li>False</li>
                  </ul>
                </td>
                <td>Wird das Attribut auf true gesetzt, so wird nach der Abarbeitung die Scannersoftware automatisch geschlossen.<br>Das Attribut muss nicht immer gesetzt werden, der Standardfall ist false.</td>
              </tr>
              <tr>
                <td>
                  WaitTime
                  <br>
                  <ul>
                    <li>Zeit in Millisekunden</li>
                    <li>Vorbelegung = 0</li>
                  </ul>
                </td>
                <td>Ist dieses Attribut größer als Null, entspricht die Wartezeit, die der Scanner nach einem Befehl wartet, &nbsp;dem angebenen Wert in Millisekunden. Ansonsten sind die Wartezeiten standardmäßig 1000(1s) für normale Befehle und 10000(10s) für Ende-Befehle.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>
