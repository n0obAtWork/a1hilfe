# Zielaufruf

<!-- source: https://amic.de/hilfe/zielaufruf.htm -->

Der gezielte Aufruf einer Ansicht kann vom Support vorgenommen werden. Dies ist eine sehr technische Einrichtung, die hier beschrieben werden soll:

An die JPL namens OLAP.j werden folgende Parameter übergeben:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>JVAR 1975</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>COMMAND</p>
        </td>
        <td>
          <p>Anzeigekommando</p>
          <ul>
            <li>Sollen die Daten nur angezeigt werden ohne Designer, dann wird hier „SHOW“ angegeben<br>Soll die Auswertung automatisiert gedruckt werden wird hier „PRINT“ angegeben</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>ANWENDUNG</p>
        </td>
        <td>
          <p>Die Anwendung, als der die Daten kommen sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>VARIANTE</p>
        </td>
        <td>
          <p>Die Variante aus der die Daten kommen sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PROFIL</p>
        </td>
        <td>
          <p>Das Profil aus dem der Filter kommen soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TITEL</p>
        </td>
        <td>
          <p>Der anzuzeigende Titel (Default leer)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PRINTER</p>
        </td>
        <td>
          <p>Drucker, auf dem die Auswertung gedruckt werden soll (COMMAND==PRINT)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PRINTAREA</p>
        </td>
        <td>
          <p>Hier gibt es drei Bereiche:</p>
          <ul>
            <li>RAW – die Rohdaten der Anwendung</li>
            <li>CHART – die grafische Auswertung des Charts</li>
            <li>PIVOT – die Pivottabelle</li>
          </ul>
        </td>
      </tr>
    </tbody>
  </table>
</div>
