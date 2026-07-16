# Mahnstamm

<!-- source: https://amic.de/hilfe/mahnstamm.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Mahnwesen einrichten > Funktion Mahnstamm **F6**

Direktsprung **[FIMSG]**.

Mahnstamm und Mahnsätze müssen immer gemeinsam eingerichtet werden, d.h. Wenn es zu einer Mahngruppe und Mahnstufe einen Datensatz im Mahnstamm existiert, muss für diese Kombination auch mindestens ein Eintrag in den Mahnsätzen existieren. Der Pfleger „[Mahnsätze einrichten](./mahnsaetze_einrichten.md)“ übernimmt dies automatisch und sollte diesem Pfleger vorgezogen werden.

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
          <p>Mahngruppe</p>
        </td>
        <td>
          <p>Angabe der Mahngruppe, für die die Bedingungen gelten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mahnstufe</p>
        </td>
        <td>
          <p>Angabe der Mahnstufe, für die die Bedingungen gelten sollen, z.B. <strong>"1"</strong> für <strong>"Mahnstufe 1"</strong><strong></strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Buchungstext</p>
        </td>
        <td>
          <p>Ist hier ein Text eingegeben, so wird dieser bei der Übernahme der Mahngebühren in die Primanota verwendet, sonst der als <a href="./mahnungen_bearbeiten.md#MahnungenBuchen">Einrichterparameter</a> hinterlegte Buchungstext „Text Hauptzeile bei Übernahme der Mahnungen in die Primanota“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular-Id<br><br></p>
        </td>
        <td>
          <p>Nummer des Mahnformulars, das ausgedruckt werden soll. Es kann somit für jede Kombination aus Mahngruppe und Mahnstufe ein eigenes Formular mit unterschiedlichem Aufbau bzw. Text hinterlegt werden. Man kann aber auch für jede Stufe dasselbe Formular hinterlegen und die unterschiedlichen Mahnstufen durch den Mahntext kenntlich machen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsgruppe</p>
        </td>
        <td>
          <p>Falls Verzugszinsen berechnet werden sollen, wird hier die Zinsgruppe angegeben, deren Werte berücksichtigt werden sollen. Bei der Berechnung der Mahnzinsen wird nur der Soll-Zinssatz herangezogen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mahnabstand</p>
        </td>
        <td>
          <p>Der Mahnabstand zwischen zwei Mahnungen. Häufig wird von der Fälligkeit bis zur ersten Mahnung noch eine Schonfrist gewährt. In diesem Fall muss hier bei Mahnstufe 1 ein Zeitraum von z.B. 14 Tagen eingetragen werden, für Mahnstufe 2 und höher wird dann z.B. 10 Tage eingetragen. Somit sind auch unterschiedliche Intervalle je Stufe möglich.</p>
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
          <p>Hier steht das Versandprofil aus dem <a href="../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md">Versandprofilstamm</a>, das zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben, so wird für diese Mahngruppe keine Mahnung versendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular bei Mail-Versand</p>
        </td>
        <td>
          <p>Optional. Hier kann ein vom Druckformular abweichendes Formular eingerichtet werden. Ist kein Formular angegeben, wird das Mahnformular (s.o.) verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular Mailbody</p>
        </td>
        <td>
          <p>Mahnungen werden als Anhang versendet. Die eigentliche Mail kann hier als Formular definiert werden. Es stehen alle auch in der Mahnung vorhandenen Druckpositionen zu Verfügung. Die Betreffzeile der Mail kann in diesem Formular im Formularbereich „Mahnung Mail Betreffzeile“ definiert werden.</p>
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
          <p>FA-ID des Formulararchiv-Eintrags eines HTML-Body-Templates, das mit Hilfe der Body-Funktion zu einem HTML-Body verarbeitet werden kann.</p>
          <p>Der Eintrag hier ist optional, jedoch muss, wenn kein Template verwendet werden soll, die DB-Funktion das Dokument komplett aufbauen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Mailbody</p>
        </td>
        <td>
          <p>Wird hier eine Funktion hinterlegt, dann wird das „<b>Formular Mailbody</b>“ ignoriert und die Funktion liefert den Text der Mail. Sie erhält als Parameter die Mahnungid, die Adressid, und die auf der Druckmaske angegebenen Zahlungsdatum und Zahlungsfrist. Die Funktion muss einen Wert vom Typen long&nbsp; varchar zurückgeben.</p>
          <p>Trägt man in dem Feld einen Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <div>
            <pre><code>--
-- Funktion zum Erzeugen des Mailbodys für Mailversand - Mahnungen.
--
 CREATE FUNCTION p_mahnwesen_htmlbody(in in_mahnungid integer, in in_adressid integer, in in_zahldatum date, in in_zahlfrist date)
 returns long varchar
 BEGIN
   declare dc_return long varchar;
   declare dc_statustext long varchar;
   declare dc_fa_id integer;
 -- hier dc_return mit Inhalt füllen. Auslesen des Mailbodys aus dem Formulararchiv
   set dc_fa_id = (select VersandBodyFaId from Mahnung m
                   join MahnStamm mast on mast.Mahngrupnummer=m.Mahngrupnummer and mast.Mahnstufnummer=m.MahnStufnummer
                   where m.MahnungId= in_mahnungid );
   if (dc_fa_id != 0) then
     set dc_return = (select cast(AMICBLOB as long varchar) from amic_fa_get_from_key(dc_fa_id));
   endif;
 -- hier ggf. Platzhalter ersetzen
   return dc_return;
 exception when others then
   set dc_statustext = errormsg()||' '||traceback();
   call fehlerprotokoll(in_text = 'p_mhb1' || dc_statustext);
 -- WICHTIG: Dem System mitteilen, dass ein Fehler aufgereten ist;
   resignal;
 END</code></pre>
          </div>
          <p><b><u>Wichtig:</u></b> Wenn die Funktion für den Mailbody einen Fehler liefert, wird kein Maildokument versendet. Bei Verwendung des Formulars für den Mailbody wird bei Fehlern ein Standard-Body „&lt;h1&gt;Mahnung&lt;/h1&gt;“ erzeugt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Betreff</p>
        </td>
        <td>
          <p>Diese Funktion wird nur ausgeführt, wenn auch die Funktion für den Mailbody angegeben wurde. Sie erhält als Parameter die Mahnungid, die Adressid, und die auf der Druckmaske angegebenen Zahlungsdatum und Zahlungsfrist. Die Funktion muss einen Wert vom Typen „long varchar“ zurückgeben. Auch wenn der Rückgabewert ein long Varchar ist, werden nur die ersten 255 Zeichen ausgewertet. In der F3-Auswahl werden nur Funktionen angeboten, die in der Datenbank angelegt sind und die entsprechenden Parameter haben.</p>
          <p>Gibt man den Namen einer nichtexistierenden Funktion an, um eine neue Funktion zu erstellen, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <p>Beispiel:</p>
          <div>
            <pre><code>--
-- Funktion zum Erzeugen der Betreff-Zeile für Mailversand Mahnungen.
--
 CREATE FUNCTION p_mahnwesen_betreff (in in_mahnungid integer, in in_adressid integer, in in_zahldatum date, in in_zahlfrist date)
 returns long varchar
 BEGIN
   declare dc_return long varchar;
   declare dc_statustext long varchar;
   declare dc_fa_id integer;
 -- hier dc_return mit Inhalt füllen.
   set dc_return = (select 'Mahnung vom ' || dateformat(MahnungungDatum, 'dd.mm.yyyy') || ' für ' || kundbezeich || ' Konto ' || b.Kontonummer from mahnung b
                    join kundenstamm k on k.kontonummer=b.kontonummer
                    where b.mahnungid = in_mahnungid);
   return dc_return;
 exception when others then
   set dc_statustext = errormsg()||' '||traceback();
   call fehlerprotokoll(in_text = 'p_mhb2' || dc_statustext);
   return dc_return;
 END</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
