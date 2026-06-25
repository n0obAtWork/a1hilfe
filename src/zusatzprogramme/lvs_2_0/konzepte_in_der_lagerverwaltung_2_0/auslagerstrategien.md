# Auslagerstrategien

<!-- source: https://amic.de/hilfe/_lvs20_auslagerstrat.htm -->

Je Vorgangsunterklasse lässt sich eine Auslagerstrategie festlegen. In der zugehörigen Prozedur werden auch die Mengen für mögliche Über- bzw. Unterlieferungen festgelegt. Durch diese „Kulanz“ bei der Auslieferung kann eine unnötig häufige Kommissionierung verhindert werden.

Hier gibt es mehrere Möglichkeiten, die allesamt mehr oder weniger streng das Prinzip FIFO (First In First Out) bzw. bei Beteiligung von Partien mit Gültigkeitsdatum FEFO (First Expire First Out) berücksichtigen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><b>Auslagerstrategien</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Wert</b></p>
        </td>
        <td>
          <p><b>Bezeichnung</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1 - FIFO Only</p>
        </td>
        <td>
          <p>Strenges FIFO</p>
        </td>
        <td>
          <p>Hier wird die Ware streng nach FIFO ausgelagert. Dabei entstehen unter Umständen viele Kommissionierungen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2 - FIFO GFO</p>
        </td>
        <td>
          <p>Greatest First out</p>
        </td>
        <td>
          <p>Bei der Allokation werden die größten Paletten zuerst allokiert. Es entsteht nur noch am Ende der Liste ein Kommissionierungsbedarf.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3 - One Charge Only</p>
        </td>
        <td>
          <p>Die ganze Lieferung darf nur aus einer Charge bestehen</p>
        </td>
        <td>
          <p>Die Lieferung der Position darf nur aus einer Charge bestehen. Ist also die eigentlich nach FIFO höher priorisierte Partie nicht in der gewünschten menge verfügbar, so wird die nächste gesucht, die in ganzer Menge verfügbar ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4 - FIFO GFO/SFO</p>
        </td>
        <td>
          <p>Greatest First out Smallest First Out</p>
        </td>
        <td>
          <p>Wie 2, jedoch wird für die Kommissionierung darauf geachtet, dass möglichst Anbruchpaletten zuerst aufgebraucht werden. Das spart Platz, bedeutet aber u.U. einen höheren Kommissionierungsaufwand.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
