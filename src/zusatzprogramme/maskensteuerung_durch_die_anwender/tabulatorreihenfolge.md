# Tabulatorreihenfolge

<!-- source: https://amic.de/hilfe/tabulatorreihenfolge.htm -->

Funktionen bei der Tabulatorreihenfolge.

Wenn man auf ein Eingabefeld klickt, so öffnet sich eine Maske mit folgenden Feldern. In „**Aktives Feld**“ wird das Feld angezeigt, dass man gerade ausgewählt hat. Klick man dann auf eine der Funktionen „**Next Tabstop, PrevTabstop, Alter Next Tabstop, Alter Prev Tabstop**“, so schließt sich diese Maske und man kann dann in das Feld klicken, dass dann das nächste, vorherige,… in der Tabulatorreihenfolge werden soll.  
![Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_1464.png "Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.")

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Bedeutung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Next Tabstop</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Prev Tabstop</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Alter Next Tabstop</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Alter Prev Tabstop</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Aktives Feld</p>
        </td>
        <td>
          <p>Anzeige des aktuellen Feldnamens</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Feldeinstellungen löschen</p>
        </td>
        <td>
          <p>Setzt die Feldeinstellungen zurück</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Eingabezwang</p>
        </td>
        <td>
          <p>Kann Ja und Nein annehmen,</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tastaturfilter</p>
        </td>
        <td>
          <p>Die Werte des Feldes werde durch das Anklicken der Zeile gesetzt. Diese Werte werden unterstützt.</p>
          <ul>
            <li>Unfiltered</li>
            <li>Digits only</li>
            <li>Yes-no</li>
            <li>Alphabetic</li>
            <li>Numeric</li>
            <li>Alphanumeric</li>
            <li>Regular Expression</li>
            <li>Edit Mask<br>Bei Regular Expression und Edit Mask öffnet sich ein weiteres Fenster in dem die entsprechenden Werte eingegeben werden können.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Makroname(leer=JPL)</p>
        </td>
        <td>
          <p>Hier kann ein Makro hinterlegt werden, in dem folgende Funktionen behandelt werden können. Ist kein Makro hinterlegt, so wird davon ausgegangen, dass die folgenden Funktionen JPL-Funktionen sind und im User JPL stehen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FieldEntry</p>
        </td>
        <td>
          <p>Funktion die beim Betreten des Feldes ausgeführt wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FieldExit</p>
        </td>
        <td>
          <p>Funktion die beim Verlassen des Feldes ausgeführt wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FieldValid</p>
        </td>
        <td>
          <p>Funktion die vor dem Verlassen des Feldes ausgeführt wird.</p>
          <p>Liefert diese Funktion einen Wert größer als 0 zurück, so wird das Feld nicht verlassen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ScreenEnty</p>
        </td>
        <td rowspan="3">
          <p>Funktion, die beim Betreten und Verlassen der Maske ausgeführt werden. Sie hat zwei Parameter, der erste ist der Name der Maske und der zweite ist ein Flag, welches angibt, wie die Maske betreten bzw. verlassen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ScreenExit</p>
        </td>
      </tr>
      <tr>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>OnSaveValid</p>
        </td>
        <td rowspan="4">
          <p>Diese Funktionen erscheinen nur dann, wenn es sich bei der Maske technisch um ein Stammdateninterface handelt - zu erkennen am Aufruf der Maske über „jpl sd_interface“ mit 5,7,8 oder 77.</p>
          <p>Alle diese Funktionen haben einen Parameter, der angibt, was gerade gemacht worden ist:</p>
          <ul>
            <li>5 für Ändern</li>
            <li>7 für Löschen</li>
            <li>8 für Neu</li>
            <li>77 für Undelete<br><br>Die Funktion OnsSaveValid wird aufgerufen, nachdem die programmeigene Funktion ausgeführt wurde und nur dann, wenn die programmeigene Funktion Ok signalisierte. OnSaveValid muss 0 zurückliefern, wenn Speichern erlaubt sein soll ansonsten einen beliebigen Wert größer 0, wenn nicht gespeichert oder gelöscht werden soll:<br>Die anderen Funktionen werden aufgerufen, nachdem gespeichert wurde, oder Speichern mit Nein oder mit Abbruch verlassen wurde.<br>Wenn man ein User-JPL – nicht bei Verwendung eines Makros - &nbsp;angebunden hat, dann werden die Funktionen sofort angelegt. Beispiel für OnSaveJa:<br><br><br><b></b>&nbsp;<br><b>Hinweis</b>: Der Modus 7 (Löschen) wird extra im Template separat behandelt, damit nicht vergessen wird, dass diese Funktion auch bei Löschen aufgerufen wird!</li>
          </ul>
          <div>
            <pre><code>proc UserSaveValid(modus)
{
  if (modus == 7)
  {
    msg emsg „löschen nicht erlaubt“
    return 1
  }
  Return 0
}</code></pre>
          </div>
          <div>
            <pre><code>proc UserSaveJa(modus)
{
  if (modus == 7)
  {
    return
  }
  return
}</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>OnSaveJa</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>OnSaveNein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><i>OnSaveAbbruch</i></p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Wenn der Makroname leer ist und es sich somit um JPL handelt, müssen die verwendeten Funktionen mit „User-Jpl editieren“ noch mit Leben gefüllt werden,
