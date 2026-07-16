# Einrichtung der Bereichsauswahl

<!-- source: https://amic.de/hilfe/_einrichtungderbereichsauswahl.htm -->

Bei der Einrichtung einer privaten Variante wird auch der Bereich privatisiert. Man erreicht die Bearbeitung des Bereichs über das Darstellungsregister

***Private Variante*** **Strg+F2** \> ***Bearbeiten*** **F5** > ***Zugehöriger Bereich*** **F5**

Es öffnet sich folgende Maske:

![Ein Bild, das Tisch enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_1272.jpg "Ein Bild, das Tisch enthält. Automatisch generierte Beschreibung")

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Spalte</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Label</p>
        </td>
        <td>
          <p>Der in dieser Spalte eingegebene Text erscheint in der Bereichsauswahl als Beschreibung der Zeile.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Typ</p>
        </td>
        <td>
          <p>Wie erfolgt die Abfrage. Dabei können folgende Werte mit F3 ausgewählt werden:<br><br></p>
          <table>
            <tbody>
              <tr>
                <th>Typ</th>
                <th></th>
              </tr>
              <tr>
                <td>von..bis..</td>
                <td>Rechts vom Label werden zwei Werte abgefragt, die im SQL-Ausdruck (siehe unten) als PAR1 und PAR2 verwendet werden können.</td>
              </tr>
              <tr>
                <td>gleich</td>
                <td>Es wird nur ein Wert abgefragt</td>
              </tr>
              <tr>
                <td>ungleich</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>like..</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>kleiner</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>größer</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>&lt;=</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>&gt;=</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>in..</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>not in..</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>clause</td>
                <td>s.o.</td>
              </tr>
              <tr>
                <td>zwei Parameter</td>
                <td>Rechts vom Label werden zwei Werte abgefragt.</td>
              </tr>
              <tr>
                <td>FS-Format</td>
                <td>Wird FS-Format ausgewählt, so muss in der Spalte FS-Format ein Format ausgewählt werden. In <b>PAR1</b> bzw. <b>VON[idx]</b> wird der Wert des FS-Formates zurückgegeben.</td>
              </tr>
              <tr>
                <td>toggle</td>
                <td>Ähnlich wie das FSFormat. D.h. es muss in der Spalte unter FSFormat etwas eingetragen werden. In <b>PAR1</b> bzw. <b>VON[idx]</b> wird das zurückgegeben, was im FSFORMAT in der Spalte Kommentar/Schnipsel eingetragen wurde.</td>
              </tr>
              <tr>
                <td>toggle mit Parameter</td>
                <td>Rechts vom Label werden zwei Werte abgefragt. Die Arbeitsweise ist wie <b>Toggle</b>, nur kann zusätzlich ein weiterer Wert abgefragt werden.</td>
              </tr>
            </tbody>
          </table>
          <p>Die Formate <b>gleich, ungleich, like, kleiner, größer, &lt;=, &gt;=, in, not in&nbsp; clause</b> haben nur noch die Bedeutung, dass nur ein Parameter abgefragt wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FS-Format</p>
        </td>
        <td>
          <p>Hier wird das zu toggle, toggle with param und FS-Format gehörende Format eingetragen. Eine Auswahl ist mit F3 möglich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Von</p>
        </td>
        <td>
          <p>Wert, der beim ersten Betreten der Bereichsauswahl bzw. nach dem Löschen des Profils im ersten Eingabefeld steht.</p>
          <p>Hier können auch Schlüsselwörter verwendet werden, die dann bestimmte Werte repräsentieren. Stellt man dem Schlüsselwort kein <b>#</b> vorneweg, dann wird der entsprechende Wert jedes Mal wieder vorbelegt, ansonsten nur beim ersten Betreten der Variante bzw. bei Löschen des Profils.</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th></th>
              </tr>
              <tr>
                <td>#TODAY</td>
                <td>Das Tagesdatum</td>
              </tr>
              <tr>
                <td>#FIRST</td>
                <td>Datum des ersten des Monats, also 01.04.2023 für den ersten April.</td>
              </tr>
              <tr>
                <td>#FIRST_LM</td>
                <td>Datum des ersten des vorangegangenen Monats, also 01.03.2023, wenn der aktuelle Monat der April ist.</td>
              </tr>
              <tr>
                <td>#ULTIMO</td>
                <td>Datum des letzten Tags des Monats</td>
              </tr>
              <tr>
                <td>#ULTIMO_LM</td>
                <td>Datum des letzten Tags des vorangegangenen Monats, also der 31.03.2023, wenn der aktuelle Monat der April ist</td>
              </tr>
              <tr>
                <td>#FIRSTDAYOFYEAR</td>
                <td>Erster Tag des aktuellen <b>Geschäftsjahres</b> aus dem Geschäftsjahresstamm.</td>
              </tr>
              <tr>
                <td>#LASTDAYOFYEAR</td>
                <td>Letzter Tag des aktuellen <b>Geschäftsjahres</b> aus dem Geschäftsjahresstamm.</td>
              </tr>
              <tr>
                <td>#BUCHWÄHRUNG oder<br>#BUCHWAEHRUNG</td>
                <td>Die Nummer der aktuellen Buchwährung.</td>
              </tr>
              <tr>
                <td>#YEAR</td>
                <td>Aktuelles Geschäftsjahr.</td>
              </tr>
              <tr>
                <td>#LASTYEAR</td>
                <td>Vorangegangenes Geschäftsjahr</td>
              </tr>
              <tr>
                <td>#PERIODE oder<br>#PERIOD</td>
                <td>Die aktuelle Periode zum Tagesdastum.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bis</p>
        </td>
        <td>
          <p>Ist beim Typen „von..bis..“, „Zwei Parameter“ oder „toggle with param“ eingetragen, dann sollte hier der Wert eingetragen werden, der in der zweiten Spalte beim ersten Betreten der Bereichsauswahl bzw. nach dem Löschen des Profils im ersten Eingabefeld stehen soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>F3-Auswahl</p>
        </td>
        <td>
          <p>Hier kann eine F3-Auswahl (Itembox) eingetragen werden. Eine Auswahl ist mit F3 möglich. Hat man eine F3-Auswahl angegeben, so erscheint hinter dem Eingabefeld ein Button, der kennzeichnet, dass hier F3 möglich ist. Es kann auch der Button verwendet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ItemPar-Variable</p>
        </td>
        <td>
          <p><u>Setzen von Itembox-Parametern (ItemPar)</u><u></u></p>
          <p>Ist an die Bereichsauswahl eine F3-Auswahl angebunden, so besteht hier die Möglichkeit die Werte der F3-Auswahl nach bereits erfassten Daten einzugrenzen.</p>
          <p>Um das ItemPar einzurichten, ist in dem Feld ItemPar-Variable der Name des Parameters anzugeben. Dieser muss sich im SQL-Text der F3-Auswahl (Itembox) wiederfinden. Anschließend wird in dem Feld ItemPar-Wert der entsprechende SQL-Ausdruck eingetragen (siehe ItemPar-Wert).</p>
          <p>Beispiel:</p>
          <p>In diesem Beispiel soll in der F2-Bereichsauswahl<strong> </strong>eine Artikelnummer ausgewählt werden. In der Bereichsauswahl wird bereits nach der Lagernummer „1“ gefiltert. Daher sollen in der F3-Auswahl zu der Artikelnummer nur alle Artikel zu dem Lager „1“ angezeigt werden.</p>
          <img src="../../../../ImagesExt/image8_1273.png" alt="">
          <p>Dazu werden in dem Feld „ItemPar-Variable“ der Name des Parameters (hier: AND_LAGERNUMMER) und dem Feld „ItemPar-Wert“ der entsprechende SQL-Schnipsel eingetragen:</p>
          <p><img src="../../../../ImagesExt/image8_1274.png" alt=""><strong></strong></p>
          <p><b><u>Hinweis</u><u></u></b></p>
          <p>Das ItemPar wird nur dann ausgewertet bzw. die Eingrenzung erfolgt nur dann, wenn die entsprechende Auswahlbedingung aktiviert wurde. Wird beispielweise die Auswahlbedingung „Lagernummer“ deaktiviert, so wird in der F3-Auswahl zu der Artikelnummer die Eingrenzung nach der Lagernummer aufgehoben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ItemPar-Wert</p>
        </td>
        <td>
          <p>Hier wird der SQL-Ausdruck für das ItemPar angegeben. Als Platzhalter für die Werte in der Bereichsauswahl werden VON[x] und BIS[x] verwendet. Das x steht hierbei für den Index der Auswahlbedingung.</p>
          <p>Beispiele:</p>
          <table>
            <tbody>
              <tr>
                <th>ItemPar-Variable</th>
                <th>ItemPar-Wert</th>
              </tr>
              <tr>
                <td>AND_LAGERNUMMER</td>
                <td>and a.LagerNummer between :VON[3] and :BIS[3]</td>
              </tr>
              <tr>
                <td>AND_ABDATUM</td>
                <td>and a.artikelabdatum &gt; ':VON[11]'</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fm</p>
        </td>
        <td>
          <p>Das Eingabeformat.</p>
          <table>
            <tbody>
              <tr>
                <th>Bezeichnung</th>
                <th></th>
              </tr>
              <tr>
                <td>char</td>
                <td>Einfache alphanumerische Eingabe. Die Länge der Eingabe ist auf 100 Zeichen (VON) bzw. 40 Zeichen (BIS) beschränkt.</td>
              </tr>
              <tr>
                <td>VC</td>
                <td>Wie char</td>
              </tr>
              <tr>
                <td>I2</td>
                <td>Die Eingabelänge ist auf 4 Zeichen beschränkt und es können nur numerische Zeichen eingegeben werden.</td>
              </tr>
              <tr>
                <td>I4</td>
                <td>Die Eingabelänge ist auf 9 Zeichen beschränkt und es können nur numerische Zeichen eingegeben werden.</td>
              </tr>
              <tr>
                <td>N0 -N4</td>
                <td>Es können nur numerische Zeichen eingegeben werden. Die Zahl ergibt die Anzahl der Nachkommastellen.</td>
              </tr>
              <tr>
                <td>DT</td>
                <td>
                  Datumseingabe. F3 öffnet einen Kalender zur Datumsauswahl. Neben der Eingabemöglichkeit eines Datums werden auch folgende Eingaben als Datum interpretiert:
                  <br>
                  <br>
                  <br>
                  <ul>
                    <li>HEUTE, TODAY</li>
                    <li>MONBEG, FIRST</li>
                    <li>MONEND, ULTIMO</li>
                    <li>QUARTBEG</li>
                    <li>QUARTEND</li>
                    <li>PERIFBEG, PERIBEG</li>
                    <li>PERIWBEG</li>
                    <li>PERIFEND, PERIEND</li>
                    <li>PERIWEND</li>
                  </ul>
                  <br>
                  Mit diesen Eingaben kann auch gerechnet werden. Beispiel: HEUTE+1
                </td>
              </tr>
              <tr>
                <td>I4CHAR</td>
                <td>Ermöglicht die Verwendung von Itemboxen, bei denen die NUM/ALFA Mechanik verwendet wird. Dabei werden Zahleneingaben nicht geprüft, jedoch die Eingabe von Text. Wird der Text eindeutig gefunden, so wird die Texteingabe durch die Nummer ersetzt. Wird der Text in der Alfa-Variante nicht gefunden, dann öffnet sich die für Alphanumerische Eingabe vorgesehene F3-Auswahl und man kann hier einen Eintrag auswählen und die Nummer wird zurückgeliefert.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Von Vorbelegung</p>
        </td>
        <td>
          <p>Hier kann eine Zeichenfolge eingegeben werden. Wird von dieser Auswahlliste eine weitere Auswahlliste aufgerufen, in der in der Bereichsauswahl in der “Von Vorbellegung“ dieselbe Zeichenfolge steht, werden die Werte aus der Vorherigen Bereichsauswahl übernommen. Eine Ausnahme bilden dabei die Werte</p>
          <ul>
            <li>KUNDNUMMER</li>
            <li>ARTIKELNUMMER</li>
            <li>LAGERNUMMER</li>
          </ul>
          <p>Diese werden in der Belegerfassung der Warenwirtschaft vorbelegt. Ruft man also z.B. aus der Lieferscheinerfassung heraus den Kundenstamm auf, so ist die Vorbelegung die im Lieferschein verwendete Kontonummer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bis Vorbelegung</p>
        </td>
        <td>
          <p>S.o.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Idx</p>
        </td>
        <td>
          <p>Eindeutige Index der Zeile. Ein vergebener Index kann nur einmal verwendet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aktiv</p>
        </td>
        <td>
          <p>Dieses Aktivkennzeichen bestimmt, wie das Häkchen auf dem Bereichsauswahlbildschirm startet bzw. dargestellt wird:</p>
          <table>
            <tbody>
              <tr>
                <th>Bezeichnung</th>
                <th></th>
              </tr>
              <tr>
                <td>wirksam</td>
                <td>Der Haken ist gesetzt und man kann in dem Eingabefeld/den Eingabefeldern Werte erfassen.</td>
              </tr>
              <tr>
                <td>unwirksam</td>
                <td>Der Haken ist nicht gesetzt und die Eingabefelder sind ausgeblendet.</td>
              </tr>
              <tr>
                <td>immer wirksam</td>
                <td>Der Haken ist gesetzt und kann nicht verändert werden.</td>
              </tr>
              <tr>
                <td>readonly</td>
                <td>Spezielle Programmfunktionalität. Hiermit werden Zeilen gekennzeichnet, die erst später bei bestimmten Programmeinstellungen aktiviert werden.</td>
              </tr>
              <tr>
                <td>immer, außer Stapel</td>
                <td>Wie „Immer wirksam“, nur im Stapelmodus wird diese Zeile wie „unwirksam“ behandelt.</td>
              </tr>
              <tr>
                <td>gelöscht</td>
                <td>Kennzeichnet, dass diese Zeile nicht mehr verwendet werden soll. Wird von A.eins so interpretiert, als ob diese Zeile nicht vorhanden wäre.</td>
              </tr>
            </tbody>
          </table>
          <p>Die Werte „wirksam“ und „unwirksam“ aktivieren die sogenannte Häkchen-Technik, mit der man einzelne Auswahlzeilen aktivieren bzw. deaktivieren kann. Wenn diese Werte verwendet werden, dann muss in der Spalte Variablenname und SQL-Ausdruck etwas eingetragen sein. Zusätzlich muss die Variable im SQL-Text verwendet werden(s.u.).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Variablenname</p>
        </td>
        <td>
          <p>Eine für diese Variante eindeutige Bezeichnung. Mit dem hier vergebenen Namen wird dann im SQL-Text auf den SQL-Ausdruck verwiesen.</p>
          <p>Beispiel:<br><br></p>
          <table>
            <tbody>
              <tr>
                <th>Variablenname</th>
                <th>SQL Ausdruck</th>
              </tr>
              <tr>
                <td>AUSW_KUNDE</td>
                <td>and isnull(k.KundId,0) between :PAR1 and :PAR2</td>
              </tr>
              <tr>
                <td>AUSW_SCHLAGNAME</td>
                <td>and a.SchlagName like ':PAR1'</td>
              </tr>
            </tbody>
          </table>
          <p>Der SQL-Text sieht dann wie folgt aus:</p>
          <div>
            <pre><code>…
SQL select :FIELDS,
 (Select list(v.VertragsNummer||'('||f.erntejahr||')')
  from Feldanerkennung f
  join Vermehrungsvertrag v on (f.vertragsid=v.vertragsid) where f.Schlagid=a.schlagid  ) as Vertraege,
 ans.AdressName||' '||ans.AdressVorname||' '||ans.AdressOrt as NameOrt
 from AckerschlagKartei a
      left outer join KundenStamm k on (k.kundid=a.VermehrerKundId)
      left outer join anschriftstamm ans on (k.AdressIdHauptAdr=ans.AdressId)
 where 1=1
 :AUSW_KUNDE
 :AUSW_SCHLAGNAME
 order by a.SchlagNummer
…</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>SQL-Ausdruck</p>
        </td>
        <td>
          <p>Ein SQL-Schnipsel. Siehe Oben</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steupa</p>
        </td>
        <td>
          <p>Wird hier ein Steuerparameter z.B. in der Form „SPA902==1“ angeben, so wird diese Zeile nur angezeigt, wenn der Steuerparameter den entsprechenden Wert hat.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sortierung</p>
        </td>
        <td>
          <p>Sortierung in der die Zeilen angezeigt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausblenden</p>
        </td>
        <td>
          <p>Für Crystal-Report. Ein <b>Ja</b> bewirkt, dass die entsprechende Zeile nicht im Kopf des Crystal-Reports angezeigt wird.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
