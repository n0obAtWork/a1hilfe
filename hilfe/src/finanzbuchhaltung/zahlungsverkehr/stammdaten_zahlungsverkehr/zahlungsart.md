# Zahlungsart

<!-- source: https://amic.de/hilfe/zahlungsart.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Zahlungsarten

Direktsprung **[FIZAH]**.

Die Zahlungsart ist ein im Kunden- und Lieferantenstamm eingetragenes Kennzeichen, über das gesteuert wird, wie die Zahlung im automatischen Zahlungsverkehr bei Ein- und Ausgang erfolgen soll.

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.](../../../ImagesExt/image8_642.png "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.")

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>Eindeutige Nummer der Zahlungsart, wie sie später im Kunden/Lieferantenstamm hinterlegt wird. Die nächste freie Nummer wird vorgeschlagen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formularklasse</p>
        </td>
        <td>
          <p>Wahlweise „Zahlungseingang“ oder „Zahlungsausgang“. Eine Auswahl mit<strong> F3</strong> ist möglich.</p>
          <p>Beim Wechseln der Formularklasse wird im Feld eRechnung Zahlungsweg der sinnvollste eRechnungs-Zahlungsweg neu vorbelegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung der Zahlungsart zur einfacheren Identifikation in Auswahllisten oder F3-Auswahlen.</p>
          <p>Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 <a href="../../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md">sprachabhängige Bezeichnungen</a> zu pflegen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Skontierbar</p>
        </td>
        <td>
          <p>Hier kann der Skontotyp eingetragen werden. Eine Auswahl mit<strong> F3</strong> ist möglich</p>
          <ul>
            <li><b>immer Skonto</b>: Skonto wir unabhängig vom Skontodatum immer gewährt/gezogen</li>
            <li><b>nie Skonto: </b>Selbst,<b> </b>wenn im Beleg Skonto vorgesehen ist und die Skontofrist noch nicht abgelaufen ist, wird kein Skonto gewährt.</li>
            <li><b>Abzug gem. Datum</b>: Dies ist die Vorbelegung. Skonto wird dann gewährt, wenn die Frist noch nicht abgelaufen ist.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Skontierbar bei Verrechnung</p>
        </td>
        <td>
          <p>Werden Rechnungen mit Gutschriften verrechnet, so kann es wünschenswert sein, bei den Gutschriften Skonto anders zu behandeln. Ist hier kein Wert eingetragen, so wird bei Gutschriften der Wert, der bei „Skontierbar“ eingetragen ist, verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DTA-Typ</p>
        </td>
        <td>
          <p>Zahlungsart bei Zahlung per Datenträgeraustausch. Der DTA-Typ wird nur bei der Formularklasse „Zahlungseingang“ abgefragt bzw. im Datenträgeraustausch verwendet. Bei Zahlungsausgang wird dieses Feld ausgeblendet. Der Hausbank muss beim DTA mitgeteilt werden, ob es sich bei den Lastschriften um eine <b>Einzugsermächtigung</b> oder um eine <b>Abbuchung</b> handelt.</p>
          <p>Hinweis: <i>Für das SEPA-Verfahren wird der Typ beim Mandat hinterlegt.</i><i></i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Echtzeitüberweisung</p>
        </td>
        <td>
          <p>Dieses Feld wird nur für Formularklasse „Zahlungsausgang“ abgefragt. Wir hier <b>Ja </b>eingetragen, so entfällt beim Erstellen der Zahlungsvorschläge die Vorlauf Frist von einem Tag und als Ausführungsdatum wird der verwendete Stichtag verwendet. Es werden bei der Berechnung keine Bankarbeitstage mehr berücksichtigt, da Echtzeitüberweisungen auch am Wochenende und an Feiertagen ausgeführt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>eRechnung Zahlungsweg<br><br></p>
        </td>
        <td>
          <p>Dieses Feld ist nur sichtbar, wenn die eRechnungs-Lizenz erworben wurde.</p>
          <p><br>In diesem Feld steht der Zahlungsweg, der beim eRechnungs-Export für diese Zahlungsart gewählt werden soll.</p>
          <p>Die Vorbelegung erfolgt anhand der Formularklasse. Bei Zahlungseingang wird der eRechnungs-Zahlungsweg mit Bankeinzug vorbelegt, bei Zahlungsausgang, wird er mit Überweisung vorbelegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>OP-Raffung Zahlungsverkehr</p>
        </td>
        <td>
          <ul>
            <li><b>Standard Raffung</b>. Alle OPs werden wie bisher in einem Zahlungsbeleg zusammengefasst.</li>
            <li><b>Einzel – OP.</b> Es wird pro OP ein Zahlungsbeleg erstellt.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Prozedur VWZ</p>
        </td>
        <td>
          <p>Hier kann man eine private Datenbankprozedur hinterlegen, die den Verwendungszwecktext individuell zusammengestellt. Sie hat drei Parameter:</p>
          <p>1.&nbsp;&nbsp; die csatz_id aus der Tabelle AMIC_DTAUS_CSATZ<br><br></p>
          <p>2.&nbsp;&nbsp; die Arte des DTA-Verfahrens. Mögliche Werte sind:</p>
          <p>#define DTAVERFAHREN_UNDEF&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; -1<br>#define DTAVERFAHREN_NODTA&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 0<br>#define DTAVERFAHREN_DTA&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 1<br>#define DTAVERFAHREN_DTINT&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 2<br>#define DTAVERFAHREN_DTAKASSE&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 8<br>#define DTAVERFAHREN_SEPA&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 16<br><br></p>
          <p>3.&nbsp;&nbsp; die Anzahl der erlaubten Zeilen. Im DTA sind maximal 13 Zeilen a 27 Zeichen zulässig. Werden diese Grenzen überschritten, so wird der Rest ignoriert.</p>
          <p>Die Prozedur muss ein Resultset vom Typen Character der Länge 27 zurückgeben. Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <p>Eine Datenbankprozedur zur Erstellung des Verwendungszwecks könnte so aussehen:</p>
          <div>
            <pre><code>create procedure p_dta_vwz( in in_csatz_id     integer,
                            in in_DTAVerfahren integer,
                            in in_AnzahlZeilen integer )
result
(
  vwz char(27)
)
BEGIN
  declare dc_zahlungid integer;
  declare dc_count     integer;
  select count(*), c.ZahlungId into dc_count, dc_Zahlungid
    from amic_dtaus_csatz c
    join zahlungsposition p on p.zahlungid = c.ZahlungId
   where csatz_id= in_csatz_id group by c.ZahlungId;
  if dc_count&gt;in_AnzahlZeilen then
--
-- Bei Überschreitung der maximalen Länge nur den Text „AUSGL. Nnn BELEGE LT.AVIS“
-- ausgeben. Und WICHTIG dem Programm mitteilen, dass eine Avise gedruckt werden soll.
--
    Update Zahlungsbeleg set ZahlungAvise    = 1
                 where ZahlungId = dc_ZahlungId;
    select cast ( ‘AUSGL.’|| dc_count ||’ BELEGE LT.AVIS’ as char(27));
  else
--
-- Und hier dem Programm mitteilen, dass keine Avise gedruckt werden soll
--
    Update Zahlungsbeleg set ZahlungAvise    = 0
                   where ZahlungId = dc_ZahlungId;
    select
    cast(
      Left( ‘RNR ‘ ||
        trim(
          if ( fibuv_klasse=4 or fibuv_klasse=5 ) and isnull(fibuv_FremdNr,’’)!=’’
          then fibuv_FremdNr
          else fibuv_nummer
          endif
        ) ||
        ‘ ‘ ||
        dateformat(fibuv_datum,’dd.mm’) || repeat(‘ ‘ , 27 ) , 27 – length(trim(amic_fstr(ZahlPosBetrag ,15,2)) ) -1
      ) ||
      ‘ ‘ ||
      trim(amic_fstr(ZahlPosBetrag ,15,2)) as char(27)
    )
    from zahlungsposition p
    left outer join fibuvorgstamm s on s.Fibuv_id=p.Fibuv_id
    where zahlungid=dc_zahlungid;
  end if;
END</code></pre>
          </div>
          <p><b><u>Wichtig: </u></b>Das Feld Zahlungsavise in der Tabelle Zahlungsbeleg muss auf 1 gesetzt werden, um dem Programm mitzuteilen, dass eine Avise gedruckt werden muss. Ansonsten muss das Feld auf 0 gesetzt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion VWZ SEPA</p>
        </td>
        <td>
          <p>Dieses Feld erscheint nur, wenn der Steuerungsparameter „DTA-Ausgabeformat“ auf „SEPA“ steht. Hier kann man eine private Datenbankfunktion hinterlegen, die den Verwendungszwecktext individuell zusammengestellt. Sie erhält als Parameter die csatz_id aus der Tabelle AMIC_DTAUS_CSATZ. Die Funktion muss einen Wert vom Typen Character zurückgeben. SEPA unterstützt nur Verwendungszwecktexte bis zu einer Länge von 140 Zeichen. Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <p>Eine Datenbankfunktion zur Erstellung des Verwendungszwecks könnte so aussehen:</p>
          <div>
            <pre><code>create function p_sepa_vwz( in in_csatz_id integer )
returns char(255)
begin
  declare out_character char(255);
  declare out_avise     char(255);
  declare dc_zahlungid integer;
  select List (
         ‘BNr. ‘ ||
         trim(
           if ( fibuv_klasse=4 or fibuv_klasse=5 ) and isnull(fibuv_FremdNr,’’)!=’’
           then fibuv_FremdNr else fibuv_nummer
           endif ) || ‘ ‘ ||
         dateformat(fibuv_datum,’dd.mm.yy’)  || ‘ ‘ ||
         trim(amic_fstr(ZahlPosBetrag ,15,2))
         , ‘ ‘ order by p.fibuv_id,p.fibuv_poszaehler ) ,
         ‘AUSGL.’||count(*)||’ BELEGE LT.AVIS’,
         c.ZahlungId
  into out_character, out_avise,dc_ZahlungId
  from amic_dtaus_csatz c
  join zahlungsposition p on p.zahlungid = c.ZahlungId
  join fibuvorgstamm s on s.Fibuv_id=p.Fibuv_id
  where csatz_id=in_csatz_id
  group by c.zahlungid;
--
-- Bei Überschreitung der maximalen Länge nur den Text „AUSGL. Nnn BELEGE LT.AVIS“
-- ausgeben. Und !!WICHTIG!! dem Programm mitteilen, dass eine Avise gedruckt
-- werden soll.
--
  if ( length(out_character)&gt;140) then
    set out_character = out_avise;
    Update Zahlungsbeleg set ZahlungAvise    = 1
                 where ZahlungId = dc_ZahlungId;
  else
    Update Zahlungsbeleg set ZahlungAvise    = 0
                 where ZahlungId = dc_ZahlungId;
  end if;
  return out_character;
end</code></pre>
          </div>
          <p><b><u>Wichtig: </u></b>Das Feld Zahlungsavise in der Tabelle Zahlungsbeleg muss auf 1 gesetzt werden, um dem Programm mitzuteilen, dass eine Avise gedruckt werden muss. Ansonsten muss das Feld auf 0 gesetzt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SEPA-Purpose-Code(TextSchlüssel)</p>
        </td>
        <td>
          <p>Hier können nur Daten eingegeben werden, die in der Tabelle <a href="./sepa_purpose_code.md">SEPAPurposeCode</a> hinterlegt wurden. Eine Auswahl mit<strong> F3</strong> ist möglich</p>
          <p><b><u>ACHTUNG: </u></b>Ist in diesem Feld ein Wert eingetragen, so wird beim SEPA-Verfahren dieser Wert mit übertragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DTINT-Verfahren trotz SEPA</p>
        </td>
        <td>
          <p>Dieses Feld erscheint nur, wenn der Steuerungsparameter „DTINT-Verfahren aktiv“ auf <b>Ja</b> steht. Ob Belege im <a href="../sepa/index.md">SEPA-Verfahren</a> abgewickelt werden wird Anhand von diversen Einstellungen entschieden. Setzt man dieses Feld jedoch auf <b>Ja</b>, werden alle diese Einstellungen ignoriert und die Zahlungsbelege werden im DTINT-Verfahren abgewickelt.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p><b>Alle folgenden Felder erscheinen nur bei aktiver Belegversand-Lizenz</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versandprofil</p>
        </td>
        <td>
          <p>Versandprofil aus dem <a href="../../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md">Versandprofilstamm</a>, welches zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben, so wird auch keine Avise versendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular</p>
        </td>
        <td>
          <p>Man kann hier ein vom Standard abweichendes Formular vom Typ Avise (Formulartyp=290) angeben um die Versandausgabe anders als die Druckausgabe zu formatieren. In der F3-Auswahl werden nur Formulare angeboten, bei denen die Archivierung aktiviert ist. Ist hier kein Formular angegeben, so wird dasselbe Formular wie beim Druck verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular Mailbody</p>
        </td>
        <td>
          <p>Avisen werden immer als Anhang an eine Mail versendet. Um die Mail selber zu gestalten, kann dafür hier ein Formular vom Typ Avise (Formulartyp=290) hinterlegt werden. Hier kann zusätzlich der Formularbereich „Avise Mail Betreffzeile“ für die Betreffzeile eingerichtet werden. Dieser Bereich wird nur beim Mailversand ausgewertet.</p>
          <p>Ein Beispielformular ist unter der Nummer -1100 zu finden.</p>
          <p><b><u>HINWEIS:</u></b> <i>Um Grafiken in das Formular mit einzubinden, kann man den bekannten HTML-Syntax &lt;img src="cid:XXXXXX" alt="mein bild" /&gt; verwenden. Für XXXXXX muss die GUID aus dem Formulararchiv, in dem die Grafik hinterlegt sein muss, angegeben werden.</i></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Anstelle des Formulars für den Mailbody können auch Datenbankprozeduren verwendet werden. Ist in dem Feld „DB-Funktion Mailbody“ etwas eingetragen, dann werden die Prozeduren verwendet und das „Formular Mailbody“ wird ignoriert</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA-Eintrag Mailbody</p>
        </td>
        <td>
          <p>FA-ID des Formulararchiv-Eintrags eines Mailbody-Templates, das mit Hilfe der Body-Funktion zu einem Mailbody verarbeitet werden kann.</p>
          <p>Der Eintrag hier ist optional, jedoch muss, wenn kein Template verwendet werden soll, die DB-Funktion das Dokument komplett aufbauen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Mailbody</p>
        </td>
        <td>
          <p>Wird hier eine Funktion hinterlegt, dann wird das „<b>Formular Mailbody</b>“ ignoriert und die Funktion liefert den Text der Mail. Sie erhält als Parameter die Zahlungsid aus der Tabelle ZAHLUNGSBELEG. Die Funktion muss einen Wert vom Typ „long varchar“ zurückgeben.</p>
          <p>Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <div>
            <pre><code>--
-- Funktion zum Erzeugen des Verwendungszwecks für SEPA.
--
CREATE FUNCTION p_fibu_Belegversand_Avise1 (in in_ZahlungId integer)
 returns long varchar
 BEGIN
   declare dc_return long varchar;
   declare dc_statustext   long varchar;
   declare dc_fa_id integer;
 -- hier dc_return mit Inhalt füllen. Auslesen des Mailbodys aus dem Formulararchiv
   set dc_fa_id = (select VersandBodyFaId from zahlungsbeleg b join zahlungsart a on a.zahlartid=b.zahlartid where zahlungid = in_zahlungid);
   if (dc_fa_id !=0) then
     set dc_return = (select cast(AMICBLOB as long varchar) from amic_fa_get_from_key(dc_fa_id));
   endif;
 -- hier ggf. Platzhalter ersetzen
   return dc_return;
exception when others then
  set dc_StatusText = errormsg()||' '||traceback();
  call fehlerprotokoll ( in_text = ' p_fibu_Belegversand_Avise1'||dc_StatusText);
-- WICHTIG: Dem System mitteilen, dass etwas schiefgegangen ist;
  resignal;
END</code></pre>
          </div>
          <p>Die Mailbody Funktion sollte in jedem Fall eventuell auftretende Fehler abfangen und ins Fehlerprotokoll schreiben und anschließend <b>resignal</b> ausführen, weil ansonsten das verarbeitende Programm nicht mitbekommt, ob ein Fehler aufgetreten ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Betreff</p>
        </td>
        <td>
          <p>Diese Funktion wird nur ausgeführt, wenn auch die Funktion für den Mailbody angegeben wurde. Es existiert eine einfache Funktion AMIC_BELEGVERSAND_BETREFF_AVISE, die Verwendet werden kann. Gibt man einen Namen einer nichtexistierenden Funktion an, um eine neue Funktion zu erstellen, dann wird die Funktion AMIC_BELEGVERSAND_BETREFF_AVISE als Vorlage verwendet und man kann seine Erweiterungen einbauen.</p>
          <div>
            <pre><code>create function "admin"."AMIC_Belegversand_Betreff_AVISE"(in in_ZahlungId integer)
returns long varchar
BEGIN
  DECLARE dc_StatusText   long varchar;
  DECLARE dc_res long varchar;
  set dc_res = (select 'Avise' || in_ZahlungId || ' für Zahlung vom ' || zahlungdatum from Zahlungsbeleg where Zahlungid = in_ZahlungId);
  return dc_res;
exception when others then
  set dc_StatusText = errormsg()||' '||traceback();
  call fehlerprotokoll ( in_text = 'AMIC_BELEGVERSAND_BETREFF_AVISE'||dc_StatusText);
  return dc_res;
END</code></pre>
          </div>
          <p>Tritt hier ein Fehler auf und ist das Ergebnis leer, wird der Festtext Avise als Betreff eingetragen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

**Organisatorisch sollen die Zahlungsarten für eRechnung so angelegt werden, dass diese nicht mit FIBU-Zahlungsarten verwechselt werden können.**
