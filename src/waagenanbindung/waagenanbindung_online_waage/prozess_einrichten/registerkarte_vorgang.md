# Registerkarte Vorgang

<!-- source: https://amic.de/hilfe/_prozess_vorgang.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><b>Bezeichnung</b></p>
        </td>
        <td>
          <p><b>Bedeutung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Art der Vorgangserzeugung</p>
        </td>
        <td>
          <p>Hier kann festgelegt werden was bei der Funktion <strong><em>Vorgang erzeugen</em></strong> ausgeführt werden soll.</p>
          <p>0 = nicht aktiv / Einrichterparameter entscheidet</p>
          <p>1 = Vorgangskopie</p>
          <p>2 = Normalvorgang</p>
          <p>Wählt man „nicht aktiv“, dann wird die Einstellung des Einrichterparameters „<a href="../funktionen_auf_der_waagenmaske/einrichterparameter_in_der_waage.md">Teildisposition/Vorgangskopie aus Auftrag</a>“ überprüft. Diese kann Nein, Teildispo oder Vorgangskopie sein.<br><br>Für die <a href="../vorgangskopie/index.md">Vorgangskopie</a> muss auf dem Feld Kunde in der Waagenmaske über die <strong>F3</strong>-Auswahl ein Vorgang (z.B. ein Auftrag) ausgewählt worden sein, sonst tritt die normale Vorgangserzeugung in Kraft.<br><br></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Druckkennzeichen bei Vorgangserzeugung setzen<br>wenn Archivierung aktiv</p>
        </td>
        <td>
          <p>Default ist Ja.</p>
          <p>Mit Hilfe dieses Feldes kann man bei aktivierter Archivierung' entscheiden, ob bei der Vorgangserzeugung in der Waage ein Druckkennzeichen für den Vorgang gesetzt wird (wenn sich im Archiv ein Dokument befindet). Will man&nbsp;dies abschalten wählt man in der Vorlage für dieses Feld Nein aus.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zielbeleg gleiche Belegnr</p>
        </td>
        <td>
          <p>Default ist Nein.<br>Hier kann man festlegen ob man für die Wiegung und den erzeugten Vorgang (z.B. Lieferschein) die gleiche Belegnummer verwenden möchte.<br>Beim Vorgang erzeugen und bei der Vorgangskopie wird dann die Belegnummer des Waagedatensatzes in den erzeugten Vorgang (z.B. Lieferschein) übertragen.<br>Wenn es die Belegnummer für den erzeugten Vorgang schon gibt, dann erscheint eine Fehlermeldung. Die Vorgangserzeugung wird abgebrochen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verhalten bei Vorgangstornierung</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Kontrakt überziehen</p>
        </td>
        <td>
          <p>Darf ein Kontrakt an der Waage überzogen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fremdkontrakt überziehen</p>
        </td>
        <td>
          <p>Darf ein Fremdkontrakt an der Waage überzogen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontraktprüfung bei Auftrag / Bestellung</p>
        </td>
        <td>
          <p>Wird dieser Schalter auf „Ja“ gestellt, so wird die Kontraktprüfung ausgestellt. Der Kontrakt kann dann überzogen werden. Bei der Lieferschien / Eingangslieferschien Erstellung werden dann die Schalter Kontrakt überziehen und Fremdkontrakt überziehen ausgewertet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partiemaske unterdrücken</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden, ob die Maske zur Anlage der Partie aufgehen soll oder ob die Partie im Hintergrund angelegt wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Typ der Vorgangserzeugungsroutine</p>
        </td>
        <td>
          <p>Es besteht die Möglichkeit, die Standard-Vorgangerzeugung an der Waage durch eine Private Vorgangerzeugung zu ersetzt. Es kann zwischen zwei Erzeugungsvarianten gewählt werden. Der Methode wird die Owaage-Id als Übergabe Parameter mitgegeben.</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Typ der Vorgangserzeugung</strong></th>
                <th></th>
              </tr>
              <tr>
                <td>JPL</td>
                <td>Es wird eine JPL Methode / J Datei aufgerufen</td>
              </tr>
              <tr>
                <td>MAKRO</td>
                <td>Es wird ein Makro aufgerufen</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Alternative Vorgangserzeugungsroutine</p>
        </td>
        <td>
          <p>Hier wird die Alternative Vorgangserzeugungsroutine hinterlegt. Als Übergabeparameter wird die Owaage_Id übergeben.</p>
          <p>Diese Routine wird dann an Stelle der eigentlichen Vorgangserzeugung aufgerufen. Zu beachten ist dabei, dass danach die VorgangsId des Beleges in das Feld owaage_v_id geschrieben wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Exportdatensatz beim Abschließen erzeugen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Kontraktverhalten beim Wiegen gegen einen Auftrag</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden, ob das Kontraktfenster beim Wiegen gegen einen Auftrag gesperrt werden soll.</p>
          <p>Das Standardverhalten ist Kontraktauswahl wählt den Auftrag ab.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorgang nach der Erzeugung drucken</p>
        </td>
        <td>
          <p>Mit dieser Einstellung wird der Normalwarenvorgang direkt nach der Erzeugung ausgestellt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
