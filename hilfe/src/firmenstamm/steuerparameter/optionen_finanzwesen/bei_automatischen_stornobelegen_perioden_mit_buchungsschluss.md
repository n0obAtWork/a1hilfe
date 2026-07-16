# Bei automatischen Stornobelegen Perioden mit Buchungsschluss zulassen (SPA 1134)

<!-- source: https://amic.de/hilfe/_SPA_1134.htm -->

Beim Zurücksetzen von Auszifferungen werden für einige Belegarten (z.B. Restposten, Skonto) automatisch Stornobelege erstellt. Steht die Periode auf Buchungsschluss oder ist sie bereits abgeschlossen, dann wird die Periode abgefragt, in die der Stornobeleg gebucht werden soll. Dieser SPA steuert, wie bei Perioden mit Buchungsschluss behandelt werden sollen.

***Hinweis: Da [Buchungsadministratoren](../../wirtschaftsjahre_und_perioden/buchungsadministratoren/index.md) in Perioden mit Buchungsschluss buchen dürfen, hat dieser SPA für sie keine Auswirkung.***

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p><b>Das Periodenabfragefenster öffnet sich und die Periode ist vorbelegt. In dem Hinweistext steht, dass die Periode des Ursprungsbeleges bereits auf Buchungsschluss gesetzt wurde, man sie aber trotzdem verwenden kann. Ansonsten muss man sie auf eine noch offene Periode ändern.</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Das Periodenabfragefenster öffnet sich und die nächste mögliche Periode wird vorbelegt. Im Hinweistext steht, dass die Periode des Urspungsbelegs nicht mehr verwendet werden darf.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
