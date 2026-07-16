# Excel-Datei bearbeiten

<!-- source: https://amic.de/hilfe/exceldateibearbeiten1.htm -->

Hauptmenü > Preise / Konditionen > Preiskalkulation tabellarisch > Individualpreiskalkulation Excel

Direktsprung **[PKXI]**

Die Excel-Datei mit den Individualpreisen kann nun bearbeitet werden. Da es eine größere Auswahl an Feldern gibt, die bearbeitet werden können, werden diese im Folgenden aufgeschlüsselt.

### Grundfunktionen

Diese Felder können editiert werden:

| Feldname | Funktion |
| --- | --- |
| Ab Menge | Die Menge, ab der der Individualpreis gilt. Bei Veränderung des Feldes wird dem ausgewählten Individualpreis keine neue Menge zugewiesen, sondern ein neuer Individualpreis als Kopie des ausgewählten Preises mit der neuen Menge erstellt. Der ausgewählte Individualpreis mit seiner Menge bleibt bestehen.<br> |
| Preis | Der Individualpreis, der ab der Menge pro Mengeneinheit gilt. Dieser kann hier gepflegt werden.<br> |
| Je | Faktor zwischen Individualpreis und Mengeneinheit. Dieser kann hier gepflegt werden.<br> |
| Preis ab | Definiert den Beginn des Gültigkeitszeitraums für den Individualpreis. Bei Veränderung des Feldes wird dem ausgewählten Individualpreis kein neuer Beginn des Gültigkeitszeitraums zugewiesen, sondern ein neuer Individualpreis als Kopie des ausgewählten Preises mit dem neuen Datum erstellt.<br>**Wichtig**: Unabhängig davon, bei welchem Individualpreis das Feld bearbeitet wird, ist stets auch der Datensatz mit der Menge 0 mit einzubeziehen. Das Feld kann auch als Teil der Vorbelegung ([Exportprofil einrichten](./exportprofil_einrichten.md)) gepflegt werden.<br> |
| Preis bis | Definiert das Ende des Gültigkeitszeitraums für den Individualpreis. Dieses gilt für alle Mengen des Preises. Es kann hier oder als Teil der Vorbelegung ([Exportprofil einrichten](./exportprofil_einrichten.md)) gepflegt werden.<br> |
| Löschen | Ein Individualpreis kann gelöscht werden. Hierfür muss dieses Feld auf „Ja“ gesetzt werden.<br>**Wichtig**: wenn die Menge 0 gelöscht wird, werden auch alle anderen Mengen im selben Gültigkeitszeitraum gelöscht.<br> |

#### Hinweis!

Das Feld **Preis** wird in der Auswahlliste sowie beim Export in die Excel-Datei per Standard auf 0 gesetzt. Dies zeigt lediglich, dass das Feld gepflegt werden kann. Beim Import der Individualpreise werden die Zeilen, die den Preis 0 haben, einfach ignoriert.

Wenn bei Veränderung des Feldes **Preis ab** die neuen Individualpreise nicht in der Auswahlliste angezeigt werden, muss das Filterkriterium **Preisauswahl** (siehe [Exportprofil einrichten](./exportprofil_einrichten.md)) angepasst werden, um nicht ausschließlich gültige Individualpreise anzuzeigen.

### Weitere Funktionen

Grundlegend können alle gelb markierten Felder gepflegt werden. Das Verhalten beim Import kann dabei aber variieren.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Feldname</p>
        </td>
        <td colspan="2">
          <p>Funktion</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Mengeneinheitsnummer</p>
        </td>
        <td>
          <p>Nummer der Mengeneinheit, die für das Feld <b>Ab Menge</b> benutzt wird. Diese kann hier gepflegt werden, muss aber eine Ableitung der Lagermengeneinheit des Artikels sein.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Preis</p>
          <p>Mengeneinheitsnummer</p>
        </td>
        <td>
          <p>Nummer der Mengeneinheit, für die der Individualpreis berechnet wird. Diese kann gepflegt werden, muss aber eine Ableitung der Lagermengeneinheit des Artikels sein.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Kundennummer,</p>
          <p>Artikelnummer</p>
        </td>
        <td>
          <p>Die Werte Können hier gepflegt werden: Es wird eine Kopie des Individualpreis für die neue Auswahl angelegt. Der Individualpreis, der in der Excel-Datei verändert wird, bleibt so bestehen.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Lagernummer</p>
        </td>
        <td>
          <p>Der Wert kann hier gepflegt werden: Es wird eine Kopie des Individualpreis für die neue Auswahl angelegt.</p>
          <p><b>Wichtig</b>: Bei Preisveränderung wird der Preis für alle Lager immer einheitlich gesetzt.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Kennzeichen</p>
          <p>(Brutto, …, Verpackung)</p>
        </td>
        <td>
          <p>Diese Werte können hier gepflegt werden.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Sperre</p>
        </td>
        <td>
          <p>Dieser Wert kann hier gepflegt werden.</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Zudem können Excel-spezifische Funktionen auf den Individualpreisen ausgeführt werden:

- Das **Duplizieren** einer Zeile legt einen neuen Individualpreis an. Wichtig ist, dass das Feld **Ab Menge** angepasst wird, da sonst der existierende Preis für die Menge von der untersten Zeile überschrieben wird.
- Das **Einfügen** einer Zeile kann auch einen neuen Individualpreis anlegen. Die Feldwerte müssen nur entsprechend gesetzt werden.
- Das **Löschen** einer Zeile löscht nicht den Individualpreis. Um einen Individualpreis zu löschen, muss das Feld **Löschen** auf „Ja“ gesetzt werden.

Abschließend kann die Excel-Datei gespeichert und geschlossen werden. Die neuen Individualpreise können nun im nächsten Schritt importiert werden.
