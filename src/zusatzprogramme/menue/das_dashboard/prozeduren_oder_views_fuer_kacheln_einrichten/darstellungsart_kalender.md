# Darstellungsart Kalender

<!-- source: https://amic.de/hilfe/kachelkalender.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p><img src="../../../../ImagesExt/image8_1525.jpg" alt=""></p>
        </td>
        <td>
          <p><strong>Kalender</strong></p>
          <p>Der Kalender ist ein Control, welches zur Auswahl eines Stichtages verwendet werden kann. Das Design ist über folgende Felder in der View/Prozedur zu steuern:</p>
          <ul>
            <li><b>SelectedDate:</b> Das Datum, das in der Anzeige als ausgewählt erscheint. Das ausgewählte Datum bestimmt den Monat, der angezeigt wird. Standard ist das Tagesdatum.</li>
            <li><b>Fontname</b>: Name der Schriftart. Standard ist „Verdana“.</li>
            <li><b>Fontsize</b>: Größe der Schriftart. Die Größe des Kalenders wird durch die Größe der Schriftart gesteuert. Standard ist 9.</li>
            <li><b>TitleBackColor</b>: Hintergrundfarbe der Überschrift mit Monat und Jahr.</li>
            <li><b>TitleForeColor</b>: Vordergrundfarbe der Überschrift mit Monat und Jahr.</li>
            <li><b>TrailingForeColor:</b> Die Farbe für die Tage, die nicht zum Monat gehören. Standard ist Transparent.</li>
            <li><b>DimensionX</b> und</li>
            <li><b>DimensionY: </b>Es besteht die Möglichkeit mehrere Monate nebeneinander und/oder untereinander darzustellen. Standardeinstellung ist 1 für X und 1 für Y. Setzt man z.B. für DimensionX auf 4 und DimensionY auf 3 sieht das Ergebnis folgendermaßen aus:<br><img src="../../../../ImagesExt/image8_1526.jpg" alt=""><br>Beispielview:<br>Um eine Datenbankvariable mit dem Stichtag setzen zu können, muss diese dann in der Refresh-Prozedur gesetzt werden. In dem Feld in_Ident1 wird der ausgewählte Tag übergeben.<br>Beispiel Refresh-Prozedur:</li>
          </ul>
          <div>
            <pre><code>CREATE VIEW p_dash_v_kalender AS
 select
   'Stichtag' as header,
   'solid' as borderstyle,
   '68/68/68' as bordercolor,
   'Verdana' as fontname,
   9.0       as fontsize,
   4         as DimensionX,
   3         as DimensionY</code></pre>
          </div>
          <div>
            <pre><code>CREATE PROCEDURE p_dash_refresh_kalender
  (in in_board integer,
   in in_kachel integer,
   in in_ident1 date      default null,
   in in_ident2 char(100) default null,
   in in_ident3 char(100) default null,
   in in_ident4 char(100) default null )
BEGIN
  create or Replace Variable pdb_stichtag date;
  set pdb_stichtag = in_ident1;
  select id_kachel from Dash_Board_Kachel_Link
    where Id_Board = in_board and id_kachel!=in_Kachel;
EXCEPTION
  when others then
    call amic_exception( ERRORMSG() || CHAR(10) || CHAR(13) || TRACEBACK(), SQLCODE , SQLSTATE , 'p_dash_refresh_kalender' , -1 );
END</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
