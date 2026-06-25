# Intrastat-Positionen

<!-- source: https://amic.de/hilfe/_intrastat1.htm -->

Hauptmenü > Warenverkauf > Intrastat > Intrastat-Meldung > Variante 1: Intrastat-Positionen

oder Direktsprung **[INTRA]**

<details>
<summary>Felder der Intrastat Positionen</summary>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Felder</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versendung/Einfuhr</p>
        </td>
        <td>
          <p>Kennzeichen ob Versendung oder Einfuhr<br>1: Versendung<br>2: Einfuhr</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | =</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Beleg ist in der Fibu aber nicht im Interstat</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Periode</p>
        </td>
        <td>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/wirtschaftsjahre_und_perioden/perioden.md">Perioden</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Jahr</p>
        </td>
        <td>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/wirtschaftsjahre_und_perioden/anlegen_eines_neuen_wirtschaftsjahres_wj_am_beispiel_2012.md">Jahr</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Melden</p>
        </td>
        <td>
          <p>Meldekennzeichen<br>1: Ja<br>9: Nein</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Meldung wurde per Pfleger auf Nein gesetzt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Addon</p>
        </td>
        <td>
          <p>Gibt an, ob zugehörige Intrastat Zusatz-Daten vorhanden sind</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Lagerumbuchungsproblem oder die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UStid Mandant</p>
        </td>
        <td>
          <p>Umsatzsteuerid des zugehörigen Mandanten</p>
          <p>Die im Vorgang hinterlegte UStid. Ist diese nicht angegeben, wird die Default-UStid des Mandantstammes herangezogen.</p>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU">Finanzbuchhaltung Ust-IdNr.</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mnd-Staatnr.</p>
        </td>
        <td>
          <p>Staatnummer des zugehörigen Mandanten</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Es konnte keine UStid ermittelt werden und ist die Meldung auch nicht über das Addon verneint. Resultierender Staat existiert nicht, oder UStdid falsch</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mnd-Staat</p>
        </td>
        <td>
          <p>Staat des zugehörigen Mandanten</p>
          <p>Der Iso-Code aus dem Staatstamm.</p>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/staatstamm/index.md">Staatstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mnd-Zollgruppe</p>
        </td>
        <td>
          <p>Zollgruppe aus dem Staatstamm</p>
          <p>(Inland, EU-Mitglied)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UStid Kunde</p>
        </td>
        <td>
          <p>Umsatzsteuerid des Kunden</p>
          <p>Im Normal-Fall die im Vorgang hinterlegte UStid.</p>
          <p>Ist diese nicht angegeben wird die Default-UStid des Kundenstammes herangezogen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Knd-Staatnr.</p>
        </td>
        <td>
          <p>Staatnummer des Kunden</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Resultierender Staat existiert nicht, oder UStdid falsch</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Knd-Staat</p>
        </td>
        <td>
          <p>Staat des Kunden</p>
          <p>Der Iso-Code aus dem Staatstamm</p>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/staatstamm/index.md">Staatstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Knd-Zollgruppe</p>
        </td>
        <td>
          <p>Zollgruppe des Kunden</p>
          <p>Zollgruppe aus dem Staatstamm</p>
          <p>(Inland, EU-Mitglied)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundennr.</p>
        </td>
        <td>
          <p>Kundennummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunde</p>
        </td>
        <td>
          <p>Kunde</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagernummer</p>
        </td>
        <td>
          <p>Lagernummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lager-Staatnr.</p>
        </td>
        <td>
          <p>Staatnummer des Lagers</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Resultierender Staat existiert nicht, oder UStdid falsch</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lager-Staat</p>
        </td>
        <td>
          <p>Staat des Lagers</p>
          <p>Der Iso-Code aus dem Staatstamm</p>
          <p>Siehe auch</p>
          <p><a href="../../../../firmenstamm/staatstamm/index.md">Staatstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lager-Zollgruppe</p>
        </td>
        <td>
          <p>Zollgruppe des Lagers</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versand-Staatnr.</p>
        </td>
        <td>
          <p>Versand-Staatnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versand-Staat</p>
        </td>
        <td>
          <p>Versand-Staat</p>
          <p>Der Iso-Code aus dem Staatstamm</p>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/staatstamm/index.md">Staatstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Region</p>
        </td>
        <td>
          <p>Region</p>
          <p>Entspricht <a href="https://www-idev.destatis.de/idev/doc/intra/doc/svz_datei_intra.pdf">Intrahandelsstatistik - Schlüsselverzeichnis zur Dateimeldung</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorgangsklasse</p>
        </td>
        <td>
          <p>Vorgangsklasse</p>
          <p>Berücksichtigt sind:</p>
          <ul>
            <li><i>700,790 Rechnung u. Storno</i></li>
            <li><i>800,890 Gutschrift und Storno</i></li>
            <li><i>1700,1790 Eingangsrechnung u. Storno</i></li>
            <li><i>1800,1890 Eingangsgutschrift und Storno</i></li>
            <li><i>5110 Lagerumbuchung</i></li>
            <li><i>5120 Artikelumbuchung</i></li>
            <li><i>5220 Produktion Stückliste</i></li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuergruppe</p>
        </td>
        <td>
          <p>Steuergruppe des Vorgangs</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuersatz</p>
        </td>
        <td>
          <p>Steuersatz der Warenbewegungsposition</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuer</p>
        </td>
        <td>
          <p>Steuer der Warenbewegungs-Position</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beteiligtes Land-Staatnr</p>
        </td>
        <td>
          <p>Staatnummer des beteiligten Landes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beteiligtes Land</p>
        </td>
        <td>
          <p>Der beteiligte Staat</p>
          <p>Der Iso-Code aus dem Staatstamm</p>
          <p>Das für die Intrastatmeldung relevante Land.</p>
          <p>Siehe auch:<br><a href="../../../../firmenstamm/staatstamm/index.md">Staatstamm</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verkehrszweig</p>
        </td>
        <td>
          <p>Verkehrszweig</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Art des Geschäftes</p>
        </td>
        <td>
          <p><a href="../intrastat_art_des_geschaeftes.md">Art des Geschäftes</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegnr</p>
        </td>
        <td>
          <p>Belegnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wert</p>
        </td>
        <td>
          <p>Wert der <a href="../intrastat_art_des_geschaeftes.md">Art des Geschäftes</a></p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Negativer Rechnungswert</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Statistischer Wert</p>
        </td>
        <td>
          <p>Statistischer Wert</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Masse</p>
        </td>
        <td>
          <p>Masse</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Eigenmasse ist 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wert/Masse</p>
        </td>
        <td>
          <p>Wert pro Masse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ME Nummer</p>
        </td>
        <td>
          <p>Mengeneinheitsnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Besondere Maßeinheit</p>
        </td>
        <td>
          <p>Besondere Maßeinheit</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel-Intrastatnummer</p>
        </td>
        <td>
          <p>Artikel-Intrastatnummer</p>
          <p>|&nbsp;&nbsp; X&nbsp;&nbsp; | = Wenn die Artikel-Intrastatnummer nicht gepflegt ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelnummer</p>
        </td>
        <td>
          <p>Artikelnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel</p>
        </td>
        <td>
          <p>Artikel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>V_Id</p>
        </td>
        <td>
          <p>Vorgangs-Identifikator</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Wabewid</p>
        </td>
        <td>
          <p>Warenbewegungs-Identifikator</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MKNDSN</p>
        </td>
        <td rowspan="5">
          <p>Technische Hilfsfelder für Bereichsauswahl</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MMNDSN</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MLAGER</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MKNDU</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MMASSE</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BadLgu</p>
        </td>
        <td>
          <p>Technisches Hilfsflag</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<details>
<summary>Suchmöglichkeiten der Intrastat Positionen</summary>

| Suchen | Beschreibung |
| --- | --- |
| Periode | Von … bis … |
| Jahr | % |
| Meldung | Von … bis … |
| Einfuhr=2 | Von … bis … |
| Geschäft | Von … bis … |
| Kundennr | Von … bis … |
| Steuergruppe | Von … bis … |
| UStid Mandant | % |
| UStid Kunde | % |
| Artikel-Intrastatnummer | % |
| Wert | Von … bis … |
| Masse | Von … bis … |
| Unvollständige Daten anzeigen<br> <br> | 0: Nein<br>2: Staat/USTID<br>3: Zollgruppe<br>4: Artikel-Intrastatnummer<br>5: Komplettanzeige |
| Vergleich UVA | 0: Nein<br>1: Ja<br> <br>Ermöglicht den Abgleich zwischen UVA und Intrastat. |
| Alle Datensätze zeigen | 0: Nein<br>1: Ja<br> <br>Einstellung *Nein* filtert die Datensätze heraus bei denen die Mandanten-, die Kunden- und die Lagerstaatnummer alle gleich sind<br>oder<br>die auf Nicht-Melden stehen<br>oder<br>Lagerumbuchungsanteile sind die nicht extra aufgeführt werden sollen.<br> <br>Somit erhält man bei Einstellung *Ja* die Gelegenheit z.B. mit Hilfe der farblichen Markierungen so Datensätze auffinden und bewerten, die man sonst nicht angezeigt bekommt, da sie vom System aussortiert werden.<br>Somit hat man die Möglichkeit bei diesen ggf. mit den Intrastat Zusatzdaten entsprechend nachzuhelfen. |

</details>

<details>
<summary>Funktionen der Intrastat Positionen</summary>

| Funktion | Beschreibung |
| --- | --- |
| Ändern (**F5**), Ansehen (**F6**), Löschen (**F7**) | Ruft den Pfleger für die [Intrastat Zusatzdaten](./intrastat_zusatzdaten.md) auf |
| Vorschau (**F11**) | Öffnet das Standard PDF Programm und zeigt eine Vorschau des Intrastat Formulares |
| Intrastat einrichten (**F10**) | Ruft die Maske [Intrastat einrichten](../intrastat_einrichten.md) auf |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Intrastat Zusatzdaten](./intrastat_zusatzdaten.md)
