# Archiv-Dokumenten-Import

<!-- source: https://amic.de/hilfe/_archivbasis.htm -->

Hauptmenü > Administration > Archiv > Importverwaltung

Direktsprung **[FAI]**

**Hier werden die Dokumenten-Importe verwaltet. Ein Dokumenten-Import wird durch ein Import-Profil beschrieben.**

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
          <p>Eindeutiger Name des Dokumenten-Import-Profils</p>
          <p>(mögliche Farbgebungen siehe „Ziel-Datenbank-Name“)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatik</p>
        </td>
        <td>
          <p>Bei Einstellung <b>Ja</b> übernimmt der Mandantenserver den Import</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ident</p>
        </td>
        <td>
          <p>Ident des Imports</p>
          <p>Zusammen mit der Bedienerklasse ist der technische Schlüssel gegeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p>-1 ist die sogenannte „Defaultklasse Kunden“, das bedeutet das keine spezielle Bedienerklasseneinschränkung vorhanden ist.</p>
          <p>-1 ist der Standard.</p>
          <p>A.eins verwendet für interne Zwecke (z.B. Archiv-Ansichten) auch andere Bedienerklassen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklassenbezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung der Bedienerklasse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Weitere Elemente …</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Import-Datenbank-Name</p>
        </td>
        <td>
          <p>Der Datenbank-Datei-Name gegen den der Import prüfen soll, ob der „richtige“ Mandant vorliegt.</p>
          <p>Durch Duplikate der Datenbank kann es leicht passieren, dass der Import Dateien aus Verzeichnissen importiert, die ausschließlich dem Original-Mandanten vorbehalten sind. Deshalb wird nun geprüft, ob der Datenbank-Datei-Name des Mandanten der den Import ausführt mit „Ziel-Datenbank-Namen“ übereinstimmt.</p>
          <p>Ist eine Differenz festzustellen wird der „Name“ gelb eingefärbt, ist zusätzlich die Automatik aktiviert, dann wird der „Name“ rot eingefärbt. Diese Farbgebungen dienen lediglich der Information. Importe führt das System nicht aus.</p>
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
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Filter/Bereichsauswahl</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Suche in Name</p>
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
          <p><strong>Funktionen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Filter / Bereichsauswahl <b>F2</b></p>
        </td>
        <td>
          <p>Öffnet die Bereichsauswahl.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Neu <b>F8</b></p>
        </td>
        <td>
          <p><a href="./archiv_import_stammdatenpfleger_formulararchiv_importe/index.md">Stammdatenpfleger</a> „Neu“</p>
          <p>Diese Funktionalität ist den SPA226 gebunden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ändern <b>F5, F6, F7</b></p>
        </td>
        <td>
          <p><a href="./archiv_import_stammdatenpfleger_formulararchiv_importe/index.md">Stammdatenpfleger</a> „Ändern, Ansehen, Löschen“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausführen <b>F9</b></p>
        </td>
        <td>
          <p>Führt das selektiere Import-Profil aus</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion anlegen <b>F10</b></p>
        </td>
        <td>
          <p>Legt eine private Funktion in der Optionbox OB_AHFORMULARARCHIV an. Die Beschriftung der Funktion ist der Name des Import-Profils mit einem vorgestellten „Import“.</p>
          <p>Nach einem Neustart des A.eins findet sich diese Funktion dann in der Anwendung „Formulararchiv“, Variante „Formulararchiv“.</p>
          <p>Ist zum Beispiel die Ident „<i>23</i>“, dann ist es die Funktion „PF_FAI_<i>23</i>“ mit dem Controlstring „^jpl fa_exec do <i>23</i>“</p>
          <p>(Siehe auch <a href="./archiv_import_stammdatenpfleger_formulararchiv_importe/funktion_anlegen/index.md">Funktion anlegen</a>)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Duplizieren <b>F11</b></p>
        </td>
        <td>
          <p>Vervielfältigt das ausgewählte Import-Profil.</p>
          <p>Der Name des Duplikats hat ein vorgestelltes „Kopie von“.</p>
          <p>Alternativ steht in <b>Ändern</b> <b>F5</b> die Funktion „<b><i>Speichern unter</i></b>“ zur Verfügung.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
