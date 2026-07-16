# Regeleinrichtung [REGEL]

<!-- source: https://amic.de/hilfe/regeleinrichtungregel.htm -->

Hauptmenü > Administration > Formulare / Abläufe > Arbeitsregeln verwalten

oder Direktsprung <strong>[ARV]</strong> oder <strong>[REGEL]</strong>

Der Einrichtungsbildschirm einer Regel gliedert sich in folgende Bereiche:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Arbeitsregel</p>
        </td>
        <td>
          <p>Hier wird die Nummer der Arbeitsregel angegeben.<br>Nummer 0 darf nicht vergeben oder verändert werden.</p>
          <p>Diese Nummer der Arbeitsregel wird beim Anlegen eines Vorgangs im Vorgangstamm gespeichert und ist unter [FRZ] für die entsprechende Vorgangsklasse einzurichten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Hier kann der Name für die Arbeitsregel festgelegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurzbezeichnung</p>
        </td>
        <td>
          <p>Kurzname für die Arbeitsregel</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

| Sperren | |
| --- | --- |
| Code | Nummer der Funktionalität für die Belege mit dieser Arbeitsregel gesperrt werden können |
| Sperre für … | Belege, die diese Arbeitsregel enthalten, können für folgende Funktionalitäten gesperrt werden:<br>1 – Druck<br>2 – Fibu-Übertrag<br>3 – Korrektur<br>4 – Ansehen<br>5 – Storno<br>6 – Umwandlung<br>7 – Artikel löschen<br>8 – Artikel neu erfassen<br>9 – Menge korrigieren<br>10 – Preis korrigieren<br>11 – Regel setzen<br>12 – Regel korrigieren |
| Typ | Die Art wie der Beleg für die entsprechende Funktionalität behandelt werden soll, wenn er diese Arbeitsregel enthält<br>F3- Auswahl:<br>\-keine<br>\-Datenbank Funktion: Eine Funktion, deren Name im nächsten Feld anzugeben ist, regelt das Verhalten für den Beleg der diese Arbeitsregel enthält.<br>\-SQL-Text: Ein SQL-Text regelt das Verhalten für den Beleg<br>\-immer sperren: Belege die diese Arbeitsregel enthalten sind immer gesperrt für die jeweilige Funktionalität, z.B. Druck, wird trotzdem versucht den Beleg zu drucken erhält man eine entsprechende Fehlermeldung mit Hinweis auf die Arbeitsregel<br><br> |
| SQL / Funktion | Hier wird der Name der Funktion angegeben die für die entsprechende Funktionalität wirken soll.<br>Gibt man hier einen Namen ein kann über die Funktion **Editieren/Neu F5** in den Pfleger gewechselt und die Funktion bearbeitet oder angelegt werden.<br>Diese Funktion muss 1 (gesperrt) oder 0 (nicht gesperrt) zurückliefern. |

Grundgerüst für eine Datenbankfunktion zur Auswertung einer Regelsperre:

Hier wurde für ein Beispiel hinter RETURN die 1 (für gesperrt) fest codiert eingetragen.  
Mit der globalen Datenbankvariablen DB_REGEL_INFO kann man einen Informationstext hinterlegen, damit der Anwender den Grund für die Sperrung erfährt.

![](../../ImagesExt/image8_257.jpg)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Nachfolgeregel</strong></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Hier stellt man ein, dass es eine Vorschrift gibt wie die Regel automatisch verändert wird, wenn ein Beleg korrigiert oder die Regel neu gesetzt wird.</p>
          <p><br>Im oberen großen Feld dieses Registers wird der Programmcode der ausgewählten Funktion als Vorschau angezeigt. Für ‚kein Nachfolger‘ bleibt das Feld leer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Typ</p>
        </td>
        <td>
          <p>F3-Auswahl:<br>kein Nachfolger<br>Datenbank Funktion<br>Privater SQL</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SQL / Funktion</p>
        </td>
        <td>
          <p>Hier wird der Name der Funktion angegeben die für die Nachfolgeregel wirken soll.</p>
          <p>Gibt man hier einen Namen ein kann über die Funktion <strong>Editieren/Neu F5</strong> in den Pfleger gewechselt und die Funktion bearbeitet oder angelegt werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Beispiel für eine Datenbankfunktion zur Ermittlung einer Nachfolgeregel:

Hier wird beim Speichern des Lieferscheins (der mit der Arbeitsregel 600 angelegt wurde) abhängig vom Betrag als Arbeitsregel die 615 oder 610 eingetragen.

![](../../ImagesExt/image8_258.jpg)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Gültigkeit</strong></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Diese Regel darf nur durch eine andere Regel ersetzt werden, die in mindestens einer der hier aufgeführten Bereiche liegt.<br>Wird kein Bereich festgelegt, gibt es keine Einschränkung.<br>Bereiche mit leeren Einträgen werden nicht gespeichert!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>von Regel</p>
        </td>
        <td>
          <p>Hier wird die Nummer der Arbeitsregel eingetragen mit der der Bereich beginnen soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>bis Regel</p>
        </td>
        <td>
          <p>Hier wird die Nummer der Arbeitsregel eingetragen mit der der Bereich enden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>nur größere Regel zulässig</p>
        </td>
        <td>
          <p>Hier kann festgelegt werden, dass Arbeitsregeln nur in Regeln mit einer größeren Nummer geändert werden können.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Wird versucht die Arbeitsregel eines Beleges in eine Regel außerhalb des angegebenen Bereichs zu ändern, bekommt man eine Fehlermeldung mit dem Hinweis Bereichsverletzung.

| Test Beleg | |
| --- | --- |
| Beleg zum Testen | Hier legt man einen Test Beleg für die Funktion ***Statement testen F10*** fest. Diese Funktion kann man aus der Option Box aufrufen, wenn man mit dem Cursor im Register Nachfolgeregel im Feld SQL/Funktion steht. |
| Vorgangsklasse | Vorgangsklasse des Testbelegs |
| BelegNummer | Nummer des Testbelegs |
