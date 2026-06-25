# Rollenkontext

<!-- source: https://amic.de/hilfe/_rollenkontext.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Einführende Erläuterungen finden sich unter [Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md).

<details>
<summary>Felder des Rollenkontext</summary>

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
          <p>Rolle</p>
        </td>
        <td>
          <p>Zuordnung des Rollenkontextes zur <a href="../rollenstamm/index.md">Rolle</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).</p>
          <p>Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen <b>leeren Kontext</b> signalisiert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontext</p>
        </td>
        <td>
          <p>Ein Rollenkontext besteht aus einer Funktion (Identifikation einer Anwendungsfunktion) und dem Auftreten der Funktion in der Benutzeroberfläche (Kontext).</p>
          <p>Da Funktionen in A.eins auch immer eine Nicht-Benutzeroberflächen-artige Bindung haben, muss dieser Fall mit berücksichtigt werden. Das wird in diesem Falle dann über einen <b>leeren Kontext</b> signalisiert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beschriftung</p>
        </td>
        <td>
          <p>Die textuelle Repräsentation einer Funktion in der Benutzeroberfläche.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktionsart</p>
        </td>
        <td>
          <p>Funktionen senden Botschaften an das A.eins-System. Die Botschaften lassen sich in den Funktionsarten nach ihrem Wesen klassifizieren.</p>
          <p>Siehe Funktionsarten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Direktsprung</p>
        </td>
        <td>
          <p>Ausgewählte Funktionen sind per Direktsprung erreichbar. Es kann mehrere Direktsprünge für die gleiche Funktion geben, diese werden hier angelistet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Eine kurze Erläuterung zu der Funktion durch den Entwickler. Es handelt sich meist um die Beschriftung, es kann aber hilfreiche Abweichungen geben, um besser eine Funktion „einschätzen“ zu helfen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anmerkung</p>
        </td>
        <td>
          <p>Anmerkung, siehe auch Bezeichnung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steupa</p>
        </td>
        <td>
          <p>Steuerparameter</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pulldown</p>
        </td>
        <td>
          <p>Gibt den technischen Bezug zum verwendeten Pulldown-Menü an.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Controlstring</p>
        </td>
        <td>
          <p>Die Botschaft die die Funktion beim Ausführen an das A.eins-System versendet.</p>
          <p>Controlstrings werden mit den Funktionsart klassifiziert, so dass man leichter erkennen kann um welchen Typus Funktion es sich handeln könnte. So gibt es Funktionen die „nur etwas zur Ansicht“ bereitstellen, aber auch Funktionen die Datenbestände löschen bzw. verändern.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besitzer</p>
        </td>
        <td>
          <p>AMIC/Privat</p>
          <p>Funktionen werden größtenteils von AMIC bereitgestellt. Aber auch A.eins-Anwender können private Funktionen bereitstellen.</p>
          <p>Siehe in dem Zusammenhang auch <a href="../rollenstamm/index.md#Controllerklasse">Controllerklasse</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Reservierung</p>
        </td>
        <td>
          <p>Otto Normalbenutzer/Entwicklung</p>
          <p>Eine mit „Otto Normalbenutzer“ ausgewiesene Funktion ist von AMIC freigegeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wann zuerst</p>
        </td>
        <td>
          <p>Wird eine Funktion ins System eingeführt (Privat oder Update), dann wird hier der Zeitpunkt vermerkt wann das geschehen ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Toolbar</p>
        </td>
        <td>
          <p>Gibt an ob sich um einen Toolbar-Kontext handelt oder nicht.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<details>
<summary>Suchmöglichkeiten des Rollenkontext</summary>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Suchkriterien</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Suchen</p>
        </td>
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
        <td>
          <p>Anzahl Datensätze</p>
        </td>
        <td>
          <p>Anzahl, Standard 500</p>
          <p>Zur Sicherheit aus Performance-Gründen. Die Auswahlliste tätigt umfangreiche Kalkulationen. Da man mit ZUGF in aller Regel als nächstes was suchen wird, ist hier die Zeit des Ersteinstiegs regelbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Direktsprung?</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Unterstützt noch besser das gezielte Suchen nach Direktsprüngen.</p>
          <p>Des Weiteren sind somit Listen mit Klassifizierungen von Direktsprüngen möglich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktionsart</p>
        </td>
        <td>
          <p>Funktionsart</p>
          <p>Siehe <a href="./funktionsarten.md">Funktionsarten</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besitzer</p>
        </td>
        <td>
          <p>AMIC/privat</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Reservierung</p>
        </td>
        <td>
          <p>Otto Normalbenutzer/Entwicklung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pulldown?</p>
        </td>
        <td>
          <p>JA/NEIN</p>
          <p>Unterstützt noch besser das gezielte Suchen nach „Pulldown“-Menüs.</p>
          <p>Des Weiteren sind somit Listen mit Klassifizierungen „Pulldown“-Menüs möglich.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rolle enthält Bedienerklasse</p>
        </td>
        <td>
          <p>von..bis (1)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rolle enthält nicht Bedienerklasse</p>
        </td>
        <td>
          <p>von..bis (2)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Toolbar?</p>
        </td>
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
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Funktionen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontext … <strong>(F10)</strong></p>
        </td>
        <td>
          <p>Entwickler-Funktion</p>
          <p>Aufruf des Kontext-Pflegers.</p>
          <p>Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben …</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontext <strong>(shift + F11)</strong></p>
        </td>
        <td>
          <p>Aufruf der Verbindungsübersicht des gewählten Kontextes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Private Sortierung/Tasten</p>
        </td>
        <td>
          <p>Aufruf des Pflegers der die „private Sortierung“ und die „privaten Tastenzuordnung“ innerhalb eines Kontextes betreut.</p>
          <p>Natürlich nur möglich für Rollenkontexte die auch einen Kontext haben …</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion Information <strong>(F9)</strong></p>
        </td>
        <td>
          <p>Aufruf eines <a href="./rollenkontext_pfleger.md">Informationsdialoges zur Funktion</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion ansehen/bearbeiten <strong>(F11)</strong></p>
        </td>
        <td>
          <p>Aufruf des Anwendfunktions-Pflegers.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ändern <strong>(F5)</strong></p>
        </td>
        <td>
          <p>Ändern der Rollenzuordnung des Kontextes.</p>
          <p>Für Details siehe <a href="../rollenklasse/rollenklassen_pfleger.md">Rollenklassenpfleger</a>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ansehen <strong>(F6)</strong></p>
        </td>
        <td>
          <p>Ansehen der Rollenzuordnung des Kontextes.</p>
          <p>Für Details siehe <a href="../rollenklasse/rollenklassen_pfleger.md">Rollenklassenpfleger</a>.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Rollenkontext: Pfleger](./rollenkontext_pfleger.md)
- [Informationen](./informationen.md)
- [Funktionsarten](./funktionsarten.md)
- [Dieses Menü](./dieses_menue.md)
- [Dieses Hauptmenü](./dieses_hauptmenue.md)
- [Diese Verbindungen](./diese_verbindungen.md)
- [Rollenantrag](./rollenantrag.md)
