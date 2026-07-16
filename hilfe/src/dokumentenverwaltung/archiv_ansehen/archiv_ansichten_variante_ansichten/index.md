# Archiv-Ansichten-Variante: Ansichten

<!-- source: https://amic.de/hilfe/_archivansichtenvaria.htm -->

Hauptmenü > Administration > Archiv > Zugriffssteuerung > Ansichten

Direktsprung **[FAA]**

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p>Feld</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Bezeichnung der Archiv-Ansicht</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p>Zugeordnete Bedienerklasse.</p>
          <p>Sind mehrere Ansichten gleichen Namens vorhanden, entscheidet die Bedienerklasse darüber, welche Archiv-Ansicht zur Verfügung gestellt wird.</p>
          <p>Somit ist es möglich, jeweils verschiedenen Bedienerklassen auch bestimmte Archiv-Ansichten zukommen zu lassen.</p>
          <p>Die Bedienerklasse -1 steht stellvertretend für alle Bedienerklassen.</p>
          <p>Die Bezeichnung dieser „Bedienerklasse“ ist „<b><i>Defaultklasse Kunden</i></b>“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklassenbezeichnung</p>
        </td>
        <td>
          <p>Bezeichnung der Bedienerklasse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besitzer</p>
        </td>
        <td>
          <p><b><i>AMIC</i></b>:</p>
          <p>Auf Kundendatenbanken handelt es sich dabei um eine „Auslieferung“.</p>
          <p><b><i>Privat</i></b>:</p>
          <p>Eine privatisierte Auslieferung oder eine neu erstellte Ansicht, deren Ansichts-Name keiner Auslieferung zugeordnet ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Grundlage</p>
        </td>
        <td>
          <p>Versucht über das Einsatzgebiet der Archiv-Ansichten zu informieren.</p>
          <p>Mögliche Identifizierungen sind:</p>
          <p>0 : Frei</p>
          <p>1 : Auswahlliste</p>
          <p>2 : Dialog</p>
          <p>3 : Extern</p>
          <p>4: Auswahl</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ansichts-Status</p>
        </td>
        <td>
          <p><b><i>Auslieferung</i></b><i>:</i></p>
          <p>Auslieferungen sind Ansichten, die mit „<i>AMIC_</i>“ beginnen und deren Besitzer „<i>AMIC</i>“ ist.</p>
          <p><b><i>Privatisierte Auslieferung:</i></b></p>
          <p>Privatisierte Auslieferungen sind Auslieferungen die in aller Regel durch die Funktion „Ansicht duplizieren“ erzeugt wurden.</p>
          <p>Sie lassen sich aber ebenso komplett neu erstellen. Das wichtige Erkennungsmerkmal ist, dass eine solche Ansicht den gleichen Namen wie eine „Auslieferung“ hat.</p>
          <p><b><i>Privat:</i></b></p>
          <p>Eine private Ansicht ist weder eine Auslieferung noch eine privatisierte Auslieferung.</p>
          <p><b><i>Ableitung:</i></b></p>
          <p>Private Ansichten, also solche mit Ansichts-Status „Private Auslieferung“ oder „Privat“, können weiter abgeleitet werden.&nbsp;</p>
          <p><b><i>Egal:</i></b></p>
          <p>Einer der obigen Ansichts-Stati.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Einsatz</p>
        </td>
        <td>
          <p>Beschreibung über den Einsatz bzw. Verwendungen der Ansichts-Definition.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ansichts-Id</p>
        </td>
        <td>
          <p>Bildet zusammen mit dem „Besitzer“ den Schlüsselbegriff der Archiv-Ansichten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausliefer-Id</p>
        </td>
        <td>
          <p>Im Falle von „privatisierten Auslieferungen“ stellt die Ausliefer-Id mit dem Ausliefer-Besitzer den Rückverweis auf die dazugehörige Auslieferung da.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausliefer-Besitzer</p>
        </td>
        <td>
          <p>Im Falle von „privatisierten Auslieferungen“ stellt die Ausliefer-Id mit dem Ausliefer-Besitzer den Rückverweis auf die dazugehörige Auslieferung da.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Different</p>
        </td>
        <td>
          <p>Natürlicherweise darf eine privatisierte Auslieferung Unterschiede zu der Auslieferung aufweisen, das ist das Wesen einer <u>privatisierten</u> Auslieferung.</p>
          <p>Anhand dieser Kennzeichnung kann nun aber rückwirkend entschieden werden, ob eine privatisierte Auslieferung sich von einer Auslieferung „unterscheidet“.</p>
          <p>Unterschiedlich sind zwei beteiligte Archiv-Ansichten, wenn sie unter Berücksichtigung der aktiven Richtlinien und der SPA-Einstellung „<i>Archiv-Richtlinien in privaten Ansichten berücksichtigen</i>“ (782) noch weitere Unterschiede aufweisen.</p>
          <p>Nach Einführung der Archiv-Richtlinien kann aber folgende Fragestellung interessant sein: Wann darf man denn eine „privatisierte Auslieferung“ gefahrlos löschen? Zur Beantwortung dieser Frage stellt eben diese Kennzeichnung eine interessante Hilfestellung zur Verfügung.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Archiv-Ansichten: Technische Unterstützung](./archiv_ansichten_technische_unterstuetzung.md)
- [Archiv-Ansicht Standard-Auslieferung: Kunden, Vorgang](./archiv_ansicht_standard_auslieferung_kunden_vorgang.md)
