# Mengeneinheiten mit Gebinde

<!-- source: https://amic.de/hilfe/_mengeneinheitenmitge.htm -->

Wenn Kartoffeln im 50 kg Sack verkauft werden und die Grundmengeneinheit "kg" ist, die Bestände in "kg" geführt werden, der Preis sich jedoch auf den "Sack" bezieht, dann muss die Gebindeumrechnung aktiviert werden. Hierzu werden in der Eingabemaske folgende Daten erfasst.

Die Maske ist in folgende Bereiche aufgeteilt:

[Kopfdaten](./index.md#Mengeneinheiten_mit_Gebinde_Kopfdaten)

[Tabreiter – Allgemein](./index.md#Mengeneinheiten_mit_Gebinde_Tabreiter_al)

Tabreiter – Zusatz

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Kopfdaten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>Nummer der zu definierenden Mengeneinheit.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurztext</p>
        </td>
        <td>
          <p>Kurztext der Mengeneinheit (z. B. für Ausdrucke), z.B. "Sack".</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Langtext</p>
        </td>
        <td>
          <p>Langtext der Mengeneinheit (z.B. für Ausdrucke)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Ausführliche Bezeichnung der Mengeneinheit, z. B. für Auswahllisten. In diesem Fall z.B. "Sack 50 kg".</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Grundmengeneinheit</p>
        </td>
        <td>
          <p>Nummer der Grundeinheit, auf die zurückgerechnet werden soll, z.B. „kg“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ergebniseinheit</p>
        </td>
        <td>
          <p>Diejenige Mengeneinheit, in der das Ergebnis der Gebindeberechnung zurückgegeben wird.</p>
          <p>Beispiel:</p>
          <p>Es wird eine Palette mit Dosen à x Liter bearbeitet, dann ist das Ergebnis der Gebindeberechnung "Liter";</p>
          <p>Handelt es sich um eine Palette mit Säcken à x kg, so kommen kg dabei heraus.</p>
          <p>Das Ergebnis eines Volumengebindes sind dann z.B. Liter oder m³ sein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./index.md">Gebindetyp</a></p>
        </td>
        <td>
          <p>Hier ist eine Angabe erforderlich, wenn ein Gebinde abgerechnet werden soll:</p>
          <p>1&nbsp;&nbsp;&nbsp; lineares Gebinde (Anzahl)</p>
          <p>2&nbsp;&nbsp;&nbsp; Gebinde 2. Stufe (Fläche)</p>
          <p>3&nbsp;&nbsp;&nbsp; Gebinde 3. Stufe (Volumen)</p>
          <p>4&nbsp;&nbsp;&nbsp; Addition (Gebi1 + Gebi2)</p>
          <p>5&nbsp;&nbsp;&nbsp; Subtraktion (Geb1 - Geb2)</p>
          <p>6&nbsp;&nbsp;&nbsp; Faktor1 * Faktor2 / Faktor3</p>
          <p>7&nbsp;&nbsp;&nbsp; Faktor1 * Faktor2 * Faktor3 * Faktor4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./gebindetypen.md">Faktorherkunft</a></p>
        </td>
        <td>
          <p>Kennzeichnung, woher die Gebinde-Faktoren für die Berechnung kommen. Es ist hier ein dreistufiges System implementiert, es können bei den Artikeln, beim Artikelstamm aber auch beim Gebinde selbst die Faktoren hinterlegt werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### Tabreiter

Hier ist eine Auflistung der einzelnen Felder auf den Tabreitern der Maske.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Allgemeine Informationen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde auf Stufe 2 eingebbar</p>
        </td>
        <td>
          <p>Dieses Kennzeichen steuert, ob es bei der (Gebinde-) Mengeneinheit zulässig sein soll, die Gebindeanzahl&nbsp;statt auf der obersten auf eingebbar der zweiten Stufe einzugeben.</p>
          <p>Beispiel:</p>
          <p>"Karton à 8 Kanister à 5 Liter"</p>
          <p>Eingabe auf Ebene 1: Anzahl Kartons</p>
          <p>Eingabe auf Ebene 2: Anzahl Kanister</p>
          <p>Eingabe auf unterster Ebene: Anzahl Liter</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Effektive Menge eingebbar</p>
        </td>
        <td>
          <p>Dieses Kennzeichen steuert, ob es bei der (Gebinde-) Mengeneinheit zulässig sein soll, die effektive Menge statt der Gebindeanzahl einzugeben</p>
          <p>Beispiel:</p>
          <p>Karton à 8 Kanister à 5 Liter</p>
          <p>Eingabe auf Ebene 1: Anzahl Kartons</p>
          <p>Eingabe auf Ebene 2: Anzahl Kanister</p>
          <p>Eingabe auf unterster Ebene: Anzahl Liter</p>
          <p>Die Eingabe der "Litermenge" ergibt dann als Ergebnis die Anzahl der Kartons.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anbruch Gebinde Behandlung</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Normal</li>
            <li>Anbruch</li>
            <li>Abrunden</li>
            <li>Aufrunden</li>
            <li>Aufrunden St.2</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anzahlermittlung angebrochene Gebinde</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Mitzählen</li>
            <li>Nicht mitzählen</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preis unabhängig von Faktoren</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Ja</li>
            <li>Nein</li>
          </ul>
          <p>JA = Preis pro Gebindeanzahl</p>
          <p>NEIN = Preis pro Ergebnismenge</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bei Mengeneingabe ersten Faktor errechnen</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Ja</li>
            <li>Nein</li>
          </ul>
          <p>Bei der Angabe von „Ja“ wird nach der Ergebnismengeneingabe der Faktor 1 neu bestimmt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bei Gebinde-Typ 8 Menge mal Faktor 3</p>
        </td>
        <td>
          <p>Spezialeinstellung.</p>
          <p>F3 Auswahl</p>
          <ul>
            <li>Ja</li>
            <li>Nein</li>
          </ul>
          <p>Bei der Angabe von „Ja“ wird der Faktor 3 bei der Ergebnismengeneingabe ignoriert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zwischenergebnis auf Gebinde-Maske</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Nein</li>
            <li>Anzeigen</li>
            <li>Eingebbar</li>
          </ul>
          <p>Steuert die Anzeige / Eingabe von Zwischenergebnissen auf der Gebinde-Maske.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rundungsstellen bei Umrechnung Menge/Gebinde</p>
        </td>
        <td>
          <p>Anzahl der Nachkommastellen die beim Runden berücksichtigt werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ohne Autofenster</p>
        </td>
        <td>
          <p>F3 Auswahl</p>
          <ul>
            <li>Ja</li>
            <li>Nein</li>
          </ul>
          <p>Steuert die Anzeige des Automatikfensters in der Vorgangsverarbeitung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit Preisbezug</p>
        </td>
        <td>
          <p>Es kann vorkommen, dass bei der Fakturierung die im Gebinde berechnete Endmenge nicht mit der tatsächlichen Menge des Gebindes übereinstimmt. Soll nun der Preis auf die tatsächlich gemessene Menge bezogen werden, so kann zusätzlich zum Mengenfeld noch ein weiteres Feld auf der Warenerfassungsmaske geöffnet werden, welches dann für die Preisberechnung herangezogen wird. Wird an dieser Stelle nun eine Mengeneinheit bestimmt, die so einen Preisbezug festlegt, dann wird das zusätzliche Feld abgefragt.</p>
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
          <p><strong>Gebindefaktoren</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde Faktor 1- 4</p>
        </td>
        <td>
          <p>Hier sind die Gebinde Faktoren einzutragen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Einheit</p>
        </td>
        <td>
          <p>Hier kann eine Mengeneinheit eingetragen werden. Diese steht im Zusammenhang mit der <a href="../../../../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/gebindebearbeitung.md#GebindebearbPreismengenbezug">Preismengenbezugsübernahme</a> bei der Erfassung eines Artikels mit Gebinde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde Text</p>
        </td>
        <td>
          <p>Kurzbezeichnung der Gebindeeinheit für Ausdrucke etc. in Standardsprache.</p>
          <p>Im Beispiel soll nach Auflösung des Gebindes der Text "kg" erscheinen; er ist hier einzutragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde änderbar</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Statistikkennzeichen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p><a href="./faktorherkunft.md">LVS Relevant</a></p>
        </td>
        <td>
          <p>Dieses Kennzeichen steuert ob die Mengeneinheit LVS-relevant ist</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LVS Default Lokalität</p>
        </td>
        <td>
          <p>Hier wird die default LVS Lokalität eingetragen, wenn das Gebinde LVS-relevant ist, wird diese Lokalität standardmäßig verwendet</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Gebindetypen](./gebindetypen.md)
- [Faktorherkunft](./faktorherkunft.md)
- [LVS-relevant](./lvs_relevant.md)
