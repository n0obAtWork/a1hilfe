# Darstellungsart Bild

<!-- source: https://amic.de/hilfe/kachelbild.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![](../../../../ImagesExt/image8_1511.png) | Bild<br>Neben den bekannten Feldern muss die View zusätzlich ein Feld **Picture** mit dem Bildinhalt zurückliefern. Dafür bietet sich das Formulararchiv an. Erlaubte Formate sind Bitmap, Icon, JPEG, GIF und PNG.<br><br>Beispielview:<br><pre><code>CREATE VIEW p_dash_warnung AS&#10; &#10; select&#10;&#10; (select&#10; count(*) from fehlerprotokoll where FehlProtDB_USER = USER)&#10; Anzahl,&#10; &#10; if Anzahl&#10; &gt; 0 then 'Fehlerprotokoll' else '' endif as footer,&#10; 'center'&#10; footeralign,&#10; &#10; '#FFFFFF'&#10; as color,&#10; '#333333' as bordercolor,&#10; 'solid'&#10; as borderstyle,&#10; &#10; &#10; &#10; 'Demo-Dashboard' as toolTipHeader,&#10; &#10; if Anzahl &gt; 0&#10; then&#10;&#10; 'Es sind ' &#124;&#124; Anzahl&#10; &#124;&#124; ' Einträge im Fehlerprotokoll enthalten. &lt;br&gt;Bitte&#10; überprüfen.'&#10; &#10; else&#10; 'Keine&#10; Fehlerprotokoll-Einträge vorhanden.'&#10; endif as&#10; toolTipText,&#10; -- &#10;&#10; -- Bilder können&#10; aus dem Archiv geladen werden. Dazu benötigt man die FA_ID.&#10; -- &#10;&#10; &#10; AMICBLOB as picture from&#10; amic_fa_get_from_key(if Anzahl = 0 then 48045 else if Anzahl&gt;10 then&#10; 48044 else 48043 endif endif) as picture</code></pre><br> |
