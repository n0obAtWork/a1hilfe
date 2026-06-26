# Rezepturen

<!-- source: https://amic.de/hilfe/_rezepturenrez.htm -->

Hauptmenü > Produktion / Abwicklung > Produktion Stammdaten > Rezepturen

oder Direktsprung **[REZ]**

Hier sind die eigentlichen Rezepturen einzugeben. Es wird unterschieden zwischen „Rezepturen“ und „Handelsstücklisten“. In der Auswahlliste ist für den hier diskutierten Fall „Rezeptur“ zu wählen.

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
          <p>Rezepturgruppe</p>
        </td>
        <td>
          <p>Rezepturgruppe zu der die Rezeptur gehören soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rezepturnummer</p>
        </td>
        <td>
          <p>Nummer der Rezeptur</p>
          <p>Die Rezepturnummer darf nicht größer als 32767 sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung (Rezeptur)</p>
        </td>
        <td>
          <p>Name der Rezeptur</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gültigkeit</p>
        </td>
        <td>
          <p>Zeitraum der Verwendung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gesperrt</p>
        </td>
        <td>
          <p>Darf (evtl. vorübergehend) nicht verwendet werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verwendungstyp</p>
        </td>
        <td>
          <p>Gibt an, wo die vorliegende Rezeptur verwendet werden kann:</p>
          <ul>
            <li>Ausschließlich in der Produktion (0 – Produktion)</li>
            <li>Ausschließlich in der Vermahlung (3 - Vermahlung)</li>
            <li>In allen Ausprägungen (1 - Alle)<br>Dieser Verwendungstyp steht nur noch im Ändern-Fall zur Auswahl, wenn eine Rezeptur mit diesem Typ angelegt wurde. Im Neu-Fall wird dieser Verwendungstyp ab sofort nicht mehr angeboten.</li>
            <li>Ausschließlich in der NzuM-Produktion (4 - NzuM-Produktion)</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anteile: Typ</p>
        </td>
        <td>
          <p>Bei der Rezepturerfassung wird gegen die hier formulierten Werte geprüft. Mit Prüfung heißt, dass eine Verprobung zur rechts dazu einzugebenden Summe erfolgt.</p>
          <ul>
            <li>Prozent mit Prüfung: prüft Prozentsumme</li>
            <li>Prozent ohne Prüfung: keine Prozentprüfung</li>
            <li>je ME ohne Prüfung: Stückliste ohne Mengenprüfung</li>
            <li>je ME mit Prüfung: Stückliste mit Mengenprüfung</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anteile: Summe</p>
        </td>
        <td>
          <p>Falls Anteile geprüft werden sollen, muss hier die Prüfsumme eingetragen werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rezeptgröße</p>
        </td>
        <td>
          <p>Diese Rezeptgröße beschreibt die produzierte Menge des Rezeptes.</p>
          <p>Werden zum Beispiel drei Komponenten zu gleichen Anteilen zusammen gemischt, kann für jede Komponente die Menge 1kg&nbsp;und die produzierte Menge 3kg angegeben werden. So werden periodische Kommazahlen bei den Mengenangaben vermieden.</p>
          <p>Es wird empfohlen, die Rezeptgröße 0 nicht zu verwenden, da es sonst zu Problemen und unerwarteten Ergebnissen bei der Verwendung der Rezeptur kommen kann.</p>
          <p>Für die NzuM-Produktion erscheint hinter diesem Feld ein Feld auf das sich die Rezeptgröße bezieht. Die Auswahl im Feld Anteile: Typ muss dafür auf ‚je ME ohne Prüfung‘ oder auf ‚je ME mit Prüfung‘ stehen</p>
          <p>Zur Auswahl steht:</p>
          <p>0 - nur Hauptprodukt<br>&nbsp;&nbsp;&nbsp;&nbsp; die Rezeptgröße bezieht sich nur auf das Hauptprodukt</p>
          <p>1 - Summe Produkte<br>&nbsp;&nbsp;&nbsp;&nbsp; die Rezeptgröße bezieht sich auf alle Produkte</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit</p>
        </td>
        <td>
          <p>Zur Berechnung Produkt-Komponente<br>% oder Stückangaben beziehen sich auf 1 ME Rezept (wichtig bei Pauschal-Komponenten!)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bewertungstyp</p>
        </td>
        <td>
          <ul>
            <li>0 - ohne Verprobung: Wert Produkt ungleich Wert Komponenten möglich</li>
            <li>1 - Komponenten addieren: Wert Komponenten bestimmen Wert Produkt</li>
            <li>2 - Produkt anteilgewichtet: Produktwert wird nach (Mengen) Anteilen auf Komponenten verteilt</li>
            <li>3 - Produkt wertgewichtet: Produktwert wird nach Anteilswert auf Komponenten verteilt</li>
            <li>4 - Produkt Preisminderung</li>
          </ul>
          <p>Bei der NzuM-Produktion stehen Auswahl 3 und 4 nicht zur Verfügung.</p>
          <p>„Ohne Verprobung“ übernimmt die Preisfindung des Produktes; Komponentenpreise werden nicht berücksichtigt. Die Verknüpfung zwischen Produkt und Komponenten ist für die Preisfindung also ohne Bedeutung.</p>
          <p>Bei der „Komponenten-Addition“ ergibt sich der Produktpreis aus der Addition der Komponenten. In diesem Fall spielt die Preisfindung des Hauptartikels keine Rolle.</p>
          <p>Die folgenden Fälle gehen davon aus, dass Hauptartikel und Summe der Komponenten aufgehen muss. Die Preisfindung (automatische Preisfindung oder manuelle Preiseingabe) findet auf dem Hauptartikel statt. Wenn die Summe der Komponenten aus deren Preisermittlung nicht mit dem Hauptartikel übereinstimmt, dann erfolgt eine wertgewichtete Verminderung oder Erhöhung auf der Komponentenseite nach folgenden Methoden:</p>
          <p>Bei „Produkt anteilgewichtet“ wird der Betrag einer Komponente aus dem Produktbetrag ermittelt. Als Verteilschlüssel dient das Verhältnis der Komponentenanteile untereinander. Das jeweilige Verhältnis multipliziert mit dem Produktgesamtbetrag ergibt den Komponentenbetrag. Der Einzelpreis einer Komponente ergibt sich dann mittels Division durch die Menge. Achtung: Auf Grund der Rundung und Speicherung auf / von 2-Nachkommastellen kann es zu Rundungsdifferenzen kommen!</p>
          <p>Bei „Produkt wertgewichtet“ wird der Preis einer Komponente aus dem Produktpreis ermittelt. Als Verteilschlüssel dient das Verhältnis der Komponentenpreise zum Zeitpunkt vor der Erfassung – je nach eingestelltem Verfahren zur Bewertung der Komponenten.</p>
          <p>Das jeweilige Verhältnis multipliziert mit dem Produktpreis ergibt dann den neuen Komponentenpreis. Dieser Preis multipliziert mit der Menge ergibt dann den Betrag der Komponente. In diesem Fall kann es zu Rundungsdifferenzen bei den Beträgen kommen. Um die Identität zwischen Summe der Komponentenbeträge und Produktbetrag sicher zu stellen, wird dieser dann um diese Differenz erhöht.</p>
          <p><u>Hinweis</u><u></u></p>
          <p>Manuell geänderte Preise werden bei gewichteten Verteilungen nicht berücksichtigt.<br>Sind alle Preise manuell verändert, kann nichts mehr verteilt werden.<br>Der Beleg geht evtl. dann nicht mehr zu Null auf.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Individuell</p>
        </td>
        <td>
          <p>Dieses Feld zeigt bei „Ja“ die Felder Planmenge, Erledigt, Disponiert, Offen auf der Maske an.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Variable Komponenten</p>
        </td>
        <td>
          <p>Dieses Feld ist nur sichtbar, wenn der Bewertungstyp auf „ohne Verprobung“ oder „Komponenten addieren“ steht. Stellt man das Feld für diese beiden Fälle auf Ja, dann besteht die Möglichkeit Komponenten bei Verwendung der Rezeptur variabel zu gestalten, also welche auszutauschen oder weitere hinzuzufügen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Währung</p>
        </td>
        <td>
          <p>Dieses Feld legt die zu verwendende Währung fest.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Komponentenläger</p>
        </td>
        <td>
          <p>Vorbelegung: nicht änderbar</p>
          <p>Steht dieses Feld auf ‚änderbar‘, ist auf der Maske für die Zusammensetzung das Feld Lager editierbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Voreinstellung des Mengenkontrollschalters:</p>
        </td>
        <td>
          <p>Die Vorbelegung des Häkchen-Feldes ‚Mengenkontrolle zwischen Produkt und Komponenten aktiv‘ auf der Komponentenmaske kann hier im Rezept getrennt nach Erfassung und Korrektur entschieden werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>bei Beleg-Erfassung</p>
        </td>
        <td>
          <p>Zur Auswahl steht für die Erfassung von Belegen:</p>
          <p>0 - laut Masken-EPA</p>
          <p>1 - mit Mengenkontrolle</p>
          <p>2 - ohne Mengenkontrolle</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>bei Beleg-Korrektur</p>
        </td>
        <td>
          <p>Zur Auswahl steht für die Korrektur von Belegen:</p>
          <p>0 - laut Masken-EPA</p>
          <p>1 - mit Mengenkontrolle</p>
          <p>2 - ohne Mengenkontrolle</p>
          <p>3 - letzte Vorgangseinstellung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preisnachkommastellen</p>
        </td>
        <td>
          <p>Nur für die Verwendungstypen Produktion oder Vermahlung eingebbar.<br>Es können maximal 6 Nachkommastellen verwendet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gegenzeilenausgleich</p>
        </td>
        <td>
          <p>Nur für den Verwendungstype Vermahlung eingebbar.<br>Wird hier „ja“ eingetragen, so wird die Wertdifferenz zwischen Produkt und Komponenten mit der <b>1. Gegenposition</b> ausgeglichen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Produktpartie anteilig</p>
        </td>
        <td>
          <p>Komponentenpartien mit Produkt anteilig bebuchen</p>
          <p>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Nein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>korrekte Bewertung</p>
        </td>
        <td>
          <p>Umrechnung von Preiseinheiten der Bewertung auf korrekte Weise.</p>
          <p>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Ja.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preise aus Tabellen</p>
        </td>
        <td>
          <p>Bewertungspreise mit Einheiten richtig übernehmen.</p>
          <p>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Ja.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Auflösungstyp</p>
        </td>
        <td>
          <p>Bei den Verwendungstypen Produktion, NzuM-Produktion und Vermahlung wird dieses Feld mit dem Wert „sichtbar“ gefüllt und ist auf der Maske für den Kunden nicht sichtbar.</p>
          <p>Typangaben beziehen sich auf die Verwendung der Rezepturmechanik für Handels-Stücklisten, werden also im Bereich Produktion nicht ausgewertet:</p>
          <ul>
            <li>manuell: Produktion manuell</li>
            <li>verdeckt: Stücklistenauflösung verdeckt</li>
            <li>sichtbar: Stücklistenauflösung sichtbar</li>
            <li>Umbuch verdeckt: belegbez. Umbuchung verdeckt</li>
            <li>Umbuch sichtbar: belegbez. Umbuchung sichtbar</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Buchungstyp</p>
        </td>
        <td>
          <p>Bei den Verwendungstypen Produktion, NzuM-Produktion und Vermahlung wird dieses Feld mit dem Wert „beide Seiten“ vorbelegt und ist auf der Maske für den Kunden nicht sichtbar.</p>
          <p>Nur für eine Handelsstückliste kann man sich entscheiden, ob das Produkt oder seine Komponenten gebucht werden sollen</p>
          <p>0 – beide Seiten</p>
          <p>1 – Komponenten</p>
          <p>2 - Produkte</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Produktpreise F11](./produktpreise_f11.md)
- [Zusammensetzung F10](./zusammensetzung_f10.md)
