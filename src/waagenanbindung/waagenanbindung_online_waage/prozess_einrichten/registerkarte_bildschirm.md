# Registerkarte Bildschirm

<!-- source: https://amic.de/hilfe/_prozess_bildschirm.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorbelegung des Klammertyps</p>
        </td>
        <td>
          <p>Belegt den Klammertyp in der Waage mit dem hier hinterlegten Klammertyp</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Spedition und LKW als Textfelder</p>
        </td>
        <td>
          <p>Mit diesem Schalter kann eingestellt werden, ob das LKW-Feld als Textfeld behandelt werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Den letzten LKW vorbelegen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Die letzte Schlagbezeichnung vorbelegen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>TAB Deck Einstellungen deaktivieren</p>
        </td>
        <td>
          <p>Mit dieser Einstellung kann die Registerkarte Einstellungen ausgeblendet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Länge der Bemerkungsfelder</p>
        </td>
        <td>
          <p>Hier kann die Textlänge der Bemerkungsfelder hinterlegt werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dispomengenfeld deaktivieren</p>
        </td>
        <td>
          <p>Blendet die Dispomengenfelder auf der Waagenmaske aus, wenn der Schalter auf Ja steht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagerplatzfelder deaktivieren</p>
        </td>
        <td>
          <p>Wenn diese Einstellung auf Ja gestellt wird, so werden die Lagerfelder auf Waagenmaske aus geblendet-</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunden-Ort mit anzeigen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Rohwarenschema mit abfragen</p>
        </td>
        <td>
          <p>Mit dieser Einstellung wir das Rohwarenschema ausgeblendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelbezeichnung nicht aktiv abfragen</p>
        </td>
        <td>
          <p>Wird der Schalter auf Nein gestellt, so kann die Artikelbezeichnung geändert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sonderqualitäten deaktivieren</p>
        </td>
        <td>
          <p>Hiermit können die Felder der Sonderqualitäten deaktiviert werden. (Felder: Feuchteprozent, Fremdfeuchte, Frachtkennzeichen)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ladungszeiten aktivieren</p>
        </td>
        <td>
          <p>Wird dieses Feld auf „Nein“ gestellt, so werden die Felder für die Ladungszeiten deaktiviert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Schlag/Sorte/Gebiet aktiv</p>
        </td>
        <td>
          <p>Wird dieses Feld auf „Nein“ gestellt, so werden die Felder für den Schlag, die Sorte und das Gebiet deaktiviert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorfracht aktiv</p>
        </td>
        <td>
          <p>Noch nicht implementiert</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Breite der Waagenmaske in Pixel</p>
        </td>
        <td>
          <p>Hiermit kann die Breite der Waagenmaske verändert werden. Sobald die Waagenmaske breit genug ist werden die Qualitätstabellen, so wie der Abstufungsrtikel und die Infratec Datentabelle auf die Registerkarte Allgemein geschoben.</p>
          <p>Sobald eine Qualitätstabelle auf der Registerkarte Allgemein angezeigt wird und es noch keine Feldreihenfolge existiert, wird Standartmäßig eine neue angelegt.</p>
          <table>
            <tbody>
              <tr>
                <th>Aktuelles Feld</th>
                <th>Vorheriges Feld</th>
                <th>Nachfolgendes Feld</th>
              </tr>
              <tr>
                <td>Kontrakt</td>
                <td>-</td>
                <td>Kunde</td>
              </tr>
              <tr>
                <td>Kunde</td>
                <td>Kontrakt</td>
                <td>Fahrer</td>
              </tr>
              <tr>
                <td>Fahrer</td>
                <td>Kunde</td>
                <td>Lager</td>
              </tr>
              <tr>
                <td>Lager</td>
                <td>Fahrer</td>
                <td>Artikel</td>
              </tr>
              <tr>
                <td>Artikel</td>
                <td>Fahrer</td>
                <td>Bemerkung</td>
              </tr>
              <tr>
                <td>Bemerkung</td>
                <td>Artikel</td>
                <td>Bemerkung2</td>
              </tr>
              <tr>
                <td>Bemerkung2</td>
                <td>Bemerkung</td>
                <td>Bemerkung3</td>
              </tr>
              <tr>
                <td>Bemerkung3</td>
                <td>Bemerkung2</td>
                <td>Silo/Ladeträger</td>
              </tr>
              <tr>
                <td>Silo/Ladeträger</td>
                <td>Bemerkung3</td>
                <td>Qualitätstabelle 1</td>
              </tr>
              <tr>
                <td>Qualitätstabelle 1</td>
                <td>Silo/Ladeträger</td>
                <td>Qualitätstabelle 2 wenn dieses auf der Registerkarte Allgemein vorhanden ist ansonsten wird in das Feld Kontrakt gesprungen.</td>
              </tr>
              <tr>
                <td>Qualitätstabelle 2 wenn dieses auf der Registerkarte Allgemein vorhanden ist.</td>
                <td>Qualitätstabelle 1</td>
                <td>Kontrakt</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Höhe der Waagenmaske in Pixel</p>
        </td>
        <td>
          <p>Hiermit kann die Höhe der Waagenmaske verändert wird. Ist die Waagenmaske hoch genug, so wird die Kontraktverteilungs Datentabelle angezeigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Prozessverhalten</p>
        </td>
        <td>
          <p>Hier kann das Verhalten für das Prozessfeld auf der Waagenmaske eingestellt werden.</p>
          <p>Folgende Auswahl ist möglich:</p>
          <ul>
            <li>Nicht Änderbar</li>
            <li>Änderbar</li>
            <li>Verstecken</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Terminalverhalten</p>
        </td>
        <td>
          <p>Hier kann das Verhalten für das Terminalfeld auf der Waagenmaske eingestellt werden.</p>
          <p>Folgende Auswahl ist möglich:</p>
          <ul>
            <li>Nicht Änderbar</li>
            <li>Änderbar</li>
            <li>Verstecken</li>
          </ul>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Pflichtfelder Gruppen</p>

Auf dieser Registerkarte können Felder als Pflichtfelder für den aktuellen Prozess definiert werden. Die Auszuwählenden Felder werden durch das System gepflegt und können nicht hinzugefügt werden. Die Pflichtfeldeingabe kann z.B.(Silo/Lagerverwaltungssystem) mehrere Eingabefelder auf der Erfassungsmaske betreffen.

Es wurde jedoch eine Möglichkeit geschaffen, dass mittels einer privaten Prozedur, Felder als Pflichtfelder zu deklarieren. Dazu muss im Steuerparameter [925](../../../firmenstamm/steuerparameter/waagensteuerung/allgemeiner_steuerparameter_fuer_die_waage_spa_925.md) die Option „PFLICHTFELDERWAAGENPROZESS“ gesetzt werden. Im Feld Wert kann dann eine Private Prozedur hinterlegt werden, die den Feldnamen des Waagenmaskenfeldes zurückgibt, welches als Pflichtfeld eingerichtet werden soll. Sollen mehrere Felder als Pflichtfelder eingerichtet werden, so muss die Prozedur alle Felder bei dem Aufruf zurückgeben.

<p class="just-emphasize">Folgende Pflichtfelder werden zurzeit unterstützt.</p>

| Bezeichnung | Betroffene Felder |
| --- | --- |
| Private Prozedur | Selbst bestimmbar |
| Silo Lagerverwaltungssystem | 1. Silo/Träger<br>2. Lokalität/Standort<br>3. Datentabelle Ladeträger/Silo |

<p class="just-emphasize">Private Prozedur</p>

Der im Steuerparameter [925](../../../firmenstamm/steuerparameter/waagensteuerung/allgemeiner_steuerparameter_fuer_die_waage_spa_925.md) hinterlegten privaten Prozedur wird die ProzessId(VorlagenId) als Parameter übergeben. Der Eingangsparameter muss **in_om_id** heißen. Die Ausgangsparameter müssen **FeldName** und **verstecken** heißen.

Wird ein Feld zurückgegeben, welches sich nicht auf der Waagenmaske befindet, so wird der Feldname in das Fehlerprotokoll geschrieben. Die Wiegung kann trotzdem erfasst werden.

Es findet keine Verprobung zwischen den Felder der privaten Prozedur und den Standardpflichtfelder statt, wenn diese eingerichtet worden sind.

<p class="just-emphasize">Eingang</p>

| Parameter | Bedeutung |
| --- | --- |
| in_om_id | Id des Prozesses |

<p class="just-emphasize">Ausgang</p>

| Parameter | Bedeutung |
| --- | --- |
| FeldName | Dieser Parameter gibt den Feldnamen auf der Maske an welcher als Pflichtfeld gekennzeichnet werden soll. |
| verstecken | Dieser Parameter wird noch nicht ausgewertet. Hier kann erstmal im Standard eine 0 zurückgegeben werden. |

<p class="just-emphasize">Beispiel</p>

```sql
CREATE PROCEDURE p_Waage_PflichtFelder ( in in_om_id integer )
            result ( FeldName char(255), verstecken integer )
--
BEGIN
  declare local temporary table ltt_fields(feldname char(255), verstecken integer) on commit delete rows;
case in_om_id
  when 14 then
    insert into ltt_fields( feldname, verstecken ) values( 'Bemerkung$', 0);
    insert into ltt_fields( feldname, verstecken ) values( 'Einlagerung$', 0);
  when 23 then
    insert into ltt_fields( feldname, verstecken ) values( 'Bemerkung2$', 0);
    insert into ltt_fields( feldname, verstecken ) values( 'FremdFeuchte$', 0);
    insert into ltt_fields( feldname, verstecken ) values( 'QWERT1$', 0);
end;
  select feldname, verstecken from ltt_fields;
END
```
