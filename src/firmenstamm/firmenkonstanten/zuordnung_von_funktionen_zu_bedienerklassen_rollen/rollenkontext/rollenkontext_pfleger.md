# Rollenkontext: Pfleger

<!-- source: https://amic.de/hilfe/_rollenkontext_pfleger.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Pflege des Rollen-Kontextes der Funktion in der Optionbox.

Ein solcher „Kontext“ beschreibt die Rechtezuordnung der Bedienerklassen an genau dieser Stelle im Programm durch die Zuweisung einer Rolle.

Ändert sich diese Zuweisung, werden also Bedienerklassen Rechte erteilt bzw. entzogen, führt das zu einer neuen Rollendefinition. Das System prüft automatisch, ob es schon eine solche Rechtekonfiguration gibt, und stellt diese zur Verfügung. Im Falle das es die gewünschte Konfiguration als Rolle noch nicht gibt wird diese vom System angelegt.

Sollen Rollen insgesamt geändert werden, also für alle Vorkommen der Rolle in allen Kontexten gleichzeitig, empfiehlt sich der [Rollenpfleger](../rollenstamm/rollenstamm_pfleger.md).

In Umgebungen in denen spezielle Bedienerklasse für die Abarbeitung und Überwachung der Rollenänderungen eingerichtet sind lassen sich sogenannte Rollenanträge erstellen und ggf. vermailen.

<details>
<summary>Felder des Rollenkontext Pfleger</summary>

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
          <p>Kontext</p>
        </td>
        <td>
          <p>Zuordnung des Rollenkontextes zu dem Kontext.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Zuordnung des Rollenkontextes zu der Funktion.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rolle</p>
        </td>
        <td>
          <p>Die zugeordnete <a href="../rollenstamm/index.md">Rolle</a> des Rollenkontextes.</p>
          <p>Rolle ist per <strong>F3</strong> aus den vorhandenen Rollen auswählbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Neue Rolle</p>
        </td>
        <td>
          <p>Zeigt an, ob die Rolle neu erstellt wird</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ist</p>
        </td>
        <td>
          <p>Status der Bedienerklasse innerhalb der zuordneten Rolle.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p><a href="../../bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md">Bedienerklasse</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll</p>
        </td>
        <td>
          <p>Der gewünschte neue Status der Bedienerklasse.</p>
          <p>Geänderte Soll-Stati im Vergleich zum Ist-Status werden farblich zur besseren Übersicht abgegrenzt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklassen-bezeichnung</p>
        </td>
        <td>
          <p>Die Bezeichnung der Bedienerklassen.</p>
          <p>Ein vorangestellter Stern (*) bedeutet das die Bedienerklasse eine Controller-Klasse ist, somit die Bedienerklasse Mitglied der <a href="../rollenstamm/index.md#Controllerklasse">Controller-Rolle</a> ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bediener</p>
        </td>
        <td>
          <p>Informatorische auf max. 255 Zeichen begrenzte Liste der Bediener der Bedienerklasse.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<details>
<summary>Funktionen des Rollenkontext Pfleger</summary>

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
          <p><strong><em>Speichern</em></strong> (<strong>F9)</strong></p>
        </td>
        <td>
          <p>Speichert ggf. Änderungen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Rollenantrag stellen</em></strong> (<strong>F10)</strong></p>
        </td>
        <td>
          <p><u>Es wird nicht gespeichert!</u><u></u></p>
          <p>Es wird ein Rollenantrag auf Änderung der Kontexte gestellt. Im Zuge der Abwicklung(*) erfolgt eine Abfrage ob sofort eine Antragsmail verschickt werden soll oder nicht. Damit bietet sich über <a href="./rollenantrag.md">Rollenantrag</a> u.a. die Möglichkeit <u>mehrere Anträge in einer Mail</u> zusammenzufassen.</p>
          <p>Somit gibt es „Antrag stellen in beiden Modi“: Einmal Änderung der Rolle und zum anderen Neu-Anlage einer Rolle mit zugehöriger Rollenklasse zur späteren Freischaltung.</p>
          <p><u>Dafür muss die Rolle aber existieren, deshalb wird eine solche Rolle dann auch hier gespeichert, aber an dieser Stelle nicht irgendwelchen Kontexten zugeordnet. Das geschieht erst im Rollenantrag.</u><u></u></p>
          <p>(*) Der Einrichterparameter <a href="../../../einrichterparameter/rollenantragsmailabfrage_epa_rollenantragsmailabfrage.md#UEB_EPA_ROLLEANTARGSMAIL_ABFRAGEN">Rollenantragsmailabfrage</a> bietet die Möglichkeit diese Abfrage abzuschalten.</p>
          <p>Sind bei Aufruf des Rollenkontext-Pflegers mehrere Kontexte ausgewählt so erfolgt bei Abarbeitung die Abfrage ob alle markierten mit der aktuellen Rolle beantragt werden sollen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>
