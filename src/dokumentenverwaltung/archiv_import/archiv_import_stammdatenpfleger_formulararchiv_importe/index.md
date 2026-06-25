# Archiv-Import-Stammdatenpfleger: Formulararchiv Importe

<!-- source: https://amic.de/hilfe/sdi_archivimport.htm -->

Der Import beschreibt nun, wo die zu importierenden Daten erwartet werden dürfen, wie sie aussehen können, und wie aus ihnen geeignet die Verschlagwortung für das Formulararchiv gewonnen werden kann.

![](../../../ImagesExt/image8_940.jpg)

Anmerkung: Diese Profile können vom Mandantenserver abgewickelt werden. In der nachfolgenden Beschreibung als MSM (Mandantenservermodus) betitelt.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Eindeutiger Name des Dokumenten-Import-Profils.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pfad</p>
        </td>
        <td>
          <p>Legt den Pfad fast, an dem die Daten bereitgestellt werden.</p>
          <p>Eine Besonderheit ist, dass System-Umgebungsvariablen wie %TEMP%, etc. pp. ausgewertet werden. Zu beachten ist, dass der Pfad erwartungsgemäß aus Sicht des importdurchführenden A.eins-Clienten zu sehen ist.</p>
          <p>Für den Einsatz im Batch-Betrieb bietet das unter anderem die Möglichkeit, mit wechselnden Pfaden zu operieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Filter</p>
        </td>
        <td>
          <p>Regulärer Ausdruck der auf die zu verarbeitenden Dateinamen reagiert. Damit besteht die Möglichkeit ein Profil nur auf ganz bestimmte Dateien eines Pfades arbeiten zu lassen. Nämlich genau denen die dem regulären Ausdruck entsprechen.</p>
          <p>Standardmäßig werden alle Dateien des Pfades bearbeitet.</p>
          <p>Beispiel: ^01.* verarbeitet nur die Dateien, die genau mit 01 beginnen.</p>
          <p>Anwendungsbeispiel ist <b>einen </b>Pfad zu haben in denen mehrere Mitarbeiter ihre Dokumente ablegen. Die jeweiligen Profile können dann alle auf diesem Pfad operieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Protokoll anzeigen</p>
        </td>
        <td>
          <p>Es wird ein Protokoll über den Import nach Beendigung dargestellt.</p>
          <p>Im MSM wird diese Einstellung nicht beachtet, also immer als NEIN behandelt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Importierte Dateien löschen</p>
        </td>
        <td>
          <p>Dateien werden nach erfolgreichem Import gelöscht.</p>
          <p>Für die Testphase kann es nützlich sein, diesen Schalter vorerst auf NEIN zu lassen.</p>
          <p>Im Produktionseinsatz ist angeraten ein JA in Erwägung zu ziehen!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mandantherkunft</p>
        </td>
        <td>
          <p>0 = Sektion ( Sektionsname des Mandanten, z.B. aus amicconf.ini)</p>
          <p>1 = Kurztext ( Kurztext der Firma)</p>
          <p>2 = Nummer (Standard, Nummer der Firma)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Import-Datenbank-Name</p>
        </td>
        <td>
          <p>Siehe <a href="../archiv_dokumenten_import.md#import_datenbank_name">Import-Datenbank-Name</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegdatum</p>
        </td>
        <td>
          <p>0 = Heute (Datum zum Zeitpunkt des Imports)</p>
          <p>1 = Dateidatum (Standard)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archiv/Druckdatum</p>
        </td>
        <td>
          <p>0 = Heute (Standard, Datum zum Zeitpunkt des Imports)</p>
          <p>1 = Dateidatum</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mailermittlung</p>
        </td>
        <td>
          <p>Datenbank-Funktion die anhand der Mail-Adresse eine Kundennummer zu ermitteln versucht</p>
          <p>(Standard amic_fa_kundnummer_mail)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateiinformation</p>
        </td>
        <td>
          <p>Optionales VBA-Script</p>
          <p>(Standard ist AMIC_FA_INFO. Das Script holt aus den NTFS-Streams Datei-Informationen. NTFS-Streams sind daran beteiligt. Das sind eigenständige Objekte des NTFS-Dateisystems.' Sie sind nicht per Explorer oder "Datei-Merkmalen" greifbar)</p>
          <p>Bei NTFS-Systemen ist es möglich, dass das Betriebssystem direkt weitere Information zu einer Datei speichern kann.</p>
          <p>Es handelt sich um Angaben wie z.B. Titel, Autor, etc. pp.</p>
          <p>A.eins kann diese Daten auslesen und macht das standardmäßig per VBA-Script AMIC_FA_INFO.</p>
          <p>Ist dieses Verhalten nicht erwünscht, dann kann an dieser Stelle einfach keine Angabe eines Scriptes erfolgen, bzw. es lässt sich ein geändertes Script hinterlegen.</p>
          <p>Zur Laufzeit können bzw. werden somit die JVARS mit Owner 5000 befüllt:</p>
          <p>JVARS_FA_NTFS_FILE</p>
          <p>JVARS_FA_NTFS_AUTHOR</p>
          <p>JVARS_FA_NTFS_CATEGORY</p>
          <p>JVARS_FA_NTFS_COMMENTS</p>
          <p>JVARS_FA_NTFS_KEYWORDS</p>
          <p>JVARS_FA_NTFS_SUBJECT</p>
          <p>JVARS_FA_NTFS_TITLE</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausnahmen</p>
        </td>
        <td>
          <p>Hier lässt sich eine Datenbank-Funktion angeben mit deren Hilfe die Ausnahmen bestimmt werden können.</p>
          <p>Beispiele sind die versteckten Dateien desktop.ini und thums.db.</p>
          <p>Aber auch Links (also Dateien mit der Extension lnk) sollen nicht importiert werden.</p>
          <p>Standard: amic_get_fai_ausnahmen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Startabfrage</p>
        </td>
        <td>
          <p>Ob beim Start des Imports eine Abfrage ausgeführt werden soll.</p>
          <p>Dort erhält man die Anzahl der vermutlich zu importierenden Dokumenten und hat eine Abbruch-Möglichkeit.</p>
          <p>Ist keine Startabfrage gewünscht, gibt es auch automatisch kein visuelles Protokoll!</p>
          <p>Im MSM wird diese Einstellung nicht beachtet, also immer als NEIN behandelt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Interaktion</p>
        </td>
        <td>
          <p>Wenn Kriterien der Datenbank-Tabelle nicht ermittelt worden sind, öffnet sich dann bei Ja ein Abfrage-Dialog.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Recherche</p>
        </td>
        <td>
          <p>Es wird versucht, anhand von Kerndaten (bekannte Archiv-Referenz, Kundennummer) bestimmte Dinge aus den schon vorhandenen Daten im System zu ermitteln.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatik</p>
        </td>
        <td>
          <p>Ja = der Mandantenserver übernimmt den eigentlichen Import der Dokumente gemäß der Profileinstellungen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Excel als XML importieren</p>
        </td>
        <td>
          <p>Steht der Schalter „Excel als XML importieren“ auf „Ja“, so wird beim Import einer Excel-Datei zusätzlich der Dateiinhalt der Excel-Datei in ein XML gespeichert. Das XML findet man in der Spalte „FA_XMLErweiterung“ in der Relation Formulararchiv wieder. Auf das XML kann über eine <a href="./index.md#sql_ereignis">SQL-Nachlaufprozedur</a> zugegriffen werden.</p>
          <p>Zum Auslesen des XMLs wird die SQL-Prozedur „amic_fa_get_XmlErweiterung_Excel“ empfohlen. Als Parameter werden die Fa_Id und die Fa_Mndnr erwartet. Die Prozedur gibt zu jeder nicht-leeren Excel-Zelle den Wert, die Spalte und Zeile sowie den Arbeitsblattnamen, in der sich die Zelle befindet, zurück.</p>
          <div>
            <pre><code>create procedure amic_fa_get_XmlErweiterung_Excel(in in_fa_id integer, in in_fa_mndNr integer)
result (
  Arbeitsblatt long varchar,
  Spalte char(255),
  Zeile integer,
  Wert long varchar)
BEGIN
.
.
.
END</code></pre>
          </div>
          <p>Hinweise:</p>
          <p>In der XML wird als Dezimaltrennzeichen immer ein Punkt (.) verwendet.</p>
          <p>Es werden nur die Dateiformat .xlsx und .xlsm unterstützt. So werden beim Import von z.B. .xls-Dateien keine XMLs generiert.</p>
          <p>Voraussetzung:</p>
          <p>Es wird die <a href="../../../firmenstamm/steuerparameter/lizenzen/excel_archivimport_lizenz_spa1127.md">Archivimport-Excel-Lizenz</a> benötigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wartezeit in Minuten</p>
        </td>
        <td>
          <p>Es existieren Scanner-Systeme die ihr Erzeugnis in mehreren Schritten erzeugen. Um diese „Reifezeit“ von A.eins zu unterstützen gibt es hier die Möglichkeit eine Wartezeit in Minuten anzugeben, bevor das A.eins-Archiv-Import-System die Datei verarbeitet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Max. Anzahl pro Durchlauf</p>
        </td>
        <td>
          <p>Da je nach Dateiaufkommen und -größe der allgemeine Mandantenserver-Betrieb in Stoßzeiten durch den Import behindert werden könnte gibt es hier die Möglichkeit die die Anzahl der zu importierenden Dateien zu konfigurieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>eRechnung import</p>
        </td>
        <td>
          <p>Steht dieser Schalter auf „Ja“, so werden importierte PDF oder XML-Dateien daraufhin geprüft, ob sie eine eRechnung im Format UBL oder CII enthalten und ggf. werden diese Dateien automatisch importiert.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

**Bei Gewinnung aus Dateiinhalt**

Hier kann ein Bereich des Dateiinhaltes festgelegt werden, der dann in der Datentabelle unter Herkunft **Dateiinhalt** zur Datenermittlung herangezogen werden kann (siehe auch [Zuordnung der Herkunft](./index.md#zuordnung_der_herkunft))

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Start-Kennung</p>
        </td>
        <td>
          <p>Siehe <a href="./gewinnung_aus_dateiinhalt.md">Gewinnung aus Dateiinhalt</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ende-Kennung</p>
        </td>
        <td>
          <p>Siehe <a href="./gewinnung_aus_dateiinhalt.md">Gewinnung aus Dateiinhalt</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Muster</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

**Sql-Ereignis**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>nach Einfügung</p>
        </td>
        <td>
          <p>Hier kann eine private Datenbank-Procedure hinterlegt werden.</p>
          <p>(Private Datenbank-Prozeduren beginnen mit einem „p_“)</p>
          <p>Die Erstanlage einer geeigneten Datenbank-Procedure wird durch die Anlage eines Templates unterstützt. (Funktion „<b>Sql-Ereignis nach Einfügung …</b>“)</p>
          <p>Mit Ihrer Hilfe lassen sich Aktionen auf der Datenbank ausführen, die nach der Einfügung z.B. in den Tabellen Formulararchiv und Archiv durchgeführt werden sollen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

**Datentabelle Verschlagwortungskriterien**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Herkunft</p>
        </td>
        <td>
          <p>Siehe <a href="./index.md#zuordnung_der_herkunft">Zuordnung der Herkunft</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA-Spalte</p>
        </td>
        <td>
          <p>Siehe <a href="./index.md#zuordnung_der_fa_spalten">Zuordnung der FA-Spalten</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>regulärer Ausdruck</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>G</p>
        </td>
        <td>
          <p>Gruppe</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pos</p>
        </td>
        <td>
          <p>Sortierkriterium</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NBV</p>
        </td>
        <td>
          <p>(N)ach(B)earbeitung(V)orher</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NBZ</p>
        </td>
        <td>
          <p>(N)ach(B)earbeitung(Z)iel</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

**Ereignisbehandlung**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>am Anfang</p>
        </td>
        <td>
          <p>Hier kann ein VBA-Skript angegeben werden, dass einmalig am Anfang des Archivimports ausgeführt wird (siehe <a href="./ereignisbehandlung.md">Ereignisbehandlung</a>).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>vor Einfügen</p>
        </td>
        <td>
          <p>Hier kann ein VBA-Skript angegeben werden, dass vor dem Einfügen jeder Datei ausgeführt wird (siehe <a href="./ereignisbehandlung.md">Ereignisbehandlung</a>).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>nach Einfügen</p>
        </td>
        <td>
          <p>Hier kann ein VBA-Skript angegeben werden, dass nach dem Einfügen jeder Datei ausgeführt wird (siehe <a href="./ereignisbehandlung.md">Ereignisbehandlung</a>).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>am Ende</p>
        </td>
        <td>
          <p>Hier kann ein VBA-Skript angegeben werden, dass einmalig am Ende des Archivimports ausgeführt wird (siehe <a href="./ereignisbehandlung.md">Ereignisbehandlung</a>).</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Zuordnung der Herkunft</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Dateiname</p>
          <p>Die Datengrundlage für die Datenzeile ist der Dateiname</p>
          <p>Es ist durch geeignete Vorbereitungen Kerndaten im Dateinamen zu hinterlegen.</p>
          <p>Etwaige Scanner-Systeme bieten solche Möglichkeiten.</p>
          <p>Hier nun ist es möglich zu beschreiben, wie A.eins diese Daten zu interpretieren hat, wenn sie denn in einer solchen Form vorliegen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Dateiinhalt</p>
          <p>Der Dateiinhalt kann eine spezifische Ausprägung vorweisen, die gezielt zur Daten-Gewinnung herangezogen werden kann.</p>
          <p>Es kann <b>ein</b> Dateninhalt über <a href="./gewinnung_aus_dateiinhalt.md">Gewinnung aus Dateiinhalt</a> definiert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Konstante</p>
          <p>Für die Fälle, wo ein Kern-Datum fest vorgegeben bzw. vorbelegt werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Abfrage</p>
          <p>Es ist die interaktive Nachverschlagwortung vorgesehen und implementiert!</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Zuordnung der Fa-Spalten</strong></p>
        </td>
        <td>
          <p><strong>Formulararchiv-Spalte</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Keine</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Belegklasse</p>
        </td>
        <td>
          <p>fa_belegklasse</p>
          <p>Die Belegklasse des Beleges. Siehe FAKLASSE für mögliche Ausprägungen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>BelegNummer</p>
        </td>
        <td>
          <p>fa_belegnummer</p>
          <p>Belegnummer falls vorhanden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>BelegReferenz</p>
        </td>
        <td>
          <p>fa_belegreferenz</p>
          <p>Belegrefrenz</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Kundennummer</p>
        </td>
        <td>
          <p>fa_kundnummer</p>
          <p>Kundnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Mandant</p>
        </td>
        <td>
          <p>fa_mandant</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>6</p>
        </td>
        <td>
          <p>Belegtyptext</p>
        </td>
        <td>
          <p>fa_belegtyptext</p>
          <p>Der Belegtyptext. Maßgeblich ist aber die Belegklasse!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>7</p>
        </td>
        <td>
          <p>Mailadresse</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>8</p>
        </td>
        <td>
          <p>Anleger</p>
        </td>
        <td>
          <p>fa_neuanlagebediener</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>9</p>
        </td>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p>fa_bedienerklasse</p>
          <p>Bedienerklasse des Belegerzeugers</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>10</p>
        </td>
        <td>
          <p>Autor</p>
        </td>
        <td>
          <p>fa_info_autor</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>Betreff</p>
        </td>
        <td>
          <p>fa_info_betreff</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>12</p>
        </td>
        <td>
          <p>Kategorie</p>
        </td>
        <td>
          <p>fa_info_kategorie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>13</p>
        </td>
        <td>
          <p>Kommentar</p>
        </td>
        <td>
          <p>fa_info_kommentar</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>14</p>
        </td>
        <td>
          <p>Stichwörter</p>
        </td>
        <td>
          <p>fa_info_stichwoerter</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>15</p>
        </td>
        <td>
          <p>Titel</p>
        </td>
        <td>
          <p>fa_info_titel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>16</p>
        </td>
        <td>
          <p>Barcode</p>
        </td>
        <td>
          <p>fa_barcode</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>17</p>
        </td>
        <td>
          <p>Dateiname</p>
        </td>
        <td>
          <p>fa_dateiname</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>18</p>
        </td>
        <td>
          <p>Klassifizierung</p>
        </td>
        <td>
          <p>fa_klasse</p>
          <p>Klassifizierung</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Funktion</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sql-Ereignis nach Einfügung</p>
        </td>
        <td>
          <p>Öffnet Editor zum Bearbeiten des Sql-Ereignisses.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datenbank-Dateiname abstimmen</p>
        </td>
        <td>
          <p>Füllt das Feld „Import-Datenbank-Dateinamen“ mit dem Datenbank-Datei-Namen des aktiven Mandanten aus.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Funktion anlegen](./funktion_anlegen/index.md)
- [Gewinnung aus Dateiinhalt](./gewinnung_aus_dateiinhalt.md)
- [Muster](./muster.md)
- [Konstante](./konstante.md)
- [Nachbearbeitung](./nachbearbeitung/index.md)
- [Ereignisbehandlung](./ereignisbehandlung.md)
- [Zuordnung JVARS – Kriterien](./zuordnung_jvars_kriterien.md)
- [Interaktion während des Importvorgangs Archiv](./interaktion_waehrend_des_importvorgangs_archiv.md)
