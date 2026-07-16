# Kundenbanken

<!-- source: https://amic.de/hilfe/_kundenbanken.htm -->

Hauptmenü > Stammdaten > Konstanten Kundenstamm > Kundenbanken

oder Direktsprung **[KUBA]**

Alle Grunddaten der Banken, mit denen das System zu tun hat, werden im allgemeinen Bankenstamm hinterlegt, egal ob es sich um eigene Hausbanken oder um Banken von Kunden, Lieferanten, Vertretern... handelt. Hierdurch wird vermieden, dass immer wiederkehrende Informationen, wie die Bankleitzahl, wiederholt werden müssen.

Die spezifischen Eingaben der Kundenbankdaten werden im Auswahlbildschirm Kundenbank <strong>erfasst,</strong> der über den Direktsprung **[KUBA]** zu erreichen ist.

In der Stammdatenmaske können folgende Daten eingegeben werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunde</p>
        </td>
        <td>
          <p>Hier kann aus dem Kundenstamm ein Kunde ausgewählt werden, für den die folgenden Einstellungen gemacht werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>IBAN</p>
        </td>
        <td>
          <p>Die „International Bank Account Number“ - kurz IBAN- wird im Zahlungsverkehr immer wichtiger. In dem ab dem 28.01.2008 gestarteten SEPA Verfahren wird sie an Stelle der Kontonummer verwendet. Bei der Erfassung der Kundenbanken wird die IBAN für deutsche, österreichische und belgische Banken anhand eines Prüfzifferverfahrens überprüft.</p>
          <p>Der Test der IBAN kann entweder für jede <a href="../../finanzbuchhaltung/zahlungsverkehr/stammdaten_zahlungsverkehr/bankenstamm.md">Bank</a> oder global per <a href="../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md">Steuerparameter</a> abgeschaltet werden.</p>
          <p>In der IBAN ist die Bankleitzahl und Kontonummer enthalten. Anhand der Bankleitzahl wird der Bankenstamm durchsucht und dann die Bank und Kontonummer eingetragen. Wird keine Bank vorgeschlagen ist entweder der Bankenstamm nicht korrekt gepflegt oder die IBAN ist nicht korrekt aufgebaut.</p>
          <p>Die IBAN kann nachträglich über ein Funktion „Generiere IBAN“ für alle Kundenbanken mit eingetragener Bank und Kontonummer erzeugt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bank</p>
        </td>
        <td>
          <p>Banknummer und Text. Eine Auswahl mit <strong>F3</strong> ist möglich</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BIC / BLZ</p>
        </td>
        <td>
          <p>Hier wird die BIC(Swift) der Bank wird angezeigt. Steht der <a href="../../firmenstamm/steuerparameter/optionen_global/bankleitzahl_und_kontonummer_anzeigen_spa_1121.md">Steuerparameter</a> 1121 „Bankleitzahl und Kontonummer anzeigen“ auf <b>Ja</b>, so wird hier auch die BLZ angezeigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontonummer</p>
        </td>
        <td>
          <p>Kontonummer des Bankkontos. Dieses Feld wird nur angezeigt, wenn der <a href="../../firmenstamm/steuerparameter/optionen_global/bankleitzahl_und_kontonummer_anzeigen_spa_1121.md">Steuerparameter</a> 1121 „Bankleitzahl und Kontonummer anzeigen“ auf <b>Ja</b> steht.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Empfänger/Zahlungspflichtiger</p>
        </td>
        <td>
          <p>Hier kann ein abweichender Empfänger eingetragen werden. Ist dieses Feld leer, so wird beim DTA die Bezeichnung des Kunden/Lieferanten herangezogen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beschreibung</p>
        </td>
        <td>
          <p>Hier kann ein Informationstext hinterlegt werden, um ggf. anderen Mitarbeitern die Bedeutung der Bank zu erläutern.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>&nbsp;Währung</p>
        </td>
        <td>
          <p>Nummer und Text der Währung, in der dieses Konto geführt wird. Eine Auswahl mit <strong>F3</strong> ist möglich</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sperre des Kontos</p>
        </td>
        <td>
          <p>Wenn ja, ist das Konto für weitere&nbsp;Verarbeitungen&nbsp;gesperrt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Standardbank</p>
        </td>
        <td>
          <p>Wenn zu einem Kunden/Lieferanten mehrere Banken eingerichtet sind, so wird die Bank, bei der hier <b>Ja</b> eingetragen ist beim automatischen Zahlungsverkehr verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gültig bis</p>
        </td>
        <td>
          <p>Termin, zu dem das Konto ausläuft.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Max. Abbuchbetrag</p>
        </td>
        <td>
          <p>Maximale Abbuchung auf ein Kunden-/Lieferantenkonto. Bei mehreren Konten werden die Beträge ggf. geteilt. Ist nur ein Konto vorhanden, erfolgt die Gesamtbuchung auf jedem Fall auf das eine Konto. Findet beim automatischen Zahlungsverkehr Berücksichtigung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Max. Einzahlbetrag</p>
        </td>
        <td>
          <p>Maximaler Einzahlungsbetrag auf ein Kun­den/Lieferantenkonto.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Zu den Kundenbanken existieren [Einrichterparameter](../../firmenstamm/einrichterparameter/kundenbanken_epa_tbkuba.md), die die Erfassungsmöglichkeiten steuern.

#### Kontraktabtretungskonto

Unter Umständen kann es dazu kommen, dass die Banknummer und Kontonummer nicht mehr geändert oder das Konto gelöscht werden darf. Dieser Fall liegt vor, wenn in einem Kontrakt ein Abtretungskonto eingetragen, der Kunde in der Kontraktgruppe des Kontrakts ist und die Banknummer und Kontonummer dieselben sind.
