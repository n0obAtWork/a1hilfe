# Excel-Import: Pfleger

<!-- source: https://amic.de/hilfe/excelimportpfleger.htm -->

Stammdatenpflege > Stammdatenpfleger > Excel-Import

oder Direktsprung [**EXCELI**]

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung](../../ImagesExt/image8_1527.png "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung")

| **Feld** | |
| --- | --- |
| Name | Die eindeutige Bezeichnung des Excelimportes. Der Name dient gleichzeitig als Beschriftung der privaten Variante.<br> |

**Register „Allgemein“**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><b>Feld</b></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Speicherort</p>
        </td>
        <td>
          <p>Der Pfad der Excel-Datei. Mit der <strong>F3</strong>-Taste öffnet sich der Datei-Explorer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Blatt-Name</p>
        </td>
        <td>
          <p>Der Name des Excel-Arbeitsblattes, das nach A.eins importiert werden soll. Der Blatt-Name wird mit „Tabelle1“ vorbelegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anwendung</p>
        </td>
        <td>
          <p>Hier wird der Name der Anwendung angegeben, unter der sich die zu erstellende Variante befinden soll. Mithilfe der <strong>F3</strong>-Taste kann eine Anwendung ausgewählt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Offset Zeile</p>
        </td>
        <td>
          <p>Mit dem „Offset Zeile“ wird die Anzahl an Zeilen angegeben, die beim Import übersprungen werden. Befinden sich die Überschriften in dem Excel-Blatt nicht in der ersten Zeile, so kann hier ein alternativer Wert eingetragen werden. Handelt es sich bei der ersten Zeile des Arbeitsblattes um die Überschriftenzeile, so ist hier eine „0“ einzutragen.</p>
          <p>Der Standardwert ist „0“.</p>
          <p>Beispiel:</p>
          <p>Befinden sich die Spaltenüberschriften in Zeile „3“, so ist hier ein Offset von „2“ einzutragen. Die ersten zwei Zeilen werden übersprungen.</p>
          <p><img src="../../ImagesExt/image8_1528.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Offset Spalte</p>
        </td>
        <td>
          <p>Mit dem „Offset Spalte“ wird die Anzahl an Spalten angegeben, die beim Import übersprungen werden. Soll der Import nicht ab der ersten Spalte erfolgen, so kann hier ein Wert ungleich „0“ eingetragen werden. Ein Wert von „0“ bedeutet, dass der Import ab der ersten Spalte erfolgen soll.</p>
          <p>Der Standardwert ist „0“.</p>
          <p>Beispiel:</p>
          <p>Beginnen die Daten in Spalte „3“ („C“), so ist hier ein Offset von „2“ anzugeben. Damit werden die ersten beiden Spalten übersprungen.</p>
          <p><img src="../../ImagesExt/image8_1529.png" alt=""></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Spalten als Text</p>
        </td>
        <td>
          <p>Standardmäßig hängt der Datentyp der Datenbankfelder von dem Format der Excel-Spalten ab (siehe <a href="./uebernahme_eines_excel_arbeitsblattes_in_eine_private_varian/umschluesselungen_excel_zu_aeins.md">Umschlüsselungen Excel zu Aeins</a>). Dieses Verhalten lässt sich abschalten, indem das Feld „Spalten als Text“ auf „Ja“ gestellt wird. Dann werden beim Excelimport alle Datenbankfelder als „long varchar“ angelegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datenbankprozedur</p>
        </td>
        <td>
          <p>Hier kann eine private Datenbankprozedur angegeben werden. Über diese Prozedur kann zeilenweise ein Datensatz aus der beim Excelimport angelegten Tabelle ausgelesen und weiterverarbeitet werden. Wird hier eine Prozedur eingetragen, so wird diese beim Excelimport immer nach dem Einfügen eines Datensatzes in diese Tabelle ausgeführt.</p>
          <p>Die Prozedur muss über folgende Parameter verfügen:</p>
          <table>
            <tbody>
              <tr>
                <th>Name</th>
                <th>Datentyp</th>
                <th></th>
              </tr>
              <tr>
                <td>in_xlsident</td>
                <td>integer</td>
                <td>Wert des Feldes „xlsident“ (Primärschlüssel)</td>
              </tr>
              <tr>
                <td>in_tablename</td>
                <td>char(128)</td>
                <td>Name der angelegten Tabelle</td>
              </tr>
            </tbody>
          </table>
          <p>In dem Feld „Datenbankprozedur“ kann mithilfe von <strong>F3 </strong>eine bereits bestehende Prozedur ausgewählt werden oder es kann ein neuer Name eingetragen werden. Diese neue Prozedur wird sofort zur Bearbeitung geöffnet und hat folgenden Aufbau:</p>
          <div>
            <pre><code>CREATE PROCEDURE p_exceli_test (in in_xlsident integer, in in_tablename char(128))
 BEGIN
 --todo: Aktion(en)
EXCEPTION
   when others then
     call amic_exception( ERRORMSG() || CHAR(10) || CHAR(13) || TRACEBACK(), SQLCODE , SQLSTATE , 'p_exceli_test' , -1 );
 END</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>

.

**Register „Sonstiges“**

Unter dem Register „Sonstiges“ werden technische Informationen zu der Variante und ihren Objekten angezeigt.

| **Feld** | |
| --- | --- |
| Tabelle | Name der Relation, in der die Daten des Excel-Arbeitsblattes gespeichert werden.<br> |
| Variante | Identifikation der Variante.<br> |
| Besitzer | Besitzer der Variante. Der Besitzer ist immer „Privat“.<br> |
| Variantentext | Identifikation des „SQL-Textes“ der Variante.<br> |
| Bereich/Profil | Identifikation der Bereichsauswahl der Variante.<br> |
| Optionbox | Identifikation des Funktionsmenüs der Variante.<br> |

| **Funktionen** | |
| --- | --- |
| Speichern **[F9]** | Beim Speichern wird – sofern nicht schon geschehen – eine private Variante erstellt. Hierbei wird noch kein Excelimport durchgeführt, sondern es wird eine „[Standardvariante](./uebernahme_eines_excel_arbeitsblattes_in_eine_private_varian/private_variante.md)“ angelegt.<br> |
| Datei öffnen | Mithilfe dieser Funktion kann die Exceldatei von hier aus direkt bearbeitet werden.<br> |
| Datenbankprozedur bearbeiten<br> | Über diese Funktion kann die private Datenbankprozedur bearbeitet werden. |
