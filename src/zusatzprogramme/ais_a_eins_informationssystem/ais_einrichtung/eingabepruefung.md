# Eingabeprüfung

<!-- source: https://amic.de/hilfe/eingabeprfung.htm -->

Hauptmenü > Administration > Werkzeuge > Informationssystem > Register Eingabeprüfung

Direktsprung **[AIS]**

![](../../../ImagesExt/image8_1022.jpg)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Eingabe erforderlich</p>
        </td>
        <td>
          <p>Wenn erzwungen werden soll, dass in das Feld ein Wert eingetragen werden soll, dann trägt man hier ein <b>Ja</b> ein. Man kann dann dieses Feld nur verlassen, wenn es Daten enthält bzw. es wird vor dem Speichern geprüft, ob es Daten enthält.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>nicht Löschen</p>
        </td>
        <td>
          <p>Dies bedeutet, dass nach dem Speichern dieses Feld nicht gelöscht wird, sondern der vorher eingegebene Inhalt erhalten bleibt. Auch springt der Cursor nicht wieder in dieses Feld, sondern in das erste Feld, in dem bei „nicht Löschen“ ein <b>Nein</b> steht.<b><u></u></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Itembox</p>
        </td>
        <td>
          <p>Will man die Möglichkeit schaffen, dass die Werte, die in dem Feld eingegeben werden können aus einer Liste von Werten auswählt werden können, so hat kann man hier eine Itembox angeben, die auf eine Tabelle verweist. Eine Liste der Itemboxen erhält man mit <b>F3</b>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Itembox eindeutig</p>
        </td>
        <td>
          <p>Steht hier ein <b>Ja</b>, so muss der eingegebene Wert in den Daten der Itembox vorhanden sein. Bei <b>Nein</b> dienen die Werte nur als Vorschlag und es können auch Werte erfasst werden, die nicht in der Itembox vorhanden sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Itembox Information</p>
        </td>
        <td>
          <p>Häufig gibt es zusätzliche Informationen zu Feldern, die sich auf andere Relationen beziehen. Eine der häufigsten Informationen, die man sehen will ist die Bezeichnung, die einem bestimmten Wert zugeordnet ist. Diese Information kann man hier erhalten. Dabei muss man das Feld angeben, wie es in der Itembox in der Returnliste steht, gefolgt von „&gt;“ und dem Maskenfeld. Beispiel:</p>
          <div>
            <code>LKW_Bezeich&gt;LKWTEXT</code>
          </div>
          <p>Das Maskenfeld LKWTEXT muss natürlich auch angelegt werden bzw. auf der Maske existieren.</p>
          <p>Man könnte auch noch mehr Informationen aus der Itembox herauslesen. Dazu kann man, mit Komma getrennt, weitere Felder in der obigen Syntax angeben. Also:</p>
          <div>
            <code>LKW_Bezeich&gt;LKWTEXT,LKW_MATCH&gt;MATCH,....</code>
          </div>
          <p>Alle Felder, die aus der Relation gelesen werden, müssen in der Returnliste der Itembox stehen. Siehe dazu Dokumentation Itembox.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Entry-Funktion,<br>Exit-Funktion und<br>Validation-Funktion</p>
        </td>
        <td>
          <p>Diese drei Funktionen dienen zur Steuerung bzw. zur Eingabeprüfung des Feldes. Sie haben alle denselben Aufbau. Es sind Funktionen innerhalb des Makros, das man bei der Einrichtung der Gruppe angegeben hat. Die Funktion muss fünf Parameter mit folgender Bedeutung haben:</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Parameter</strong></th>
                <th><strong>Beschreibung</strong></th>
              </tr>
              <tr>
                <td>1. string</td>
                <td>Maskenname</td>
              </tr>
              <tr>
                <td>2. integer</td>
                <td>Nummer des Feldes auf der Maske</td>
              </tr>
              <tr>
                <td>3. string</td>
                <td>Feldinhalt</td>
              </tr>
              <tr>
                <td>4. integer</td>
                <td>Zeile, falls das Feld ein Array ist</td>
              </tr>
              <tr>
                <td>5. integer</td>
                <td>Status. Je nach Art des Aufrufs enthält es verschiedene Werte. Siehe Panther–Dokumentation.</td>
              </tr>
            </tbody>
          </table>
          <p>Wenn innerhalb des Makros Funktionen mit diesem Aufbau existieren, so ist es möglich diese mit <b>F3</b> auszuwählen.</p>
          <div>
            <pre><code>function EineEntryFunktion(aa:string ; bb :
      integer;a:string ; b,c : integer ):integer;
begin
 EineEntryFunktion:=0;
end;</code></pre>
          </div>
          <p>Die Validation-Funktion unterscheidet sich dadurch von den anderen, dass der Rückgabewert ausgewertet wird. Ein Wert ungleich 0 bewirkt, dass das Feld nicht verlassen werden kann.</p>
          <p><strong>Hinweis:</strong></p>
          <p>Wird ein Makro 2.0 (C#) als Ffeldmakro angegeben, so entfällt die Angabe einer Funktion – Der Funktionsname ergibt sich aus dem AISMakro-Interface.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
