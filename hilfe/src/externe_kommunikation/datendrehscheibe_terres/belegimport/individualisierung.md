# Individualisierung

<!-- source: https://amic.de/hilfe/_terres_belegimport_indivi.htm -->

#### Belege erzeugen

Zum individualisieren der Belegerzeugung können am Steuerparameter „[829](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegimport_spa_829.md)“ Makros hinterlegt werden. Diese werden zu den angegebenen Zeiten aufgerufen.

Die Makros werden mit 4 Übergabeparametern aufgerufen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong><img alt="*" src="../../../ImagesExt/image8_1558.jpg">&nbsp; Parameter</strong></p>
        </td>
        <td>
          <p><strong><img alt="*" src="../../../ImagesExt/image8_1558.jpg">&nbsp; Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; PARAM1</p>
        </td>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; Dieser Parameter enthält den Modus, durch welchen das Makro aufgerufen wurde. Mögliche Werte stehen in der folgenden Tabelle.</p>
          <img alt="*" src="../../../ImagesExt/image8_1556.jpg">
          <table>
            <tbody>
              <tr>
                <th><strong>Makrotyp</strong></th>
                <th><strong>Wert</strong></th>
              </tr>
              <tr>
                <td>MAKRO_KOPF_START</td>
                <td>KOPFSTART</td>
              </tr>
              <tr>
                <td>MAKRO_KOPF_ENDE</td>
                <td>KOPFENDE</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_START</td>
                <td>POSISTART</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_ZWISCHEN</td>
                <td>POSIZWISCHEN</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_ENDE</td>
                <td>POSIENDE</td>
              </tr>
              <tr>
                <td>MAKRO_BELEG_SPEICHERN</td>
                <td>BELEGSPEICHERN</td>
              </tr>
            </tbody>
          </table>
          <img alt="*" src="../../../ImagesExt/image8_1556.jpg">
        </td>
      </tr>
      <tr>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; PARAM2</p>
        </td>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; Dieser Parameter enthält den Namen des aktuellen „Vorgangshelper“ JPP-Objekts.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; PARAM3</p>
        </td>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; Dieser Parameter enthält den JVARS-Owner in dem die Vorgangskopfdaten liegen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; PARAM4</p>
        </td>
        <td>
          <p><img alt="*" src="../../../ImagesExt/image8_1556.jpg">&nbsp;&nbsp; Dieser Parameter enthält den JVARS-Owner in dem die Positionsdaten der aktuellen Position liegen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Die Daten für den Vorgang und die Positionen werden in JVARS zwischengespeichert. Diese können im Makro über den entsprechenden JVARS-Owner ausgelesen und geändert werden. Alternativ können über den Namen des Vorgangshelper-Objekts eigene JPP-Funktionen aufgerufen werden, um die Verarbeitung zu beeinflussen.

### Vorgangskopf JVARS

| ![\*](../../../ImagesExt/image8_1557.jpg "*") JVAR Name | ![\*](../../../ImagesExt/image8_1557.jpg "*") Beschreibung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_ID | ![\*](../../../ImagesExt/image8_1556.jpg "*") ID des Beleges. Anhand dieser Nummer kann auf die XML-Daten zugegriffen werden. |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_KundenNummer | ![\*](../../../ImagesExt/image8_1556.jpg "*") Nummer des Kunden |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Klasse | ![\*](../../../ImagesExt/image8_1556.jpg "*") Klasse des Beleges |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Unterklasse | ![\*](../../../ImagesExt/image8_1556.jpg "*") Unterklasse des Beleges |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_LagerNummer | ![\*](../../../ImagesExt/image8_1556.jpg "*") Nummer des Lagers |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_IstGutschrift | ![\*](../../../ImagesExt/image8_1556.jpg "*") Wenn ist im Originalbeleg die Summe negativ, steht dieser Wert auf 1 und die Klasse auf 1800. |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_BelegNummer | ![\*](../../../ImagesExt/image8_1556.jpg "*") Belegnummer des Ursprungsbelegs. Dieser Wert wird in die Referenznummer geschrieben. |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Periode | ![\*](../../../ImagesExt/image8_1556.jpg "*") Periode des Belegs |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_BelegDatum | ![\*](../../../ImagesExt/image8_1556.jpg "*") Datum des Belegs |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Jahr | ![\*](../../../ImagesExt/image8_1556.jpg "*") Jahr des Belegs |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_ValutaDatum | ![\*](../../../ImagesExt/image8_1556.jpg "*") Valutatdatum des Belegs |

### Position JVARS

| ![\*](../../../ImagesExt/image8_1555.jpg "*") JVAR Name | ![\*](../../../ImagesExt/image8_1555.jpg "*") Beschreibung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_PosZuAb | ![\*](../../../ImagesExt/image8_1556.jpg "*") Zu/Abschlagskennzeichen *(wird nicht verwendet)* |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Menge | ![\*](../../../ImagesExt/image8_1556.jpg "*") Menge der Position |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_Betrag | ![\*](../../../ImagesExt/image8_1556.jpg "*") Betrag der Position |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_ArtikelNr | ![\*](../../../ImagesExt/image8_1556.jpg "*") Artikelnummer der Position |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") VALUE_PosAnlegen | ![\*](../../../ImagesExt/image8_1556.jpg "*") Diese JVAR steht im Standard immer auf 1. Soll die folgende Position in dem Beleg nicht angelegt werden, so muss die JVAR auf 0 gesetzt werden. |

#### Belegimport

Der normale Belegimport importiert die Daten aus dem im Steuerparameter „[829](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegimport_spa_829.md)“ hinterlegten Pfad. Dort kann jedoch auch eine individuelle Prozedur hinterlegt werden, in der man einen eigenen Import starten kann.

Mit der folgenden Prozedur kann man eine Datei direkt ins Formulararchiv und die entsprechenden Tabellen laden. Die Datei wird dabei nicht umbenannt, das muss die private Prozedur selber machen.

```sql
Select terresImportBeleg('\\\\NetzwerkPfad\\temp\\test.xml')
```

#### Kontrollmakro

Für den Belegimport wurde ein Kontrollmakro „TERRES_ER_Kontrollmakro“ hinterlegt welches das Fibu Sperrkennzeichen setzt, wenn die Summe des Terresbeleges sich von der Summer des A.eins Beleges unterscheidet. In dem Beispiel wird das Makro nach dem Speichern des A.eins Beleges aufgerufen. In dem Beispiel darf es nur eine Rundungsdiffernz von 0.05 Euro geben.

Damit beim Aufruf des Makros bestimmt werden kann, ob es sich um eine A.eins Eingansgrechnung handelt, die aus einem Terresbeleg erzeugt wird, wird die JAVR "TERRESBELEG" gestzt. Der Owner für die JAVR ist im BAG "TERRESBELEGIDENT" gespeichert. Ist der Inhalt der JVAR eine 1, so handelt es sich um eine A.eins Eingangsrechnung welche aus einer Terresrechnung erzeugt wird. Die JAVR und der BAG werden nach dem beenden des Beleges gelöscht.
