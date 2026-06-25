# Buchungstyp (KtrBuchTyp)

<!-- source: https://amic.de/hilfe/buchungstypktrbuchtyp.htm -->

Wird eine Ware gebucht, so wirkt diese Buchung auf einen bestimmten Bestand.

Um unterscheiden zu können, auf welche Bestände eine Buchung wirken soll, gibt es den Buchungstyp.

Der Buchungstyp findet sich in der Tabelle Warenbewegung als Feld KtrBuchTyp.

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

<p class="just-emphasize">Buchungstyp im Kontrakt</p>

Der Buchungstyp eines Kontrakts bestimmt Quelle und Ziel einer Buchung (z.B. Eigenware zu Fremdware). Eine nachträgliche Änderung des Buchungstyps ist deshalb nicht möglich, da damit alle bereits erfolgten Buchungen und Bestände im Zusammenhang mit diesem Kontrakt ebenfalls geändert werden müssten, die Zusammenhänge aber nicht überall ersichtlich sind.
