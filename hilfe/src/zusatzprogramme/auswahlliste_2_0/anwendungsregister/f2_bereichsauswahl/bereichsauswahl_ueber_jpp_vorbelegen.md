# Bereichsauswahl über JPP Vorbelegen

<!-- source: https://amic.de/hilfe/_JPPANWCOND.htm -->

Wird eine Anwendung programmgesteuert (z.B. über Makro) aufgerufen, so kann man die Bereichsauswahl vorbelegen. Dazu dient das JPP-Objekt „JAnwCond“. Fett geschriebene Parameter sind Pflichtangaben.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Funktion</p>
        </td>
        <td>
          <p>Parameter</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="5">
          <p>Init</p>
        </td>
        <td>
          <p><b>Profil</b></p>
        </td>
        <td rowspan="3">
          <p>Diese Funktion muss zu Beginn aufgerufen werden. Die drei Pflicht-Parameter identifizieren die Bereichsauswahl. Wird der Parameter WithLastCond mit 1 übergeben, dann wird das angegeben Profil als Vorbelegung geladen, ansonsten wird jedes Mal die Standardeinstellung als Basis verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>CondId</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Besitzer</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>WithLastProf</p>
        </td>
        <td>
          <p>Sollen die letzten Werte dieses Profils als Basis verwendet werden? Standardeinstellung ist <b>0</b> für <b>Nein</b><b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>CondAktiv</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden, ob standardmäßig alle aktivierbaren Häkchen aus sind (Wert = 0) oder gesetzt sind (Wert = 1). Wird dieser Parameter nicht angegeben, werden die Häkchen so gesetzt, wie es in der Anwendung vorgegeben ist. Dies ist die Standardeinstellung.</p>
          <p>Die Zeilen in der Bereichsauswahl, die mit den Funktionen SetVon und SetBis angegeben werden, sind immer aktiv.</p>
        </td>
      </tr>
      <tr>
        <td rowspan="2">
          <p>SetVon</p>
        </td>
        <td>
          <p><b>Idx</b></p>
        </td>
        <td>
          <p>Der Index, wie er in der Einrichtung der Bereichsauswahl angegeben wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Von</b></p>
        </td>
        <td>
          <p>Der Wert, der in der Bereichsauswahl verwendet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td rowspan="2">
          <p>SetBis</p>
        </td>
        <td>
          <p><b>Idx</b></p>
        </td>
        <td>
          <p>Der Index, wie er in der Einrichtung der Bereichsauswahl angegeben wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Bis</b></p>
        </td>
        <td>
          <p>Der Wert, der in der Bereichsauswahl verwendet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Finit</p>
        </td>
        <td></td>
        <td>
          <p>Der Aufruf erfolgt als letztes, bevor die Anwendung aufgerufen wird. Wird das JPP-Object vorher abgeräumt (JPP_DEL), dann wird diese Funktion automatisch aufgerufen, wenn dies noch nicht geschehen ist.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Beispiel:
