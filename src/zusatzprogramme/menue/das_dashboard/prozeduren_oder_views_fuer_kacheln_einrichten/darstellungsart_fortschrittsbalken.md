# Darstellungsart Fortschrittsbalken

<!-- source: https://amic.de/hilfe/kachelfortschrittsbalken.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![](../../../../ImagesExt/image8_1494.png) | Fortschrittsbalken<br>Der Fortschrittsbalken benötigt zusätzlich zu den Feldern, die auch die Darstellungsart Text haben, noch die Felder, die den ihn beschreiben:<br>ProgressbarMinimum, muss den Datenbanktypen integer liefern. Standard ist 0.<br>ProgressbarMaximum, muss den Datenbanktypen integer liefern. Standard ist 100.<br>**ProgressbarValue**, muss den Datenbanktypen integer liefern. Der Wert sollte zwischen Minimum und Maximum liegen.<br>ProgressbarText (Optional). Wenn nicht angegeben, so wird „{nnn}% vom {ProgressbarMaximum}“ ausgegeben<br> <br>Beispielview:<br><pre><code>CREATE VIEW p_dash_fortschritt AS&#10; &#10;select&#10; &#10; 'Auftragseingang' as header,&#10; 'von&#10; 01.01.' &#124;&#124; year(Today(*)) &#124;&#124;' bis heute' as footer,&#10; 'Text' as&#10; text,&#10; &#10; '255/255/255' as Backcolor,&#10; &#10; '63/63/63' as bordercolor,&#10; &#10; 'solid' as borderstyle,&#10; &#10; -- Der Fortschrittsbalken benötigt folgende&#10; Felder&#10; &#10; &#10; 0 as progressdBarMinimum,&#10; &#10; (select count() from amic_v_vorgaenge vs&#10; where vs.v_klassnummer=400 and vs.V_Datum=today()) &#10; as progressdBarMaximum,&#10; &#10; (select count() from amic_v_vorgaenge vs&#10; where vs.v_klassnummer=400 and vs.V_Datum=today() and v_statusUmwand &gt;=&#10; 5)&#10; as&#10; progressdBarValue,&#10; ' ' as&#10; progressdBarText</code></pre><br> <br> |
