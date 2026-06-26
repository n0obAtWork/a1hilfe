# Bedienerklasse: Pfleger

<!-- source: https://amic.de/hilfe/_bedienerklassepflege.htm -->

### Register:

<details>
<summary>Allgemein</summary>

| Felder | Beschreibung |
| --- | --- |
| Bedienerklasse | Eindeutige numerische Identifikation der Bedienerklasse<br> |
| Bezeichnung | Bezeichnung der Bedienerklasse<br> |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.<br>Standard: 0, ohne Filiale<br>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.<br> |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte<br> |
| Login-Sperre | JA/NEIN<br>Login-Sperre aller Bediener dieser Bedienerklasse<br> |
| Formular FiBu-Infofenster | JA/NEIN<br>[Zusätzliche Informationen in der Finanzbuchhaltung aktivieren](../../../../finanzbuchhaltung/op_verwaltung/formular_fibu_infofenster.md)<br><br> |
| Toolbar aktiv | JA/NEIN<br>Aktiviert die standardmäßig ausgelieferte Toolbar.<br> |
| Abteilung | Hier kann die Bedienerklasse einer Abteilung zugeordnet werden.<br>[Abteilungen](../../../abteilungen.md)<br> |
| Controllerklasse | [Controllerklasse](./standard_bedienerklassen.md)<br>Controllerklassen werden gelb hervorgehoben.<br> |
| Sicherheitsklasse | [Sicherheitsklasse](./standard_bedienerklassen.md)<br> |

</details>

 

<details>
<summary>Formulararchiv</summary>

Hier kann für eine Bedienerklasse festgelegt werden, welche Formulare des Archivs (abhängig von den zugeordneten Bedienerklassen) eingesehen werden dürfen.

Diese Rechte können auch beim [Bediener](./standard_bedienerklassen.md) (Register Formulararchiv) eingesehen werden.

</details>

<details>
<summary>Passwortrichtlinien</summary>

| Felder | Beschreibung |
| --- | --- |
| Mindestlänge | Wie viele Zeichen das Passwort mindestens haben soll.<br> |
| Höchstlänge | Wie viele Zeichen das Passwort maximal haben darf.<br>Das Limit liegt bei 10 Zeichen.<br> |
| Zahlen | Wie viele Zeichen im Passwort mindestens eine Zahl (0-9) sein müssen.<br> |
| Sonderzeichen | Wie viele Zeichen im Passwort mindestens ein Sonderzeichen sein müssen.<br>Ausgenommen sind folgende Zeichen: „ \\ ; - ‘<br> |
| Aktualisierung in Tagen | Nach wie vielen Tagen seit der letzten Passwortänderung das Passwort wieder geändert werden muss.<br> |

    
**Die Passwortrichtlinien werden je nach Bedienerklasse individuell verwaltet und gepflegt.**

**Die Bearbeitung und Anpassung der Passwortrichtlinien ist ausschließlich Systemadministratoren vorbehalten.**

Zur Änderung des eigenen Passworts kann über den Direktsprung **[PWD]** die entsprechende Ansicht zur Passwortänderung geöffnet werden.

Wird ein Wert auf **0** gesetzt oder das Kontrollkästchen deaktiviert, so wird die jeweilige Bedingung bei der Prüfung nicht berücksichtigt.

Die Richtlinie **„Aktualisierung in Tagen“** wird nur beim Anmeldevorgang in **A.eins** überprüft.

### Funktion:

</details>

<details>
<summary>Bedienerklasse: Pfleger Funktionen</summary>

Im Pfleger stehen folgende Funktionen zur Verfügung:

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
          <p><strong>Speichern F9</strong></p>
        </td>
        <td>
          <p>Speichert die Daten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Speichern unter … Shift+F9</strong></p>
        </td>
        <td>
          <p>Übernimmt die Daten der ausgewählten Bedienerklasse.</p>
          <p>Das System schlägt eine neue Bedienerklasse vor.</p>
          <p>Die Einstellungen hinsichtlich des Formulararchivs sind in diesem Modus nicht zu beeinflussen, die Registerkarte „Formulararchiv“ wird ausgeblendet.</p>
          <p>Bei Ausführung der Funktion <strong><em>Speichern</em></strong><i> </i><strong>F9</strong> wird die neue Bedienerklasse zur weiteren Verwendung im System vorbereitet (der Vorgang kann etwas dauern und sollte nicht abgebrochen werden):</p>
          <p><b>Es werden dabei u.a. sämtliche Schutzeinstellungen der Funktionen (</b><a href="../../zugriffsrechte_funktionen.md">Zugriffsrechte Funktionen</a><b>) der Ursprungsbedienerklasse übernommen.</b><b></b></p>
          <p>Ferner werden dabei alle Einstellungen der Ursprungsbedienerklasse „geclont“.</p>
          <p>Bei den Formulararchiv-Einstellungen sind die Belege der Ursprungsklasse erlaubt, in den Einstellungen der Ursprungsklasse sind die Einstellungen der neuen Bedienerklasse erlaubt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Löschen <strong>F7</strong></p>
        </td>
        <td>
          <p>Bedingung: Die Bedienerklasse darf keine Bediener mehr enthalten.</p>
          <p>Alle Einstellungen der Bedienerklasse in den abhängigen Einstellungen (<a href="../../zugriffsrechte_funktionen.md">Zugriffsrechte Funktionen</a>, EPA, …) werden herausgenommen.</p>
          <p>Zum jetzigen Zeitpunkt betrifft dies folgende Tabellen:</p>
          <ul>
            <li>NUMKREISBEDIENER</li>
            <li>KORRKLASSBEDLINK</li>
            <li>BEDIENERPROFIL</li>
            <li>ABTEILBEDIENLINK</li>
            <li>OWAAGENUMKREIS</li>
            <li>FIBUBELEGNUMKR</li>
            <li>BEDIENERGRUPLINK</li>
            <li>USERFELDER</li>
            <li>KONTRKLNUMKREIS</li>
            <li>FORMULARARCHIVBEDIENER</li>
            <li>AMIC_NEWS</li>
            <li>FA_IMPORT</li>
            <li>FA_IMPORT_REGEL</li>
            <li>FORMULARARCHIV_MANAGER</li>
            <li>AMIC_MENU_MESSAGES</li>
            <li>FA_VIEW_PROFIL</li>
            <li>FA_VIEW_PROFIL_DETAIL</li>
            <li>ANWENDFUNKKLASSE</li>
            <li>ANWENDOBKLASSE</li>
            <li>ANWENDVARKLASSE</li>
            <li>AMICINFOSYSMSKZUORD</li>
            <li>ROLLENKLASSE</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Neu <strong>F8</strong></p>
        </td>
        <td>
          <p>Neue Bedienerklasse anlegen.</p>
          <p>Siehe aber auch <a href="./bedienerklasse_pfleger.md#Speichern_unter_SF9">"Speichern unter ... SF9"</a> für die Anlage einer neuen Bedienerklasse mit gleichzeitiger Übernahme der Bedienerklassen-relevanten Einstellungen!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>EPAs zeigen <strong>F10</strong></p>
        </td>
        <td>
          <p>Individuelle Steuerungen von Abläufen können in Anwendungen über Einrichterparameter (EPA) vorgenommen werden.</p>
          <p>Diese Funktion ruft die entsprechende Anwendung zur Ansicht und Pflege der Einrichterparameter auf.</p>
          <p>Hauptmenü &gt; &nbsp;Administration &gt; &nbsp;Steuerung &gt; &nbsp;EPAs zeigen</p>
          <p>Direktsprung <strong>[EPAZ]</strong><b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zugriffsrechte übertragen…</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>NKZ-Zuordnung</p>
        </td>
        <td>
          <p>Nummernkreiszuordnung der ausgewählten Bedienerklasse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Betriebsstätte</p>
        </td>
        <td>
          <p>Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.</p>
          <p>Standard: 0, ohne Filiale</p>
          <p>Konflikte, die sich beim Wechsel der Betriebstätte bzgl. „Name Sicherheit“ ergeben könnten, werden erkannt und der Wechsel dann unterbunden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung Betriebsstätte</p>
        </td>
        <td>
          <p>Bezeichnung der Betriebsstätte</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>
