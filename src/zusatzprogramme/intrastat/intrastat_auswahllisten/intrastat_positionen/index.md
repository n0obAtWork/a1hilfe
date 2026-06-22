# Intrastat-Positionen

<!-- source: https://amic.de/hilfe/_intrastat1.htm -->

Hauptmenü > Warenverkauf > Intrastat > Intrastat-Meldung > Variante 1: Intrastat-Positionen

oder Direktsprung **[INTRA]**

<details>
<summary>Felder der Intrastat Positionen</summary>

| Felder | Beschreibung |
| --- | --- |
| Versendung/Einfuhr | Kennzeichen ob Versendung oder Einfuhr<br>1: Versendung<br>2: Einfuhr<br> <br>| X | =<br>| X | = Beleg ist in der Fibu aber nicht im Interstat |
| Periode | Siehe auch:<br>[Perioden](../../../../firmenstamm/wirtschaftsjahre_und_perioden/perioden.md) |
| Jahr | Siehe auch:<br>[Jahr](../../../../firmenstamm/wirtschaftsjahre_und_perioden/anlegen_eines_neuen_wirtschaftsjahres_wj_am_beispiel_2012.md) |
| Melden | Meldekennzeichen<br>1: Ja<br>9: Nein<br> <br>| X | = Meldung wurde per Pfleger auf Nein gesetzt |
| Addon | Gibt an, ob zugehörige Intrastat Zusatz-Daten vorhanden sind<br> <br>| X | = Lagerumbuchungsproblem oder die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein |
| UStid Mandant | Umsatzsteuerid des zugehörigen Mandanten<br>Die im Vorgang hinterlegte UStid. Ist diese nicht angegeben, wird die Default-UStid des Mandantstammes herangezogen.<br>Siehe auch:<br>[Finanzbuchhaltung Ust-IdNr.](../../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU) |
| Mnd-Staatnr. | Staatnummer des zugehörigen Mandanten<br> <br>| X | = Es konnte keine UStid ermittelt werden und ist die Meldung auch nicht über das Addon verneint. Resultierender Staat existiert nicht, oder UStdid falsch<br>| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein<br>| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Mnd-Staat | Staat des zugehörigen Mandanten<br>Der Iso-Code aus dem Staatstamm.<br>Siehe auch:<br>[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Mnd-Zollgruppe | Zollgruppe aus dem Staatstamm<br>(Inland, EU-Mitglied) |
| UStid Kunde | Umsatzsteuerid des Kunden<br>Im Normal-Fall die im Vorgang hinterlegte UStid.<br>Ist diese nicht angegeben wird die Default-UStid des Kundenstammes herangezogen. |
| Knd-Staatnr. | Staatnummer des Kunden<br> <br>| X | = Resultierender Staat existiert nicht, oder UStdid falsch<br>| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein<br>| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Knd-Staat | Staat des Kunden<br>Der Iso-Code aus dem Staatstamm<br>Siehe auch:<br>[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Knd-Zollgruppe | Zollgruppe des Kunden<br>Zollgruppe aus dem Staatstamm<br>(Inland, EU-Mitglied) |
| Kundennr. | Kundennummer |
| Kunde | Kunde |
| Lagernummer | Lagernummer |
| Lager-Staatnr. | Staatnummer des Lagers<br> <br>| X | = Resultierender Staat existiert nicht, oder UStdid falsch<br>| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein<br>| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Lager-Staat | Staat des Lagers<br>Der Iso-Code aus dem Staatstamm<br>Siehe auch<br>[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Lager-Zollgruppe | Zollgruppe des Lagers |
| Versand-Staatnr. | Versand-Staatnummer |
| Versand-Staat | Versand-Staat<br>Der Iso-Code aus dem Staatstamm<br>Siehe auch:<br>[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Region | Region<br>Entspricht [Intrahandelsstatistik - Schlüsselverzeichnis zur Dateimeldung](https://www-idev.destatis.de/idev/doc/intra/doc/svz_datei_intra.pdf) |
| Vorgangsklasse | Vorgangsklasse<br>Berücksichtigt sind:<br>\- *700,790 Rechnung u. Storno*<br>\- *800,890 Gutschrift und Storno*<br>\- *1700,1790 Eingangsrechnung u. Storno*<br>\- *1800,1890 Eingangsgutschrift und Storno*<br>\- *5110 Lagerumbuchung*<br>\- *5120 Artikelumbuchung*<br>\- *5220 Produktion Stückliste* |
| Steuergruppe | Steuergruppe des Vorgangs |
| Steuersatz | Steuersatz der Warenbewegungsposition |
| Steuer | Steuer der Warenbewegungs-Position |
| Beteiligtes Land-Staatnr | Staatnummer des beteiligten Landes |
| Beteiligtes Land | Der beteiligte Staat<br>Der Iso-Code aus dem Staatstamm<br>Das für die Intrastatmeldung relevante Land.<br>Siehe auch:<br>[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Verkehrszweig | Verkehrszweig |
| Art des Geschäftes | [Art des Geschäftes](../intrastat_art_des_geschaeftes.md) |
| Belegnr | Belegnummer |
| Wert | Wert der [Art des Geschäftes](../intrastat_art_des_geschaeftes.md)<br> <br>| X | = Negativer Rechnungswert |
| Statistischer Wert | Statistischer Wert |
| Masse | Masse<br> <br>| X | = Eigenmasse ist 0 |
| Wert/Masse | Wert pro Masse |
| ME Nummer | Mengeneinheitsnummer |
| Besondere Maßeinheit | Besondere Maßeinheit |
| Artikel-Intrastatnummer | Artikel-Intrastatnummer<br> <br>| X | = Wenn die Artikel-Intrastatnummer nicht gepflegt ist. |
| Artikelnummer | Artikelnummer |
| Artikel | Artikel |
| V_Id | Vorgangs-Identifikator |
| Wabewid | Warenbewegungs-Identifikator |
| MKNDSN | Technische Hilfsfelder für Bereichsauswahl |
| MMNDSN |
| MLAGER |
| MKNDU |
| MMASSE |
| BadLgu | Technisches Hilfsflag |

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
