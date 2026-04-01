# Rollenkontext

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Einführende Erläuterungen finden sich unter Zugriffsrechte Funktionen.

<details>
  <summary>Felder des Rollenkontext</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Felder</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Rolle</td>
          <td>Zuordnung des Rollenkontextes zur Rolle.</td>
        </tr>
        <tr>
          <td>Funktion</td>
          <td>
            <p>Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).</p>
            <p>Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen leeren Kontext signalisiert.</p>
          </td>
        </tr>
        <tr>
          <td>Kontext</td>
          <td>
            <p>Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).</p>
            <p>Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen leeren Kontext signalisiert.</p>
          </td>
        </tr>
        <tr>
          <td>Beschriftung</td>
          <td>Die textuelle Repräsentation einer Funktion in der Benutzeroberfläche.</td>
        </tr>
        <tr>
          <td>Funktionsart</td>
          <td>
            <p>Funktionen senden Botschaften an das A.eins-System. Die Botschaften lassen sich in den Funktionsarten nach ihrem Wesen klassifizieren.</p>
            <p>Siehe Funktionsarten.</p>
          </td>
        </tr>
        <tr>
          <td>Direktsprung</td>
          <td>Ausgewählte Funktionen sind per Direktsprung erreichbar. Es kann mehrere Direktsprünge für die gleiche Funktion geben, diese werden hier angelistet.</td>
        </tr>
        <tr>
          <td>Bezeichnung</td>
          <td>Eine kurze Erläuterung zu der Funktion durch den Entwickler. Es handelt sich meist um die Beschriftung, es kann aber hilfreiche Abweichungen geben, um besser eine Funktion „einschätzen“ zu helfen.</td>
        </tr>
        <tr>
          <td>Anmerkung</td>
          <td>Anmerkung, siehe auch Bezeichnung.</td>
        </tr>
        <tr>
          <td>Steupa</td>
          <td>Steuerparameter</td>
        </tr>
        <tr>
          <td>Pulldown</td>
          <td>Gibt den technischen Bezug zum verwendeten Pulldown-Menü an.</td>
        </tr>
        <tr>
          <td>Controlstring</td>
          <td>
            <p>Die Botschaft die die Funktion beim Ausführen an das A.eins-System versendet.</p>
            <p>Controlstrings werden mit den Funktionsart klassifiziert, so dass man leichter erkennen kann um welchen Typus Funktion es sich handeln könnte. So gibt es Funktionen die „nur etwas zur Ansicht“ bereitstellen, aber auch Funktionen die Datenbestände löschen bzw. verändern.</p>
          </td>
        </tr>
        <tr>
          <td>Besitzer</td>
          <td>
            <p>AMIC/Privat</p>
            <p>Funktionen werden größtenteils von AMIC bereitgestellt. Aber auch A.eins-Anwender können private Funktionen bereitstellen.</p>
            <p>Siehe in dem Zusammenhang auch Controllerklasse.</p>
          </td>
        </tr>
        <tr>
          <td>Reservierung</td>
          <td>
            <p>Otto Normalbenutzer/Entwicklung</p>
            <p>Eine mit „Otto Normalbenutzer“ ausgewiesene Funktion ist von AMIC freigegeben.</p>
          </td>
        </tr>
        <tr>
          <td>Wann zuerst</td>
          <td>Wird eine Funktion ins System eingeführt (Privat oder Update), dann wird hier der Zeitpunkt vermerkt wann das geschehen ist.</td>
        </tr>
        <tr>
          <td>Toolbar</td>
          <td>Gibt an ob sich um einen Toolbar-Kontext handelt oder nicht.</td>
        </tr>
      </tbody>
    </table>
  </div>
</details>

<details>
  <summary>Suchmöglichkeiten des Rollenkontext</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Suchkriterien</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Suchen</td>
          <td>
            <p>Like</p>
            <p>Sucht in den Feldern</p>
            <ul>
              <li>Rolle</li>
              <li>Funktion</li>
              <li>Kontext</li>
              <li>Beschriftung</li>
              <li>Direktsprung</li>
              <li>Bezeichnung</li>
              <li>Pulldown</li>
              <li>Controlstring</li>
            </ul>
          </td>
        </tr>
        <tr>
          <td>Anzahl Datensätze</td>
          <td>
            <p>Anzahl, Standard 500</p>
            <p>Zur Sicherheit aus Performance-Gründen. Die Auswahlliste tätigt umfangreiche Kalkulationen. Da man mit ZUGF in aller Regel als nächstes was suchen wird, ist hier die Zeit des Ersteinstiegs regelbar.</p>
          </td>
        </tr>
        <tr>
          <td>Direktsprung?</td>
          <td>
            <p>JA/NEIN</p>
            <p>Unterstützt noch besser das gezielte Suchen nach Direktsprüngen.</p>
            <p>Des Weiteren sind somit Listen mit Klassifizierungen von Direktsprüngen möglich.</p>
          </td>
        </tr>
        <tr>
          <td>Funktionsart</td>
          <td>
            <p>Funktionsart</p>
            <p>Siehe Funktionsarten.</p>
          </td>
        </tr>
        <tr>
          <td>Besitzer</td>
          <td>AMIC/privat</td>
        </tr>
        <tr>
          <td>Reservierung</td>
          <td>Otto Normalbenutzer/Entwicklung</td>
        </tr>
        <tr>
          <td>Pulldown?</td>
          <td>
            <p>JA/NEIN</p>
            <p>Unterstützt noch besser das gezielte Suchen nach „Pulldown“-Menüs.</p>
            <p>Des Weiteren sind somit Listen mit Klassifizierungen „Pulldown“-Menüs möglich.</p>
          </td>
        </tr>
        <tr>
          <td>Rolle enthält Bedienerklasse</td>
          <td>von..bis (1)</td>
        </tr>
        <tr>
          <td>Rolle enthält nicht Bedienerklasse</td>
          <td>von..bis (2)</td>
        </tr>
        <tr>
          <td>Toolbar?</td>
          <td>
            <p>JA/NEIN/EGAL</p>
            <p>Unterstützt das Auffinden von Toolbar-Funktionen.</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>

  Zu (1): Das bedeutet im Falle das ein Kontext aufgelistet wird nicht zwangsläufig das die Funktion in der Funktionsgruppe auch tatsächlich der Bedienerklasse zugänglich ist. Es könnte nämlich sein, das eine vorherige Funktion die tatsächliche Erreichbarkeit im Programm rollentechnisch ablehnt.

  Zu (2): Die so gelisteten Funktionen sind im jeweils zugehörigen Kontext garantiert nicht für die Bedienerklassen erreichbar.
</details>

<details>
  <summary>Funktionen des Rollenkontext</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Funktionen</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Kontext … <strong>(F10)</strong></td>
          <td>
            <p>Entwickler-Funktion</p>
            <p>Aufruf des Kontext-Pflegers.</p>
            <p>Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben …</p>
          </td>
        </tr>
        <tr>
          <td>Kontext <strong>(shift + F11)</strong></td>
          <td>Aufruf der Verbindungsübersicht des gewählten Kontextes</td>
        </tr>
        <tr>
          <td>Private Sortierung/Tasten</td>
          <td>
            <p>Aufruf des Pflegers der die „private Sortierung“ und die „privaten Tastenzuordnung“ innerhalb eines Kontextes betreut.</p>
            <p>Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben …</p>
          </td>
        </tr>
        <tr>
          <td>Funktion Information <strong>(F9)</strong></td>
          <td>Aufruf eines Informationsdialoges zur Funktion.</td>
        </tr>
        <tr>
          <td>Funktion ansehen/bearbeiten <strong>(F11)</strong></td>
          <td>Aufruf des Anwendfunktions-Pflegers.</td>
        </tr>
        <tr>
          <td>Ändern <strong>(F5)</strong></td>
          <td>
            <p>Ändern der Rollenzuordnung des Kontextes.</p>
            <p>Für Details siehe Rollenklassenpfleger.</p>
          </td>
        </tr>
        <tr>
          <td>Ansehen (F6)</td>
          <td>
            <p>Ansehen der Rollenzuordnung des Kontextes.</p>
            <p>Für Details siehe Rollenklassenpfleger.</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</details>

<p class="siehe-auch">Siehe auch:</p>

- [Rollenkontext: Pfleger](pfleger.md)