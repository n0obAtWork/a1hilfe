# Darstellungsart Bild

<!-- source: https://amic.de/hilfe/kachelbild.htm -->

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
          <p><img src="../../../../ImagesExt/image8_1511.png" alt=""></p>
        </td>
        <td>
          <p><strong>Bild</strong></p>
          <p>Neben den bekannten Feldern muss die View zusätzlich ein Feld <b>Picture</b> mit dem Bildinhalt zurückliefern. Dafür bietet sich das Formulararchiv an. Erlaubte Formate sind Bitmap, Icon, JPEG, GIF und PNG.<br><br>Beispielview:</p>
          <div>
            <pre><code>CREATE VIEW p_dash_warnung AS
 select
   (select count(*) from fehlerprotokoll where FehlProtDB_USER = USER) Anzahl,
   if Anzahl &gt; 0 then 'Fehlerprotokoll' else '' endif as footer,
   'center' footeralign,
   '#FFFFFF' as color,
   '#333333' as bordercolor,
   'solid' as borderstyle,
   'Demo-Dashboard' as toolTipHeader,
   if Anzahl &gt; 0
   then
      'Es sind ' || Anzahl || ' Einträge im Fehlerprotokoll enthalten. &lt;br&gt;Bitte überprüfen.'
   else
     'Keine Fehlerprotokoll-Einträge vorhanden.'
   endif as toolTipText,
 --
 -- Bilder können aus dem Archiv geladen werden. Dazu benötigt man die FA_ID.
 --
   AMICBLOB as picture from amic_fa_get_from_key(if Anzahl = 0 then 48045 else if Anzahl&gt;10 then 48044 else 48043 endif endif) as picture</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
