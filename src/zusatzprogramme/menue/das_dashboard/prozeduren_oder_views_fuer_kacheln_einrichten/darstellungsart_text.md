# Darstellungsart Text

<!-- source: https://amic.de/hilfe/kacheltext.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![Ein Bild, das Text enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_1493.png "Ein Bild, das Text enthält. Automatisch generierte Beschreibung") | Text<br>Für die Darstellungsart Text benötigt die View zusätzlich zu den Standardfeldern nur das Feld **Text**. Optional kann noch Textalign verwendet werden, um anzugeben, wo der Text dargestellt wird. Mögliche Werte sind 'left', 'center' und 'right'. Wird Textalig nicht angegeben, so wird der Text zentriert dargestellt.<br> <br>Beispielview:<br><pre><code>CREATE VIEW p_dash_button_oder_text AS&#10; select&#10; &#10; 'Auftragsvolumen' as Header,&#10; &#10; trim(AMIC_FSTR(sum(WaBewWert), 20, 2))&#10; as Text,&#10; 'center' as&#10; Textalign&#10; &#10; 'vom 01.01.'&#124;&#124; year(today(*)) &#124;&#124; ' bis heute' as&#10; Footer,&#10; '255/255/255' as&#10; color,&#10; &#10; 'solid' as borderstyle,&#10; '#333333' as&#10; bordercolor&#10; from&#10; . . .</code></pre><br> |
