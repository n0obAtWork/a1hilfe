# Fibu-Übertrag von Rohware-Sammeldruck-Belegen

<!-- source: https://amic.de/hilfe/_rwb_fib_sammel.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung: Fibu Übertrag Sammeldruck Einkauf

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung: Fibu Übertrag Sammeldruck Verkauf

Direktsprung **[RWBV]**

Mit dieser Auswahllisten-Variante können Rohware-Sammeldruck-Belege entsprechend der getroffenen Bereichseinschränkungen geschlossen an die Finanzbuchhaltung übergeben werden, wenn der Rohwareparameter [Sammelbuchungen bei Sammeldruck](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_122) mit dem Wert ‚**Ja**‘ belegt ist.  
Dargestellt wird hier je eine Zeile pro Sammeldruck-Beleg. Sind Einzelbelege zu einem Sammeldruck-Beleg jedoch nicht zu einem Fibu-Beleg zusammenfassbar, so wird hier pro entstehendem Fibu-Beleg eine Zeile dargestellt. Das ist insbesondere dann der Fall, wenn die zum Sammeldruck gehörenden Einzelbelege unterschiedliche Vorgaben der zu verwendenden Buchungsperioden oder verschiedene Zahlungsziele haben! Es kann also vorkommen, dass ein Sammeldruck-Beleg in der Finanzbuchhaltung in mehrere Belege aufgeteilt wird!  
Es werden in der Standard-Auswahlliste die folgenden Inhalte dargestellt:  
    

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Auswahlliste Fibu-Übertrag Sammeldruck</strong></p>
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
          <p>SBel.Datum</p>
        </td>
        <td>
          <p>Sammelbeleg-Datum = Druckdatum der zugehörigen Einzelbelege als Sammeldruck</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Drucknummer</p>
        </td>
        <td>
          <p>Sammeldruck-Nummer = Belegnummer des Sammeldruck-Belegs</p>
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
          <p>Belege</p>
        </td>
        <td>
          <p>Anzahl der zugehörigen Einzelbelege</p>
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
          <p>VFKtr</p>
        </td>
        <td>
          <p>Anzahl der Einzelbelege Beleg mit Vorfakturierungskontrakt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Währung</p>
        </td>
        <td>
          <p>Bei Fremdwährungsbelegen die Nummer der Währung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Valuta</p>
        </td>
        <td>
          <p>Berechnetes Zahlungsziel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>StGrp</p>
        </td>
        <td>
          <p>Steuergruppe des Belegs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Jahr</p>
        </td>
        <td>
          <p>Wirtschaftsjahr der Belege</p>
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
          <p>Liste der Kurzbezeichnungen der Erfasser der Einzelbelege</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Die Funktion ‚***Fibu-Übertrag***‘ überprüft jeden gewählten Beleg hinsichtlich der notwendigen Voraussetzungen für die Übertragung an die Finanzbuchhaltung und erzeugt, wenn diese nicht erfüllt sind, entsprechende Fehlermeldungen. Andernfalls wird der Beleg für den Fibu-Übertrag gekennzeichnet und der Mandanten-Server-Prozess angewiesen, die Übertragung vorzunehmen.
