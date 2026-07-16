# Archiv-Ansicht-Definition

<!-- source: https://amic.de/hilfe/_archivansichtdefinit.htm -->

Ausgelöst wird eine Archiv-Ansicht über die Funktion ***Archiv anzeigen*** **CF12** im jeweiligen Programm-Kontext in A.eins. ([Ansichten allgemein](../ansichten_allgemein/index.md))

Eine Archiv-Ansichts-Definition ist im einfachsten Fall eine von AMIC vorkonfektionierte Beschreibung, mit deren Hilfe A.eins im Archiv recherchiert.

Der Programm-Kontext in A.eins stellt automatisch (fest vorgegebene) Kriterien zur Verfügung, die in der Archiv-Ansicht-Definition zur Auswertung und Bestimmung, welche Archiv-Einträge in der Archiv-Ansicht aufgelistet werden sollen, herangezogen werden können.

Alle ausgelieferten Archiv-Ansicht-Definitionen, die von AMIC mitgeliefert und bei Programmupdate aktualisiert werden, finden sich in der Variante Archiv-Ansichten-Variante: Ansichten ( nur AMIC – Auslieferung ).

Tabelle 5 Wichtige Archiv-Ansicht-Definitions-Begriffe

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Sammelbegriff für Archiv-Ansichten gleichen Typus.</p>
          <p>Der Name des Ansichtsprofils.</p>
          <p>Eine so zusätzlich angelegte Ansichts-Definition kann nun auf einfache Weise ins A.eins-System integriert werden. Dazu bindet man eine private Funktion in die entsprechende Optionbox ein. Der Controlstring lautet ^jpl fa_view Name, wobei dann für Name der jeweilige Name zu setzen ist.</p>
          <p>Alle AMIC-Standard-Ansichten werden intern auf die Funktionalität fa_view umgesetzt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p>Bedienerklasse</p>
          <p>Hinweis: Die Bedienerklasse -1 bedeutet stets alle Bedienerklassen.</p>
          <p>Eine Ansichtsdefinition kann für eine einzelne Bedienerklasse bestimmt werden. Diese hat dann zur Laufzeit für ein Mitglied der Bedienerklasse Vorrang vor der speziellen Bedienerklasse -1.</p>
          <p>Somit ist es möglich individuelle Ansichtsdefinitionen für spezielle Bedienerklassen auszuprägen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besitzer</p>
        </td>
        <td>
          <p>AMIC: Eine von AMIC ausgelieferte Ansichtsdefinition</p>
          <p>Privat: Eine vom A.eins-Anwender erstellte Ansichtsdefinition</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ansichts-Id</p>
        </td>
        <td>
          <p>Technische Identifikation einer Ansichtsdefinition.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Über die Pflege-Funktionen in den unter „[Archiv-Ansichten definieren](../archiv_ansichten_definieren/index.md)“ angegebenen Varianten erhält man Zugriff auf folgende weitere allgemeine Merkmale einer Archiv-Ansichten-Definition:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td>
          <p>Dokument</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt ob „persönliche Dokumente“ beim Öffnen/Ausführen der Ansicht über <strong>CF12</strong> dem Kontext der entsprechenden A.eins-Lokalität zugewiesen werden sollen – oder nicht.</p>
          <p>A.eins bietet die Möglichkeit „persönliche/private“ Dokumente, die sogenannten „freien Dokumente“ (Belegklasse 9000) ins System zu importieren. Diese haben die Eigenschaft, dass, wenn man an geeigneter Stelle eine Formulararchiv-Ansicht aufruft, sie eben diesen aufgerufenen Kontext zugeordnet werden. Weitere Erläuterungen/Konfigurierungen bei der Behandlung der „Details“ .</p>
          <img src="../../../ImagesExt/image8_919.png" alt="">
          <p>„Geeignete Stellen“ sind somit Profile/Ansicht – Umgebungen, in denen man eben hier diese Möglichkeit vorsieht.</p>
          <p>So bietet es die Möglichkeit, 9000er-Zuweisungen auf bestimmte Bereiche einzugrenzen. Eben weil man u. U. viel mit <strong>SF4</strong> arbeitet und ansonsten eine „Fehlzuweisung“ passieren könnte.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zusatz</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt, ob zusätzlich zu den durch die Ansicht-Definition bestimmten Archiv-Einträgen auch alle die Archiv-Einträge aufgelistet werden, die weder eine Kundennummer noch eine Beleg-Referenz haben.</p>
          <p>Unterstützung einer möglichen Organisationsform hinsichtlich des Fehlens von Kundennummer und Belegreferenz von Archiv-Einträgen. Solche Dokumente kommen vornehmlich per Scanner-Importverfahren oder ähnlich gearteten Verfahren ins System. Jedenfalls ist die Absicht, genau diese im Grunde noch nicht zugeordneten Belege an jeder Stelle im Programm, in der in das Formulararchiv gesehen wird, den Sachbearbeitern zur Kenntnis zu bringen.</p>
          <p>Diese haben dann die Möglichkeit den Beleg einzusehen und ggf. durch „Bearbeiten“ eine Kundennummer und/oder Referenznummer zuzuordnen, um somit den Beleg „abzuarbeiten“.</p>
          <p>Unterstützung bezüglich dieser Bearbeitung bietet auch der Punkt „Autoedit“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Autoedit</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt, ob bei der Funktion <strong>F5</strong> in der Formulararchiv-Anzeige die Felder „Kundennummer“ und „Beleg-Referenz“ automatisch mit dem jeweiligen A.eins-Kontext belegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausschluss</p>
        </td>
        <td>
          <p>Hier kann eine durch Komma getrennte Liste von Beleg-Klassen angegeben werden, deren Archiv-Einträge nicht angelistet werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Durchstart</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Ergibt die durch die Ansichts-Definition veranlasste Archiv-recherche, dass genau ein „Dokument“ gefunden wurde, so wird die Anzeige dieses einen Dokumentes sofort ohne weitere Rückfrage durch das Programm eingeleitet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Hinzufügen</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>In der Formulararchiv-Anzeige wird die Funktion <strong><em>Hinzufügen</em></strong> <strong>F8</strong> aktiviert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bed.Schutz</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt ob das Schutz-System über die Bedienerklasse des Archives aktiviert wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Priv. Import aktiviert</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt ob während der Archiv-Recherche der „Private Import“ durchgeführt wird.</p>
          <p><a href="./archiv_privater_import.md">Archiv: Privater Import</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Profilpfad</p>
        </td>
        <td>
          <p>Ein hier hinterlegter Pfad wird von A.eins automatisch um den Kurznamen des jeweiligen ausführenden Bedieners erweitert und bestimmt dann den Pfad für den „Privaten Import“.</p>
          <p><a href="./archiv_privater_import.md">Archiv: Privater Import</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Profilfilter</p>
        </td>
        <td>
          <p>Reguläres Muster für Importfilter</p>
          <p><a href="./archiv_privater_import.md">Archiv: Privater Import</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Einsatz</p>
        </td>
        <td>
          <p>Beschreibung über den Einsatz bzw. Verwendungen der Ansichts-Definition.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Grundlage</p>
        </td>
        <td>
          <p>Versucht über das Einsatzgebiet der <a href="">Archiv-Ansichten</a> zu informieren.</p>
          <p>Mögliche Identifizierungen sind:</p>
          <p>0 : Frei</p>
          <p>1 : Auswahlliste</p>
          <p>2 : Dialog</p>
          <p>3 : Extern</p>
          <p>4: Auswahl</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Variante</p>
        </td>
        <td>
          <p>Variante, die zur Formulararchiv-Anzeige herangezogen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anwendung</p>
        </td>
        <td>
          <p>Anwendung, die zur Formulararchiv-Anzeige herangezogen wird.</p>
          <p>Zurzeit ist diese auf „fa_anzeige“ festgelegt und nicht veränderbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dialog</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Bestimmt, ob die Formulararchiv-Anzeige als Dialog oder Nicht-Dialog gestartet wird. Empfehlung=JA</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorschau</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Aktiviert in dieser Ansicht eine erweiterte Archiv-Ansichts-Technologie.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Archiv: Privater Import](./archiv_privater_import.md)
