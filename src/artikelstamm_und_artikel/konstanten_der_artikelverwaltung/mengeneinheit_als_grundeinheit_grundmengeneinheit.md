# Mengeneinheit als Grundeinheit (Grundmengeneinheit)

<!-- source: https://amic.de/hilfe/_mengeneinheitalsgrun.htm -->

Die Grundeinheit ist die einfachste Form einer Mengeneinheit. Wenn in einem Unternehmen keine Umrechnungen erforderlich werden, dann sind auch nur diese Grundeinheiten zu erfassen.

Ein typisches Beispiel hierfür sind Artikel, die in der Einheit Stück eingekauft und verkauft werden, deren Lagerbestand in Stück geführt wird und deren Preis in Ein- und Verkauf sich auf die Einheit Stück bezieht. Hier gibt es also nur den Fall Mengeneinheit = Grundeinheit.

In der Grundversion von A.eins werden die gängigsten Mengeneinheiten, wie Stück, Liter, kg, etc. eingerichtet als Grundeinheit mit ausgeliefert. Falls im konkreten Fall keine weiteren Mengeneinheiten benötigt werden, kann auf die Erfassung verzichtet werden.  
Für die Anlage der (Grund -) Mengeneinheiten werden praktisch lediglich die Texte der gepflegt, also kg, Ltr., Stück usw.. Es sind dies die jeweils kleinsten, nicht mehr teilbaren, Mengeneinheiten des Systems. Häufig sind es ohnehin nur diese Einheiten mit denen in einem Unternehmen gearbeitet wird. Wenn nämlich mit konstanten Mengenbezügen (Einkaufs-/Verkaufs-/Preiseinheit identisch) und ohne Gebinde gearbeitet wird sind keine Umrechnungen erforderlich und es genügt die Erfassung der Grundeinheiten:

Folgende Felder stehen hier zu Erfassung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Grundmengeneinheit – Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>Nummer der zu definierenden Mengeneinheit. Die Eingabe der Nummern ist aus technischen Gründen auf 4 Stellen begrenzt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurztext</p>
        </td>
        <td>
          <p>Kurzbezeichnung der Mengeneinheit, wie sie ausgedruckt werden sollen; also z.B. kg, Stück, Ltr. etc.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ISO Name</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Langtext</p>
        </td>
        <td>
          <p>Langtext, welcher anstelle des Kurztext ausgedruckt werden kann</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Ausführliche Bezeichnung der Mengeneinheit, z. B. für Auswahllisten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DataNormKurz</p>
        </td>
        <td>
          <p>Die Kurzbezeichnung des DataNorm Verfahrens. Erforderlich, wenn Datenaustausch mit anderen Unternehmen auf Grundlage dieses Verfahrens erfolgen soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Statistikkennzeichen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Rundung bei Umrechnung</p>
        </td>
        <td>
          <p>Erlaubt sind die Werte 0 bis 4. Bei allen Mengen, die auf diese Mengeneinheit referenzieren, wird dieser Rundungsfaktor angewendet. Bei schon aktiven Mengeneinheiten ist eine Abänderung dieses Wertes nur nach oben erlaubt, um ggf. schon geschriebene Belege nicht im Nachherein zu verändern.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeneinheit Preisbezug</p>
        </td>
        <td>
          <p>Mengeneinheit für den Preisbezug</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UN-Mengeneinheit</p>
        </td>
        <td>
          <p>Hier wird die internationale Mengeneinheitsbezeichnung aus der UN Recomendation No 20 eingetragen. Diese Mengeneinheiten werden u.a. bei openTRANS und eRechnung im Datenaustausch verwendet.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Wie oben angesprochen, genügt in vielen Fällen bereits diese einfache Form der Mengeneinheitsdefinition. Dies gilt häufig auch, wenn sich Preis- und Lagermengeneinheit unterscheiden.

Die Problematik, dass die Bestände in **kg** geführt werden, der Preis sich jedoch auf 100 kg bezieht, kann im Rahmen der Preisfindung mit einem entsprechenden Preisbezug (100) gelöst werden.
