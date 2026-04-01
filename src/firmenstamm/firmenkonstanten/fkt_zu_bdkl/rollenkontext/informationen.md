# Informationen

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Dieser Dialog zeigt je nach Funktionsart zusätzliche Informationen zu einer Funktion in den Feldern „Funktions-Detail“ und „Funktions-Information“ an.

<details>
  <summary>Funktions-Informationen:</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th colspan="2">Funktions-Informationen</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Funktion</td>
          <td>Die Identifikation der Funktion</td>
        </tr>
        <tr>
          <td>Beschriftung</td>
          <td>Textuelle Repräsentation der Funktion in der Benutzeroberfläche (Label)</td>
        </tr>
        <tr>
          <td>Reservierung</td>
          <td>Wenn was anderes als „Otto Normalbenutzer“</td>
        </tr>
        <tr>
          <td>Bezeichnung</td>
          <td>Wenn sie sich von der „Beschriftung“ unterscheidet.</td>
        </tr>
        <tr>
          <td>Funktionsart</td>
          <td>Funktionsart</td>
        </tr>
        <tr>
          <td>Controlstring</td>
          <td>Die Botschaft die bei Ausführung der Funktion an A.eins gesendet wird.</td>
        </tr>
        <tr>
          <td>Anmerkung</td>
          <td>Wenn vorhanden eine Anmerkung</td>
        </tr>
        <tr>
          <td>Pulldown</td>
          <td>Wenn es um eine Pulldown-Zuweisung der Funktion vorhanden ist.</td>
        </tr>
        <tr>
          <td>Direktsprung</td>
          <td>Wenn Direktsprünge zur Funktion vorhanden sind.</td>
        </tr>
        <tr>
          <td>Maske</td>
          <td>Wenn die Funktion einen Dialog aufruft, der Maskenname</td>
        </tr>
        <tr>
          <td>Titel</td>
          <td>Wenn die Funktion einen Dialog aufruft, der Titel der Maske</td>
        </tr>
        <tr>
          <td>Menü/Favorit</td>
          <td>Wenn die Funktion einem Menü-Favoriten zugeordnet ist.</td>
        </tr>
        <tr>
          <td>Menü-Überschrift</td>
          <td>
            <p>Wenn es bei der Funktion sich einen Hauptmenü-Eintrag handelt, dann wird hier der „Pfad“ aufgelistet.</p>
            <p>Zum Beispiel für OSQL:</p>
            <p>Administration->Werkzeuge->AMIC SQL Zugriff</p>
          </td>
        </tr>
        <tr>
          <td>Menü-Aufrufer</td>
          <td>
            <p>Gibt den „Weg“ im Haupt-Menü an, um von ganz links nach rechts zu gelangen.</p>
            <p>Zum Beispiel für OSQL:</p>
            <p>MENU_2/Firmenstamm</p>
            <p>menu_14/MENU_3_9786_AMIC</p>
            <p>Das bedeutet der Kontext MENU_2 ruft durch die Funktion „Firmenstamm“ den Kontext „menu_14“ auf, wo wiederum die Funktion „MENU_3_9786_AMIC“ den Kontext „menu_41“ aufruft in dem sich die Menü-Funktion „MENU_AMIC_SQL“ befindet.</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</details>