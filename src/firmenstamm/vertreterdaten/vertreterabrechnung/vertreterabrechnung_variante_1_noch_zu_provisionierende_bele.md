# Vertreterabrechnung Variante 1(noch zu provisionierende Belege)

<!-- source: https://amic.de/hilfe/_vertreterabrechnungv1.htm -->

Diese Varianten charakterisieren die Vorgehensweise während der Vertreterabrechnung. Diese lässt sich in zwei Stufen unterteilen:

In der 1. Variante sind alle Belege zu sehen, die innerhalb des gewählten Profils noch nicht durch die Provisionierung gelaufen sind. Jetzt kann man in der 1.Stufe durch Markieren Belege provisionieren. Die Provisionierung (F9) erzeugt gemäß der untenstehenden Konventionen Einträge in die Relation Warenprovision, d.h. die Provisionierung erfolgt auf WarenBewegungsebene. Nach der Provisionierung verschwinden die Belege aus der Anzeige und laufen in die 2. Variante. Die Provisionierung selbst kann also für durch den Mandantenserver gelaufene Belege zu jeder Zeit erfolgen. Allerdings sollte man vor dem Auslösen dieser Funktion sicherstellen, dass alle Stammdaten im Bereich Vertreter korrekt eingerichtet sind. Während der Provisionierung wird in der Statuszeile (unten am Bildschirm angezeigt, welcher Beleg gerade durch die Provisionierung läuft. Durch beliebigen Tastendruck kann man die Provisionierung abbrechen und sie hält nach Abarbeiten des aktuellen Beleges an. Man kann sie dann zu einem späteren beliebigen Zeitpunkt fortsetzen, die bereits abgearbeiteten Belege tauchen in der Liste der noch zu bearbeitenden Belege nicht mehr auf).

<details>
<summary>Felder:</summary>

| Feld | Bedeutung |
| --- | --- |
| Klasse | Klasse der Abrechnung |
| VertGrup | Vertretergruppe |
| Beleg | Beleg für die Abrechnung |
| BelegDat | Beleg Datum |
| Kunde | Kunden ID |
| Bezeichnung | Kunden Name |
| Nettobetrag | |
| Steuerbetrag | |

</details>

 

<details>
<summary>Suchmöglichkeiten</summary>

| Feld | Bedeutung |
| --- | --- |
| Jahr | Von… Bis… |
| Belegnummer | Von… Bis… |
| Belegdatum | Von… Bis… (Datum) |
| Kunde | Von… Bis… |
| VertreterGruppe | Von… Bis… |
| Tour | Von… Bis… |
| Station | Von… Bis… |
| Periode | Von… Bis… (Monatszahl) |
| Unterklasse | Von… Bis… |
| EK/VK | 0: Alles  
1: Verkauf  
2: Einkauf |
| Kundengruppe | % |
| Oberkunde | Von… Bis… |

</details>

<details>
<summary>Funktionen:</summary>

| Funktion | Beschreibung |
| --- | --- |
| Keine Provision (F7) | |
| Provisionsermittlung (F9) | |

</details>
