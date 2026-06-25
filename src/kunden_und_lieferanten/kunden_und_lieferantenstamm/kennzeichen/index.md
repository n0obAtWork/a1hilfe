# Kennzeichen

<!-- source: https://amic.de/hilfe/_kennzeichen.htm -->

Verschiedene Funktionalitäten in **A.eins** werden über die Kundenkennzeichen gesteuert:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Diverses Konto</b></p>
        </td>
        <td>
          <p>Das Konto steht für die Abwicklung von Kunden, für die kein eigenes Konto geführt werden soll, zur Verfügung. Beim Aufruf dieses Kunden zur Fakturierung wird automatisch die Adressmaske zur Eingabe der Anschriftdaten geöffnet.</p>
          <p>Diese Funktion sollte allerdings, da sie unter Verzicht auf viele Informationsmöglichkeiten eingesetzt wird, sehr sparsam verwendet werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Barverkauf Konto</b></p>
        </td>
        <td>
          <p>Z.Z. nicht aktiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Sammelrechnung</b></p>
        </td>
        <td>
          <p>Es kann eingestellt werden, ob ausschließlich Sammel- oder Einzelrechnungen erstellt werden oder es fallabhängig ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Bruttorechnung</b></p>
        </td>
        <td>
          <p>Bei <strong>"J"</strong> erhält der Kunde Bruttorechnungen, d.h., alle Beträge — auch die Einzelpreise — werden brutto ausgewiesen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>KoKoRe</b></p>
        </td>
        <td>
          <p>Dieses Kennzeichen wird gesetzt, wenn mit dem Kunden ein Kontokorrentverhältnis besteht.<br>Das bedeutet, der Kunde ist sowohl Lieferant/Hersteller als auch Kunde. Sehen Sie hierzu auch Kundentyp, im Vorgangsbereich <strong>"Kontokorrent" </strong>und in der Funktion <strong>"FIBU- Merkmale"</strong> den Zinsbereich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Versandanschrift</b></p>
        </td>
        <td>
          <p>Dieses Feld hat keine steuernde Bedeutung in A.eins.</p>
          <p>Es kann frei verwendet werden!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Raffung</b></p>
        </td>
        <td>
          <p>Z.Z. nicht aktiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Rabattsperre</b></p>
        </td>
        <td>
          <p>ZZ. nicht aktiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Liefersperre</b></p>
        </td>
        <td>
          <p>Bei einer weichen Liefersperre wird bei dem Versuch, den Kunden zu beliefern, eine Warnung angezeigt. Die Lieferung ist aber trotzdem möglich. Bei einer harten Sperre dagegen ist der Kunde komplett für Lieferungen gesperrt.</p>
          <p>Bei der Umwandlung von Belegen ist es jedoch möglich, auch Belege dieser Kunden weiterzuverarbeiten (z.B. in Rechnungen umwandeln), die eine Liefersperre haben. Das ist sinnvoll, damit alte Belege, die ohne Liefersperre erfasst wurden, auch noch umgewandelt werden können. Neue Belege können nicht mehr erzeugt werden. Das kann per SPA aktiviert werden. Im Normalfall <strong>(SPA "Umwandlung von Belegen auch bei Liefersperre" = Nein)</strong> ist dieses <strong>nicht</strong> möglich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Liefersperrengrund</b></p>
        </td>
        <td>
          <p>Wenn der Kunde für Lieferungen gesperrt ist, kann hier ein Grund für die Sperre angegeben werden, der zusätzlich zur normalen Fehlermeldung angezeigt wird. Siehe auch Sperrbemerkungen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Zu-/Abschlagsperre</b></p>
        </td>
        <td>
          <p>Z.Z. nicht aktiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Vorgang Erfassungssperre</b></p>
        </td>
        <td>
          <p>Bei <strong>"J" kann für den Kunden kein neuer Vorgang erfasst werden.</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Bonität</b></p>
        </td>
        <td>
          <p>Kennzeichen für die Beurteilung der Kreditwürdigkeit. Es wird manuell vergeben und kann z.B. bei der Vorgangserfassung angezeigt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Beladefaktor</b></p>
        </td>
        <td>
          <p>Wenn bei Beladungen besondere Erschwernisse auftauchen, die zu erhöhter Berechnung führen sollen, so kann hier der Faktor, um den sich der Preis erhöhen soll, eingetragen werden.</p>
          <p>Der Preis wird dann um diesen Faktor erhöht/verringert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Entladefaktor</b></p>
        </td>
        <td>
          <p>Wie oben unter Beladefaktoren, nur werden die Erschwernisse der Entladung berücksichtigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>eindeutige Kundengruppe</b></p>
        </td>
        <td>
          <p>------------ noch nicht dokumentiert --------------</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Löschkennzeichen</b></p>
        </td>
        <td>
          <p>Gibt an, ob dieser Kunde gelöscht wurde (Eine endgültige Löschung aus den Datenbeständen hätte zur Folge, dass historische Daten nicht mehr einsehbar wären. Deshalb wird nur ein Löschkennzeichen gepflegt, um die Verwendung einzuschränken.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Buchstellenstamm</b></p>
        </td>
        <td>
          <p>------------- noch nicht dokumentiert --------------</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Belegversand</b></p>
        </td>
        <td>
          <p>Gibt an, ob der Belegempfänger die Belege durch eine Versandmechanik erhalten möchte, sofern dies für die Belegklasse vorgesehen ist. Siehe hierzu <a href="../../../zusatzprogramme/mailversand_allgemein/index.md">Belegversand</a></p>
          <p>Die Einstellungsmöglichkeiten sind analog zum openTRANS:</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Wert</strong></th>
                <th><strong>Bezeichnung</strong></th>
                <th><strong>Bedeutung</strong></th>
              </tr>
              <tr>
                <td>0</td>
                <td>Nein</td>
                <td>Es wird kein Belegversand verwendet</td>
              </tr>
              <tr>
                <td>1</td>
                <td>Mit Belegdruck</td>
                <td>Der Belegversand erfolgt analog zum Rechnungsdruck</td>
              </tr>
              <tr>
                <td>2</td>
                <td>Statt Belegdruck</td>
                <td>Der Belegversand erfolgt anstelle des Rechnungsdrucks<br>Hinweis:<br>Bitte beachten Sie, dass diese Einstellung „Statt Rechnungsdruck“ unter Umständen die Einstellung „mit Rechnungsdruck“ im Bereich openTRANS überschreibt. Ist eine dieser Einstellungen auf „statt Rechnungsdruck“ gestellt, so entfällt der Druck!</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Marktstandsatz/<br>Klasse</b></p>
        </td>
        <td>
          <p>------------ noch nicht dokumentiert --------------</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>MSA-Liste sofort<br>abzeigen</b></p>
        </td>
        <td>
          <p>------------ noch nicht dokumentiert --------------</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Nummernkreis<br>Paletten</b></p>
        </td>
        <td>
          <p>------------ noch nicht dokumentiert --------------</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Archiv-Referenz</b></p>
        </td>
        <td>
          <p>------------ noch nicht dokumentiert --------------</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [openTRANS](./opentrans.md)
