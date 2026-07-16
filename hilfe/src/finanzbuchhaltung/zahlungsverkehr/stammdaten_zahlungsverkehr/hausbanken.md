# Hausbanken

<!-- source: https://amic.de/hilfe/hausbanken.htm -->

Hauptmenü > Finanzbuchhaltung > Stammdaten > Hausbanken

Direktsprung **[BNKH]**.

Hausbanken sind die eigenen Banken des jeweiligen Man­danten für den Zahlungsverkehr. Sie werden im Automatischen Zahlungsverkehr, von e-Clearing, der Wechselbuchhaltung und vom Zahlungsverkehr Bank verwendet. Hier werden alle Informationen hinterlegt, die für die Abwicklung der Bankgeschäfte notwendig sind:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td colspan="2">
          <p><strong>Allgemein</strong></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Hausbanknummer</p>
        </td>
        <td>
          <p>Eine frei zu vergebende eindeutige Nummer, über die dann auf die Hausbank verwiesen wird.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Währung</p>
        </td>
        <td>
          <p>Währung, in der das Hausbankkonto geführt wird.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>IBAN</p>
        </td>
        <td>
          <p>Ab dem 28.01.2008 wird für den Zahlungsverkehr SEPA (Single Euro Payments Area) eingeführt. Dieses Verfahren benötigt die IBAN (International Bank Account Number). Diese kann/muss in dem Feld IBAN eingetragen werden. Es erscheint folgender Hinweis, wenn keine IBAN eingetragen wird:</p>
          <p><b>Wenn sie den Zahlungsverkehr ab dem 20.01.2008 auf SEPA-Basis laufen lassen, müssen sie die IBAN eintragen.</b></p>
          <p>Hat man eine IBAN eingetragen, so wird aus dieser (für Deutschland, Österreich und Belgien) die Bank und Kontonummer generiert. Werden diese Daten nicht vorgeschlagen, so ist entweder die IBAN nicht nach dem Standardschema aufgebaut, falsch eingegeben oder die Stammdaten der Banken sind nicht korrekt gepflegt (z.B. nicht eingetragener Staat) .</p>
          <p>Anschließend wird sie über ein Prüfziffernverfahren getestet. Bei fehlerhafter Nummer erscheint folgende Fehlermeldung:</p>
          <p><b>Die Prüfziffernberechnung ergibt, dass diese IBAN falsch ist.</b></p>
          <p>Diese Meldung ist nur eine Warnmeldung. Änderungen werden trotz Meldung abgespeichert.</p>
          <p>Der Test der IBAN kann entweder für einzelne <a href="./bankenstamm.md">Banken</a> oder global per <a href="../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md">Steuerparameter</a> abgeschaltet werden.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Bank</p>
        </td>
        <td>
          <p>Verweis auf die im <a href="./bankenstamm.md">Bankenstamm</a> festgelegte Bank. Man kann direkt die Bezeichnung oder die BLZ eingeben. In der F3-Auswahl kann zusätzlich auch nach BIC oder der Banknummer gesucht werden.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>BIC / BLZ</p>
        </td>
        <td>
          <p>Hier werden der BIC(Bank Identifier Code) und die Bankleitzahl der Bank angezeigt, wie sie im Bankenstamm hinterlegt sind.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Bankkonto</p>
        </td>
        <td>
          <p>Kontonummer des Bankkontos. Hier wird die 10-stellige Kontonummer erwartet. Hat man bis keine IBAN angegeben, so wird soweit eine Kontonummer eingetragen wurde, für deutsche, österreichische und belgische Banken eine IBAN vorgeschlagen. Diese vorgeschlagene IBAN ist in jedem Fall zu überprüfen, da die IBAN ausschließlich von der kontoführenden Bank vergeben wird.</p>
          <p>Die Vorbelegung der IBAN kann per <a href="../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_vorbelegung_nach_standardverfahren_spa_896.md">Steuerparameter</a> abgeschaltet werden.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Matchcode</p>
        </td>
        <td>
          <p>Zusätzliche Möglichkeit ein Suchkriterium festzulegen.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><strong>Datenträgeraustausch</strong></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>SEPA-Version</p>
        </td>
        <td>
          <p>Da das SEPA-Verfahren in ständiger Entwicklung ist, existieren bei unterschiedlichen Banken auch unterschiedliche Versionen. Welche Versionen implementiert sind findet man unter <a href="../sepa/sepa_kennzeichen_im_hausbankenstamm.md">SEPA-Kennzeichen im Hausbankenstamm</a></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>SEPA-Zeichensatz</p>
        </td>
        <td>
          <p>1)&nbsp;&nbsp; In den XML-Dateien darf nur ein eingeschränkter Datensatz verwendet werden. Hier kann A.ens zwischen zwei System unterscheiden:</p>
          <ul>
            <li>Einfacher Zeichensatz mit 0-9, a-z, A-Z sowie den Sonderzeichen ":", "?", ",", "-", " ", "(", "+", ".", ")", "/". Umlaute werde umgewandelt: Beispiel Ä=AE.</li>
            <li>Erweiterter Zeichensatz. Wie der einfache Zeichensatz, aber zusätzlich mit den Zeichen "&amp;", "*", "$", "%". Umlaute werden NICHT umgewandelt.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Sammel-/Einzelaufträge</p>
        </td>
        <td>
          <p>Das SEPA-Verfahren sieht ein Kennzeichen vor, mit dem man der Bank mitteilen kann, ob man Sammel- oder Einzelaufträge im Kontoauszug der Bank erhalten möchte. In A.eins kann das Verhalten pro Hausbank festgelegt werden. Unter Umständen wird das Kennzeichen vom Institut nicht ausgewertet, da jede Bank die Verarbeitung unterschiedlich handhabt (die Deutsche Bank ignoriert z.B. dieses Kennzeichen und führt immer Sammelbuchungen aus.).</p>
          <ul>
            <li>laut Vereinbarung mit der Hausbank. Diese ist die Standardeinstellung.</li>
            <li>Sammelbuchung durchführen</li>
            <li>Einzelbuchung durchführen</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Auftraggeber DTA</p>
        </td>
        <td>
          <p>Beim Datenträgeraustausch wird der hier eingetragene Wert als Auftraggeber übermittelt. Es ist dabei unabhängig, ob es sich um den DTA im SEPA-Format, im alten Format oder um Auslands-DTA handelt.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>AuslandsDTA Name</p>
          <p>AuslandsDTA Str.</p>
          <p>AuslandsDTA Ort</p>
        </td>
        <td>
          <p>Diese Felder werden für den Auslandszahlungsverkehr und im SEPA-DTA verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Ansprechpartner</p>
        </td>
        <td>
          <p>Der Ansprechpartner beim Auslandszahlungsverkehr mit übertragen.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>Konten Finanzbuchhaltung</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Finanzbuchhaltung</p>
        </td>
        <td>
          <p>Dieses Konto wird beim Erstellen der Belege im auomatischen Zahlungsverkehr und beim e-Clearing verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Verrechnungskonto</p>
        </td>
        <td>
          <p>Dieses Konto wird bei Bankbuchungen aus der Wechselbuchhaltung verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Besitzwechsel</p>
          <p>Wechselobligo</p>
          <p>Wechselobligo</p>
        </td>
        <td>
          <p>Diese Konten werden von der <a href="../../wechselbuchhaltung/index.md">Wechselbuchhaltung</a> verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>Nummernkreis</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Schecknummer</p>
        </td>
        <td>
          <p>Man kann den <a href="../zahlungen_bearbeiten/scheckdruck.md">Scheckdruck</a> so Einrichten, dass die Schecknummer aus einem Nummernkreis gezogen wird. Dieser muss hier hinterlegt werden.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Zahlungsverkehr Bank</p>
        </td>
        <td>
          <p>Beim Erstellen der Fibu-Belege werden Belegnummern vergeben. Im Modul <a href="../../e_clearing/index.md">e-Clearing</a> – dort muss unter Optionen die Verwendung dieses Nummernkreises aktiviert werden&nbsp; - und beim Erfassen von Bankzahlungen wird dieser Nummernkreis verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Autom.Zahlungsverkehr</p>
        </td>
        <td>
          <p>Dieser Nummernkreis wird beim Erstellen der Fibubelege aus dem automatischen Zahlungsverkehr verwendet. Dort muss der Einrichterparameter „Nummernkreis der Hausbank verwenden“ auf <b>Ja</b> stehen. Ist die Option „Schecknummer als Belegnummer vergeben“ beim Übertrag in die Primanota gesetzt und eine Schecknummer vergeben, dann wird nach wie vor die Schecknummer verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>Obergrenze/Limits</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Kredit-Limit</p>
        </td>
        <td>
          <p>Wird von A.eins zurzeit nicht ausgewertet. Wird jedoch beim Erfassen von Zahlungen mit angezeigt.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Haben-Obergrenze</p>
        </td>
        <td>
          <p>Wird von A.eins zurzeit nicht ausgewertet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Wechsel-Kontingent</p>
        </td>
        <td>
          <p>Wird von A.eins zurzeit nicht ausgewertet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>Anschrift</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Anschrift</p>
        </td>
        <td>
          <p>Anschrift der Bank. Weitere Anschriftenmerkmale können über F10 gepflegt werden.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>e-Clearing</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Abweichendes Bankkonto e-Clearing</p>
        </td>
        <td>
          <p>Normalerweise sind Kontonummern auf 10 Stellen normiert. Einige Banken übermitteln in den Kontoauszügen für e-Clearing manchmal längere Nummern, in denen z.B. die Filialnummer noch enthalten ist. Diese längere Kontonummer kann hier eingetragen werden und wird dann bei der Bestimmung der Hausbank im Modul e-Clearing verwendet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Auftraggeber aus Verwendungszweck verwenden (MT940)</p>
        </td>
        <td>
          <p>Diese e-Clearing-Option gilt nur für das Dateiformat MT940 – Swift. Wenn eine Bank den Auftraggeber nicht in dem dafür vorgesehenen Feld liefert, so kann man hier einstellen, dass stattdessen die erste Zeile des Verwendungszwecks als Auftraggeber verwendet wird. Diese Option wird nur angewendet, wenn das Feld Auftraggeber leer ist. Zusätzlich kann im Pfleger für die Geschäftsvorfallcodes für einzelne Geschäftsvorfälle diese Option abgeschaltet werden.</p>
          <p>Der Auftraggeber wird für die Bestimmung des Kontos benötigt.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>eRechnung</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Bei eRechnungsversand verwenden</p>
        </td>
        <td>
          <p>Wenn dieses Feld auf „ja“ steht, wird das entsprechende Konto bei der Erstellung einer eRechnung verwendet, andernfalls nicht.<br>Dieses Feld ist nur bei aktiver eRechnung-Lizenz freigeschaltet.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><b>Bemerkungen</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Bemerkungen</p>
        </td>
        <td>
          <p>Frei zu verwendendes Textfeld.</p>
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
