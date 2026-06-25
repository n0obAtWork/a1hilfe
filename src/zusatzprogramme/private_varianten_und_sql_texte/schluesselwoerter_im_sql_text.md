# Schlüsselwörter im SQL-Text

<!-- source: https://amic.de/hilfe/SchluesselwoerterImSQLText.htm -->

Die hier aufgeführten Schüsselwörter gelten für die Auswahlliste im alten Design(AW 1.0), die im neuen Design(AW 2.0) und die F3-Auswahl(IB). Teilweise stehen Schlüsselwort nicht in jedem Teil zur Verfügung (siehe Hinweis). Alle Schlüsselwörter müssen großgeschrieben werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><b>Gütigkeitsbereich</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>VAR</p>
        </td>
        <td></td>
        <td>
          <p>Mithilfe von VAR können zusammengesetzte Inhalte oder Formeln für das SQL-Statement vordefiniert werden.<br><br></p>
          <div>
            <pre><code>VAR Name
      A.AdressName+', '+A.AdressVorname
FIELD
      Nummer,S.KundNummer,I4,8
FIELD Name,Name,char,20
SQL select :FIELDS from
      Kundenstamm s join Anschriftstamm a on
      a.adressid=s.adressidhauptadr</code></pre>
          </div>
          <p><br>Das SQL wird erweitert auf:</p>
          <div>
            <pre><code>Select S.KUNDNUMMER,A.AdressName+', '+A.AdressVorname
      NAME, from Kundenstamm s join
      Anschriftstamm a on a.adressid=s.adressidhauptadr</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>FIELD</p>
        </td>
        <td></td>
        <td>
          <p>Beschreibung einer Spalte. Die ersten vier Parameter müssen immer in einer festen Reihenfolge angegeben werden:</p>
          <p>1)&nbsp;&nbsp; Spaltenüberschrift. Die Überschrift kann sich in der AW 2.0 über mehrere Zeilen erstrecken. Um das zu erreichen, fügt man mit &lt;br&gt; einen Zeilenumbruch ein.</p>
          <p>2)&nbsp;&nbsp; Datenbankfeld. Der Name muss eindeutig sein.</p>
          <p>3)&nbsp;&nbsp; <a href="./feldtyp_im_sql_text.md">Feldtyp</a></p>
          <p>4)&nbsp;&nbsp; Breite der Spalte in Zeichen. Für die F3-Auswahl 2.0 gibt diese Zahl die maximale Breite an, in der die Spalte dargestellt wird, da hier die Breite der Spalte aus dem Feldinhalt errechnet wird.</p>
          <div>
            <pre><code>…
FIELD
      Name,name,char,20
FIELD Typ,KundTyp,FS
      KundTyp,10
FIELD
      Matchcode,MA,char,15
FIELD
      Kunden-&lt;br&gt;bezeichnung,Kundbezeich,char,30
…</code></pre>
          </div>
          <p>Zusätzlich existieren noch weitere Parameter, die wiederrum über ein Schlüsselwort verfügen. Sie können in beliebiger Kombination verwendet werden.</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th></th>
              </tr>
              <tr>
                <td>GROUP=</td>
                <td>Siehe <a href="../auswahlliste_2_0/anwendungsregister/index.md#AnwendungsregisterGruppierung">Anwendungsregister/Gruppierung</a></td>
              </tr>
              <tr>
                <td>SUM= oder<br>SUM</td>
                <td rowspan="2">Siehe <a href="../darstellung_der_auswahlliste/summierung_in_der_auswahlliste/index.md">Summierung der Auswahlliste</a></td>
              </tr>
              <tr>
                <td>SUMFORMAT=</td>
              </tr>
              <tr>
                <td>COLOR=</td>
                <td rowspan="4">Siehe <a href="../darstellung_der_auswahlliste/farbgestaltung_der_auswahlliste/index.md">Farbgestaltung der Auswahlliste</a></td>
              </tr>
              <tr>
                <td>BGCOLOR=</td>
              </tr>
              <tr>
                <td>FGCOLOR=</td>
              </tr>
              <tr>
                <td>STYLE=</td>
              </tr>
              <tr>
                <td>HIDDEN</td>
                <td>Spalte wird zwar angelegt jedoch nicht angezeigt. Sie kann über den <a href="../darstellung_der_auswahlliste/feldauswahl_der_auswahlliste.md">Gestaltungsdialog</a> jederzeit eingeblendet werden.<br><br></td>
              </tr>
              <tr>
                <td>NOINFO</td>
                <td>Diese Spalte wird nicht im Infofenster dargestellt.</td>
              </tr>
              <tr>
                <td>ROWS=</td>
                <td>Innerhalb des Infofensters kann eine Spalte in mehreren Zeilen angezeigt werden. Hinter Rows gibt man die maximale Anzahl Zeilen an.</td>
              </tr>
              <tr>
                <td>MULTILINE= oder<br>MULTILINE</td>
                <td>Die Auswahlliste 2.0 kann Daten über mehrere Zeilen darstellen. Gibt man ein Gleichheitszeichen gefolgt von einer Zahl an, so werden maximal so viele Zeilen dargestellt.</td>
              </tr>
              <tr>
                <td>EXTENDEDFILTER</td>
                <td>Nur Auswahlliste 2.0. Siehe <a href="../darstellung_der_auswahlliste/feldauswahl_der_auswahlliste.md">Feldauswahl der Auswahlliste</a></td>
              </tr>
              <tr>
                <td>FILTERCOMPARISION=</td>
                <td>
                  Nur Auswahlliste 2.0 und F3-Auswahl 2.0. Siehe auch
                  <a href="../darstellung_der_auswahlliste/feldauswahl_der_auswahlliste.md">Feldauswahl der Auswahlliste</a>
                  <br>
                  Diese Option konkurriert mit EXTENDEDFILTER und wird dann nicht ausgewertet. Man kann hier angeben, wie die Filterzeile suchen soll. Möglicher Werte sind:
                  <br>
                  <br>
                  <br>
                  <ul>
                    <li><b>gleich</b></li>
                    <li><b>ungleich</b></li>
                    <li><b>kleiner</b></li>
                    <li><b>kleiner oder gleich</b></li>
                    <li><b>größer als</b></li>
                    <li><b>größer oder gleich</b></li>
                    <li><b>beginnt mit</b></li>
                    <li><b>beginnt nicht mit</b></li>
                    <li><b>endet mit</b></li>
                    <li><b>endet nicht mit</b></li>
                    <li><b>enthält</b></li>
                    <li><b>enthält nicht</b><br>Die Werte können mit oder ohne Leerzeichen angegeben werden, also „endet nicht mit“ oder „endetnichtmit“. Groß- und Kleinschreibung muss nicht beachtet werden.<br>Beispiel:</li>
                  </ul>
                  <pre><code>FIELD
            Bezeichnung,sachkontbez,char,53,FILTERCOMPARISION=BeginntMit</code></pre>
                </td>
              </tr>
              <tr>
                <td>TIPTEXT=</td>
                <td>Hinter diesem Schlüsselwort folgt ein beschreibender Text zu dieser Spalte. Enthält der Text ein Komma, so muss der Text in Anführungszeichen stehen. Der Text erscheint, wenn man mit dem Mauszeiger über die entsprechende Spalte geht.</td>
              </tr>
              <tr>
                <td>TITEL= oder<br>TITLE=</td>
                <td>
                  Hinter dem Gleichheitszeichen folgt der Name einer Datenbankprozedur. Mit dieser Prozedur kann der Titel dynamisch angepasst werden. Diese Datenbankprozedur hat einen Parameter (char 255) in dem der originale Titel geliefert wird. Sie soll eine Überschrift liefern, die dann verwendet wird. Liefert die Funktion als Rückgabewert „-DONOTSHOW-“, dann wird die Spalte ausgeblendet.
                  <br>
                  Dieses Beispiel sorgt dafür, dass die entsprechenden Spalten ausgeblendet werden, wenn keine Daten nach Handelsrecht geführt werden.
                  <br>
                  <pre><code>create function "admin"."anka_ueberschrift"( in
            in_alt char(255) )
returns
            char(255)
begin
  declare
            dc_Ergebnis char(255);
  Set
            dc_Ergebnis=(select FIRST(AnKaNachSteuerrecht) from anlagenkartei
            where AnKaNachSteuerrecht=1 order by ankainventarNummer
);
            if dc_Ergebnis is NULL then
    set dc_Ergebnis ='-DONOTSHOW-';
            else
    set dc_ergebnis =
            in_alt;
            endif;
  return
            dc_Ergebnis;
end;</code></pre>
                </td>
              </tr>
              <tr>
                <td>MINWIDTH=</td>
                <td>
                  <b>Nur für die</b>
                  <a href="../f3_auswahl_2_0_itembox/index.md"><b>F3-Auswahl 2.0</b></a>
                  <b>.<br></b>
                  Dort wird die Spaltenbreite dynamisch berechnet, und zwar anhand der Breite der Überschrift, der maximal Anzahl Zeichen der Sichtbaren Zeilen und der unter 4) angegebene Breite der Spalte(s.o.). Die Breite ist dabei die maximal angezeigte Breite und MINWIDTH ist die Breite in Zeichen, die nicht unterschritten werden soll.
                  <br>
                  <br>
                  Beispiel:
                  <br>
                  <pre><code>.
FIELD
            Bezeichnung,AnkaGrupBezeich,char,40,MINWIDTH=30
.</code></pre>
                  <br>
                  <table>
                    <tbody>
                      <tr>
                        <th>
                          <p>Mit MINWIDTH</p>
                        </th>
                        <th>
                          <p>Ohne MINWIDTH</p>
                        </th>
                      </tr>
                      <tr>
                        <td>
                          <p><img src="../../ImagesExt/image8_1335.png" alt=""></p>
                        </td>
                        <td>
                          <p><img src="../../ImagesExt/image8_1336.png" alt=""></p>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </td>
              </tr>
              <tr>
                <td>XML=</td>
                <td></td>
              </tr>
              <tr>
                <td>JVARS(owner,Feldname)<br>oder<br>JAVARS_OWNER_FELDNAME</td>
                <td>
                  Nur Auswahlliste 2.0.
                  <br>
                  Ist hinter einer FIELD – Zeile dieses Schlüsselwort angegeben, so wird immer dann, wenn genau ein Datensatz markiert ist, der Wert aus der Datentabelle in die JVar mit dem angegebenen Owner und Feldnamen geschrieben. Diese JVars werden nicht automatisch gelöscht.
                  <br>
                  <br>
                  Beispiel:
                  <br>
                  <pre><code>FIELD Konto,KontoNummer,I4,10,
            JVARS(10001,Kontonummer)</code></pre>
                  <br>
                  oder
                  <br>
                  <pre><code>FIELD Konto,KontoNummer,I4,10,
            JVARS_10001_Kontonummer)</code></pre>
                </td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>SQL</p>
        </td>
        <td></td>
        <td>
          <p>Enthält das SQL-Statement, welches die darzustellenden Daten liefert.</p>
          <div>
            <pre><code>SQL select :FIELDS
 from
      AnKaGruppe
 where
      (1=1
   :AUSW_ANKAGRUPPE
 order by
      AnKaGrupNummer</code></pre>
          </div>
          <p>FIELDS fast alle mit FIELD und VAR angegebenen Spalten zusammen (siehe oben).<br>AUSW_ANKAGRUPPE ist der für die F2-Bereichsauswahl verwendete Variablenname.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>IDENT</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Eine Liste von bis zu vier Feldern, die dazu dienen, eine Datenzeile eindeutig zu identifizieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>IDSQL</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Ein SQL-Statement, mit dem der in der Auswahlliste markierte Datensatz eindeutig identifiziert wird. Innerhalb dieses Statements kann auf die hinter IDENT stehenden Felder über die Platzhalter ID1, ID2, ID3 und ID4 zugegriffen werden.</p>
          <div>
            <pre><code>…
IDENT s.KundId
IDSQL select
      *
      from KUNDENSTAMM
      s
      where s.kundid =
      :ID1
…</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>RETURN</p>
        </td>
        <td></td>
        <td>
          <p>Eine Liste von Feldern, die an das Programm zurückgeliefert werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FIXCOL</p>
        </td>
        <td></td>
        <td>
          <p>Legt die Anzahl der Spalten fest, die beim horizontalen Scrollen nicht bewegt werden, also immer Links in der Datentabelle stehen bleiben. In der Auswahlliste 2.0 lässt sich durch den kleinen Pin in der Titelzeile jede Spalte fixieren. Diese Einstellung wird beim erneuten Aufruf der Variante wieder verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>OPTIONS</p>
        </td>
        <td></td>
        <td>
          <p>Dem Schlüsselwort OPTIONS folgen ein oder mehreren durch Komma getrennte weitere Schlüsselwörter.</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th><b>Bereich</b></th>
                <th><b>Bedeutung</b></th>
              </tr>
              <tr>
                <td>ONEONLY&nbsp; oder<br>ALLWAYSONE</td>
                <td>AW</td>
                <td>Es kann immer nur eine Zeile zurzeit in der Auswahlliste markiert werden.</td>
              </tr>
              <tr>
                <td>NOPOINT</td>
                <td></td>
                <td>Der NULL-Wert wird nicht durch einen Punkt dargestellt, sondern die Zelle bleibt einfach leer. Die Darstellung des NULL-Wertes wird in der Auswahlliste 2.0 über das <a href="../auswahlliste_2_0/darstellungsregister.md">Darstellungsregister</a> festgelegt.</td>
              </tr>
              <tr>
                <td>NOSORT</td>
                <td>AW</td>
                <td>Die Funktion zum Sortieren einer Auswahlliste durch Klicken in die Kopfzeile einer Spalte wird nicht ausgeführt. Es kann also keine Individuelle Sortierung vorgenommen werden</td>
              </tr>
              <tr>
                <td>VONBIS2DBVAR</td>
                <td>AW</td>
                <td>Die Werte, die in der Bereichsauswal angegeben wurden, werden in Datenbankvariablen hinterlegt. Die Namen lauten db_von_1, db_bis_1, usw. Diese Werte sollten nur sparsam verwendet werden, da sie jeweils von der nächsten Anwendung wieder überschrieben werden.<br><br></td>
              </tr>
              <tr>
                <td>NOSTAPEL</td>
                <td>AW</td>
                <td>Die Stapelfunktionalität steht für diese Variante nicht zur Verfügung.</td>
              </tr>
              <tr>
                <td>STAPELMODUS</td>
                <td>AW 2.0</td>
                <td>
                  Die Variante ist immer im Stapel-Bearbeitungsmodus. Im StapelModus muss ein Feld STAPEL_ID aus der Tabelle STAPEL_CONTENT in der Fieldanweisung existieren, es kann auch auf HIDDEN gestellt sein.
                  <br>
                  <pre><code>FIELD Stapel
            ID,stapel_id,I4,10,HIDDEN</code></pre>
                  <br>
                  Beispiel siehe Vorgangstapel (Direksprung [VRS])
                </td>
              </tr>
              <tr>
                <td>INFOBOX</td>
                <td>IB</td>
                <td>Statt die F3-Auswahl bei Auswahl einer Zeile zu schließen, wird die F3 Funktion, die in der Optionbox eingerichtet ist, ausgelöst.</td>
              </tr>
              <tr>
                <td>INSERT</td>
                <td>IB</td>
                <td>Bewirkt, dass das Ergebnis dem Feld, auf dem die F3-Auswahl aufgerufen wurde, hinzugefügt wird (also das Feld nicht überschrieben wird) und das Feld nicht verlassen wird.</td>
              </tr>
              <tr>
                <td>NOTAB</td>
                <td>IB</td>
                <td>Das Feld, zu dem man aus der F3-Auswahl zurückkehrt, wird nicht verlassen.</td>
              </tr>
              <tr>
                <td>OLDSTYLE</td>
                <td>IB</td>
                <td>Diese F3-Auswahl wird immer im alten Design dargestellt, unabhängig von den Einstellungen im Bedienerstamm.</td>
              </tr>
              <tr>
                <td>NEWSTYLE</td>
                <td>IB</td>
                <td>Wird das Schlüsselwort „NEWSTYLE“ verwendet, so wird die F3-Auswahl im neuen Design (<a href="../f3_auswahl_2_0_itembox/index.md">F3-Auswahl 2.0</a>) dargestellt, unabhängig von den Einstellungen im Bedienerstamm.</td>
              </tr>
              <tr>
                <td>OHNEEINSTIEGSVARIANTE</td>
                <td>IB</td>
                <td>Die Prüfung auf dem Feld wird bei gesetzter OPTION OHNEEINSTIEGSVARIANTE mit der von AMIC vorgesehenen Variante durchgeführt. Unabhängig davon wird bei F3 die vom Anwender eingestellte Variante aufgerufen.</td>
              </tr>
              <tr>
                <td>NOITEMWAHL</td>
                <td>IB 2.0</td>
                <td>Das Eingabefeld für die Eingrenzung über das SQL-Statement per ITEMWAHL steht nicht zur Verfügung und kann somit auch nicht mit <strong>F2</strong> bzw. <strong>Strg+Y</strong> eingeblendet werden.<br><br></td>
              </tr>
              <tr>
                <td>ALFA oder NUM</td>
                <td>IB</td>
                <td>Wenn man in einem Eingabefeld sowohl nach Nummer als auch nach Text suchen möchte, dann muss man eine F3-Auswahl für die Suche nach Nummer definieren und als ALTER(native) eine weitere F3-Auswahl für die Suche nach einem Text definieren. Die F3-Auswahl für die Suche nach Nummer muss <b>OPTIONS NUM</b> enthalten, die für die Suche nach Text muss <b>OPTIONS ALFA</b> enthalten</td>
              </tr>
              <tr>
                <td>MATCH</td>
                <td>IB</td>
                <td>Für F3-Auswahlen mit dieser Option wird LOOKUP nicht ausgewertet.</td>
              </tr>
              <tr>
                <td>MARKALL</td>
                <td>AW</td>
                <td>Beim Betreten der Variante werden sofort alle Zeilen markiert. Wird in der Auswahlliste 2.0 nicht mehr unterstützt.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>ALTER</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Eine Liste von F3-Auswahlen, die zur Auswahl angeboten werden:</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>WARNINGFUNTION</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p>Es ist möglich ein Hintergrundbild einzublenden. Die Funktion wird bei jedem Refresh der Auswahlliste aufgerufen und muss einen Wert vom Typen Integer zurück liefern. Gültige Werte sind:</p>
          <p>0 = Kein Hintergrundbild</p>
          <p>1 = Information</p>
          <p>2 = Warnung</p>
          <p>3 = Fehler</p>
          <p>Syntax-Beispiel</p>
          <div>
            <pre><code>WARNINGFUNCTION p_namederdatenbankfuntion()
      //ACHTUNG: Vollständig mit Klammern</code></pre>
          </div>
          <p>oder</p>
          <div>
            <pre><code>WARNINGFUNCTION if db_bedienerid=-1 then 3 else 0
      endif</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>STRIKEOUT</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p>Man kann Werte einer Zeile durchgestrichen darstellen. Dazu dient das Schlüsselwort STRICKEOUT gefolgt von dem Feld, welches angibt, ob das Feld durchgestrichen ist(Wert = 1) oder nicht(Wert=0)<br><br></p>
          <div>
            <code>STRICKEOUT f.dsgvo_loekennz</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>SAVE</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>SAVE gefolgt vom Datenbankfeldname:<br><br></p>
          <div>
            <code>SAVE Spaltenname</code>
          </div>
          <p>Der Spaltenname muss ohne Handle angegeben werden und natürlich in der Datenbankabfrage vorhanden sein. Dieses Schlüsselwort bewirkt, dass beim Klicken in eine Zeile der Wert aus dem Feld in die Zwischenablage geschrieben wird und somit anschließend mit Strg+V irgendwo eingefügt werden kann.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>XML</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Die <a href="../darstellung_der_auswahlliste/auswahllisten_legende.md">Farblegende</a> der Auswahlliste</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DELPROC</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Veraltete Technik um Löschfunktionen für Anwendungen anzubinden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>INFOWINDOW</p>
        </td>
        <td>
          <p>AW 1.0</p>
        </td>
        <td>
          <p>Hiermit kann gesteuert werden, wie das Informationsfenster beim Öffnen der Anwendung reagiert. Mögliche Einstellungen sind:</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>INFOWINDOW AN</td>
                <td>Das Informationsfenster wird sofort aktiv geöffnet</td>
              </tr>
              <tr>
                <td>INFOWINDOW AUS</td>
                <td>Dies ist die Standardeinstellung. Das Informationsfenster muss per Funktionsaufruf gestartet werden.</td>
              </tr>
              <tr>
                <td>INFOWINDOW NIE</td>
                <td>Die Funktionalität des Informationsfensters steht für diese Variante nicht zur Verfügung.</td>
              </tr>
            </tbody>
          </table>
          <p>Das Informationsfenster wird nur für Anwendungen angeboten, die auf unterster Ebene geöffnet wurden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DEFAULT</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p>Im SQL-TEXT für die neue Auswahlliste existiert ein neues Schlüsselwort DEFAULT:</p>
          <div>
            <pre><code>DEFAULT fa_kundnummer=:GETVALUE (IDENT,Kundnummer),fa_belegreferenz=:GETVALUE
      (IDENT,belegreferenz)</code></pre>
          </div>
          <p>Oder</p>
          <div>
            <pre><code>DEFAULT fa_kundnummer=":GETVALUE (IDENT,Kundnummer)",fa_belegreferenz=":GETVALUE
      (IDENT,belegreferenz)"</code></pre>
          </div>
          <p>Liefert beides mal dasselbe Ergebnis, die Hochkomma werden entfernt. Hochkamma sind dann notwendig, wenn das, was zurückgeliefert wird, ein Komma enthält. Komma sind ansonsten Trennzeichen:</p>
          <div>
            <pre><code>DEFAULT  KundSQL="select kundnummer
      ,  Kundbezeich from kundenstamm where Kundid=:GETVALUE(IDENT,KUNDID)"</code></pre>
          </div>
          <table>
            <tbody>
              <tr>
                <th>Parameter</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>IDENT</td>
                <td>Wert aus der IDENTLISTE der darunterliegenden Auswahlliste</td>
              </tr>
              <tr>
                <td>RETURN</td>
                <td>Wert aus der RETURNLISTE der darunterliegenden Auswahlliste</td>
              </tr>
              <tr>
                <td>FELD</td>
                <td>Wert eines Feldes aus der darunterliegenden Maske</td>
              </tr>
              <tr>
                <td>SVMAIN</td>
                <td>Ein Wert aus dem letzten SVMAIN-Kontext</td>
              </tr>
              <tr>
                <td>CEMAIN</td>
                <td>Ein Wert aus dem letzten CEMAIN-Kontext</td>
              </tr>
            </tbody>
          </table>
          <p>Siehe auch <a href="../darstellung_der_auswahlliste/standardvorbelegung_in_der_auswahlliste_definieren_nur_auswa/index.md">Standardvorbelegung</a>.</p>
          <p>ACHTUNG: Da GETVALUE sich auf die vorherige MASKE/Auswahlliste bezieht, ist es hier nicht möglich von der Auswahlliste 2.0 die Funktion F4 (Druck/Quickreport)&nbsp; aufzurufen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PILEAPP und<br>PILE_TYP</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Es gibt einen Mechanismus zur Stapelbildung von Vorgängen. Auswahllisten mit dem Schlüsselwort PILEAPP signalisieren dem System, dass es sich um eine Auswahllisten mit Vorgangsdaten handelt. Hinter PILEAPP muss der Name der Anwendung folgen, die dann beim Umschalten in die Stapelverarbeitung verwendet werden soll. Innerhalb der PILEAPP-Anwendung muss dann das Schlüsselwort PILE_TYP verwendet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>INFO</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Die Bezeichnung der F3-Auswahl, wie sie in der Varianten-Auswahl erscheint:<br><br></p>
          <div>
            <code>INFO nach Bezeichnung</code>
          </div>
          <p><img src="../../ImagesExt/image8_1337.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MASK</p>
        </td>
        <td>
          <p>IB 1.0</p>
        </td>
        <td>
          <p>Gibt die verwendete Dialogmaske der F3-Auswahl - und damit die Größe – an. Dieses Feld ist optional. Mögliche Werte sind ITEM60, ITEM80, ITEM100 und ITEM200: Ist keiner dieser Werte gesetzt, dann wird ITEM 80 verwendet<br><br></p>
          <div>
            <code>MASK ITEM100</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>TITLE</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Titelzeile der F3-Auswahl im alten Design.</p>
          <div>
            <pre><code>TITLE Gesamtverzeichnis
Direktsprünge</code></pre>
          </div>
          <p><img src="../../ImagesExt/image8_1338.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>IB_LABEL</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Gibt den Text an, der vor dem Eingabefeld steht.<br><br></p>
          <div>
            <code>IB_LABEL Bezeichnung wie</code>
          </div>
          <p><img src="../../ImagesExt/image8_1339.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BEEP</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Mit diesem Schlüsselwort wird festgelegt, dass ein System-Beep gesendet wird, bevor die F3-Auswahl geöffnet wird. Es muss die Frequenz und die Dauer in ms angegeben werden. Fehlt die Angabe der Dauer, so wird ein Ton für die Dauer von 1 Sek ausgegeben.</p>
          <p>Die dritten und nachfolgenden Parameter sind eine Liste von JVARS, des Owners 1971 (JVAR_BEEP), die gesetzt sein müssen, damit dieser Beep ausgelöst wird. Fehlt die Angabe gänzlich, wird der Beep in jedem Fall ausgelöst.</p>
          <div>
            <code>BEEP 800, 350, SVPOSBAR2BEEP</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>OPTIONBOX</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Mit diesem Schlüsselwort kann man an die Optionbox der F3-Auswahl eine weitere Optionbox anhängen. In dieser Optionbox können dann z.B. Funktionen für den Aufruf für Stammdatenpfleger oder die Hilfe-Funktion enthalten sein.</p>
          <div>
            <code>OPTIONBOX OB_IB_KOSTENSTELLEN</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARAMS</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Die PARAMS-Anweisung dient dazu, vom Programm übermittelte Parameter mit einem Standardwert zu versorgen. Diese werden nur angenommen, wenn die entsprechenden Parameter nicht versorgt worden sind. Dies Standardwerte werden auch verwendet, um die F3-Auswahl zu testen. Wie bei allen Kommandos kann das PARAMS -Statement auch mehrere durch Komma getrennte Parameter enthalten</p>
          <div>
            <pre><code>PARAMS  KONTO = 70000,JAHR = 2012, PERI =
      12</code></pre>
          </div>
          <p>Im SQL können dann KONTO, JAHR, PERI wie folgt verwendet werden:</p>
          <div>
            <pre><code>…
where a.kontonummer = :KONTO
   and
      c.Jahrnummer = :JAHR
   and c.Perinummer =
      :PERI
…</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>LPARAMS</p>
        </td>
        <td></td>
        <td>
          <p>Wie PARAMS, nur werden LPARAMS in den durch das Schlüsselwort COPY erstellten SQL-Texten überschrieben.<br><br></p>
          <div>
            <pre><code>COPY AW_Preise_VK_Preise
LPARAMS EKVK=EK</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>ITEMWAHL</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>ITEMWAHL hat zwei Bedeutungen-</p>
          <p>1)&nbsp;&nbsp; Verwendet man ITEMWAHL als Schlüsselwort, so dient es dazu, den Typen der Eingabe genauer zu bestimmen:</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th></th>
              </tr>
              <tr>
                <td>ALFA</td>
                <td>Standardeinstellung. Es erfolgt keine Prüfung, man kann alles eingeben.</td>
              </tr>
              <tr>
                <td>INT</td>
                <td>Nur Integer-Zahlen werden durchgelassen.</td>
              </tr>
              <tr>
                <td>REAL</td>
                <td>Nur Floating Zahlen werden durchgelassen.</td>
              </tr>
              <tr>
                <td>DAT</td>
                <td>Datumseingaben sind erlaubt. Gibt man hier etwas an, so wird ein Hinweistext ausgegeben:<br><img src="../../ImagesExt/image8_1340.png" alt=""></td>
              </tr>
            </tbody>
          </table>
          <p>Hinter dem Typen kann durch Komma getrennt ein Vorbelegung eingetragen werden, die Verwendet wird, falls kein Wert an die F3Auswahl übergeben wurde. Wird bei Typ Datum keine Vorbelegung angegeben, so wird das Tagesdatum verwendet um einen SQL-Syntaxfehler zu vermeiden.</p>
          <p>2)&nbsp;&nbsp; Verwendet man ITEMWAHL mit vorangestelltem Doppelpunkt im SQL-Text, dann wird dort vor dem Ausführen des Statements der Wert aus dem Eingabefeld eingesetzt<br><br></p>
          <div>
            <pre><code>SQL select
 SachKontBezeich,KontoNummer,
      SachKontOPKennz,SachKontSteuKenn,SachKontZinsKenn,SachKontErfSperr,SachKontJWKKennz
 from
      SachKontstamm
 where
      isnull(sachkontloekennz,0)=0 and kontonummer &gt;= ':ITEMWAHL'</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>ITEM1</p>
          <p>ITEM2</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Benötigt man zwei Kriterien, um die Suchabfrage einzugrenzen, so kann man im Eingabefeld die Werte durch kommt getrennt eingeben und im SQL-Text dann die Platzhalter ITEM1 und ITEM2 verwenden. Es gelten dieselben Bedeutungen wie für ITEMWAHL.</p>
          <p>Im Folgendem Beispiel wird nach der Bezeichnung und dem Vornamen gesucht:</p>
          <div>
            <pre><code>SQL
 select
      :FIELDS
 from KundenStamm
      kus, AnschriftStamm
 where
      (AdressIdHauptAdr = AdressId)
 and (KundBezeich
      like ':ITEM1%')
 and
      (AdressVorname like ':ITEM2%') and
      (KundLoeKennz = 0)</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>MUSTENTER nn</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Gibt an, dass mindestens nn Zeichen eingeben werden müssen, bevor das SQL-Statement ausgeführt wird.<br><br></p>
          <p><u>Für die IB 2.0 gilt:</u><u></u></p>
          <p>Es kann auch MUSTENTER 0 angegeben werden. Dann startet das Laden der Daten nicht sofort, es muss aber kein .Zeichen eingegeben werden.</p>
          <p>Dieses Kennzeichen kann - ohne private Ableitung zu bilden - über die Funktion „Mindesteingabe“ zentral pro F3-Auswahl gesetzt werden. Das ermöglicht es private Ableitungen zu vermeiden.</p>
          <p>Vom Entwickler kann mit der Funktion FLD_ITEM_OPTION ein Wert gesetzt werden.</p>
          <div>
            <pre><code>FLD_ITEM_OPTION("IB_Artikel_nu","MUSTENTER
      4")</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>LOOKUP</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>LOOKUP dient dazu, einen Datensatz eindeutig zu Identifizieren und muss im SQL-Befehl angegeben werden, wenn die F3-Auswahl dazu dient, zu prüfen, ob ein Datensatz existiert.</p>
          <div>
            <pre><code>SQL select kontobezeich kundbezeich,
      :FIELDS
 from kontostamm
      k
 where k.KontoNummer &gt;=':ITEMWAHL'
  and ( KontoTyp
      in (1,2) )
  :LOOKUP
 order by
      k.KontoNummer
LOOKUP and k.Kontonummer =
      ':ITEMWAHL'</code></pre>
          </div>
          <p>Bei der Auflistung wird LOOKUP im SQL weggelassen, bei der Prüfung jedoch eingesetzt. Es ergibt sich dann folgendes Statement wenn als Konto 10000 ausgewählt.wurde:</p>
          <div>
            <pre><code>select kontobezeich kundbezeich, :FIELDS
 from kontostamm
      k
 where k.KontoNummer &gt;='10000'
  and ( KontoTyp
      in (1,2) )
  and
      k.Kontonummer = '10000'
 order by k.KontoNummer</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>FIBU_INFO</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Spezialität der <a href="../../finanzbuchhaltung/op_verwaltung/formular_fibu_infofenster.md">Konteninformation</a> in der Finanzbuchhaltung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FORM nnnn</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Veraltet. Hier konnte die Formularnummer des Formulars für die Kurzliste hinterlegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>VERIFY und<br>VERIFY_INFO</p>
        </td>
        <td>
          <p>IB</p>
        </td>
        <td>
          <p>Wenn eine F3-Auswahl eine Ergebnismenge liefert, die ggf. nicht gültig sind, kann hier eine F3-Auswahl angegeben werden, mit deren Hilfe die Gültigkeit noch einmal geprüft werden kann. Durch Komma getrennt gibt man den Feldnamen an, der für ITEMWAHL - also als Eingabe - verwendet werden soll. Das Schlüsselwort VERIFY_INFO enthält den Text, der ausgegeben wird, wenn die Prüfung fehlschlägt.</p>
          <div>
            <pre><code>VERIFY
      IB_VERIFY_BAUSTELLE_ARTIKEL,ArtikelId
VERIFY_INFO Artikel gehört nicht zur
      Baustelle</code></pre>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>PRESQL und<br>CTRLSTR</p>
        </td>
        <td>
          <p>AW</p>
        </td>
        <td>
          <p>Hier kann eine JPL-Funktion aufgerufen werden. Diese läuft vor dem SQL-Statement und kann z.B. dafür verwendet werden, JVARS zu setzen, die dann im SQL verwendet werden.<br><br></p>
          <div>
            <code>PRESQL ^jpl rollenkontext_prepare</code>
          </div>
          <div>
            <code>CTRLSTR ^jpl del_aw_rwsbearb_temp</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>FIXEDSIZE</p>
        </td>
        <td>
          <p>IB 2.0</p>
        </td>
        <td>
          <p>Die neue F3-Auswahl bestimmt die Spaltenbreite und die Breite des gesamten Fensters anhand der breite der Daten. Ist das nicht gewollt und möchte man die Größe der F3-Auswahl festgelegen, so geschieht dies mit diesem Schlüsselwort. Dabei stehen hinter dem Schlüsselwort die Breite und die Höhe in Pixeln.</p>
          <div>
            <code>FIXEDSIZE 1152,512</code>
          </div>
          <p>Die Größe lässt sich auch Programgesteuert festlegen. Dies geschieht über FLD_ITEM_OPTION und der Syntax ist ein klein wenig anders, da die Breite und Höhe hier in Klammern und ohne Leerzeichen stehen muss.</p>
          <div>
            <pre><code>FLD_ITEM_OPTION(itemBox, "FIXEDSIZE(1152,512)"
      )</code></pre>
          </div>
          <p>Ist vom Programm eine feste Größe vorgegeben, kann diese in der Itembox mit</p>
          <div>
            <code>FIXEDSIZE 0,0</code>
          </div>
          <p>wieder abgestellt werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
