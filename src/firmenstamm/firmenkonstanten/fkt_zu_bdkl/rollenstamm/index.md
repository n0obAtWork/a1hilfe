# Rollenstamm
 
Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenstamm

oder Direktsprung **[ROLLE]**

Es ergibt sich für jeden Kontext „Listen“ von Bedienerklassen, die die Funktion ausführen dürfen. Eine Rolle ist ein Synonym für eine solche Liste. Es ist eine bis zu 255 Zeichen umfassende freiwählbare Zeichenkette, die innerhalb der Rollen eindeutig sein muss.

Es ist möglich für jeden Kontext eine eigene Rolle anzulegen, da aber für den Großteil der Funktionen sich in der Praxis gleiche Rollen ergeben, wird man sich für diese auf eine Rolle einigen wollen.

<details>
  <summary>Felder des Rollenstamm</summary>

  | Felder | Beschreibung |
  | :----- | :----------- |
  | Rolle | <p>Eindeutiger Bezeichner für eine Rolle. Bis zu 255 Zeichen.</p><p>Die Bezeichner sind nach der Erstinitialisierung technisch anmutend durchnummeriert: R000R, R001R, R002R, … usw. Diese Bezeichnung hat den Vorteil in anderen Auswahllisten leichter auffindbar zu sein.</p><p>Es gibt eine nicht durch ihren Namen, der ist ebenso frei wählbar, aber durch ihre Funktionalität ausgezeichnete Rolle: Die sogenannte Controller-Rolle. Diese Controller-Rolle ist die Rolle, die neuen Kontexten -also Funktionen, die zu Kontexten hinzugefügt werden oder auch per Neuanlage oder Update ins System kommen- zugeordnet werden. Das System unterbindet das Löschen dieser Controller-Rolle bzw. stellt auch ggf. sicher das eine solche Controller-Rolle existiert, falls von Nöten. Eine Controller-Rolle muss es geben, um neue Funktionalitäten zunächst einmal nur eine dafür vorgesehenen Bedienerschaft zugänglich zu machen.</p> |
  | Anzahl Bedienerklassen | <p>Informatorische Anzahl der zugeordneten Bedienerklassen zu dieser Rolle.</p><p>Es kann Rollen ohne Bedienerklassen geben. Ordnet man einem Kontext eine solche Rolle zu darf kein Anwender die betreffende Funktion in dem Kontext ausführen.</p> |
  | Anzahl Kontexte | <p>Informatorische Anzahl der zugeordneten Kontexte zu dieser Rolle.</p><p>Je nach Einsatzgebiet der Rolle kann diese Anzahl 18.000+ oder auch nur sehr wenige bis gar keine Kontexte umfassen.</p> |
  | Bedienerklassen dürfen | <p>Informatorische Schutzstring-Auflistung der „positiven“ Bedienerklassen.</p><p>Die Darstellung ist gerafft, d.d. aufeinanderfolgende Bedienerklassen sind zusammengefasst.</p><p>Aber Achtung: 1-3 bedeutet die Bedienerklassen 1, 2 und 3 – wenn es denn überhaupt eine Bedienerklasse 2 gibt. Sonst eben nur die Bedienerklassen 1 und 3. Aber das sollte keine wirkliche Einschränkung darstellen.</p> |
  | Bedienerklassen dürfen nicht | <p>Informatorische Schutzstring der Bedienerklassen die nicht der Rolle zugeordnet sind.</p><p>Rundet die Übersicht in Kombination mit „Bedienerklassen dürfen“ ab: Je nach Fragestellung kann man sich in der passenden Umgebung schnell orientieren.</p> |
</details>


<details>
  <summary>Suchfunktionen des Rollenstamm</summary>

  |    | Suchkriterien |
  | :- | :------------ |
  | Rolle | Suchbegriff |
</details>

<details>
  <summary>Funktionen des Rollenstamm</summary>

  |    | Funktionen |
  | :- | :--------- |
  | Neu **(F8)** | <p>Anlage einer neuen Rolle.</p><p>Für Details siehe Rollenpfleger.</p> |
  | Ändern **(F5)** | <p>Ändern einer Rolle</p><p>Als Besonderheit beinhaltet diese Funktion auch das „Umbenennen einer Rolle, sowie über „Speichern unter“ das Erzeugen einer neuen Rolle mit Vorgabe der Bedienerklassenzuordnung der Vorlage.</p><p>Für Details siehe Rollenpfleger.</p> |
  | Ansehen **(F6)** | <p>Ansehen der Bedienerklassenzuordnung.</p><p>Für Details siehe Rollenpfleger.</p> |
  | Löschen **(F7)** | <p>Löscht eine Rolle.</p><p>Allerdings nicht die Controller-Rolle und ebenso keine Rolle die zugeordnete Rollenkontexte hat!</p><p>Für Details siehe Rollenpfleger.</p> |
  | Rollen tauschen **(F9)** | <p>Hiermit lassen sich von zwei Rollen die Bedienerklassenzuordnung und/oder die Rollenkontexte austauschen.</p><p>Für Details siehe „Rollen tauschen“.</p> |
  | Rollen vereinigen **(Shift+F9)** | <p>Vereinigt ausgewählte Rolle in einer Ziel-Rolle. Die Ziel-Rolle kann dabei entweder eine neue oder eine der beteiligten Rollen sein. Nach der Operation besitzt die Ziel-Rolle die Vereinigung aller Bedienerklassen der beteiligten Rollen und alle betroffenen Rollenkontexte der involvierten Rollen sind der Zielrolle zugeordnet.</p><p>Für Details siehe „Rollen vereinigen“.</p> |
  | Rollenkontexte **(F10)** | Ruft die Anwendung „Diese Funktionen“ auf, die eine Spezialisierung einer Variante der Rollenkontexte ist und nur die Mitglieder dieser Rolle anzeigt. |
</details>

## Alternatives

<details>
  <summary>Funktionen des Rollenstamm</summary>

  Neu *(F8)*
  : <p>Anlage einer neuen Rolle.</p>
    <p>Für Details siehe Rollenpfleger.</p>
  
  Ändern *(F5)*
  : <p>Ändern einer Rolle</p>
    <p>Als Besonderheit beinhaltet diese Funktion auch das „Umbenennen einer Rolle, sowie über „Speichern unter“ das Erzeugen einer neuen Rolle mit Vorgabe der Bedienerklassenzuordnung der Vorlage.</p>
    <p>Für Details siehe Rollenpfleger.</p>
  
  Ansehen *(F6)*
  : <p>Ansehen der Bedienerklassenzuordnung.</p>
    <p>Für Details siehe Rollenpfleger.</p>

  Löschen *(F7)*
  : <p>Löscht eine Rolle.</p>
    <p>Allerdings nicht die Controller-Rolle und ebenso keine Rolle die zugeordnete Rollenkontexte hat!</p>
    <p>Für Details siehe Rollenpfleger.</p>
  
  Rollen tauschen *(F9)*
  : <p>Hiermit lassen sich von zwei Rollen die Bedienerklassenzuordnung und/oder die Rollenkontexte austauschen.</p>
    <p>Für Details siehe „Rollen tauschen“.</p>

  Rollen vereinigen *(Shift+F9)*
  : <p>Vereinigt ausgewählte Rolle in einer Ziel-Rolle. Die Ziel-Rolle kann dabei entweder eine neue oder eine der beteiligten Rollen sein. Nach der Operation besitzt die Ziel-Rolle die Vereinigung aller Bedienerklassen der  beteiligten Rollen und alle betroffenen Rollenkontexte der involvierten Rollen sind der Zielrolle zugeordnet.</p>
    <p>Für Details siehe „Rollen vereinigen“.</p>
  
  Rollenkontexte *(F10)*
  : Ruft die Anwendung „Diese Funktionen“ auf, die eine Spezialisierung einer Variante der Rollenkontexte ist und nur die Mitglieder dieser Rolle anzeigt.
</details>

<details>
  <summary>Funktionen des Rollenstamm - with html table</summary>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <!-- this is a reason to choose html-table because markdown can't do col-/rowspan -->
          <th colspan="2">Funktionen</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Neu <strong>(F8)</strong></td>
          <td>
            <p>Anlage einer neuen Rolle.</p>
            <p>Für Details siehe Rollenpfleger.</p>
          </td>
        </tr>
        <tr>
          <td>Ändern <strong>(F5)</strong></td>
          <td>
            <p>Ändern einer Rolle</p>
            <p>Als Besonderheit beinhaltet diese Funktion auch das „Umbenennen einer Rolle, sowie über „Speichern unter“ das Erzeugen einer neuen Rolle mit Vorgabe der Bedienerklassenzuordnung der Vorlage.</p>
            <p>Für Details siehe Rollenpfleger.</p>
          </td>
        </tr>
        <tr>
          <td>Ansehen <strong>(F6)</strong></td>
          <td>
            <p>Ansehen der Bedienerklassenzuordnung.</p>
            <p>Für Details siehe Rollenpfleger.</p>
          </td>
        </tr>
        <tr>
          <td>Löschen <strong>(F7)</strong></td>
          <td>
            <p>Löscht eine Rolle.</p>
            <p>Allerdings nicht die Controller-Rolle und ebenso keine Rolle die zugeordnete Rollenkontexte hat!</p>
            <p>Für Details siehe Rollenpfleger.</p>
          </td>
        </tr>
        <tr>
          <td>Rollen tauschen <strong>(F9)</strong></td>
          <td>
            <p>Hiermit lassen sich von zwei Rollen die Bedienerklassenzuordnung und/oder die Rollenkontexte austauschen.</p>
            <p>Für Details siehe „Rollen tauschen“.</p>
          </td>
        </tr>
        <tr>
          <td>Rollen vereinigen <strong>(Shift+F9)</strong></td>
          <td>
            <p>Vereinigt ausgewählte Rolle in einer Ziel-Rolle. Die Ziel-Rolle kann dabei entweder eine neue oder eine der beteiligten Rollen sein. Nach der Operation besitzt die Ziel-Rolle die Vereinigung aller Bedienerklassen der beteiligten Rollen und alle betroffenen Rollenkontexte der involvierten Rollen sind der Zielrolle zugeordnet.</p>
            <p>Für Details siehe „Rollen vereinigen“.</p>
          </td>
        </tr>
        <tr>
          <td>Rollenkontexte <strong>(F10)</strong></td>
          <td>Ruft die Anwendung „Diese Funktionen“ auf, die eine Spezialisierung einer Variante der Rollenkontexte ist und nur die Mitglieder dieser Rolle anzeigt.</td>
        </tr>
      </tbody>
    </table>
  </div>
</details>

<p class="siehe-auch">Siehe auch:</p>

- [Rollenstamm: Pfleger](pfleger.md)
- [Rollen tauschen](rollen_tauschen.md)
- [Rollen vereinigen](rollen_vereinigen.md)
