# Buchungstypen

<!-- source: https://amic.de/hilfe/buchungstypen.htm -->

Wird eine Ware gebucht, so wirkt diese Buchung auf einen bestimmten Bestand.

Um unterscheiden zu können, auf welche Bestände eine Buchung wirken soll, gibt es den Buchungstyp.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Buchungstypen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Eigenwarebuchung</p>
        </td>
        <td>
          <p>Diese Buchung verändert nur die Bestände der Eigenware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Vorverkauf</p>
        </td>
        <td>
          <p>Diese Buchung verändert Fremdware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Voreinkauf</p>
        </td>
        <td>
          <p>Diese Buchung verändert Fremdlager</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Einlagerung</p>
        </td>
        <td>
          <p>Diese Buchung verändert Fremdware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Kommission</p>
        </td>
        <td>
          <p>Diese Buchung verändert Fremdlager</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Buchungstyp eines Kontrakts</p>

Beim Vorverkauf und Voreinkauf wurde bei der Erfassung stets ein Kontrakt (oder Sammelkontrakt) angelegt – die Anlage des Kontraktes war verbindlich. Mit Einführung der Buchungstypen Einlagerung und Kommission kann man nun optional auch auf die Führung solcher Bestandskontrakte verzichten. Insbesondere im Kommissionsgeschäft kann damit auf eine Vielzahl von zusätzlichen Kontrakten verzichtet werden. In den Steuerungsparametern findet man unter den Nummern 96,97 und 99 die Einstellung zu Kontraktbehandlung für die Buchungstypen Vorverkauf, Voreinkauf und Kommission. Der Buchungstyp Einlagerung wird derzeit immer ohne Kontraktbuchführung abgewickelt.

Es wird in einer späteren Version auch möglich sein, normale Einkaufs und Verkaufskontrakte für die Bestandsführung der Vorverkäufe, Voreinkäufe und Kommissionen heranzuziehen. Hierfür wurde im Kontraktstamm eine Kennzeichnung geschaffen ( KtrBuchTyp!). In unseren Auswertungen und Auswahllisten wurde die Selektion auch nach diesem Kennzeichen schon integriert.
