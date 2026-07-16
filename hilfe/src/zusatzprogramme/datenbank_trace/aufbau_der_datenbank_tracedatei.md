# Aufbau der Datenbank-Tracedatei

<!-- source: https://amic.de/hilfe/aufbauderdatenbanktracedatei.htm -->

Die erzeugte Datenbank-Tracedatei ist als OSQL-Einspielscript formuliert.

```sql
LOADTUETTEL;
insert into amic_tracefile (TraceZeit,TraceCursorNo,TraceMaske,TraceVerbrauch,     TraceError,TraceCursor,TracePlan,TraceSelect,TraceUser,TraceStatus,TraceTrace) values (%s)
```

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p>Felder der Datenbank-Tracedatei</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tracezeit</p>
        </td>
        <td>
          <p>Zeitstempel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceCursorNo</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>TraceMaske</p>
        </td>
        <td>
          <p>Diese Maske war zum Zeitpunkt aktiv</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceVerbrauch</p>
        </td>
        <td>
          <p>Zeitverbrauch in Millisekunden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceError</p>
        </td>
        <td>
          <p>Rückgabe-Status des Datenbank-Servers auf die Datenbank-Anweisung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceCursor</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>TraceSelect</p>
        </td>
        <td>
          <p>Datenbank-Anweisung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceUser</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>TraceStatus</p>
        </td>
        <td>
          <p>Stati die das technische Umfeld beschreiben</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TraceTrace</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
