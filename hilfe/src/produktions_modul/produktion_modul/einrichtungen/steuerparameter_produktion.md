# Steuerparameter (Produktion)

<!-- source: https://amic.de/hilfe/_steuerparameterprodu.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

oder Direktsprung **[SPA]**

Die einzurichtenden Steuerparameter findet man indem man in der Auswahlliste die Bereichsauswahl auf die [Gruppe Rezeptur/Stückliste/Produktion](../../../firmenstamm/steuerparameter/rezeptur_stueckliste_produktion/index.md) abgrenzt.

![](../../../ImagesExt/image8_265.jpg)

Zunächst sind die Steuerparameter **[SPA]** einzurichten:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Steuerparameter</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 666:<br>Positionsumbuchung Mengenbehandlung</p>
        </td>
        <td>
          <p>Berechnung des Komponentenanteils in der Maske Positionskalkulation. 0 bedeutet der Anteil wird wie in der Komponente angegeben berechnet. 1 bedeutet Menge wird wie im Rezept angegeben berechnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 28:<br>Stücklistenverwaltung angeschlossen</p>
        </td>
        <td>
          <p>Steuert das Programmverhalten bei Artikeln mit hinterlegten Rezepturen. Die Rezepturen werden nur aufgelöst, wenn hier „Ja“ eingetragen ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 302:<br>Komponentenlagerwahl für Produktion</p>
        </td>
        <td>
          <p>Legt fest, aus welchem Lager die Komponenten für eine Rezeptur genommen werden.</p>
          <p>Das Lager für die Komponenten ist in der Regel gleich Lager des Produktes, hier ist nur bei speziellen Einrichtungen anderes einzustellen.</p>
          <p>0 = wie Zugangslager</p>
          <p>1 = wie im Rezept hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 321:<br>Komponentendaten auf Produktionsmaske unveränderbar</p>
        </td>
        <td>
          <p>Falls im Produktionsmodul keine Komponentendaten während der Erfassung verändert werden dürfen ist hier „Ja“ einzustellen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 322:<br>Korrektursperre bei Importdaten</p>
        </td>
        <td>
          <p>Werden Daten aus einem vorgelagerten Produktionssystem importiert, sollen sie möglicherweise (i.d.R.) nicht mehr korrigiert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 458:<br>Lagerplatzverwaltung auch bei Produktion</p>
        </td>
        <td>
          <p>Wenn die Lagerplatzverwaltung aktiviert ist, sollen ggf. auch die Buchungen für Produkt, Komponente und Rezeptur lagerplatzbezogen erfolgen. Dies ist hier dann zu aktivieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 309:<br>Rezeptur-Definition aus Vorgangsbearbeit</p>
        </td>
        <td>
          <p>Die Rezepturdefinition aus der Belegerfassung heraus ist z.Z. noch nicht aktiv; der Parameter zieht noch nicht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 492:</p>
          <p>Nur eine Artikelgruppe in Rezeptur?</p>
        </td>
        <td>
          <p>Artikel können Artikelgruppen zugeordnet werden. Mit diesem Parameter ist es nun möglich zu erreichen, dass nur Artikel einer Artikelgruppe in der Rezeptur enthalten sein dürfen.</p>
          <p>JA: Es dürfen in einer Rezeptur nur Artikel aus einer Artikelgruppe genommen werden.</p>
          <p>Nein: Es dürfen in einer Rezeptur Artikel aus mehreren Artikelgruppen genommen werden.</p>
          <p>Der Preis des Produktes einer Stückliste kann sich nach dem im Rezept eingestellten Verfahren oder aus der Preisfindung für diesen Artikel ergeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 285+286:<br>Bewertung im Produktionsmodul /<br>Pr.Klasse für Komp.+Prod. in Produktion</p>
        </td>
        <td>
          <p>Hier wird das Bewertungsverfahren zur Ermittlung von Zu- und Abgangswerten eingestellt. Entscheidet man sich für den letzten, durchschnittlichen (der Periode, des Jahres) oder gewogenen Einkaufspreis bei Parameter 285, ist der Eintrag unter 286 bedeutungslos. Wird dort Listenpreis eingestellt, ist unter 286 die Listenpreisnummer (Lispreisnummer Einkauf) einzustellen. Folgende Bewertungsverfahren stehen zur Verfügung:</p>
          <p>1 = Gew. EKP</p>
          <p>2 = Letzter EKP</p>
          <p>3 = Durchschn. EKP der Periode</p>
          <p>4 = Listenpreis</p>
          <p>5 = Durchschn. EKP des Jahres</p>
          <p>6 = laut Artikelbewertung</p>
          <p>100 = per Preisfindung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 621:<br>Produktion mit Partiepreisfindung</p>
        </td>
        <td>
          <p>Bei „Ja“ wird bei der Preisfindung der Komponenten zunächst geprüft, ob ein Partiepreis vorhanden ist. Falls ja, wird dieser genommen, sonst findet die übliche Preisfindung der Produktion statt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 688:<br>Produktion Bewertung korrekte Preisumrechnung</p>
        </td>
        <td>
          <p>Dieser Steuerparameter dient als Vorbelegung im Pfleger für Rezepturen für das Feld: Korrekte Bewertung.</p>
          <p>Wird im Rezept dort&nbsp; „Ja“ eingestellt, wird eine korrigierte Fassung der Umrechnung der Bewertungspreise&nbsp; aktiviert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 689:<br>Produktion Preiseinheiten aus Tabellen</p>
        </td>
        <td>
          <p>Dieser Steuerparameter dient als Vorbelegung&nbsp; im Pfleger für Rezepturen für das Feld: Preise aus Tabellen.</p>
          <p>Wird im Rezept dort&nbsp; „Ja“ eingestellt, werden&nbsp; alle Preise, die nicht aus Bewertungsmethoden kommen, mit ihren zugehörigen Preiseinheiten und Preismengeneinheiten übernommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 491:<br>Nur n Produktionszugänge pro Vorgang, 0=beliebig?</p>
        </td>
        <td>
          <p>Legt fest, ob mehr als eine Produktionsbuchung pro Beleg erfasst werden kann.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 409+410+411+412:<br>Produktion<br>Partiezuordnungszwang + Partiemengenausgleichszwang</p>
        </td>
        <td>
          <p>Regeln Buchungszwänge in Verbindung mit der Partiebuchhaltung. Generell gilt bei Eintragung „Nein“, dass Partiezuordnungen nicht erzwungen werden, freie Zuordnungen bei der Erfassung aber möglich sind. Für die Komponenten kann der Zuordnungszwang (409) für alle Artikel oder für die, denen im Artikelstamm das Kennzeichen Partiezwang bzw. Saatgutartikel zugeordnet wurde, eingestellt werden. Der Parameter 410 regelt dann, ob die Mengen Partien vollständig zuzuordnen sind. O.a. Ausführungen gelten entsprechend für das Produkt (411,412): Partiezuordnung für alle Artikel oder aber die mit Partiezwang / bzw. Saatgutartikel gekennzeichneten; Produktionsmengen müssen vollständig zugeordnet werden (oder nicht).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 415:</p>
          <p>Preis aus Partie übernehmen</p>
        </td>
        <td>
          <p>Preise aus Partien werden übernommen, wenn der Preis ungleich 0 ist und in der Rezeptur der Bewertungstyp nicht „anteilsgewichtet“ oder „wertgewichtet“ ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 600:<br>Prod. Gebindefakt. Warenposition verwend</p>
        </td>
        <td>
          <p>Wird dieser Parameter auf „Ja“ gesetzt, dann werden für neue Belege manuell abgeänderte Gebindefaktoren berücksichtigt. Alte Belege bleiben davon unberührt. Es wird die Einstellung „Ja“ empfohlen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 471:<br>Mengennormalisierung nur in % Rezepturen</p>
        </td>
        <td>
          <p>Mengennormalisierung: Nur relevant für % Rezepturen; hier sollte der Parameter auf „Ja“ geschaltet sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Parameter 962:</p>
          <p>Produktions-Schnellerfassung aktiv</p>
        </td>
        <td>
          <p>Aktiviert&nbsp; das Modul zur Produktions-Schnellerfassung, wenn hier „Ja“ eingetragen ist. Die Standardeinstellung ist „Nein“.</p>
          <p>ACHTUNG: Die Schnellerfassung verfügt nicht über den vollen Leistungsumfang, wie er im Standard-Produktionsmodul mittels der Direktsprünge PROB und PROE zur Verfügung steht.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
