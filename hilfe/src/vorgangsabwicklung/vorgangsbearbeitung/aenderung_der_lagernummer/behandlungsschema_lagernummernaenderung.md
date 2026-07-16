# Behandlungsschema Lagernummernänderung

<!-- source: https://amic.de/hilfe/_behandlungsschema.htm -->

Administration > Formulare/Abläufe > Behandlungsschema

Mit dem Direktsprung **[BEH]** Behandlungsschema können Sie den Behandlungsschemapfleger aufrufen. Es wird eine Standardbehandlung ausgeliefert, die Sie für Ihre Anwendung modifiziert ablegen können.

Administration > Formulare/Abläufe > Formularzuordnung/Vorgangsunterklasse

Welches Behandlungsschema für welche Vorgangsunterklasse verwendet wird, legen Sie in der Formularzuordnung **[FRZ]** auf der Registerkarte Abwicklung fest.

Das Behandlungsschema gibt Ihnen die Möglichkeit, bestimmte Vorgehensweisen bei der Lagernummernänderung auszuschließen, Meldungen ein- oder auszuschalten und eine Behandlungsvorgabe für bestimmte Fälle vorzugeben.

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
          <p>Artikelfindung</p>
        </td>
        <td>
          <ul>
            <li>Identische Artikelnummer suchen – Verwenden Sie diese Option, wenn Sie in allen Lägern die gleichen Artikelnummern verwenden.</li>
            <li>Identische Artikelnummer + Lagerstamm verproben – Wir empfehlen diese Option, wenn Sie in allen Lägern identische Artikelnummern verwenden und diese den gleichen Lagerstammeintrag haben</li>
            <li>Über den Artikelstamm suchen – Verwenden Sie diese Option, wenn Sie unterschiedliche Artikelnummern pro Lager verwenden, die jedoch einen gemeinsamen Artikelstamm verwenden. Voraussetzung ist jedoch, dass zu einem Artikelstamm nur ein Eintrag pro Lager existiert.</li>
          </ul>
        </td>
        <td>
          <p>Auf welche Weise sollen Artikel im neuen Lager gefunden werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontraktbehandlung</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Sie bestimmen hier, ob ein ausgewählter Kontrakt beibehalten werden soll, wenn es möglich ist (Lagerspezifische Kontrakte werden entfernt) oder Kontrakte grundsätzlich beim Lagernummernwechsel entfernt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ungültige/abgewählte Kontrakte neu finden</p>
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
          <p>Manuelle Preise bei abweichender Preisgruppe</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Pro Lager können unterschiedliche Preise (Preisgruppen) gelten. Definieren Sie, ob manuell eingegebene Preise bei abweichender Preisgruppe beibehalten oder entfernt werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Manuelle Preise bei identischer Preisgruppe</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Der manuell eingegebene Preis kann eine Spezialität für das Lager sein, der bei Lagerwechsel entfernt oder beibehalten werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatische Preise bei abweichender Preisgruppe</p>
        </td>
        <td>
          <p>Beibehalten/Entfernen mit und ohne Warnung</p>
        </td>
        <td>
          <p>Pro Lager können unterschiedliche Preise (Preisgruppen) gelten. Definieren Sie, ob automatisch ermittelte Preise bei abweichender Preisgruppe beibehalten oder entfernt werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nullpreis im Ziel</p>
        </td>
        <td>
          <p>Jeweils mit und ohne Warnung</p>
          <ul>
            <li>Beibehalten</li>
            <li>Abbruch</li>
            <li>Akzeptieren</li>
          </ul>
        </td>
        <td>
          <p>Ist in einer Warenposition nach dem Lagernummernwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partiebehandlung</p>
        </td>
        <td>
          <p>Jeweils mit und ohne Warnung:</p>
          <ul>
            <li>Abbruch – Bei Partiebeteiligung wird abgebrochen</li>
            <li>Beibehalten – Partie wird beibehalten wenn möglich. Lagerspezifische Partien werden entfernt.</li>
            <li>Artikel in Partie anlegen – Der Artikel des neuen Lagers wird wenn nötig der Partie hinzugefügt.</li>
            <li>Partie immer abwählen – ebendies</li>
          </ul>
        </td>
        <td>
          <p>Wie ist bei Beteiligung von Partien zu verfahren?</p>
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
          <p>Bei Folgeartikel abbrechen</p>
        </td>
        <td>
          <p>Ja/Nein</p>
        </td>
        <td>
          <p>ebendies</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel im Ziellager anlegen</p>
        </td>
        <td>
          <p>Weiter/Abbruch mit/ohne Warnung</p>
        </td>
        <td>
          <p>Wenn der Artikel nicht im Ziel-Lager existiert müsste dieser dort angelegt werden. Dies kann erlauft sein (Weiter) oder eben nicht (Abbruch) Optional kann eine zusätzliche Warnmeldung ausgegeben werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
