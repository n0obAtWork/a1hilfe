# Stammdaten 2 (Kontrakt)

<!-- source: https://amic.de/hilfe/_stammdaten2kontrakt.htm -->

Für weitergehende Abwicklungen stehen weitere Parameter zur Verfügung:

| Stammdaten 2 |
| --- |
| Musterkontrakt  
 | Kennzeichen, ob es sich bei den erfassten Daten um einen Musterkontrakt („Template“) handeln soll. Musterkontrakte werden bei Kontraktauswahlen niemals herangezogen und können auf keinen Fall bebucht werden. Außerdem sind sie natürlich nicht unbedingt einer bestimmten Kontraktgruppe zugeordnet.  
Ein Musterkontrakt kann beim Anlegen eines neuen Kontraktes als Vorbelegungshilfe herangezogen, aber auch für spätere Updates (z. B. Aufnahme neuer Artikel oder Preise in die zugeordneten Kontrakte) verwendet werden.  
Im praktischen Einsatz legt man sich seine typischen Kontrakte mit allen Bedingungen, Texten etc. an und greift dann bei der Neuerfassung hierauf zu und gibt die abweichenden Daten ein. |
| Lagerspezifisch | Ein Kontrakt kann fest an ein Lager gebunden sein, d.h. bei Abholung aus einem anderen Lager werden die Kontraktbedingungen nicht gezogen. |
| Ziellager | Das Feld Ziellager ist ein rein informatives Feld. Es kann ein Lager eingetragen werden. |
| Warengruppenzuordnung | Kennzeichen, ob der Kontrakt warengruppendefiniert ist. Wenn ja, wird bei der Suche nach Kontrakten zu einem Artikel durch alle Warenpositionen gesucht, ob eine dabei ist, die der richtigen Warengruppe zugehört. Es erfolgt dann eine Abbuchung aus dem Kontrakt. |
| Rabatte zulassen | Hier wird entschieden, ob automatische Gruppen- und Zeilenrabatte zulässig sind. |
| RW-Rechnung an HK  
*(nur bei Rohwarekontrakten)* | Wenn ein Kontrakt verschiedenen Kunden/Lieferanten zugeordnet ist, so kann die Rechnungsstellung für den Hauptkunden oder die Lieferanschrift erfolgen. |
| RW-Zahlungspfl. = HK  
*(nur bei Rohwarekontrakten)* | Festlegung, ob die Zahlung durch den Hauptkunden oder den Lieferempfänger erfolgt. |
| Abbuchungsmengen  
*(nur bei Rohwarekontrakten)* | Das Kennzeichen Abbuchungsmenge gibt an, wie die Menge abgebucht wird.  
Hierfür gibt es die Kennzeichen „Brutto“, „Netto“ und „—„. Das Kennzeichen „—„ wird wie „Netto“ behandelt.  
Bei Brutto wird die angelieferte Menge abgebucht.  
Bei Netto wird die Menge abgebucht, welche nach Trocknung, etc. ermittelt wurde. |
| Rohwarezusatz  
*(nur bei Rohwarekontrakten)* | Kennzeichen, ob es sich um einen Rohwaren-Kontrakt handelt, der mit Qualitätsmerkmalen, Sortierartikeln und Kostenarten nebst zugehörigen Tabellen belastet sein könnte. |
| Kontrakt-Währung | Verweis auf die im Kontrakt verwendete Währung. |
| Kontrakt-Sprache | Sprachkennzeichen für z.B. die automatische Verwendung entsprechender Anschreiben.  
Wird mit der Sprache des Kunden (Kundenstamm) vorbelegt. |
| Kontraktausweichliste | Die Kontrakt-Ausweichliste dient dazu, dem Kunden, der einen Kontrakt über eine Auswahl an Artikeln abgeschlossen hat, die Möglichkeit zu geben, ähnliche Artikel, die nicht explizit im Kontrakt erwähnt worden sind, anstelle der aufgeführten Artikel zu vergleichbaren Konditionen abzunehmen. Die Ausweichartikel werden in einer (für den Kunden unsichtbaren) Liste geführt, ohne dass sie in jedem Kontrakt explizit eingetragen sein müssten. Zur Erfassung siehe Kontrakt -Ausweichliste.  
Hier wird die Nummer der Ausweichliste eingegeben. |
| Variante Kontraktdruck  
 | Druckvariante des Kontraktbestätigungsschreibens, siehe dazu „Kontrakt-Varianten“. Ein Verkaufskontrakt kann, in Abhängigkeit davon, um was für ein Verkaufsgeschäft es sich handelt, unterschiedliche Ansprüche an die vertragliche Gestaltung haben. In den Kontraktvarianten können beliebig viele „Vertragsformulare“ definiert werden. Der benötigte wird hier angebunden. Praktisch wird damit eine „Unterklasse“ der Kontraktklasse „Verkaufskontrakt“ definiert. Es empfiehlt sich, durch die Anlage entsprechender Musterkontrakte, die Erfassung zu erleichtern.  
Neben der Ausdruckgestaltung werden in den Kontraktvarianten u.a. auch Textbausteine erfasst. Diese können im Abschnitt 1 der Kontraktstammdaten angesehen und ggf. für den individuellen Kontrakt angepasst werden. Es handelt sich also um (sinnvolle) Vorbelegungen. Wenn in dem Text Stopppositionen festgelegt wurden, so werden diese jetzt nacheinander abgefragt. |
| Variante Erledigungsschreiben  
 | Druckvariante des Erledigungsschreibens.  
(siehe Variante Kontraktdruck) |
| Variante Kontraktstorno | Druckvariante für den Stornobeleg.  
(siehe Variante Kontraktdruck) |
| Standard-Parität | Für den Kontrakt gültige übliche Parität. |
| Vertretergruppe | Zuordnung eines Vertreters zum Kontrakt (Vorbelegung 0 = Vertretergruppe aus Debitoren-/Kreditoren Stamm). |
| Gesperrt | Hier kann der Kontrakt gesperrt werden. Sollte der Kontrakt gesperrt sein, kann man den Kontrakt in der Belegauswahl nicht ziehen. |
| Herkunfts- / Zielland | Herkunftsland oder Zielland des Kontrakts. |
| Herkunfts- / Zielregion | Herkunftsregion oder Zielregion des Kontrakts. |
| Abtretungskonto | Hier kann ein Abtretungskonto hinterlegt werden. Zur Auswahl stehen die Konten, die bei jedem Kunden der Kontraktgruppe hinterlegt sind. |
| BLZ / Bank | Bankleitzahl und Bank des Abtretungskontos |
