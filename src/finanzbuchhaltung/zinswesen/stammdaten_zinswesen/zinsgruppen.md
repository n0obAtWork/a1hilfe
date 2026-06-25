# Zinsgruppen

<!-- source: https://amic.de/hilfe/zinsgruppen.htm -->

Hauptmenü \> Mahn-/Zahl-/Zinswesen \> Stammdaten \> Zinsgruppen

Direktsprung **[ZIG]**

Im Pfleger für Zinsgruppen lassen sich alle weiteren Einstellungen für die Zinsabrechnung vornehmen.

![Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung](../../../ImagesExt/image8_693.png "Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung")

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
          <p>Zinsgruppe<br><br></p>
        </td>
        <td>
          <p>Nummer der Zinsgruppe, wie sie dann im Kundenstamm, im Mahnstamm oder in den Wechselkosten hinterlegt wird. Die Zinsgruppe 0 bedeutet immer, dass keine Zinsrechnung vorgenommen werden soll. Es ist also nicht nötig hier etwas einzutragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Text zur Identifikation der Zinsgruppe.</p>
          <p>Ist der Steuerungsparameter 34 "Mehrsprachigkeit aktiv“ in A.eins gesetzt, so hat man auf diesem Feld die Möglichkeit mit F3 <a href="../../../firmenstamm/a_eins_sprache/sprachabhaengige_bezeichnung_in_den_stammdaten.md">sprachabhängige Bezeichnungen</a> zu pflegen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ertragskonto<br><br></p>
        </td>
        <td>
          <p>GuV Konto, auf das vom automatischen Buchungsmodul die Sollzinsen gebucht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aufwandskonto<br><br></p>
        </td>
        <td>
          <p>GuV-Konto, auf das vom automatischen Buchungsmodul die Habenzinsen gebucht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ertragskonto Gutschrift/ Aufwandskonto Gutschrift<br><br></p>
        </td>
        <td>
          <p>In der Praxis kann es vorkommen, dass für Kunden Zinsen individuell angepasst werden müssen. Dafür existiert das Modul „Individuelle Zinsgutschrift“.&nbsp; Hier werden zu einer Zinsabrechnung Gutschriften erstellt. Dabei wird das hier angegebene Konto anstelle des Ertrags- bzw. Aufwandskontos verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kostenstelle Soll/Haben<br><br></p>
        </td>
        <td>
          <p>Die beim Zinsertrag bzw. Zinsaufwand verwendete <a href="../../kostenrechnung/kostenstellen.md">Kostenstelle</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kostenträger Soll/Haben<br><br></p>
        </td>
        <td>
          <p>Der beim Zinsertrag bzw. Zinsaufwand verwendete <a href="../../kostenrechnung/kostentraeger.md">Kostenträger</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kostenobjekt Soll/Haben</p>
        </td>
        <td>
          <p>Das beim Zinsertrag bzw. Zinsaufwand verwendete <a href="../../kostenrechnung/kostenobjekte/index.md">Kostenobjekt</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerschlüssel<br><br></p>
        </td>
        <td>
          <p>Steuerschlüssel, der bei den automatischen Buchungen verwendet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gesamtsaldo verbuchen<br><br></p>
        </td>
        <td>
          <p>Ist hier ein Haken gesetzt, wird der Saldo aus Soll und Habenzinsen gebildet und es entsteht beim Kunden nur ein Buchungssatz. Die Bagatellzinsen wirken sich dann erst auf den Saldo aus. Ansonsten entstehen zwei getrennte Belege.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsen nicht buchbar<br><br></p>
        </td>
        <td>
          <p>Ist hier der Haken gesetzt, lassen die Zinsen sich zwar errechnen und stehen auch in der Zinsliste, es wird jedoch bei der Übernahme in die Primanota für diese Zinsen kein Beleg erstellt. Sie sind also nur kalkulatorisch / informatorisch.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsen verzinsen</p>
        </td>
        <td>
          <p>Ist hier ein Haken gesetzt, werden die automatisch erstellten Belege bei der nächsten Zinsrechnung mit herangezogen und wieder verzinst. Im anderen Fall bekommen diese Belege ein Kennzeichen, dass sie nicht mit verzinst werden. Bei der Bezahlung dieser Belege kommt es zu einer Besonderheit:</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th>OP-Saldo</th>
                <th>Zinssaldo</th>
              </tr>
              <tr>
                <td>AR</td>
                <td>1.000,00 S</td>
                <td>1.000,00 S</td>
              </tr>
              <tr>
                <td>Zinsen (nicht zu verzinsen)</td>
                <td>15,00 S</td>
                <td>0,00 S</td>
              </tr>
              <tr>
                <td>Zahlung hier rauf</td>
                <td>1.015,00 H</td>
                <td>1.015,00 H</td>
              </tr>
              <tr>
                <td>Ergibt einen offenen Saldo von:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</td>
                <td>0,00 H</td>
                <td>15,00 H</td>
              </tr>
            </tbody>
          </table>
          <p>Da hier ungewollt ein Zinssaldo von 15,00 Euro bleiben würde, wird ein interner Beleg erzeugt.</p>
          <table>
            <tbody>
              <tr>
                <th></th>
                <th>OP-Saldo</th>
                <th>Zinssaldo</th>
              </tr>
              <tr>
                <td>Ergibt einen offenen Saldo von:</td>
                <td>0.00€</td>
                <td>15,00 H</td>
              </tr>
              <tr>
                <td>Zinsumbuchung (nicht zu verzinsen)</td>
                <td>-15,00 S</td>
                <td>0,00 S</td>
              </tr>
              <tr>
                <td>Zinsumbuchung</td>
                <td>15,00 S</td>
                <td>15,00 S</td>
              </tr>
              <tr>
                <td>Ergibt einen offenen Saldo von:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</td>
                <td>0,00 H</td>
                <td>0,00 H</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsen nicht berechnen bei OPSaldo 0</p>
        </td>
        <td>
          <p>Hat der Kunde einen aktuellen OP-Saldo von 0 Euro, so wird die Zinsabrechnung für dieses Konto nicht durchgeführt. Die Belege werden dann bei der nächsten Zinsabrechnung berücksichtig. Am Ende des Zinslaufes erscheint dann ggf. folgende Meldung:</p>
          <p>Hinweis: Zinsliste für Konto nnnnn nicht erstellt, da OP-Saldo gleich 0.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsabrechnung auch bei Zinssaldo 0 erstellen</p>
        </td>
        <td>
          <p>Wenn in einer Zinsperiode zwar Belege zur Verzinsung anstehen, aber der errechnete Soll- und Habenzins 0 ist, dann wird keine Zinsabrechnung erstellt. Alle Belege erscheinen dann in der folgenden Abrechnung. Will man die Belege den Abrechnungen periodengerecht zuweisen, so muss man hier den Haken setzen.</p>
          <p><b>Achtung:</b> <i>Diese Einstellung wirkt nicht, wenn der Schalter für „<b>Zinsen nicht berechnen bei OPSaldo 0</b>“ gesetzt ist und der OP-Saldo 0 ist.</i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsen in Kundenwährung buchen</p>
        </td>
        <td>
          <p>Grundsätzlich werden Zinsen in der Buchwährung errechnet und gebucht. Ist hier jedoch ein Haken gesetzt, werden beim Erstellen des Beleges die Zinsen zusätzlich in die Währung des Kunden umgerechnet und als Fremdwährungsbeleg gebucht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zinsabschlag berechnen</p>
        </td>
        <td>
          <p>Hiermit wird festgelegt, ob beim Buchen <a href="./zinsabschlag.md">Zinsabschlagsteuer</a> berechnet werden soll oder nicht.<b><u></u></b></p>
          <p><b><u>Wichtig:</u> </b><i>Das Kennzeichen „Zinsabschlag berechnen“ aus der Zinsgruppe wurde in älteren Versionen nicht ausgewertet. Es muss nun gepflegt werden!</i><b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontokorrentverzinsung oder Verzugszinsen:</p>
        </td>
        <td>
          <p>Hiermit wird lediglich festgelegt, ob der erste Tag oder der letzte Tag mitzählt. Für den gesamten Zinssaldo macht das keinen Unterschied, jedoch dann, wenn ein Beleg über den Zeitraum von mehreren Perioden verzinst wird.<br>AR fällig am 24.01. Zinsliste läuft bis zum 31.01. Bei Kontokorrentverzinsung sind dies 7 Tage bei Verzugszinsen jedoch nur 6 Tage. Wenn dann die Rechnung am 15.02 bezahlt wird, werden bei der Kontokorrentverzinsung 14 Tage und bei den Verzugszinsen 15 Tage gerechnet. Bei beiden Arten ergibt sich eine Dauer von 21 Tagen, nur das in den Zinsperioden unterschiedliche Werte ankommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ab Datum Soll% Haben%:<br><br></p>
        </td>
        <td>
          <p>In dieser Tabelle werden die Zinssätze Periodengerecht getrennt nach Soll und Haben hinterlegt. In ihr werden somit die Entwicklung der Zinssätze archiviert. Aus diesem Grund sollte man hier nie nur den Zinssatz ändern, sondern immer einen neuen Zeitraum angeben.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p><b>Folgende Felder erscheinen nur bei aktiver Belegversand-Lizenz</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versandprofil</p>
        </td>
        <td>
          <p>Versandprofil aus dem <a href="../../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md">Versandprofilstamm</a>, das zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben, so wird auch keine Avise versendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular bei Mail-Versand</p>
        </td>
        <td>
          <p>Optional. Hier kann ein vom Druckformular abweichendes Formular eingerichtet werden. In der F3-Auswahl werden nur Formulare angeboten, bei denen die Archivierung aktiviert ist. Ist kein Formular angegeben, wird das vor dem Druck angegebene Formular verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formular Mailbody</p>
        </td>
        <td>
          <p>Zinsabrechnungen werden als Anhang versendet. Die eigentliche Mail kann hier als Formular definiert werden. Es stehen alle auch in der Zinsabrechnung vorhandenen Druckpositionen zu Verfügung. Es müssen mindestens der Kopfbereich(Bereichsnummer= 311) und eine Positionszeile (Bereichsnummer 314) eingerichtet sein. Die Betreffzeile der Mail kann in diesem Formular im Formularbereich „Zinsabrechnung Betreffzeile“ definiert werden.</p>
          <p>Ein Beispielformular ist unter der Nummer -1140 zu finden.</p>
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
          <p>FA-ID des Formulararchiv-Eintrags eines Mailbody-Templates, das mit Hilfe der Body-Funktion zu einem HTML-Body verarbeitet werden kann.</p>
          <p>Der Eintrag hier ist optional, jedoch muss, wenn kein Template verwendet werden soll, die DB-Funktion das Dokument komplett aufbauen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Mailbody</p>
        </td>
        <td>
          <p>Wird hier eine Funktion hinterlegt, dann wird das „<b>Formular Mailbody</b>“ ignoriert und die Funktion liefert den Text der Mail. Sie erhält als Parameter die Zinslistnummer, Kontonummer, Adressid und das Wertstellungsdatum, welches in der Druckmaske angegeben wurde. Die Funktion muss einen Wert vom Typen long varchar zurückgeben. In der F3-Auswahl werden nur Funktionen angeboten, die in der Datenbank angelegt sind und die entsprechenden Parameter haben.</p>
          <p>Trägt man in dem Feld den Namen einer Prozedur ein, die noch nicht existiert, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <p>Beispiel:</p>
          <div>
            <pre><code>--
-- Funktion zum Erzeugen der MailbBodys für Mailversand Zinsabrechnung
--
 CREATE FUNCTION p_Zinsabrechnung_htmlbody(in in_zinslistnummer integer, in in_kontonummer integer, in in_adressid integer, in in_Wertstellung date)
 returns long varchar
 BEGIN
   declare dc_return long varchar;
   declare dc_statustext long varchar;
   declare dc_fa_id integer;
 -- hier dc_return mit Inhalt füllen. Auslesen des Mailbodys aus dem Formulararchiv
   set dc_fa_id = (select VersandBodyFaId from zinsabrechnung b
                   join zinsgruppe a on a.zinsgrupnummer=b.ZinsGrupNummer
                   where b.zinslistnummer = in_zinslistnummer
                   and b.KontoNummer = in_kontoNummer);
   if (dc_fa_id !=0) then
     set dc_return = (select cast(AMICBLOB as long varchar) from amic_fa_get_from_key(dc_fa_id));
   endif;
 -- hier ggf. Platzhalter ersetzen
   return dc_return;
 exception when others then
   set dc_statustext = errormsg()||' '||traceback();
   call fehlerprotokoll(in_text = 'p_zig1' || dc_statustext);
 -- WICHTIG: Dem System mitteilen, dass ein Fehler aufgereten ist;
   resignal;
 END</code></pre>
          </div>
          <p><b><u>Wichtig:</u></b> Wenn die Funktion für den Mailbody einen Fehler liefert, wird kein Maildokument versendet. Bei Verwendung des Formulars für den Mailbody wurde bei Fehlern ein Standard-Body „&lt;h1&gt;Zinsabrechnung&lt;/h1&gt;“ verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Betreff</p>
        </td>
        <td>
          <p>Diese Funktion wird nur ausgeführt, wenn auch die Funktion für den Mailbody angegeben wurde. Sie erhält als Parameter die Zinslistnummer, Kontonummer, Adressid und das Wertstellungsdatum, welches in der Druckmaske angegeben wurde. Die Funktion muss einen Wert vom Typen „long varchar“ zurückgeben. Auch wenn der Rückgabewert ein long Varchar ist, werden nur die ersten 255 Zeichen ausgewertet. In der F3-Auswahl werden nur Funktionen angeboten, die in der Datenbank angelegt sind und die entsprechenden Parameter haben.</p>
          <p>Gibt man den Namen einer nichtexistierenden Funktion an, um eine neue Funktion zu erstellen, dann wird ein Template mit den korrekten Parametern angelegt und zum Bearbeiten geöffnet.</p>
          <p>Beispiel:</p>
          <div>
            <pre><code>--
-- Funktion zum Erzeugen der Betreffzeile für Mailversand Zinsabrechnung.
--
 CREATE FUNCTION p_Zinsabrechnung_betreff(in in_zinslistnummer integer, in in_kontonummer integer, in in_AdressId integer, in in_Wertstellung date)
 returns long varchar
 BEGIN
   declare dc_return long varchar;
   declare dc_statustext long varchar;
   declare dc_fa_id integer;
 -- hier dc_return mit Inhalt füllen.
   set dc_return = (select 'Zinsabrechnung vom ' || dateformat(in_Wertstellung, 'dd.mm.yyyy') || ' für ' || kundbezeich || ' Konto ' || b.Kontonummer from zinsabrechnung b
                    join kundenstamm k on k.kontonummer=b.kontonummer
                    where b.zinslistnummer = in_zinslistnummer
                      and b.KontoNummer = in_kontoNummer);
   return dc_return;
 exception when others then
   set dc_statustext = errormsg()||' '||traceback();
   call fehlerprotokoll(in_text = 'p_zig2' || dc_statustext);
   return dc_return;
 END</code></pre>
          </div>
          <p>Fehler in der Datenbankfunktion für die Betreffzeile führen <u>nicht</u> dazu, dass kein Mailversand stattfindet.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
