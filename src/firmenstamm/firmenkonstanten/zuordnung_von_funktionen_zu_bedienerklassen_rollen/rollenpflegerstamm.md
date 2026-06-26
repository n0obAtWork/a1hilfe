# Rollenpflegerstamm

<!-- source: https://amic.de/hilfe/rollenpflegerstamm.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rollenpflegerstamm

oder

Hauptmenü > Stammdatenpflege > Stammdatenpfleger > Zugriffsrechte Rollenpflegerstamm

oder Direktsprung **[ROPST]**

Die Zugriffsrechte der jeweiligen Pfleger ergeben sich aus den Zugriffsrechten jeweiliger dedizierter Kontexte, den sogenannten „bestimmenden Kontexten“ oder auch der „Pfleger-Rollenbindung“.

Wird zur Laufzeit ein Pfleger angefordert, so entscheidet der zugeordnete Kontext, ob eine Anforderung erlaubt ist.

Besteht keine Autorisierung durch den bestimmenden Rollenkontext wird das Laufzeitsystem den Anwender informieren und zwecks administrativer Unterstützung eine Warnung ins Fehlerprotokoll eingestellt.

<details>
<summary>Felder des Rollenpflegerstamm</summary>

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
          <p>Pflegerstamm</p>
        </td>
        <td>
          <p><a href="../../../zusatzprogramme/pflegerstamm.md">Pflegerstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besitzer</p>
        </td>
        <td>
          <p>Dieses Feld wird z.Z. nicht angezeigt.</p>
          <p>Momentan sind keine anderen Besitzer von Pflegerstämmen als AMIC vorgesehen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Methode</p>
        </td>
        <td>
          <p>Pflegerstamm-Methode</p>
          <p>Mögliche Ausprägungen sind</p>
          <ul>
            <li>Neu</li>
            <li>Ändern</li>
            <li>Ansehen</li>
            <li>Löschen</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Optionbox</p>
        </td>
        <td>
          <p>Die Optionbox des bestimmenden Kontextes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Die Funktion des bestimmenden Kontextes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rolle</p>
        </td>
        <td>
          <p>Die Rolle des bestimmenden Kontextes</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<details>
<summary>Suchmöglichkeiten Rollenpflegerstamm</summary>

| Suchkriterien | |
| --- | --- |
| Suchen | Sucht in den Feldern<br><ul><li>Pflegerstamm</li><li>Optionbox</li><li>Funktion</li><li>Rolle</li></ul> |
| Methode | Sucht im Feld „Methode“ |

</details>

<details>
<summary>Funktionen:</summary>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Funktionen</strong></p>
        </td>
        <td colspan="2"></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Funktion Informationen (<b>F9</b>)<b></b></p>
        </td>
        <td>
          <p>Aufruf eines <a href="./rollenkontext/rollenkontext_pfleger.md">Informationsdialoges zur Funktion</a>.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Funktion ansehen/bearbeiten (<b>F11</b>)</p>
        </td>
        <td>
          <p>Aufruf des Anwendfunktions-Pflegers.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Kontext … (<b>F10</b>)</p>
        </td>
        <td>
          <p>Aufruf des Optionbox-Pflegers (steht ausschließlich der Entwicklung zur Verfügung)</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

</details>
