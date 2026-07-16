# Vertreterabrechnung Variante 2(Provisionierte unabgerechnete Bewegungen)

<!-- source: https://amic.de/hilfe/_vertreterabrechnungv2.htm -->

In der 2.Variante sind alle durch den 1.Schritt erzeugten Warenprovisionen zu sehen, wenn sie innerhalb des gewählten Profils liegen und noch nicht abgerechnet sind. Zu diesem Zeitpunkt hat man zwei Möglichkeiten:

entweder man löscht die Provisionierung **F7**, d.h. die Einträge in die Relation Warenprovision werden für alle markierten Belege gelöscht und die Stati werden gemäß der untenstehenden Konvention gelöscht. Dabei kann ein Löschen nur dann erfolgen, wenn noch keine Warenbewegung des markierten Beleges bereits abgerechnet ist. Wenn dieses Kriterium erfüllt ist, wird die Provisionierung des markierten Beleges zurückgesetzt und die zugehörigen Einträge verschwinden in der Auswahlliste und landen in der 1. Variante.

oder man führt die Vertreterabrechnung **F9** durch, d.h. hier kann man für über das Profil ausgewählte Vertreter durch Markierung die Provisionierungen in Abrechnungslisten zusammenfassen. Hierbei bekommen alle markierten Warenprovisionen die gleiche Abrechnungsnummer, die aus dem Nummernkreis gezogen wird, die in MNDNK (Mandantenstammnummernkreiszuordnung) der Vertreterabrechnung zugeordnet ist. Auch in diesem Fall verschwinden die markierten Sätze aus der Auswahlliste und landen in der 4. Variante.

<details>
<summary>Felder:</summary>

| Feld | Bedeutung |
| --- | --- |
| Vert. | Vertreter |
| Bezeichnung | Bezeichnung des Vertreters |
| AbrLi-Nr. | Abrechnungslisten-Nummer |
| AnzBel | Anzahl Belege |
| Peri/Jahr | Periode / Jahr |
| Belege v | Belege von |
| Bis | Belege bis |
| Gebinde | |
| Menge | |
| Gewicht | |
| Wert | |
| BezMenge | Bezeichnung Menge |
| BezWert | Bezeichnung Wert |
| Provision | |
| Anteil | |
| VertGrup | Vertreter Gruppe |
| ProvGrup | Provisionsgruppe |

</details>

<details>
<summary>Suchmöglichkeiten</summary>

| Feld | Bedeutung |
| --- | --- |
| Jahr | Von… |
| AbrLi-Nr. | Von… Bis… |
| Vertreter | Von… Bis… |
| Kunde | Von… Bis… |

</details>

<details>
<summary>Funktionen:</summary>

| Funktion | Beschreibung |
| --- | --- |
| Keine Provision **(F7)** | |
| Provisionsermittlung **(F9)** | |

</details>
