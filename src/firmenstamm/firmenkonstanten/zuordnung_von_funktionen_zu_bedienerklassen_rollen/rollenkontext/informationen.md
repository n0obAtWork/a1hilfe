# Informationen

<!-- source: https://amic.de/hilfe/_rollenkontext_info.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenkontext

oder Direktsprung **[ROLLE]**

oder

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Funktionen

oder Direktsprung **[ZUGF]**

Dieser Dialog zeigt je nach [Funktionsart](./funktionsarten.md) zusätzliche Informationen zu einer Funktion in den Feldern „Funktions-Detail“ und „Funktions-Information“ an.

<details>
<summary>Funktions-Informationen:</summary>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Funktions-Informationen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Die Identifikation der Funktion</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beschriftung</p>
        </td>
        <td>
          <p>Textuelle Repräsentation der Funktion in der Benutzeroberfläche (Label)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Reservierung</p>
        </td>
        <td>
          <p>Wenn was anderes als „Otto Normalbenutzer“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Wenn sie sich von der „Beschriftung“ unterscheidet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktionsart</p>
        </td>
        <td>
          <p><a href="./funktionsarten.md">Funktionsart</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Controlstring</p>
        </td>
        <td>
          <p>Die Botschaft die bei Ausführung der Funktion an A.eins gesendet wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anmerkung</p>
        </td>
        <td>
          <p>Wenn vorhanden eine Anmerkung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pulldown</p>
        </td>
        <td>
          <p>Wenn es um eine Pulldown-Zuweisung der Funktion vorhanden ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Direktsprung</p>
        </td>
        <td>
          <p>Wenn Direktsprünge zur Funktion vorhanden sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Maske</p>
        </td>
        <td>
          <p>Wenn die Funktion einen Dialog aufruft, der Maskenname</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Titel</p>
        </td>
        <td>
          <p>Wenn die Funktion einen Dialog aufruft, der Titel der Maske</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menü/Favorit</p>
        </td>
        <td>
          <p>Wenn die Funktion einem Menü-Favoriten zugeordnet ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menü-Überschrift</p>
        </td>
        <td>
          <p>Wenn es bei der Funktion sich einen Hauptmenü-Eintrag handelt, dann wird hier der „Pfad“ aufgelistet.</p>
          <p>Zum Beispiel für OSQL:</p>
          <p>Administration-&gt;Werkzeuge-&gt;AMIC SQL Zugriff</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menü-Aufrufer</p>
        </td>
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
