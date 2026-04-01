# Rollenkontext: Pfleger

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Pflege des Rollen-Kontextes der Funktion in der Optionbox.

Ein solcher „Kontext“ beschreibt die Rechtezuordnung der Bedienerklassen an genau dieser Stelle im Programm durch die Zuweisung einer Rolle.

Ändert sich diese Zuweisung, werden also Bedienerklassen Rechte erteilt bzw. entzogen, führt das zu einer neuen Rollendefinition. Das System prüft automatisch, ob es schon eine solche Rechtekonfiguration gibt, und stellt diese zur Verfügung. Im Falle das es die gewünschte Konfiguration als Rolle noch nicht gibt wird diese vom System angelegt.

Sollen Rollen insgesamt geändert werden, also für alle Vorkommen der Rolle in allen Kontexten gleichzeitig, empfiehlt sich der Rollenpfleger.

In Umgebungen in denen spezielle Bedienerklasse für die Abarbeitung und Überwachung der Rollenänderungen eingerichtet sind lassen sich sogenannte Rollenanträge erstellen und ggf. vermailen.

<details>
  <summary>Felder des Rollenkontext Pfleger</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Felder</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Kontext</td>
          <td>Zuordnung des Rollenkontextes zu dem Kontext.</td>
        </tr>
        <tr>
          <td>Funktion</td>
          <td>Zuordnung des Rollenkontextes zu der Funktion.</td>
        </tr>
        <tr>
          <td>Rolle</td>
          <td>
            <p>Die zugeordnete Rolle des Rollenkontextes.</p>
            <p>Rolle ist per F3 aus den vorhandenen Rollen auswählbar.</p>
          </td>
        </tr>
        <tr>
          <td>Neue Rolle</td>
          <td>Zeigt an, ob die Rolle neu erstellt wird</td>
        </tr>
        <tr>
          <td>Ist</td>
          <td>Status der Bedienerklasse innerhalb der zuordneten Rolle.</td>
        </tr>
        <tr>
          <td>Bedienerklasse</td>
          <td>Bedienerklasse</td>
        </tr>
        <tr>
          <td>Soll</td>
          <td>
            <p>Der gewünschte neue Status der Bedienerklasse.</p>
            <p>Geänderte Soll-Stati im Vergleich zum Ist-Status werden farblich zur besseren Übersicht abgegrenzt.</p>
          </td>
        </tr>
        <tr>
          <td>Bedienerklassen-bezeichnung</td>
          <td>
            <p>Die Bezeichnung der Bedienerklassen.</p>
            <p>Ein vorangestellter Stern (*) bedeutet das die Bedienerklasse eine Controller-Klasse ist, somit die Bedienerklasse Mitglied der Controller-Rolle ist.</p>
          </td>
        </tr>
        <tr>
          <td>Bediener</td>
          <td>Informatorische auf max. 255 Zeichen begrenzte Liste der Bediener der Bedienerklasse.</td>
        </tr>
        <tr>
          <td></td>
          <td></td>
        </tr>
      </tbody>
    </table>
  </div>
</details>

<details>
  <summary>Funktionen des Rollenkontext Pfleger</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Funktionen</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong><em>Speichern</em> (F9)</strong></td>
          <td>Speichert ggf. Änderungen.</td>
        </tr>
        <tr>
          <td><strong><em>Rollenantrag stellen</em> (F10)</strong></td>
          <td>
            <p>Es wird nicht gespeichert!</p>
            <p>Es wird ein Rollenantrag auf Änderung der Kontexte gestellt. Im Zuge der Abwicklung(*) erfolgt eine Abfrage ob sofort eine Antragsmail verschickt werden soll oder nicht. Damit bietet sich über Rollenantrag u.a. die Möglichkeit mehrere Anträge in einer Mail zusammenzufassen.</p>
            <p>Somit gibt es „Antrag stellen in beiden Modi“: Einmal Änderung der Rolle und zum anderen Neu-Anlage einer Rolle mit zugehöriger Rollenklasse zur späteren Freischaltung.</p>
            <p>Dafür muss die Rolle aber existieren, deshalb wird eine solche Rolle dann auch hier gespeichert, aber an dieser Stelle nicht irgendwelchen Kontexten zugeordnet. Das geschieht erst im Rollenantrag.</p>
            <p>(*) Der Einrichterparameter Rollenantragsmailabfrage bietet die Möglichkeit diese Abfrage abzuschalten.</p>
            <p>Sind bei Aufruf des Rollenkontext-Pflegers mehrere Kontexte ausgewählt so erfolgt bei Abarbeitung die Abfrage ob alle markierten mit der aktuellen Rolle beantragt werden sollen.</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</details>