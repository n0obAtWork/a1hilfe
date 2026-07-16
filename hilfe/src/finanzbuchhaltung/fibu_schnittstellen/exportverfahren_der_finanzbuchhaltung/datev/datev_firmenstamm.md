# DATEV-Firmenstamm

<!-- source: https://amic.de/hilfe/datevfirmenstamm.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > DATEV Firmenstamm

![Ein Bild, das Text enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_798.png "Ein Bild, das Text enthält. Automatisch generierte Beschreibung")

Hier werden einmalig Daten erfasst, die für die Erkennung der Datei erforderlich sind.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feldname</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datei-Format</p>
        </td>
        <td>
          <p>Es stehen drei Dateiformate zur Verfügung</p>
          <p><u>(OBE) Ordnungsbegriffserweiterung</u><br>Es gelten für dieses Format folgende Einschränkungen:</p>
          <p>Referenznummern dürfen nur Numerisch sein.</p>
          <p>Belegnummer/Referenznummer darf nur maximal 6 Stellen haben.</p>
          <p>Kostenstellennummer darf maximal 4 Stellen haben</p>
          <p>Sachkonten dürfen nur 4 Stellen und Personenkonten nur 5 Stellen haben. Der Bereich der Debitoren ist auf 10000 bis 69999 und der der Kreditoren auf 70000 bis 99999 festgelegt.</p>
          <p>(KNE) Kontonummernerweiterung.</p>
          <p>Das Format KNE ist seit 08.2000 gültig. Es kann nur von den aktuellen DATEV-Windows-Programmen mit Schnittstelle importiert werden. Es ist hier eine Absprache mit dem Steuerberater notwendig. Dieses Format unterscheidet sich in folgenden Punkten von dem Aufzeichnungsverfahren OBE:</p>
          <p>Referenznummern können auch Alphanumerisch sein.</p>
          <p>Belegnummer/Referenznummer können bis zu 12 Stellen haben.</p>
          <p>Umsatz kann statt 10 Stellen bis zu 12 Stellen (incl. Nachkommastellen) haben.</p>
          <p>Kostenstellen können bis zu 8 Stellen haben (OBE nur 4).</p>
          <p>Sachkonten können bis zu 8 Stellen haben. Hier gelten die von der DATEV vorgegebenen Regeln (Nummer der Personenkonten muss eine Stelle mehr haben als die der Sachkonten)</p>
          <p>Die Namen der zu übertragenden Dateien werden statt DV01 bzw. DE001/DE002 zu EV01 und ED00001/ED00002.</p>
          <p>Format 3.0</p>
          <p>Dieses Format ist seit 2012 gültig und soll die bisherigen Postversandverfahren OBE und KNE ablösen. Zum Jahreswechsel&nbsp; 2017/2018 wurden diese Formate von DATEV abgekündigt.</p>
          <p>Die Namen der zu übertagenden Dateien lauten:<br><br></p>
          <p>EXTF_BUCHUNGSSTAPEL_ID_JJJJPP.csv</p>
          <p>Und</p>
          <p>EXTF_STAMMDATEN_ID_JJJJPP.csv</p>
          <p>ID steht für die Datev-Id. Dadurch sind die Dateien immer dem entsprechenden Datenbestand in A.eins zuzuordnen.</p>
          <p>JJJJ Steht für das beim Export angegebene Wirtschaftsjahr.</p>
          <p>PP für die „Bis-Periode“.</p>
          <p>Format 7.0</p>
          <p>Dies ist eine Weiterentwicklung des Formats 3.0 und seit 2018 gültig.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Übertragungs-Format</p>
        </td>
        <td>
          <p>Hier kann angegeben werden, ob die Steuerinformation über das Steuerkonto übermittelt wird oder ob nur der DATEV-Steuerschlüssel übermittelt wird und somit auf Seiten der DATEV automatisch der UST-Betrag&nbsp; errechnet und auf das entsprechende Sammelkonto gebucht wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sachkontonummernlänge</p>
        </td>
        <td>
          <p>Bei den Format 3.0, 7.0 und KNE kann man hier die Länge der Sachkontonummer eintragen. Diese darf zwischen 4 und 8 liegen. Es wird dann automatisch davon ausgegangen, dass die Personenkonten jeweils um eine Stelle länger sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateien überschreiben</p>
        </td>
        <td>
          <p>Dieses Feld ist nur für die Formate OBE und KNE aktiv. Ab Format 3.0 werden für verschiedene Exporte immer eigene Dateien geschrieben, da im Dateinamen die DATEV-Id enthalten ist.</p>
          <p>Hiermit wird gesteuert, ob die Daten die bestehenden Dateien überschreiben sollen oder ob sie angehängt werden. Die Standardeinstellung ist <b>Ja</b>, so dass die Dateien immer ohne Prüfung neu erstellt werden. Stellt man hier <b>Nein</b> ein, so ändert sich das Verhalten etwas. Zum einen werden die Daten an die Dateien angehängt, zum anderen ist es nur noch dann erlaubt die Dateiausgabe zu wiederholen, wenn im Verzeichnis keine Steuerungsdatei (DV01 oder EV01) enthalten ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ersatzkontonummer aus Forderungsgruppe</p>
        </td>
        <td>
          <p>Wird dieses Feld auf <b>Ja </b>gestellt, so erhält man die Möglichkeit zu <a href="../../../stammdaten_der_fibu/forderungsgruppen.md">Forderungsgruppen</a> (Direktsprung <strong>[FORG]</strong>) Ersatzkontonummer für Personenkonten zu hinterlegen. Es wird dann beim DATEV-Übertrag an Stelle der Personenkontonummer aus dem Beleg die in der Forderungsgruppe hinterlegte Personenkontonummer verwendet. D.h. es wird lediglich ein Konto pro Forderungsgruppe übertragen. Dies dient lediglich dem Zweck, den DATEV-Übertrag zu ermöglichen, obwohl man bei der Einrichtung der Kontonummern nicht bedacht hat, dass DATEV sowohl bei der Länge als auch beim Bereich für Debitoren und Kreditoren Einschränkungen macht.&nbsp;<br>Für dieses Verfahren ergeben sich folgende Einschränkungen:</p>
          <p>OP-Führung ist auf der DATEV-Seite nicht möglich.</p>
          <p>Auszifferungsinformationen können nicht an die DATEV übertragen werden.</p>
          <p>Mahnwesen ist für die DATEV-Seite nicht möglich.</p>
          <p>Der Übertrag der Stammdaten ( Personenkonten ) macht keinen Sinn mehr.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion Mandantentrennung</p>
        </td>
        <td>
          <p>Diese Datenbankfunktion dient dazu, zu bestimmen, welcher Beleg zu welchem Mandanten gehört. Ist keine Funktion hinterlegt, wird keine Mandantentrennung vorgenommen. Dies ist die Standardeinstellung. Die Funktion bekommt als Parameter die FiBuV_Id übergeben und muss die Nummer des Mandanten (siehe unten) zurückliefern. Die Funktion könnte wie folgt aussehen:</p>
          <div>
            <pre><code>CREATE Function AMIC_FIBU_DATEVMEHRMANDANT( in in_FiBuV_Id       integer )
returns integer
BEGIN
  declare Ergebnis integer;
  set Ergebnis = (select MandNummer from MandantenZuordnung);
  return Ergebnis;
END</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>Prozedur Zusatzdaten</p>
        </td>
        <td>
          <p>Diese Funktion wird nur für die Formate 3.0 und 7.0 ausgewertet. Von A.eins werden nur bestimmte Felder aus der Feldbeschreibung des Buchungsstapels verwendet. Gibt man hier eine Prozedur an, so können alle nicht von A.eins verwendeten Felder Individuell belegt werden. Diese Prozedur muss folgenden Aufbau haben:</p>
          <div>
            <pre><code>CREATE PROCEDURE p_DATEVEXPORT_ZUSATZDATEN
  (
    in in_Fibuv_id integer,
    in in_Fibuv_poszaehler integer,
    in in_Fibuvp_buchtyp integer
  )
RESULT (
  fieldNumber       integer,
  StringValue  char(255),
  IntValue     integer,
  DateValue    date,
  NumericValue numeric(15,6)
)
BEGIN
 .
 .
 .
END</code></pre>
          </div>
          <p>Diese Funktion wird einmal pro ausgegebener Datenzeile aufgerufen und muss je Feld, welches versorgt werden soll, einen Datensatz zurückliefern.</p>
          <p>Die Feldnummer ist die Feldnummer laut DATEV-Dokumentation des Buchungsstapels<br>In die Felder StringValue, IntValue, DateValue, NumericValue erwartet A.eins je nach Datentyp einen Eintrag. Wir Null als Wert zurückgeliefert, so werden keine Daten ausgegeben.</p>
          <p>Beispiel, in dem die Zinssperre aus dem Kundenstamm und das KOST-Datum mit dem Tagesdatum versorgt werden soll.</p>
          <div>
            <pre><code>CREATE PROCEDURE p_DATEVEXPORT_ZUSATZDATEN
  (
    in in_Fibuv_id integer,
    in in_Fibuv_poszaehler integer,
    in in_Fibuvp_buchtyp integer
  )
RESULT (
  fieldNumber       integer,
  StringValue  char(255),
  IntValue     integer,
  DateValue    date,
  NumericValue numeric(15,6)
)
BEGIN
  select 19, null, KundZinssperr, null, null
    from kundenstamm k
    join fibuvorgposition p on p.kontonummer=k.kontonummer
    Where p.fibuv_id = in_Fibuv_id and fibuv_poszaehler=1
  union
  select 104, null, null, today(*), null from dummy
END</code></pre>
          </div>
          <p>Hinweis: Der Parameter in_fibuv_poszaehler ist immer der Zaehler der Zeile mit dem Erlöskonto</p>
          <p>Von A.eins werden die Felder Typgerecht ausgelesen und auch auf die korrekte Ausgabelänge geprüft. Bei Felder, die zu Lang geliefert werden, wird ein entsprechender Eintrag ins Fehlerprotokoll geschrieben. Die Ausgabe in die Datei erfolgt bei Zahlen jedoch ungekürzt, zu lange Texte werden auf die zulässige Länge gekürzt.</p>
          <p>Folgende Felder werden von A.eins versorgt und können nicht geändert werden:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Nummer laut DATEV- Dokumentation</b></th>
                <th><b>Bedeutung</b></th>
              </tr>
              <tr>
                <td>1</td>
                <td>Umsatz</td>
              </tr>
              <tr>
                <td>2</td>
                <td>Sollhaben-Kennzeichen</td>
              </tr>
              <tr>
                <td>7</td>
                <td>Konto</td>
              </tr>
              <tr>
                <td>8</td>
                <td>Gegenkonto</td>
              </tr>
              <tr>
                <td>9</td>
                <td>Steuerschlüssel oder Berichtigungsschlüssel</td>
              </tr>
              <tr>
                <td>10</td>
                <td>Belegdatum</td>
              </tr>
              <tr>
                <td>11</td>
                <td>Belegfeld1</td>
              </tr>
              <tr>
                <td>12</td>
                <td>Belegfeld2</td>
              </tr>
              <tr>
                <td>13</td>
                <td>Skonto</td>
              </tr>
              <tr>
                <td>14</td>
                <td>Buchungstext</td>
              </tr>
              <tr>
                <td>40</td>
                <td>EU-Mitgliedstaat u. UStIdNr.</td>
              </tr>
              <tr>
                <td>41</td>
                <td>EG-Steuersatz</td>
              </tr>
              <tr>
                <td>114</td>
                <td>Festschreibungskennzeichen</td>
              </tr>
              <tr>
                <td>118</td>
                <td>Generalumkehr</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer / Mandantenbezeichnung</p>
        </td>
        <td>
          <p>Es ist möglich den Export so zu steuern, dass die Belege auf mehrere Mandanten aufgeteilt werden. Zu diesen Mandanten muss eine eindeutige Nummer und optional eine Mandantenbezeichnung erfasst werden. Über die Mandantennummer werden dann die Belege dem DATEV-Mandanten zugeordnet. Um zu bestimmen, welche Nummer welchem Mandanten zuzuordnen ist, muss man eine Datenbankfunktion anlegen (siehe oben).<br>&nbsp;Beim Erstellen der Datei wird zusätzlich zum angegebenen Pfad ein Unterverzeichnis pro Mandanten angelegt. Dieses Verzeichnis bekommt als Namen die Mandantenbezeichnung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datenträger Nr.</p>
        </td>
        <td>
          <p>Dies ist eine frei wählbare 3stellige Nummer. Wird ab Format 3.0 nicht mehr verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Passwort, Beratername, Beraternummer und Mandant DATEV</p>
        </td>
        <td>
          <p>Dies sind Informationen, die man vom Steuerberater erhält. Wenn vom Steuerberater diese Informationen nicht gefordert/geliefert werden, so sind hier Nullen einzutragen. In den Formaten 3.0/7.0 werden die Felder Passwort und Beratername nicht mehr benötigt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
