# Beispiele

<!-- source: https://amic.de/hilfe/_vorgangsmappeProfile_beispiele.htm -->

Hier werden Beispiele für Einstellungen des Streckenprofils dargestellt.

[Gültigkeit](./beispiele.md#beispiele_gueltigkeit)

### Gültigkeit![](../../../ImagesExt/image8_1357.png)

Bei den [Gültigkeiten](./index.md#registerAllgemein2) gilt generell, ein eingetragener Wert hat eine höhere Priorität als ein nicht vorhandener Wert. Ausschlaggebend dafür sind die Felder „Klasse“, „Unterklasse“ und „Grid“. Zu beachten ist dabei, dass die Unterklasse vor dem Grid zählt

#### Beispiel 1:

| Klasse | Unterklasse | Grid |
| --- | --- | --- |
| | | |
| 700 | | |

In diesem Beispiel befinden sich nur zwei Gültigkeiten. Da bei beiden Einträgen keine Unterklassen und kein Grid‘s eingetragen wurden, sind diese in diesem Beispiel uninteressant.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><u>Klasse 400</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Da kein Eintrag zur Klasse 400 existiert, wird der Eintrag ohne Klasse verwendet.</p>
        </td>
        <td>
          <p><u>Klasse 700</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
            </tbody>
          </table>
          <p>Der Eintrag zur Klasse 700 existiert, deswegen wird dieser verwendet.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Beispiel 2:

| Klasse | Unterklasse | Grid |
| --- | --- | --- |
| | | |
| | | 3 |
| 700 | | |
| 700 | | 3 |

Dieses Beispiel behandelt die Klasse und das Grid. Damit kann für bestimmte Klassen in bestimmten Grids eine Gültigkeit eingetragen werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><u>Klasse 800, Grid 1</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td></td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>3</td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Da kein Klasseneintrag existiert und das Grid nicht passt, wird der erste Eintrag verwendet.</p>
        </td>
        <td>
          <p><u>Klasse 800, Grid 3</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b></b>&nbsp;</td>
                <td><b>3</b></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>3</td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Hier passt wiederrum das Grid, die Klasse ist beim zweiten Eintrag egal, deswegen wird dieser Eintrag verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><u>Klasse 700, Grid 1</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td></td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td>3</td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>In diesem Fall existiert ein Eintrag mit der passenden Klasse, fürs Grid jedoch nicht. Aus diesem Grund wird der dritte Eintrag verwendet.</p>
        </td>
        <td>
          <p><u>Klasse 700, Grid 3</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td></td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b>3</b></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
            </tbody>
          </table>
          <p>Hier passen sowohl die Klasse als auch das Grid, aus diesem Grund wird der vierte Eintrag verwendet.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Beispiel 3:

| Zeile | Klasse | Unterklasse | Grid |
| --- | --- | --- | --- |
| 1 | | | |
| 2 | 700 | | 2 |
| 3 | 700 | 17 | |
| 4 | 700 | 17 | 3 |
| 5 | 800 | | |
| 6 | 800 | 9900 | |

Dieses Beispiel verwendet alle Felder, die für die Bestimmung der Gültigkeit gebraucht werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><u>Klasse 700, Unterklasse 0, Grid 1</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td><b></b>&nbsp;</td>
                <td><b></b>&nbsp;</td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Hier wird die erste Gültigkeit verwendet, da die anderen Gültigkeiten nicht passen, denn entweder ist das Grid oder die Unterklasse inkorrekt.</p>
        </td>
        <td>
          <p><u>Klasse 700, Unterklasse 17, Grid 1</u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b>17</b></td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>In diesem Fall wird der dritte Eintrag verwendet, denn die Klasse und Unterklasse passen, im vierten Eintrag würde das Grid nicht passen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><u>Klasse 700, Unterklasse 17, Grid 2</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b>17</b></td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Hier wird auch der dritte Eintrag verwendet, beim zweiten Eintrag würde zwar das Grid passen, jedoch gilt die Unterklasse vor dem Grid.</p>
        </td>
        <td>
          <p><u>Klasse 700, Unterklasse 17, Grid 3</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b>17</b></td>
                <td><b>3</b></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>In diesem Fall passen alle Daten, deswegen wird der vierte Eintrag verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><u>Klasse 700, Unterklasse 4, Grid 2</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b>700</b></td>
                <td><b></b>&nbsp;</td>
                <td><b>2</b></td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
          <p>Hier wird wiederum der zweite Eintrag verwendet. Da der zweite Eintrag für jede Unterklasse gilt und kein anderer passender Eintrag mit der Unterklasse existiert.</p>
        </td>
        <td>
          <p><u>Klasse 800, Unterklasse 17, Grid 3</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td><b>800</b></td>
                <td><b></b>&nbsp;</td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
              <tr>
                <td>800</td>
                <td>9900</td>
                <td></td>
                <td><b></b>&nbsp;</td>
              </tr>
            </tbody>
          </table>
          <p>Hier gilt wieder, die Unterklasse passt nicht, deswegen wird der fünfte Eintrag verwendet, wo die Unterklasse egal ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><u>Klasse 800, Unterklasse 9900, Grid 3</u><u></u></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Klasse</strong></th>
                <th><strong>Unterklasse</strong></th>
                <th><strong>Grid</strong></th>
                <th><strong>&nbsp;</strong></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td></td>
                <td>2</td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>700</td>
                <td>17</td>
                <td>3</td>
                <td></td>
              </tr>
              <tr>
                <td>800</td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td><b>800</b></td>
                <td><b>9900</b></td>
                <td><b></b>&nbsp;</td>
                <td><b><img alt="stock_undo_15x15" src="../../../ImagesExt/image8_1378.png"></b></td>
              </tr>
            </tbody>
          </table>
          <p>In diesem Fall wird der sechste Eintrag verwendet, da Klasse und Unterklasse passen.</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
