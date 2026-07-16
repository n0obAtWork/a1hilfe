# Fibu-Übertrag von Rohware-Einzelbelegen

<!-- source: https://amic.de/hilfe/_rwb_fib_einzel.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung: Fibu Übertrag Rohware Einkauf

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung: Fibu Übertrag Rohware Verkauf

Direktsprung **[RWBV]**

Die Standard-Auswahlliste zum Fibu-Übertrag von Rohware-Einzel-Abrechnungen enthält abgerechnete Rohwarebelege entsprechend der getroffenen Bereichseinschränkungen. Belege, die Teil eines Sammeldruck-Belegs sind, werden hier nur aufgeführt, wenn der Rohwareparameter [Sammelbuchungen bei Sammeldruck](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_122) mit dem Wert ‚**Nein**‘ belegt ist. Zur besseren Übersicht werden die folgenden Inhalte dargestellt:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Auswahlliste Fibu-Übertrag (Einzelabrechnungen)</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>VFKtr</p>
        </td>
        <td>
          <p>Beleg mit Vorfakturierungskontrakt (Ja/Nein)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegdatum</p>
        </td>
        <td>
          <p>Rechnungsdatum des Belegs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegnummer</p>
        </td>
        <td>
          <p>Rechnungsnummer des Belegs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Klasse</p>
        </td>
        <td>
          <p>ER für Eingangsrechnung<br>ERS für Stornoeingangsrechnung<br>AR für Ausgangsrechnung<br>ARS für Stornoausgangsrechnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fib</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th colspan="2"><b>Fibu-Übertrag-Kennzeichen</b></th>
              </tr>
              <tr>
                <td><b>--</b></td>
                <td>noch nicht übertragen</td>
              </tr>
              <tr>
                <td><b>i.B.</b></td>
                <td>in Bearbeitung, Übertrag läuft gerade</td>
              </tr>
              <tr>
                <td><b>ja</b></td>
                <td>Beleg ist schon übertragen</td>
              </tr>
              <tr>
                <td><b>nn</b></td>
                <td>Beleg kann nicht übertragen werden (schon weiterverarbeitet, storniert oder Stornobeleg eines nicht übertragenen Belegs)</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontonummer</p>
        </td>
        <td>
          <p>Kunden-/Lieferanten-Nummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunde/Lieferant</p>
        </td>
        <td>
          <p>Kunden-/Lieferantenname</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LiefNr.</p>
        </td>
        <td>
          <p>Lieferscheinnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lief.Dat.</p>
        </td>
        <td>
          <p>Lieferscheindatum</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wiegenummer</p>
        </td>
        <td>
          <p>Nummer des Wiegescheins</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Filiale</p>
        </td>
        <td>
          <p>Filialnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Status</p>
        </td>
        <td>
          <p>Abrechnungs-Stufe<br>(Abschlag, Folgeabschlag, Finale)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sperren</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th colspan="2"><b>Sperrkennzeichen des Belegs</b></th>
              </tr>
              <tr>
                <td><b>B</b></td>
                <td>Bearbeitungssperre</td>
              </tr>
              <tr>
                <td><b>K</b></td>
                <td>Sperre wegen Kreditlimitüberschreitung</td>
              </tr>
              <tr>
                <td><b>W</b></td>
                <td>Weiterverabeitungssperre</td>
              </tr>
              <tr>
                <td><b>F</b></td>
                <td>Fibu-Übertrag-Sperre</td>
              </tr>
              <tr>
                <td><b>R</b></td>
                <td>Rechnungseingangsbuch-/Rechnungsausgangsbuch-Sperre</td>
              </tr>
              <tr>
                <td><b>U</b></td>
                <td>Umwandlungsperre</td>
              </tr>
              <tr>
                <td><b>f</b></td>
                <td>Filial-Sperre</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Druckkennzeichen</p>
        </td>
        <td>
          <p>Kennzeichen, ob der Belege bereits gedruckt wurde</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ab</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th colspan="2"><b>Abschlag-Status-Kennzeichen</b></th>
              </tr>
              <tr>
                <td><b>--</b></td>
                <td>Ohne Abschlag</td>
              </tr>
              <tr>
                <td><b>Ab</b></td>
                <td>Abschlag abgerechnet</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>FAb</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th colspan="2"><b>Folge-Abschlag-Status-Kennzeichen</b></th>
              </tr>
              <tr>
                <td><b>--</b></td>
                <td>Ohne Folge-Abschlag</td>
              </tr>
              <tr>
                <td><b>Sp</b></td>
                <td>Folge-Abschlag vorgesehen aber noch nicht freigegeben</td>
              </tr>
              <tr>
                <td><b>Fr</b></td>
                <td>Folge-Abschlag zur Abrechnung freigegeben</td>
              </tr>
              <tr>
                <td><b>Ab</b></td>
                <td>Folge-Abschlag abgerechnet</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fin</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th colspan="2"><b>Final-Abrechnungs-Status-Kennzeichen</b></th>
              </tr>
              <tr>
                <td><b>Sp</b></td>
                <td>Finale noch nicht zur Abrechnung freigegeben</td>
              </tr>
              <tr>
                <td><b>Fr</b></td>
                <td>Finale zur Abrechnung freigegeben</td>
              </tr>
              <tr>
                <td><b>Ab</b></td>
                <td>Finale ist abgerechnet</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>NV</p>
        </td>
        <td>
          <p>Anzahl der Nachvergütungsbelege</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Einlagerung</p>
        </td>
        <td>
          <p>Einlagerungskennzeichen (nur im Einkauf)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vereinnahmung</p>
        </td>
        <td>
          <p>Vereinnahmungskennzeichen (nur im Einkauf)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abrplan über Jahreswechsel</p>
        </td>
        <td>
          <p>Kennzeichen für Abrechnung mit ProForma-Beleg über Jahreswechsel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ist Proforma</p>
        </td>
        <td>
          <p>Kennzeichen für Proforma-Abrechnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Regel</p>
        </td>
        <td>
          <p>Vergebene Arbeitsregel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erl.</p>
        </td>
        <td>
          <p>Erledigungskennzeichen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lager</p>
        </td>
        <td>
          <p>Lagernummer des Belegs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelnummer</p>
        </td>
        <td>
          <p>Artikelnummer der Lieferwarenposition des Belegs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>RwGr.</p>
        </td>
        <td>
          <p>Nummer der Rohwarengruppe</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>RwGr-Bezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung der Rohwarengruppe</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Schema</p>
        </td>
        <td>
          <p>Nummer des Abrechnungsschemas</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Schemabezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung des Abrechnungsschemas</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erstbediener</p>
        </td>
        <td>
          <p>Kurzbezeichnung des 1. Bedieners, der den Fibu-Übertrag angefordert hat. (Nur bei Vier-Augen-Prinzip, siehe Steuerparameter <a href="../../../firmenstamm/steuerparameter/fibu_uebertrag_warenwirtschaft/kein_fibuuebertrag_vieraugenprinzip_mit_mitarbeitern_aus_zwe.md">Kein Fibuübertrag (Vieraugenprinzip) mit Mitarbeitern aus zwei Abteilungen</a> )</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zustimmungsbediener</p>
        </td>
        <td>
          <p>Kurzbezeichnung des Bedieners, der den angeforderten Fibu-Übertrag bestätigt hat. (Nur bei Vier-Augen-Prinzip, siehe Steuerparameter <a href="../../../firmenstamm/steuerparameter/fibu_uebertrag_warenwirtschaft/kein_fibuuebertrag_vieraugenprinzip_mit_mitarbeitern_aus_zwe.md">Kein Fibuübertrag (Vieraugenprinzip) mit Mitarbeitern aus zwei Abteilungen</a> )</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erfasser</p>
        </td>
        <td>
          <p>Kurzbezeichnung des Erfassers</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Die Funktion ‚***Fibu-Übertrag***‘ überprüft jeden gewählten Beleg hinsichtlich der notwendigen Voraussetzungen für die Übertragung an die Finanzbuchhaltung und erzeugt, wenn diese nicht erfüllt sind, entsprechende Fehlermeldungen. Andernfalls wird der Beleg für den Fibu-Übertrag gekennzeichnet und der Mandanten-Server-Prozess angewiesen, die Übertragung vorzunehmen.
