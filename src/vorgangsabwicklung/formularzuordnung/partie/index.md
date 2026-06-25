# Partie

<!-- source: https://amic.de/hilfe/_frz_register_partie.htm -->

Es gibt eine Reihe von Einstellungen, die jetzt nicht mehr wie früher unter SPA vorgenommen werden, sondern speziell für Vorgangsunterklassen hinterlegt werden.

    
HINWEIS:

*Man achte bitte darauf, dass alle relevanten Unterklassen bezüglich ihrer Partieeinstellungen überprüft werden.*

Folgende Felder stehen ihnen auf dieser Registerkarte zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Alternative Itembox</p>
        </td>
        <td>
          <p>Hier kann eine alternative Itembox zur Auswahl von Partien hinterlegt werden. Diese bietet mehr Flexibilität bezüglich der unterschiedlichen Unterklassen.</p>
          <p>Wir raten immer ausgehend von der Standard Itembox auf&nbsp; die korrekten Angaben der Returnwerte zu achten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./datenbankfunktion_fuer_verteilung.md">DB-Prozedur für Verteilung</a></p>
        </td>
        <td>
          <p>Hier kann eine private Prozedur für die Verteilung eingetragen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./datenbankfunktion_fuer_gebindeparameter.md">DB-Funktion für Gebindeparameter</a></p>
        </td>
        <td>
          <p>Hier kann eine private Prozedur für die Gebindeparameter eingetragen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./datenbankprozedur_fuer_neuanlage.md">DB-Prozedur für Neuanlage</a></p>
        </td>
        <td>
          <p>Hier kann eine private Prozedur für die Neuanlage eingetragen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DB-Prozedur für MDE Sperre</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>DB-Funktion Partie zulassen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Partiefenster automatisch</p>
        </td>
        <td>
          <p>Diese Einstellung gibt es nur bei Umbuchungen. Sie legt fest, wie das Partiefenster sich zu verhalten hat.</p>
          <table>
            <tbody>
              <tr>
                <th><b>Nein</b></th>
                <th>Das Partiefenster öffnet sich nicht automatisch.</th>
              </tr>
              <tr>
                <td><b>Ja</b></td>
                <td>Das Partiefenster öffnet sich immer automatisch.</td>
              </tr>
              <tr>
                <td><b>Nur bei Partiezwang</b></td>
                <td>Das Partiefenster öffnet sich nur, wenn im Artikel Partiezwang eingestellt ist.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Übernahme von Preisen</p>
        </td>
        <td>
          <p>Die Einstellung regelt das Verhalten für die Preisübernahme bei Partien, bei denen Preise hinterlegt wurden. Da es pro Warenposition nur einen Preis gibt, muss hier festgelegt werden, wie die Preisfindung zustande kommt. Derzeit gibt es folgende Varianten:</p>
          <table>
            <tbody>
              <tr>
                <th><b>erster Preis</b><br><b></b>&nbsp;</th>
                <th>der Preis der ersten bepreisten Partie wird übernommen (nach Reihenfolge in der Zuordnungsmaske)</th>
              </tr>
              <tr>
                <td><b>größter Preis</b></td>
                <td></td>
              </tr>
              <tr>
                <td><b>kleinster Preis</b></td>
                <td></td>
              </tr>
              <tr>
                <td><b>nur eine Partie mit Preisen</b></td>
                <td>es darf nur eine bepreiste Partie zugeordnet werden</td>
              </tr>
              <tr>
                <td><b>keine Preisfindung</b></td>
                <td>Preise werden ignoriert</td>
              </tr>
              <tr>
                <td><b>Partie nicht nehmen</b></td>
                <td>Partien mit Preisen dürfen nicht gezogen werden</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatische Verteilung</p>
        </td>
        <td>
          <p>Die Einstellung liefert nach der&nbsp; Mengenangabe einen Partievorschlag.</p>
          <p>Hierbei werden zwei unterschiedliche Verfahren berücksichtigt:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Verkauf</b></th>
                <th>Partien mit positiven Bestand werden der Reihe nach (ältere Partien zuerst) zugeordnet, bis die gewünschte Menge aus der Warenposition erreicht wird.</th>
              </tr>
              <tr>
                <td><b>Einkauf</b></td>
                <td>Es wird die jüngste Partie unabhängig vom Bestand&nbsp; vorgeschlagen</td>
              </tr>
              <tr>
                <td><b>Nur bei Partiezwang</b></td>
                <td>Das Partiefenster öffnet sich nur, wenn im Artikel Partiezwang eingestellt ist.</td>
              </tr>
            </tbody>
          </table>
          <p><u>ACHTUNG</u><i>:<br>Wie schon in früheren Versionen kann man in der Itembox der Artikelauswahl eine „PartieId“ übergeben. Es wird nur diese Partie vorgeschlagen!</i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatische Partie aus Belegnummer</p>
        </td>
        <td>
          <p>Dieser Schalter erzwingt eine automatische Partieerzeugung mit der aktuellen Belegnummer. Sämtliche auf diesem Beleg erfassten Artikel werden automatisch in die Partie aufgenommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel hinzufügen erlaubt</p>
        </td>
        <td>
          <p>Mit dieser Einstellung schaltet man frei, ob Partien um nicht eingetragene Artikel ergänzt werden dürfen. Vorausgesetzt wird aber zusätzlich, dass in der Partie das Kennzeichen „Fremdartikelzulässig“ gesetzt ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bestandsprüfung</p>
        </td>
        <td>
          <p>Für die Bestandsprüfung sind folgende Werte zulässig.</p>
          <table>
            <tbody>
              <tr>
                <th><b>keine Prüfung</b></th>
                <th>der Partiebestand darf auch negativ werden</th>
              </tr>
              <tr>
                <td><b>nur Hinweis</b></td>
                <td>nach der Partiezuordnung wird lediglich&nbsp; auf&nbsp; Negativbestände hingewiesen. Man darf die Zuordnung abschließen</td>
              </tr>
              <tr>
                <td><b>Abbruch</b></td>
                <td>bei Bestandunterschreitung ist ein Abspeichern der Zuordnung nicht möglich!</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Behandlungskennzeichen auswerten</p>
        </td>
        <td>
          <p>Legt fest, ob das Kennzeichen „Partiefenster automatisch“ ausgewertet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Prod.Partie aus Komp. 1</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Maschinentagebuch führen</p>
        </td>
        <td>
          <p>Mit dieser Einstellung kann gesteuert werden, ob ein Tagebuch geführt werden soll.</p>
          <p>Soll das Tagebuch geführt werden, kann in einem Maschinentagebuch eingetragen werden, welche Partie bei einer Produktion in welche Partie überführt werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Private Tagebuchfunktion</p>
        </td>
        <td>
          <p>Hier kann eine private Datenbankfunktion hinterlegt werden, die <u>zusätzlich</u> nach der allgemeinen Tagebuchfunktion aufgerufen wird.</p>
          <p>In dieser können dann abweichende Verarbeitungen ausgelöst werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Itembox prüft hart</p>
        </td>
        <td>
          <p>Hier kann man festlegen, ob bei der Partieauswahl nur die Partien die durch die Itembox vorgeschlagen werden ausgewählt werden können (Ja) oder ob man auch eine beliebige andere Partie wählen oder sogar eine neue Partie mit F8 anlegen und auswählen kann (Nein)</p>
          <p>.<b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Korrektursperre</p>
        </td>
        <td>
          <p>Mit dieser Option kann für eine Vorgangsunterklasse die Korrektur von Partiezuordnungen geregelt werden. Dafür stehen in dem Feld „Korrektursperre“ die folgenden Einstellungsmöglichkeiten zur Auswahl:<br><br></p>
          <table>
            <tbody>
              <tr>
                <th><b>Nein</b></th>
                <th>Korrekturen in der Partiezuordnung sind generell erlaubt</th>
              </tr>
              <tr>
                <td><b>Hinweis</b></td>
                <td>Bei der Korrektur einer Partiezuordnung wird eine Warnmeldung ausgegeben; die Korrektur wird aber akzeptiert.</td>
              </tr>
              <tr>
                <td><b>Fehlerprotokoll und Hinweis</b></td>
                <td>Zusätzlich zum Hinweis auf dem Bildschirm wird ein Eintrag im Fehlerprotokoll erstellt</td>
              </tr>
              <tr>
                <td><b>Gesperrt</b></td>
                <td>Korrekturen in der Partiezuordnung sind nicht möglich</td>
              </tr>
            </tbody>
          </table>
          <p>Grundsätzlich wird bei der <i>Erst</i>erfassung eines Beleges niemals eine Prüfung vollzogen. Bei der Korrektur eines Beleges werden folgenden Aktionen geprüft:</p>
          <p>Änderung der Partie, der Partieverteilung oder der Mengenverteilung</p>
          <p>Löschen einer Warenpositionszeile mit Partie</p>
          <p>Ferner wird immer eine Prüfung beim Löschen oder und Stornieren von Vorgängen, sowie bei der Erstellung von Stornobelegen ausgeführt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verteilmaske Posteil</p>
        </td>
        <td>
          <p>Bei dieser Option werden zwei Einstellungen angeboten:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Nein</b></th>
                <th>Der Wechsel von Positionsteil zurück zur Hauptmaske erfolgt wie gehabt (ohne zusätzliche Anzeige)</th>
              </tr>
              <tr>
                <td><b>Immer</b></td>
                <td>Beim Wechsel vom Positionsteil zurück in die Hauptmaske des Vorganges wird automatisch die Partieverteilungsmaske aufgeblendet</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gruppenauswertung ignorieren</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Immer Verteilungsprozedur</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Neuerfassung erlaubt</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Vererbung Qualität und Addon</p>
        </td>
        <td>
          <p>Im Schalter können folgende Einstellungen vorgenommen werden:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Keine Vererbung</b></th>
                <th>Es findet keine Vererbung statt.</th>
              </tr>
              <tr>
                <td><b>Qualität und Addon</b></td>
                <td>Siehe Einstellung „Vererbung Qualität“ und „Vererbung Addon“.</td>
              </tr>
              <tr>
                <td><b>Vererbung Qualität</b></td>
                <td>Im Update-Fall werden die Verweise der Ziel-Partie getilgt die weder ein Untersuchungsdatum noch eine Satzart haben.</td>
              </tr>
              <tr>
                <td><b>Vererbung Addon</b></td>
                <td>Das Addon wird jeweils vererbt.</td>
              </tr>
            </tbody>
          </table>
          <p>In allen Klassen, bei denen Gegenzeilenverbuchungen mit Partie auftreten wird dann entsprechend reagiert. Diese sind:</p>
          <ul>
            <li>Produktion mit maximal einer unterschiedlichen Partie im Produkt und maximal einer unterschiedlichen Partie im Komponentenbereich</li>
            <li>Vermahlung mit maximal einer unterschiedlichen Partie in den Herkunftsbuchungszeilen und maximal einer unterschiedlichen Partie in den Zielbuchungszeilen.</li>
            <li>Lagerumbuchung mit maximal einer unterschiedlichen Partie im Abgang und maximal einer unterschiedlichen Partie im Zugang</li>
            <li>Artikelumbuchung mit maximal einer unterschiedlichen Partie im Abgang und maximal einer unterschiedlichen Partie im Zugang</li>
            <li>Lagerplatzumbuchung mit genau einer unterschiedlichen Partie im Abgang und genau einer unterschiedlichen Partie im Zugang</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tagebuchmengeneinheit</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Export im Tagebuch</p>
        </td>
        <td>
          <p>Wird hier <b>Ja </b>eingetragen, so wird für Vorgänge(Maschinentagebuch) und Kontrakte eine Historie in die Tabelle VorgangAlsXML geschrieben. Für Kontrakte muss zusätzlich wurde auf dem Reiter „Allgemein“ eine Prozedur unter „Kontraktexportprozedur“ ein Prozedur eingetragen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partielaufzeit prüfen</p>
        </td>
        <td>
          <p>Mit dieser Einstellung wird festgelegt, ob bei der Partieauswahl die Partielaufzeit geprüft werden soll.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Datenbankfunktion für Verteilung](./datenbankfunktion_fuer_verteilung.md)
- [Datenbankfunktion für Gebindeparameter](./datenbankfunktion_fuer_gebindeparameter.md)
- [Datenbankprozedur für Neuanlage](./datenbankprozedur_fuer_neuanlage.md)
- [Beispiele für Datenbankfunktionen](./beispiele_fuer_datenbankfunktionen.md)
