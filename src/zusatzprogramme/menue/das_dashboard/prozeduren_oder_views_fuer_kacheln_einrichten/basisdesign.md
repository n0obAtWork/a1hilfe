# Basisdesign

<!-- source: https://amic.de/hilfe/kachelbasisdesign.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Jede Prozedur bzw. jede View kann zur Gestaltung die folgenden Felder verwenden. Es wird empfohlen eine View für das Basisdesign im Dashboard zu hinterlegen und für die einzelnen Kacheln in den Prozeduren nur dort Werte zurückzuliefern, wo eine Abweichung vom Basisdesign gewünscht wird.

| Allgemeingültige Felder |
| --- |
| Pixelsize | Optional. Jede Kachel hat standardmäßig eine Seitenlänge von 166 Pixeln bzw. ein Vielfaches davon. Es kann hier ein Wert zwischen 120 und 360 Pixeln für die zu verwendenden Kantenlänge angegeben werden.<br> |
| Header | Text, der als Überschrift der Kachel verwendet wird. Wird dieses Feld nicht von der View geliefert oder ist leer, dann wird keine Überschriftzeile generiert und der Platz steht dem Mittelteil zur Verfügung.<br> |
| HeaderForecolor<br> | Optional. Die Vordergrundfarbe der Kopfzeile. Ist sie nicht gesetzt, so ist die Schriftfarbe Schwarz. Die Angabe aller Farben erfolgt in RGB-Form, entweder hexadezimal mit einem # vorweg oder dezimal durch einen Schrägstich '/' getrennt.<br><br>'#FF0000‘ as headerbordercolor<br>Oder<br>'255/00/00'<br> |
| HeaderBackcolor<br> | Optional. Die Hintergrundfarbe der Kopfzeile. Ist sie nicht gesetzt, behält der Hintergrund dieselbe Farbe wie der Mittelteil.<br> |
| HeaderBackcolor2 | Optional. Wird HeaderBackcolor2 mit angegeben und unterscheidet sich von Backcolor, dann wird die Hintergrundfarbe der Kachel als Farbverlauf dargestellt:<br><code>Select '255/128/0' as headercolor, '255/254/0' as headercolor2</code><br> |
| HeaderBorderStyle | Rahmen um die Kachel-Überschriftszeile.<br>• 'none' as borderstyle<br>• 'solid' as borderstyle<br>• 'raised' as borderstyle<br>• 'inset‘ as borderstyle<br>Standardeinstellung ist 'none'<br> |
| HeaderBorderColor | Die Rahmenfarbe wird nur beim Borderstyle 'solid' ausgewertet.<br> |
| Headeralign | Ausrichtung der Überschrift. Mögliche Werte sind<br>• 'left' as headeralign<br>• 'center' as headeralign<br>• 'right' as headeralign<br> |
| Footer | Text, der in der Fußzeile erscheint. Wird dieses Feld nicht von der View geliefert oder ist leer, dann wird keine Überschriftzeile generiert und der Platz steht dem Mittelteil zur Verfügung.<br> |
| FooterForecolor,<br>FooterBackcolor,<br>FooterBackcolor2<br>FooterBorderstyle<br>FooterBorderColor,<br>FooterAlign<br> | Siehe Beschreibung zu Header. |
| Forecolor,<br>Backcolor,<br>Backcolor2,<br>Borderstyle,<br>BorderColor<br>TextAlign<br> | Siehe Beschreibung zu Header. |
| Tooltipheader<br>Tooltiptext<br>Tolltipfooter<br> | Zusätzlich besteht die Möglichkeit der Kachel einen Tooltip zuzuordnen. Dafür kann man in der View diese Felder verwenden. Diese Texte lassen sich **nicht** mit HTML formatieren. Nur ein Zeilenumbruch kann mit &lt;br> erzwungen werden.<br>![](../../../../ImagesExt/image8_1491.png)<br> |
| Id1<br>Id2<br>Id3<br>Id4<br> | Diese Felder werden dann benötigt, wenn man aus der Kachel heraus einen Pfleger aufrufen will, oder wenn man einen Punkt/eine Zeile auswählen will. Diese IDs entsprechen den Ident´s, wie man sie z.B. aus der Auswahlliste kennt. |
| Selected<br> | Bei den Darstellungsarten „Geographische Karte“ und „Tabelle“ kann ein Punkt bzw. eine Zeile ausgewählt werden. Möchte man, dass nach dem Refreshen einer Kachel (siehe auch [Refresh-Prozedur](../kachel_einrichten.md#RefreshProzedur)) der Punkt/die Zeile, die den Datensatz kennzeichnet, wieder angezeigt wird, dann kann man das mit diesem Feld erreichen. Das Feld hat den Typen integer. Die erste Zeile mit dem Wert 1 wird dann markiert. Es ist nicht möglich mehrere Datensätze zu markieren.<br> <br>Beispielprozedur:<br><pre><code>CREATE PROCEDURE p_dash_geographicMap ()&#10;BEGIN&#10; if varexists('pdb_adressid') = 0 then&#10; create or replace variable pdb_adressid integer = -1;&#10; endif;&#10; &#10; select&#10;&#10; ans.AdressId as ID1,&#10; if pdb_adressid = ans.Adressid then 1 else 0 endif as selected,&#10; &#10; if Adresstyp = 11 then 0&#10; else if Adresstyp = 12 then 1&#10; else if Adresstyp = 15 then 2&#10; else 3&#10; endif endif endif as Serie,&#10; &#10; if Adresstyp in (11,12,15) then&#10; AMIC_FORMLST_GETBEZEICH('ADRESSTYP',Adresstyp)&#10; else&#10; 'Sonstige'&#10; endif as SeriesTitle,&#10; &#10; '&lt;u&gt;&lt;b&gt;' &#124;&#124; ans.adressname &#124;&#124; '&lt;/b&gt;&lt;/u&gt;&lt;br&gt;'&#10; &#124;&#124; ans.adressStrasse &#124;&#124; '&lt;br&gt;'&#10; &#124;&#124; ans.adressPLZ1 &#124;&#124; ' ' &#124;&#124; ans.adressOrt &#124;&#124; '&lt;br&gt;' as label,&#10; &#10; cast(posi.ST_LAT() as numeric(15,6)) as X,&#10; cast(posi.ST_LONG() as numeric(15,6)) as Y&#10; &#10; from anschriftgeodata geo&#10; join anschriftstamm ans on ans.adressid=geo.adressid&#10; join Kundenstamm k on k.KundId = ans.adressnummer&#10; where ans.adressnummer &gt;= 0&#10; and KundLoeKennz=0&#10; &#10;EXCEPTION&#10; when others then&#10; call amic_exception( ERRORMSG() &#124;&#124; CHAR(10) &#124;&#124; CHAR(13) &#124;&#124; TRACEBACK(), SQLCODE , SQLSTATE , 'p_dash_geographicMap' , -1 );&#10; END</code></pre><br> |

Die Texte Überschrift und Fußzeile können mit einfachen HTML-Tags formatiert werden. Der Text

```xml
<font color=“#000000“ size=5><u><b>Dieser</b> Text wird unterstrichen</u></font>
```

wird folgendermaßen dargestellt:

![](../../../../ImagesExt/image8_1492.png)
