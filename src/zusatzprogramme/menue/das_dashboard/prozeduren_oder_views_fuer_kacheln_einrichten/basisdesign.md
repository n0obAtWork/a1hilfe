# Basisdesign

<!-- source: https://amic.de/hilfe/kachelbasisdesign.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Jede Prozedur bzw. jede View kann zur Gestaltung die folgenden Felder verwenden. Es wird empfohlen eine View für das Basisdesign im Dashboard zu hinterlegen und für die einzelnen Kacheln in den Prozeduren nur dort Werte zurückzuliefern, wo eine Abweichung vom Basisdesign gewünscht wird.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p>Allgemeingültige Felder</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pixelsize</p>
        </td>
        <td>
          <p>Optional. Jede Kachel hat standardmäßig eine Seitenlänge von 166 Pixeln bzw. ein Vielfaches davon. Es kann hier ein Wert zwischen 120 und 360 Pixeln für die zu verwendenden Kantenlänge angegeben werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Header</p>
        </td>
        <td>
          <p>Text, der als Überschrift der Kachel verwendet wird. Wird dieses Feld nicht von der View geliefert oder ist leer, dann wird keine Überschriftzeile generiert und der Platz steht dem Mittelteil zur Verfügung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>HeaderForecolor</p>
        </td>
        <td>
          <p>Optional. Die Vordergrundfarbe der Kopfzeile. Ist sie nicht gesetzt, so ist die Schriftfarbe Schwarz. Die Angabe aller Farben erfolgt in RGB-Form, entweder hexadezimal mit einem # vorweg oder dezimal durch einen Schrägstich '/' getrennt.</p>
          <p><br>'#FF0000‘ as headerbordercolor</p>
          <p>Oder</p>
          <p>'255/00/00'</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>HeaderBackcolor</p>
        </td>
        <td>
          <p>Optional. Die Hintergrundfarbe der Kopfzeile. Ist sie nicht gesetzt, behält der Hintergrund dieselbe Farbe wie der Mittelteil.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>HeaderBackcolor2</p>
        </td>
        <td>
          <p>Optional. Wird HeaderBackcolor2 mit angegeben und unterscheidet sich von Backcolor, dann wird die Hintergrundfarbe der Kachel als Farbverlauf dargestellt:</p>
          <div>
            <code>Select '255/128/0' as headercolor, '255/254/0' as headercolor2</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>HeaderBorderStyle</p>
        </td>
        <td>
          <p>Rahmen um die Kachel-Überschriftszeile.</p>
          <ul>
            <li>'none' as borderstyle</li>
            <li>'solid' as borderstyle</li>
            <li>'raised' as borderstyle</li>
            <li>'inset‘ as borderstyle<br>Standardeinstellung ist 'none'</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>HeaderBorderColor</p>
        </td>
        <td>
          <p>Die Rahmenfarbe wird nur beim Borderstyle 'solid' ausgewertet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Headeralign</p>
        </td>
        <td>
          <p>Ausrichtung der Überschrift. Mögliche Werte sind</p>
          <ul>
            <li>'left' as headeralign</li>
            <li>'center' as headeralign</li>
            <li>'right' as headeralign</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Footer</p>
        </td>
        <td>
          <p>Text, der in der Fußzeile erscheint. Wird dieses Feld nicht von der View geliefert oder ist leer, dann wird keine Überschriftzeile generiert und der Platz steht dem Mittelteil zur Verfügung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FooterForecolor,</p>
          <p>FooterBackcolor,</p>
          <p>FooterBackcolor2</p>
          <p>FooterBorderstyle</p>
          <p>FooterBorderColor,</p>
          <p>FooterAlign</p>
        </td>
        <td>
          <p>Siehe Beschreibung zu Header.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Forecolor,</p>
          <p>Backcolor,</p>
          <p>Backcolor2,</p>
          <p>Borderstyle,</p>
          <p>BorderColor</p>
          <p>TextAlign</p>
        </td>
        <td>
          <p>Siehe Beschreibung zu Header.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tooltipheader</p>
          <p>Tooltiptext</p>
          <p>Tolltipfooter</p>
        </td>
        <td>
          <p>Zusätzlich besteht die Möglichkeit der Kachel einen Tooltip zuzuordnen. Dafür kann man in der View diese Felder verwenden. Diese Texte lassen sich <b><u>nicht</u></b> mit HTML formatieren. Nur ein Zeilenumbruch kann mit &lt;br&gt; erzwungen werden.</p>
          <p><img src="../../../../ImagesExt/image8_1491.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Id1</p>
          <p>Id2</p>
          <p>Id3</p>
          <p>Id4</p>
        </td>
        <td>
          <p>Diese Felder werden dann benötigt, wenn man aus der Kachel heraus einen Pfleger aufrufen will, oder wenn man einen Punkt/eine Zeile auswählen will. Diese IDs entsprechen den Ident´s, wie man sie z.B. aus der Auswahlliste kennt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Selected</p>
        </td>
        <td>
          <p>Bei den Darstellungsarten „Geographische Karte“ und „Tabelle“ kann ein Punkt bzw. eine Zeile ausgewählt werden. Möchte man, dass nach dem Refreshen einer Kachel (siehe auch <a href="../kachel_einrichten.md#RefreshProzedur">Refresh-Prozedur</a>) der Punkt/die Zeile, die den Datensatz kennzeichnet, wieder angezeigt wird, dann kann man das mit diesem Feld erreichen. Das Feld hat den Typen integer. Die erste Zeile mit dem Wert 1 wird dann markiert. Es ist nicht möglich mehrere Datensätze zu markieren.</p>
          <p>Beispielprozedur:</p>
          <div>
            <pre><code>CREATE PROCEDURE p_dash_geographicMap ()
BEGIN
  if varexists('pdb_adressid') = 0 then
   create or replace variable pdb_adressid integer = -1;
  endif;
 select
  ans.AdressId as ID1,
  if pdb_adressid = ans.Adressid then 1 else 0 endif as selected,
    if Adresstyp = 11 then 0
    else if Adresstyp = 12 then 1
    else if Adresstyp = 15 then 2
    else 3
    endif endif endif as Serie,
    if Adresstyp in (11,12,15) then
      AMIC_FORMLST_GETBEZEICH('ADRESSTYP',Adresstyp)
    else
      'Sonstige'
    endif as SeriesTitle,
    '&lt;u&gt;&lt;b&gt;' || ans.adressname || '&lt;/b&gt;&lt;/u&gt;&lt;br&gt;'
             || ans.adressStrasse || '&lt;br&gt;'
             || ans.adressPLZ1 || ' ' || ans.adressOrt || '&lt;br&gt;'  as label,
    cast(posi.ST_LAT() as numeric(15,6))  as X,
    cast(posi.ST_LONG() as numeric(15,6))  as Y
    from anschriftgeodata geo
    join anschriftstamm ans on ans.adressid=geo.adressid
    join Kundenstamm k on k.KundId = ans.adressnummer
    where ans.adressnummer &gt;= 0
      and KundLoeKennz=0
EXCEPTION
   when others then
     call amic_exception( ERRORMSG() || CHAR(10) || CHAR(13) || TRACEBACK(), SQLCODE , SQLSTATE , 'p_dash_geographicMap' , -1 );
 END</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Die Texte Überschrift und Fußzeile können mit einfachen HTML-Tags formatiert werden. Der Text

```xml
<font color=“#000000“ size=5><u><b>Dieser</b> Text wird unterstrichen</u></font>
```

wird folgendermaßen dargestellt:

![](../../../../ImagesExt/image8_1492.png)
