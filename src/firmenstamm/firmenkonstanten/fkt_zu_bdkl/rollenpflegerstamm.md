# Rollenpflegerstamm

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
      <thead>
        <tr>
          <th colspan="2">Felder</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Pflegerstamm</td>
          <td>Pflegerstamm</td>
        </tr>
        <tr>
          <td>Besitzer</td>
          <td>
            <p>Dieses Feld wird z.Z. nicht angezeigt.</p>
            <p>Momentan sind keine anderen Besitzer von Pflegerstämmen als AMIC vorgesehen.</p>
          </td>
        </tr>
        <tr>
          <td>Methode</td>
          <td>
            <p>Pflegerstamm-Methode</p>
            <p>Mögliche Ausprägungen sind
              <ul>
                <li>Neu</li>
                <li>Ändern</li>
                <li>Ansehen</li>
                <li>Löschen</li>
              </ul>
            </p>
          </td>
        </tr>
        <tr>
          <td>Optionbox</td>
          <td>Die Optionbox des bestimmenden Kontextes</td>
        </tr>
        <tr>
          <td>Funktion</td>
          <td>Die Funktion des bestimmenden Kontextes</td>
        </tr>
        <tr>
          <td>Rolle</td>
          <td>Die Rolle des bestimmenden Kontextes</td>
        </tr>
      </tbody>
    </table>
  </div>
</details>

<details>
  <summary>Suchmöglichkeiten Rollenpflegerstamm</summary>

  | Suchkriterien |    |
  | :------------ | :- |
  | Suchen | Sucht in den Feldern<ul><li>Pflegerstamm</li><li>Optionbox</li><li>Funktion</li><li>Rolle</li></ul> |
  | Methode | Sucht im Feld „Methode“ |
</details>

<details>
  <summary>Funktionen:</summary>

  | Funktionen |    |
  | :--------- | :- |
  | Funktion Informationen **(F9)** | Aufruf eines Informationsdialoges zur Funktion. |
  | Funktion ansehen/bearbeiten **(F11)** | Aufruf des Anwendfunktions-Pflegers. |
  | Kontext … **(F10)** | Aufruf des Optionbox-Pflegers (steht ausschließlich der Entwicklung zur Verfügung) |
</details>