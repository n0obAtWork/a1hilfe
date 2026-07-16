# Behandlungsschema

<!-- source: https://amic.de/hilfe/_behandlungsschemata.htm -->

Administration > Formulare/Abläufe > Behandlungsschema

Mit dem Direktsprung **[BEH]** Behandlungsschema können Sie den Behandlungsschemapfleger aufrufen. Es wird eine Standardbehandlung ausgeliefert, die Sie für Ihre Anwendung modifiziert ablegen können.

Administration > Formulare/Abläufe > Formularzuordnung/Vorgangsunterklasse

Welches Behandlungsschema für welche Vorgangsunterklasse verwendet wird, legen Sie in der Formularzuordnung **[FRZ]** auf der Registerkarte Abwicklung fest.

Das Behandlungsschema gibt Ihnen die Möglichkeit, bestimmte Vorgehensweisen bei der Kundennummernänderung auszuschließen, Meldungen ein- oder auszuschalten und eine Behandlungsvorgabe für bestimmte Fälle vorzugeben.

Da ein Behandlungsschema unter Umständen auch von einem Makro aufgerufen wird, ist es stets möglich, Meldungen abzuschalten.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Behandlungsschemakriterien</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Kriterium</b></p>
        </td>
        <td>
          <p><b>Werte</b></p>
        </td>
        <td>
          <p><b>Verfahrensweise</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beteiligung von Kontrakten</p>
        </td>
        <td>
          <p>Weiter oder Abbruch je mit oder ohne Meldung</p>
        </td>
        <td>
          <p>Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Kontrakten verbieten oder zumindest zur Meldung bringen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenwechsel unter Kunden gleicher Kontraktgruppe</p>
        </td>
        <td>
          <p>Weiter oder Abbruch je mit oder ohne Meldung</p>
        </td>
        <td>
          <p>Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Kontrakten verbieten oder nur zur Meldung bringen, wenn es sich im Kunden mit gleicher Kontraktgruppe handelt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenwechsel unter Kunden ungleicher Kontraktgruppe</p>
        </td>
        <td>
          <p>Kontrakt entfernen oder Abbruch je mit oder ohne Meldung</p>
        </td>
        <td>
          <p>Sie können hier bestimmen, ob ein Kontrakt im Fall ungleicher Kontraktgruppen der Kunden abgewählt wird, oder in diesem Fall der Kundenwechsel verboten ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontraktneufindung starten</p>
        </td>
        <td>
          <p>Ja/Nein</p>
        </td>
        <td>
          <p>Wird aus der vorherigen Einstellung ein Kontrakt abgewählt, so kann er mit dieser Einstellung neu für den neuen Kunden vorbelegt werden, sofern ein Kontrakt vorhanden ist</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beteiligung von Partien</p>
        </td>
        <td>
          <p>Weiter oder Abbruch je mit oder ohne Meldung</p>
        </td>
        <td>
          <p>Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Partien verbieten oder zumindest zur Meldung bringen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abwahl von Partien ohne Zwang</p>
        </td>
        <td>
          <p>Ja/Nein</p>
        </td>
        <td>
          <p>Ist die Partie einer Warenposition nicht kundenspezifisch, so muss sie nicht abgewählt werden. Dies kann jedoch auf Wunsch dennoch geschehen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partie-Neufindung wenn nötig</p>
        </td>
        <td>
          <p>Ja/Nein</p>
        </td>
        <td>
          <p>Hat ein Artikel kein Partiezwang und wird die Partie abgewählt, weil sie nicht zum neuen Kunden zugeordnet werden kann, so kann hier definiert werden, ob eine neue Partie gesucht und ggf. zugeordnet werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Manuelle Konditionen</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>An dieser Stelle können Sie definieren, ob manuell eingegebene Konditionen beibehalten oder entfernt werden sollen. Dies kann jeweils mit oder ohne Meldung geschehen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abgeänderte automatische Konditionen</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>An dieser Stelle wird definiert, ob abgeänderte automatische Konditionen beibehalten oder entfernt werden. Dies kann jeweils mit oder ohne Meldung geschehen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preisbehandlung manuelle Preise</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Diese Einstellung regelt die Neufindung bzw. Beibehaltung manuell abgeänderter Preise (nicht bei Kontraktpreisen!)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Behandlung &nbsp;manueller Gesamtbeträge</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Diese Einstellung regelt die Neufindung von Preisen bzw. Beibehaltung von manuell geänderten Beträgen (Gesamtpreisen).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preisbehandlung automatische Preise</p>
        </td>
        <td>
          <p>Beibehalten oder neu finden</p>
        </td>
        <td>
          <p>Wird der Kundenwechsel auf einen Kunden mit abweichender Preisklasse vollzogen, so können Preise beibehalten oder neu ermittelt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Auto Preise ggf. auch in der Kasse</p>
        </td>
        <td>
          <p>Ja / Nein</p>
        </td>
        <td>
          <p>Sollen automatische Preise neu gefunden werden, so muss hier festgelegt werden, ob dies auch in der Kasse passieren soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wechsel zu Kunden mit abweichender Währung</p>
        </td>
        <td>
          <p>Währung beibehalten/Abbruch mit und ohne Warnung</p>
        </td>
        <td>
          <p>Wird der Kundenwechsel auf einen Kunden mit abweichender Währung vollzogen, so kann die Währung beibehalten oder der Kundenwechsel abgebrochen werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beteiligung von Strecke führt zu</p>
        </td>
        <td>
          <p>Entfernen / Abbruch mit und ohne Warnung</p>
        </td>
        <td>
          <p>Ist eine Strecke an diesem Beleg beteiligt, so kann diese entfernt oder der Wechsel abgebrochen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nullpreis im Ziel</p>
        </td>
        <td>
          <p>Beibehalten / Abbruch / Akzeptieren mit und ohne Warnung</p>
        </td>
        <td>
          <p>Ist in einer Warenposition nach dem Kundenwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abweichende Zahlungsbedingung</p>
        </td>
        <td>
          <p>Beibehalten / Abbruch / Akzeptieren mit und ohne Warnung</p>
        </td>
        <td>
          <p>Wird durch den Kundenwechsel eine andere Zahlungsbedingung gewählt, als im Vorgang definiert wurde, so kann entschieden werden, ob dies erlaubt sein soll, die alte Zahlungsbedingung erhalten bleiben soll oder die neue verwendet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Behandlung der Versandanschrift</p>
        </td>
        <td>
          <p>Verwerfen der Versandanschrift / Erhalten der Versandanschrift</p>
        </td>
        <td>
          <p>Ist in einer Warenposition nach dem Kundenwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verkaufsbeschränkungen nicht aufgelöst</p>
        </td>
        <td>
          <p>Abbruch oder weiter mit Warnung wenn möglich</p>
        </td>
        <td>
          <p>Bestehen für Artikel des Belegs Verkaufsbeschränkungen, für die der neue Kunde kein Zertifikat hat, so muss eine Entscheidung getroffen werden:</p>
          <p><b>Einstellung "Abbruch":</b></p>
          <p>Abbruch des Kundenwechsels mit der Angabe der betroffenen Artikel.</p>
          <p><b>Einstellung „Weiter mit Warnung wenn möglich“:</b></p>
          <p>In Abhängigkeit der Einstellung des Steuerparameters 1001 wird verfahren:</p>
          <p>* Die Einstellung "Erfassung abweisen" sorgt für einen Abbruch</p>
          <p>* Die Einstellung "Abfrage in der GUI" sorgt dafür, dass der Kundenwechsel vollzogen wird, aber für die entsprechenden Artikel eine Warnung ausgegeben wird.</p>
          <p>Der Anwender muss in diesem Fall die Kaufberechtigung des Kunden persönlich prüfen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nachhaltigkeit Statusänderung</p>
        </td>
        <td>
          <p>Weiter oder Abbruch je mit oder ohne Meldung</p>
        </td>
        <td>
          <p>Wird der Status einer oder mehrerer Positionen von Nachhaltigkeit zu Nicht-Nachhaltigkeit oder umgekehrt geändert, so wird hier gewarnt und ggf. auch abgebrochen.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
