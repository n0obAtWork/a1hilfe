# Intrastat-Meldung (Versendung)

<!-- source: https://amic.de/hilfe/intrastatvariante2.htm -->

Hauptmenü > Warenverkauf > Intrastat > Intrastat-Meldung > Variante 2: Intrastat-Meldung (Versendung)

oder Direktsprung **[INTRA]**

<details>
<summary>Felder der Intrastat-Meldung</summary>

| Feld | Bezeichnung |
| --- | --- |
| Jahr | Siehe auch:<br>[Jahr](../../../firmenstamm/wirtschaftsjahre_und_perioden/anlegen_eines_neuen_wirtschaftsjahres_wj_am_beispiel_2012.md) |
| Periode | Siehe auch:<br>[Perioden](../../../firmenstamm/wirtschaftsjahre_und_perioden/perioden.md) |
| Kalenderjahr | |
| Monat | |
| Beteiligtes Land | Staat des zugehörigen Mandanten (Iso-Code aus dem Staatstamm)<br>Siehe auch:<br>[Staatstamm](../../../firmenstamm/staatstamm/index.md) |
| UStid Mandant | Umsatzsteuerid des zugehörigen Mandanten<br>Im Normal-Fall die im Vorgang hinterlegte UStid. Ist diese nicht angegeben wird die Default-UStid des Mandantstammes herangezogen.<br>Siehe auch:<br>[Finanzbuchhaltung Ust-IdNr.](../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU) |
| UStid Kunde | Umsatzsteuerid des Kunden |
| Artikel-Intrastatnummer | Die im Artikelstamm hinterlegte Intrastatnummer des Artikels:<br>&#124; X &#124; = Intrastat-Artikel Nummer wurde nicht im Artikelstamm hinterlegt |
| Artikel | ID des Artikels |
| Art des Geschäftes | Art des Geschäftes |
| Verkehrszweig | Verkehrszweig |
| Region | |
| Wert | |
| Statischer Wert | |
| Masse | |
| Besondere Maßeinheit | |
| Paginiernummer | Die Meldungen benötigen eine fortlaufende Nummer |
| DATA | Hilfsfeld für den Export von ACCI-Dateien. |

</details>

<details>
<summary>Suchmöglichkeiten der Intrastat-Meldung</summary>

| Suchen | Beschreibung |
| --- | --- |
| Periode | Von… bis… |
| Jahr | Von… bis… |
| Artikel-Intrastatnummer | % |
| UstId Mandant | % |
| UstId Kunde | % |
| Verkehrszweig | % |

</details>

<details>
<summary>Funktionen der Intrastat Meldung</summary>

| Suchen | Beschreibung |
| --- | --- |
| Versand erzeugen (**F9**) | Erstellt die Intrastat Dateien, je nach Einstellungen der [Intrastat Einrichtung](./intrastat_einrichten.md) (XML/ASCII).<br>Der Export wird **nicht** nach den eingegeben Suchkriterien erstellt, sondern immer auf Basis aller Daten (eingegrenzt von Jahr & Periode) |
| Intrastat einrichten (**F10**) | Ruft die Maske zu Intrastat Einrichtung auf |

</details>
