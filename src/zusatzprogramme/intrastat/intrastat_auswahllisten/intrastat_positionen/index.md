# Intrastat-Positionen

<!-- source: https://amic.de/hilfe/_intrastat1.htm -->

Hauptmenü > Warenverkauf > Intrastat > Intrastat-Meldung > Variante 1: Intrastat-Positionen

oder Direktsprung **[INTRA]**

<details>
<summary>Felder der Intrastat Positionen</summary>

| Felder | Beschreibung |
| --- | --- |
| Versendung/Einfuhr | Kennzeichen ob Versendung oder Einfuhr  
1: Versendung  
2: Einfuhr  
   
| X | =  
| X | = Beleg ist in der Fibu aber nicht im Interstat |
| Periode | Siehe auch:  
[Perioden](../../../../firmenstamm/wirtschaftsjahre_und_perioden/perioden.md) |
| Jahr | Siehe auch:  
[Jahr](../../../../firmenstamm/wirtschaftsjahre_und_perioden/anlegen_eines_neuen_wirtschaftsjahres_wj_am_beispiel_2012.md) |
| Melden | Meldekennzeichen  
1: Ja  
9: Nein  
   
| X | = Meldung wurde per Pfleger auf Nein gesetzt |
| Addon | Gibt an, ob zugehörige Intrastat Zusatz-Daten vorhanden sind  
   
| X | = Lagerumbuchungsproblem oder die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein |
| UStid Mandant | Umsatzsteuerid des zugehörigen Mandanten  
Die im Vorgang hinterlegte UStid. Ist diese nicht angegeben, wird die Default-UStid des Mandantstammes herangezogen.  
Siehe auch:  
[Finanzbuchhaltung Ust-IdNr.](../../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU) |
| Mnd-Staatnr. | Staatnummer des zugehörigen Mandanten  
   
| X | = Es konnte keine UStid ermittelt werden und ist die Meldung auch nicht über das Addon verneint. Resultierender Staat existiert nicht, oder UStdid falsch  
| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein  
| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Mnd-Staat | Staat des zugehörigen Mandanten  
Der Iso-Code aus dem Staatstamm.  
Siehe auch:  
[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Mnd-Zollgruppe | Zollgruppe aus dem Staatstamm  
(Inland, EU-Mitglied) |
| UStid Kunde | Umsatzsteuerid des Kunden  
Im Normal-Fall die im Vorgang hinterlegte UStid.  
Ist diese nicht angegeben wird die Default-UStid des Kundenstammes herangezogen. |
| Knd-Staatnr. | Staatnummer des Kunden  
   
| X | = Resultierender Staat existiert nicht, oder UStdid falsch  
| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein  
| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Knd-Staat | Staat des Kunden  
Der Iso-Code aus dem Staatstamm  
Siehe auch:  
[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Knd-Zollgruppe | Zollgruppe des Kunden  
Zollgruppe aus dem Staatstamm  
(Inland, EU-Mitglied) |
| Kundennr. | Kundennummer |
| Kunde | Kunde |
| Lagernummer | Lagernummer |
| Lager-Staatnr. | Staatnummer des Lagers  
   
| X | = Resultierender Staat existiert nicht, oder UStdid falsch  
| X | = Die Mandanten-Staatsnummer + Kunden-Staatsnummer + Lager-Staatsnummer stimmen überein  
| X | = Zollgruppenzuordnung ist nicht Inland bzw. EU-Mitglied |
| Lager-Staat | Staat des Lagers  
Der Iso-Code aus dem Staatstamm  
Siehe auch  
[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Lager-Zollgruppe | Zollgruppe des Lagers |
| Versand-Staatnr. | Versand-Staatnummer |
| Versand-Staat | Versand-Staat  
Der Iso-Code aus dem Staatstamm  
Siehe auch:  
[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Region | Region  
Entspricht [Intrahandelsstatistik - Schlüsselverzeichnis zur Dateimeldung](https://www-idev.destatis.de/idev/doc/intra/doc/svz_datei_intra.pdf) |
| Vorgangsklasse | Vorgangsklasse  
Berücksichtigt sind:  
\- *700,790 Rechnung u. Storno*  
\- *800,890 Gutschrift und Storno*  
\- *1700,1790 Eingangsrechnung u. Storno*  
\- *1800,1890 Eingangsgutschrift und Storno*  
\- *5110 Lagerumbuchung*  
\- *5120 Artikelumbuchung*  
\- *5220 Produktion Stückliste* |
| Steuergruppe | Steuergruppe des Vorgangs |
| Steuersatz | Steuersatz der Warenbewegungsposition |
| Steuer | Steuer der Warenbewegungs-Position |
| Beteiligtes Land-Staatnr | Staatnummer des beteiligten Landes |
| Beteiligtes Land | Der beteiligte Staat  
Der Iso-Code aus dem Staatstamm  
Das für die Intrastatmeldung relevante Land.  
Siehe auch:  
[Staatstamm](../../../../firmenstamm/staatstamm/index.md) |
| Verkehrszweig | Verkehrszweig |
| Art des Geschäftes | [Art des Geschäftes](../intrastat_art_des_geschaeftes.md) |
| Belegnr | Belegnummer |
| Wert | Wert der [Art des Geschäftes](../intrastat_art_des_geschaeftes.md)  
   
| X | = Negativer Rechnungswert |
| Statistischer Wert | Statistischer Wert |
| Masse | Masse  
   
| X | = Eigenmasse ist 0 |
| Wert/Masse | Wert pro Masse |
| ME Nummer | Mengeneinheitsnummer |
| Besondere Maßeinheit | Besondere Maßeinheit |
| Artikel-Intrastatnummer | Artikel-Intrastatnummer  
   
| X | = Wenn die Artikel-Intrastatnummer nicht gepflegt ist. |
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
| Unvollständige Daten anzeigen  
   
 | 0: Nein  
2: Staat/USTID  
3: Zollgruppe  
4: Artikel-Intrastatnummer  
5: Komplettanzeige |
| Vergleich UVA | 0: Nein  
1: Ja  
   
Ermöglicht den Abgleich zwischen UVA und Intrastat. |
| Alle Datensätze zeigen | 0: Nein  
1: Ja  
   
Einstellung *Nein* filtert die Datensätze heraus bei denen die Mandanten-, die Kunden- und die Lagerstaatnummer alle gleich sind  
oder  
die auf Nicht-Melden stehen  
oder  
Lagerumbuchungsanteile sind die nicht extra aufgeführt werden sollen.  
   
Somit erhält man bei Einstellung *Ja* die Gelegenheit z.B. mit Hilfe der farblichen Markierungen so Datensätze auffinden und bewerten, die man sonst nicht angezeigt bekommt, da sie vom System aussortiert werden.  
Somit hat man die Möglichkeit bei diesen ggf. mit den Intrastat Zusatzdaten entsprechend nachzuhelfen. |

</details>

<details>
<summary>Funktionen der Intrastat Positionen</summary>

| Funktion | Beschreibung |
| --- | --- |
| Ändern (F5), Ansehen (F6), Löschen (F7) | Ruft den Pfleger für die [Intrastat Zusatzdaten](./intrastat_zusatzdaten.md) auf |
| Vorschau (F11) | Öffnet das Standard PDF Programm und zeigt eine Vorschau des Intrastat Formulares |
| Intrastat einrichten (F10) | Ruft die Maske [Intrastat einrichten](../intrastat_einrichten.md) auf |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Intrastat Zusatzdaten](./intrastat_zusatzdaten.md)
