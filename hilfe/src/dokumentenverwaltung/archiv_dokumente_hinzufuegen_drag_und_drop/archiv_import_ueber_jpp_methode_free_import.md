# Archiv-Import über JPP-Methode Free_Import

<!-- source: https://amic.de/hilfe/archivimportberjppmethodefreei.htm -->

Die Aufgabe der JPP-Methode Free_Import aus dem JPP-Objekt JFA_Import ist es Dateien gemäß den in [FAI einrichtbaren Import-Profilen](../archiv_import/index.md) in das Archiv zu verbringen.

Diese Methode wird von diversen Aeins-internen Applikationen aufgerufen, u.a. Mandantenserver (Profile mit Automatik-Kennzeichnung), aber auch Abwicklungen in den „Bereichen“ FAI und FAA.

Im Mandantenserver-Betrieb werden automatisch die Schalter „Protokoll“ und „Start-Abfrage“ auf „Nein“ gesetzt.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Parameter:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fai_id</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>„Schlüssel“ der Relation fa_import</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fai_pfad</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard: …</p>
          <p>Ist dieser Pfad angegeben und ungleich …, so überschreibt dieser Wert die Profil-Vorgabe fai_pfad.</p>
          <p>Unterstützt werden hier JVARS, d.h. es wird zur Laufzeit der Methode der Inhalt einer JVAR herangezogen. Ein Beispiel für die Syntax ist: @jvars(5004,userpath)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>receiver</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard: …</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>mandser</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard: FALSE</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>olderas</p>
        </td>
        <td>
          <p>Obsolete</p>
        </td>
        <td>
          <p>Versorgung über das Feld „Wartezeit in Minuten“<br>Siehe im gleichen Zusammenhang auch die nun mögliche Parametrisierung „Max. Anzahl pro Durchlauf“</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
