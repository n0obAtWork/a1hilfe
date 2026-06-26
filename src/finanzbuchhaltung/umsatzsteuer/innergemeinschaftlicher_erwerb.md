# Innergemeinschaftlicher Erwerb

<!-- source: https://amic.de/hilfe/innergemeinschaftlichererwerb.htm -->

Bei Warenbezug aus dem EU-Ausland tritt an Stelle der Einfuhrumsatzsteuer der Tatbestand des innergemeinschaftlichen Erwerbs (§ 1 Abs. 1 Nr. 5 UStG, § 1 a UStG). Das bedeutet, dass Unternehmer und andere Erwerbssteuerpflichtige für „Importe“ aus anderen EU-Ländern keine Einfuhrumsatzsteuer an den Zoll bezahlen müssen, sondern ihre Erwerbe in der Umsatzsteuer-Voranmeldung beim zuständigen Finanzamt anzumelden haben. In A.eins sind die Steuerklasse 101 und 102 davon betroffen. Um Lieferanten zu kennzeichnen, dass sie dem innergemeinschaftlichen Wahrenverkehr unterliegen, richtet man eine gesonderte Steuergruppe für EU-Kunden/Lieferanten ein. Diese ist dann im Kundenstamm zu hinterlegen.

Einrichtung

Um die Steuer für den Innergemeinschaftlichen Erwerb auf dem USTVA-Formular ausweisen zu können, sind im [Steuersatzpfleger](./steuersaetze_einrichten/stammdaten_steuersaetze.md) alle relevanten Kombinationen dieser Steuergruppe einzutragen. Dabei existieren zwei Möglichkeiten der Einrichtung.

1. Der Innergemeinschaftliche Erwerb wird nicht auf Konten gebucht, sondern nur in dem Umsatzsteuerauswertungen fiktiv errechnet und ausgegeben.  
    

| **Feld** | **Beschreibung** |
| --- | --- |
| Steuerformel | Normale Steuer<br> |
| Steuersatz | 0,00 als tatsächlicher Steuersatz<br> |
| Satz innergm.Erw. | Steuersatz, dem der Artikel eigentlich unterliegt.<br> |
| AW-Kennz. Umsatz | Auswertungsposition zur Steuerung des Umsatzsteuerformulars. In der Beispielliste für [Auswertungspositionen](./steuersaetze_einrichten/stammdaten_auswertungspositionen.md) wären es die Zeilen 350, 360 oder 370 und somit die Kennziffern 89,93 oder 95 je nach Einfuhrsteuersatz.<br> |
| Steuer | Bleibt frei, also 0.<br> |
| Einfuhrsteuer (MwSt) | Hier gehört die Auswertungsposition analog des AW-Kennzeichens hinein, also auch 350, 360 oder 370 je nach Einfuhrsteuersatz. Auf dem Umsatzsteuerformular wird hier dann die fiktive anfallende Erwerbssteuer errechnet und ausgewiesen.<br> |
| Einfuhrsteuer (Vst) | Hier gehört die Auswertungsposition hinein, die die Vorsteuerbeträge aus dem innergemeinschaftlichen Erwerb von Gegenständen kennzeichnet. In der Beispielliste wäre es die Zeile 560. Es wird dann analog zur Einfuhrsteuer1 die errechnete fiktiv anfallende Erwerbssteuer wieder abgezogen.<br> |

<em>2.</em> Es werden zwei weitere Steuerzeilen an den Belegen angehängt, die die Steuer auf den Konten ausweisen. Um dies zu erreichen, sind folgende Einstellungen notwendig.  
    
**ACHTUNG**: *Die Steuerzeilen werden erst zum Zeitpunkt des Buchens erzeugt.  
*  

| **Feld** | **Beschreibung** |
| --- | --- |
| Steuerformel | Innergemeinschaftlicher Erwerb<br> |
| Steuersatz | 0,00 als tatsächlicher Steuersatz ( Feld ist gesperrt)<br> |
| Satz innergem.Erw. | Steuersatz, dem der Artikel eigentlich unterliegt.<br> |
| Konto innergem.Erw. MwSt. | Steuerkonto auf das die anfallende Erwerbssteuer gebucht werden soll.<br> |
| Konto innergem.Erw.VSt. | Steuerkonto von dem die anfallende Erwerbssteuer abgezogen werden soll.<br> |
| AW-Kennz. Umsatz | Auswertungsposition zur Steuerung des Umsatzsteuerformulars. In der Beispielliste für [Auswertungspositionen](./steuersaetze_einrichten/stammdaten_auswertungspositionen.md) wären es die Zeilen 350, 360 oder 370 und somit die Kennziffern 89,93 oder 95 je nach Einfuhrsteuersatz.<br> |
| Steuer | Bleibt frei, also 0. (Feld ist gesperrt)<br> |
| Einfuhrsteuer (MwSt) | Hier gehört die Auswertungsposition analog des AW-Kennzeichens hinein, also auch 350, 360 oder 370 je nach Einfuhrsteuersatz. Es werden die im Beleg gebuchten Werte herangezogen.<br> |
| Einfuhrsteuer (Vst) | Hier gehört die Auswertungsposition hinein, die die Vorsteuerbeträge aus dem innergemeinschaftlichen Erwerb von Gegenständen kennzeichnet. In der Beispielliste wäre es die Zeile 560. Es werden die im Beleg gebuchten Werte herangezogen.<br> |

In den Auswertungen werden beide Arten gleichberechtigt behandelt. Eine Umstellung der Steuerformel auf „innergemeinschaftlichen Erwerb“ ist also auch unterjährig möglich.
