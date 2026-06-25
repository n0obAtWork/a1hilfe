# TSE Auswahlliste

<!-- source: https://amic.de/hilfe/_tseauswhalliste.htm -->

In der TSE-Auswahlliste, Hauptmenü > Barvorgänge > TSE Pflegen, oder Direktsprung **[TSE]**, sind alle TSE-Konfigurationen der Technischen Sicherungs-Einrichtungen zu sehen.

<details>
<summary>Felder der Auswahlliste</summary>

| Feld | Beschreibung |
| --- | --- |
| TSE-ID | Gibt die TSE-ID an. |
| Ab Datum | Gibt an, ab wann die TSE-Einstellung gelten soll.<br>Es kann zu einer gleichen TSE-ID mehrere Einträge mit verschiedenen **Ab Datum** geben.<br>In Bezug auf das aktuelle Tagesdatum ergibt sich aber logischerweise immer eine einzige TSE-Einstellung, die dann **aktiv** ist. Diese wird auch im Standard ***grün*** gekennzeichnet. (Siehe auch Aktiv-Datum)<br>Mehrere **Ab Datum** -Konfigurationen können immer dann eingesetzt werden, wenn Einstellungen vom Datum abhängen.<br>(Erzeugen kann man eine TSE-ID mit mehreren **Ab Datum** über **F5** und **Speichern unter.**) |
| Bezeichnung | Zeigt die eingetragene Bezeichnung der TSE an, dient zur leichteren Identifizierung durch den Anwender. |
| TSE-Typ | Gibt an, um welche Art von TSE es sich handelt. |
| Client ID | Gibt die Client ID an, mit dem die TSE initialisiert wurde. |
| Laufwerk | Gibt das Laufwerk an, auf dem die TSE initialisiert wurde. (Dient auch als „Fallback“ für automatisches Mapping, falls die automatische Suche die TSE nicht „auffinden“ konnte) |
| Aktueller Fundort | Zeigt den aktuellen Fundort in der Form Rechner/Laufwerk |
| Hardware-Host | Gibt an, an welchem Rechner angeschlossen war, als die TSE initialisiert wurde. |
| Manueller-Host | Gibt an, welcher Host im Falle des automatischen Mappings herangezogen werden soll, also in aller Regel ein UNC-Pfad. Zum Beispiel: **\\\\Rechnername\Freigabename** |
| Kassen… | Gibt an, an welche Kasse die TSE gebunden ist. |
| Aktiv-Datum | Zeigt zu jeder TSE-Einstellung das Datum der TSE-Einstellung an, die am aktuellen Datum aktiv ist. (siehe auch **Ab Datum**) |
| Seriennummer | Die eindeutige Seriennummer der TSE |
| A.eins TSE-Lizenznummer | Die durch A.eins zugewiesene Kennung zur TSE-Seriennummer |
| A.eins TSE-Lizenznummer gültig bis | Die der durch A.eins zugewiesene Kennung zur TSE-Seriennummer ist bis zu diesem Datum gültig. |
| Anzahl ECC-Fehler | Anzahl der ECC-Fehler. Dieser Fehler zeigt, dass Speicherbereiche im TSE-Stick defekt sind. Wenn der Wert größer als 0 ist, dann muss ein neuer TSE-Stick angefordert werden. |
| ECC-Fehler seit | Datum ab dem der erste ECC-Fehler festgestellt wurde. |

</details>

<details>
<summary>Suchmöglichkeiten der Auswahlliste</summary>

| Suchen | Beschreibung |
| --- | --- |
| TSE-ID | 0…9999999999999 |
| Ab Datum | Von… Bis… |
| Client ID | Name der TSE mit der Client Id… |
| Laufwerk | Von… Bis… (Alphabetisch) |
| Aktiv-Datum | Von… Bis… (Zeitintervall) |
| Hardware-Host | Hardware-Host |
| Manueller Host | Manueller Host |
| Seriennummer | Seriennummer der TSE |
| A.eins TSE-Lizenznummer | Von… Bis… |
| A.eins TSE-Lizenznummer gültig bis | Von… Bis… (Zeitintervall) |

</details>

<details>
<summary>Funktionen der Auswahlliste</summary>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Funktion</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ändern <strong>(F5)</strong>, Ansehen <strong>(F6)</strong>, Löschen <strong>(F7)</strong>, Neu <strong>(F8)</strong></p>
        </td>
        <td>
          <p>Ruft den Pfleger der TSE auf.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Selbsttest …</p>
        </td>
        <td>
          <p>Führt Selbsttest für ausgewählte TSE-Einträge durch.<br><br>Die Swissbit-TSE bietet die Funktion an einen Selbsttest durchzuführen. Die Swissbit-TSE besteht darauf regelmäßige Selbsttests auszuführen und stellt Funktionen zur Verfügung abzufragen, ob ein Selbsttest durchgeführt werden muss.</p>
          <p>Der Selbsttest ist eine zeitintensive Operation und kann durchaus mal eine Minute dauern.</p>
          <p>Um einen gleichzeitigen Buchungsbetrieb auszuschließen, ist es notwendig offene zugeordnete Kassen zu schließen.</p>
          <p><strong>Selbsttest bei Kasseneröffnung</strong></p>
          <p>Um zu verhindern, dass die Systeme bei den ersten Buchungen einen Selbsttest durchführen, wird im Rahmen der Kasseneröffnung ein Selbsttest durchgeführt.</p>
          <p><strong>Selbsttest im Buchungsbetrieb</strong></p>
          <p>Das Verfahren wird von A.eins im Rahmen der Buchungen ausführlich und hinreichend behandelt.</p>
          <p>Selbsttests werden nur dann ausgeführt, wenn das System signalisiert, dass es einen Selbsttest benötigt.</p>
          <p><strong>Selbsttest auf Anforderung</strong></p>
          <p>Praxiserfahrungen und die Fähigkeit, dass es technisch geht, führen zu 2 Möglichkeiten einen Selbsttest <strong><em>händisch</em></strong> durchzuführen:</p>
          <ul>
            <li>In der Anwendung <strong><em>TSE-Einstellungen</em></strong> führt die Funktion <strong><em>Ansehen </em>[F6]</strong> in den Dialog <strong>TSE-Einrichtung</strong> für die gewünschte TSE.<br>Die Funktion <strong><em>Swissbit TSE Selbsttest</em></strong> führt dann den technischen Selbsttest durch.</li>
            <li>In der Anwendung <strong><em>TSE-Einstellungen</em></strong> lassen sich entsprechende Einträge markieren und dann die Funktion <strong><em>Selbsttest</em></strong> <strong>[F10]</strong> ausführen.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Export TAR Zeitraum …</p>
        </td>
        <td>
          <p>Führt einen zeitraumbegrenzten Export der Transaktionen der TSE durch.</p>
          <p>Der Dialog <strong>TSE TAR-Export eines Zeitraumes</strong> fragt die Zeitraum-Daten der TSE gespeicherten Transaktionen ab und stellt über die Funktion <strong><em>Export erzeugen</em></strong> <strong>[F10]</strong> die Daten ins Datei-System ein.</p>
          <p>Der Export erfolgt in das fest vorgegebene Verzeichnis:<strong>\Export\TAR</strong>. Dieses wird ggf. angelegt.</p>
          <p>Die Ergebnis-Dateien sind TAR-Dateien, deren Bezeichnung zwecks leichter Wiedererkennung bzw. Identifizierung gemäß folgendem Muster gebildet werden:</p>
          <p>1.&nbsp;&nbsp; 9-stellige links mit Nullen aufgeführte Tse_id</p>
          <p>2.&nbsp;&nbsp; Trenner "<strong><em>von</em></strong>"</p>
          <p>3.&nbsp;&nbsp; Von-Zeitpunkt als Jahr, Monat, Tag, Stunde, Minute</p>
          <p>4.&nbsp;&nbsp; Trenne "<strong><em>bis</em></strong>"</p>
          <p>5.&nbsp;&nbsp; Bis-Zeitpunkt als Jahr, Monat, Tag, Stunde, Minute</p>
          <p>6.&nbsp;&nbsp; Trenner "<strong><em>SW</em></strong>" (steht für "Swissbit")</p>
          <p>7.&nbsp;&nbsp; Laufwerksbuchstabe zum Beispiel "<strong><em>T</em></strong>"</p>
          <table>
            <tbody>
              <tr>
                <th><b>Feld</b></th>
                <th><b>Beschreibung</b></th>
              </tr>
              <tr>
                <td>TSE</td>
                <td>Gibt den technischen Schlüssel bestehend aus Tse-Id und <strong>Ab Datum-</strong>Wert an.</td>
              </tr>
              <tr>
                <td>Bezeichnung</td>
                <td>Beschreibung aus dem zugehörigen Stammdatensatz.</td>
              </tr>
              <tr>
                <td>Export von</td>
                <td>Datum des Von-Zeitpunkts.</td>
              </tr>
              <tr>
                <td>um</td>
                <td>Uhrzeit in Stunden: Minuten des Von-Zeitpunkts.</td>
              </tr>
              <tr>
                <td>Export bis</td>
                <td>Datum des Bis-Zeitpunktes.</td>
              </tr>
              <tr>
                <td>um</td>
                <td>Uhrzeit in Stunden: Minuten des Bis-Zeitpunkts.</td>
              </tr>
            </tbody>
          </table>
          <table>
            <tbody>
              <tr>
                <th><b>Funktionen</b></th>
                <th><b>Beschreibung</b></th>
              </tr>
              <tr>
                <td>Export erzeugen</td>
                <td>Stellt den Export des gewünschten Zeitraums her.<br>Eine Fortschrittsanzeige in der A.eins-Nachrichtenzeile gibt Informationen zum Fortschritt aus.<br>Der Export kann je nach Netzwerk-Verbindung längere Zeit in Anspruch nehmen. Somit gilt die <i>Empfehlung</i> - sofern möglich - den Export an der Maschine zu machen, an der die TSE physikalisch angebunden ist.<br>Nach erfolgreichem Abschluss des Exportes öffnet sich automatisch der Windows-Explorer mit dem Ziel-Verzeichnis. (z. Z. .\Export\TAR)</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tse Wiederherstellen</p>
        </td>
        <td>
          <p>Löst die Wiederherstellung der ausgewählten gelöschten TSE aus.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<p class="siehe-auch">Siehe auch:</p>

- [TSE-Pfleger](./tse_pfleger.md)
