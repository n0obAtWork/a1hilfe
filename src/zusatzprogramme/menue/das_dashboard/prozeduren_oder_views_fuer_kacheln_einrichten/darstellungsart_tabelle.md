# Darstellungsart Tabelle

<!-- source: https://amic.de/hilfe/kacheltabelle.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![](../../../../ImagesExt/image8_1524.png) | Tabelle<br>In dieser Tabelle lassen sich ausgewählte Daten darstellen. Die Tabelle ist nicht für Massendaten vorgesehen und bietet nicht die Möglichkeiten der Auswahlliste. Die angegebene Klick-Funktion reagiert beim Klick auf die Zeile. Aus der zugrundeliegenden View werden alle Spalten, die mit „col(“ beginnen, angezeigt. Dabei steht in Klammern die Überschrift. Die Breite richtet sich nach der Breite der Datenfelder, numerische Werte werden immer mit zwei Nachkommastellen ausgegeben.<br> <br>Beispielview:<br><pre><code>Create view p_dash_tabelle&#10; &#10;Select TOP&#10; 20&#10; (select Count(*) from offenerposten where KontoNummer&#10; = k.KontoNummer ) as AnzahlOPs,&#10; (select sum(kontoSumerfSoll-KontoSumerfhaben) from&#10; kontosummen where KontoNummer = k.KontoNummer ) as SummeOPs,&#10; &#10; kundbezeich as "col(Bezeichnung)",&#10; &#10; AnzahlOPs as "col(OPs)",&#10; &#10; SummeOPs as "col(Saldo)",&#10; &#10; KundId as&#10; ID1,&#10; &#10; ans.AdressId as ID2,&#10; if&#10; ans.AdressId = pdb_adressid then 1 else 0 endif as selected&#10; &#10; From Kundenstamm k&#10; join&#10; anschriftstamm ans on ans.adressid=k.adressidhauptadr and adresstyp =&#10; 11&#10; &#10; join anschriftgeodata geo on&#10; ans.adressid=geo.adressid&#10; where&#10; kundid&gt;0 and kundloekennz=0&#10; order by&#10; AnzahlOps desc</code></pre><br> <br>Um auf das Klicken in eine Zeile zu reagieren und ggf. mehr Informationen anzuzeigen, kann dies mit der Refresh-Prozedur geschehen. An die Prozedur werden die Werte übergeben, die in der View/der Prozedur als ID1, ID2, ID3 und ID4 geliefert werden.<br>Beispiel Refresh-Prozedur:<br><pre><code>CREATE PROCEDURE&#10; p_dash_Refresh_tableview&#10; (in in_board integer,&#10; in in_kachel&#10; integer,&#10; in in_ident1 integer&#10; default null,&#10; in in_ident2 integer&#10; default null,&#10; in in_ident3 integer&#10; default null,&#10; in in_ident4 integer&#10; default null )&#10;--&#10;BEGIN&#10; create or replace Variable&#10; pdb_adressid integer =0;&#10; set pdb_adressid =&#10; in_ident2;&#10; select id_kachel from&#10; Dash_Board_Kachel_Link&#10; where Id_Board = in_board and&#10; id_kachel!=in_Kachel;&#10; EXCEPTION&#10; when others then&#10; call amic_exception( ERRORMSG() &#124;&#124;&#10; CHAR(10) &#124;&#124; CHAR(13) &#124;&#124; TRACEBACK(), SQLCODE , SQLSTATE , 'p_dash_Refresh'&#10; , -1 );&#10;END</code></pre><br> <br> |
