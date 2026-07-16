# Anlegen einer Partie über Partiestammdatenverwaltung

<!-- source: https://amic.de/hilfe/_anlegeneinerpartiebe1.htm -->

Hauptmenü > Partieverwaltung > Chargen / Partien > Partie-Stammdaten

oder Direktsprung **[PAR]**

Über die Funktion ***Neu*** **F8** wird eine neue Partie angelegt.

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
          <p>Partienummer</p>
        </td>
        <td>
          <p>Vorschlag einer automatischen Systemnummer, die überschrieben werden kann (über Mandantennummernkreis <strong>[MNDNK]</strong> wird ein Zählkreis den Partien zugeordnet)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung der Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Matchcode</p>
        </td>
        <td>
          <p>Matchcode der Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Laufzeit</p>
        </td>
        <td>
          <p>Zeitraum, in dem die Partie bebucht werden kann</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gesperrt</p>
        </td>
        <td>
          <p>Gesperrte Partien können nicht bebucht werden. Durch private Varianten ist es möglich, gesperrte Partien zu selektieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bestandsprüfung aussetzen</p>
        </td>
        <td>
          <p>Wird die Bestandsprüfung ausgesetzt, dann wird beim Zuordnen der Partie nicht überprüft ob genügend Bestand vorhanden ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Qualitaetsstatus</p>
        </td>
        <td>
          <p>Hier wird festgelegt, ob noch eine Qualitätsuntersuchung der Partie sinnvoll ist oder nicht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erledigung</p>
        </td>
        <td>
          <p>Die Partie wird nicht zur Auswahl angeboten bzw. berücksichtigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenbereich (Verkauf)</p>
        </td>
        <td>
          <p>alle: alle Kunden können aus dieser Partie beliefert werden</p>
          <p>Liste: Es kann eine Liste derjenigen Kunden aufgebaut werden die aus dieser Partie beliefert werden dürfen. Diese Eingabemaske wird per Knopf <strong><em>Kunden</em></strong> in der Optionbox geöffnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lieferantenbereich (Eink)</p>
        </td>
        <td>
          <p>alle: alle Lieferanten können dieser Partie liefern</p>
          <p>Liste: Es kann eine Liste derjenigen Lieferanten aufgebaut werden die diese Partie liefern dürfen. Diese Eingabemaske wird per Knopf <strong><em>Lieferanten</em></strong> in der Optionbox geöffnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Warengruppenbereich</p>
        </td>
        <td>
          <p>Es können Warengruppen dieser Partie zugeordnet werden, deren Artikel aus dieser Partie beliefert werden dürfen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fremdartikel zulässig</p>
        </td>
        <td>
          <p>deaktiv</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fixpreise im Verk./Eink.</p>
        </td>
        <td>
          <p>Es können Preise in den Partien hinterlegt werden, die bei der Vorgangserfassung für den Einkauf sowie den Verkauf automatisch vorgeschlagen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Währung</p>
        </td>
        <td>
          <p>Währung der Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit</p>
        </td>
        <td>
          <p>Grundmengeneinheit der für diese Partie relevanten Artikel. In der Folgemaske kann eine Artikelauswahl über alle Artikel getroffen werden die diese Grundmengeneinheit besitzen. Wenn dies nicht gewünscht ist, kann hier mit einer Grundmengeneinheit = 0 gearbeitet werden (dies verursacht eine Meldung, die ignoriert werden kann).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Minderwertigkeit-Kennzeichen(in %)</p>
        </td>
        <td>
          <p>Hier kann man minderwertige Partien in prozentualen Werten angeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archiv-Referenz</p>
        </td>
        <td>
          <p>Nummer wird automatisch beim Anlegen der Partie vorbelegt. Die Partienummer kommt in der Referenznummer vor.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Menü</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Ende</em></strong> <strong>ESC</strong></p>
        </td>
        <td>
          <p>Abbruch der Erfassung mit Abfrage (Maske wird verlassen)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Speichern</em></strong> <strong>F9</strong></p>
        </td>
        <td>
          <p>Speichern der erfassten Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Anschrift</em></strong> <strong>F4</strong></p>
        </td>
        <td>
          <p>Hinterlegung der Partieanschrift (evtl. als Hilfsmittel für Formulareinrichtung)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Bewegungen</em></strong> <strong>SF9</strong></p>
        </td>
        <td>
          <p>Übersicht der Partiebewegungen in Menge, Wert, Artikel und Belegnummern</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Artikel</em></strong> <strong>F2</strong></p>
        </td>
        <td>
          <p>Nachdem diese Partie mit <strong><em>Speichern</em></strong> <strong>F9</strong> gesichert wurde, erscheint eine weitere Funktion <strong><em>Artikel</em></strong> <strong>F2</strong>. Über diese Funktion können dieser Partie der/die Artikel zugeordnet werden. Die Vorbelegung „Typ der Zuordnung“ wird über den Steuerungsparameter 9 (siehe Steuerungsparameter [SPA] Partieverwaltung) gesteuert.</p>
          <p>Ist die Zuordnung erfolgt, können weitere Funktionen für die Artikel angewendet werden:</p>
          <ul>
            <li>Mengen: Erfassung der Sollmengen- und Werte</li>
            <li>Bewegungen: Informationen über die bereits vorhandenen Bewegungen des markierten Partieartikels</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Kunden</em></strong></p>
        </td>
        <td>
          <p>Erscheint im Menü erst, wenn im Feld Kundenbereich „Liste“ ausgewählt wurde. Öffnet die Maske Partiekunden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Lieferanten</em></strong></p>
        </td>
        <td>
          <p>Erscheint im Menü erst, wenn im Feld Lieferantenbereich „Liste“ ausgewählt wurde. Öffnet die Maske Partielieferanten.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
