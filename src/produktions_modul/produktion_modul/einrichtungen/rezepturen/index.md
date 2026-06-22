# Rezepturen

<!-- source: https://amic.de/hilfe/_rezepturenrez.htm -->

Hauptmenü > Produktion / Abwicklung > Produktion Stammdaten > Rezepturen

oder Direktsprung **[REZ]**

Hier sind die eigentlichen Rezepturen einzugeben. Es wird unterschieden zwischen „Rezepturen“ und „Handelsstücklisten“. In der Auswahlliste ist für den hier diskutierten Fall „Rezeptur“ zu wählen.

| Felder |
| --- |
| Rezepturgruppe | Rezepturgruppe zu der die Rezeptur gehören soll |
| Rezepturnummer | Nummer der Rezeptur<br>Die Rezepturnummer darf nicht größer als 32767 sein. |
| Bezeichnung (Rezeptur) | Name der Rezeptur |
| Gültigkeit | Zeitraum der Verwendung |
| Gesperrt | Darf (evtl. vorübergehend) nicht verwendet werden |
| Verwendungstyp | Gibt an, wo die vorliegende Rezeptur verwendet werden kann:<br>• Ausschließlich in der Produktion (0 – Produktion)<br>• Ausschließlich in der Vermahlung (3 - Vermahlung)<br>• In allen Ausprägungen (1 - Alle)<br>Dieser Verwendungstyp steht nur noch im Ändern-Fall zur Auswahl, wenn eine Rezeptur mit diesem Typ angelegt wurde. Im Neu-Fall wird dieser Verwendungstyp ab sofort nicht mehr angeboten.<br>• Ausschließlich in der NzuM-Produktion (4 - NzuM-Produktion) |
| Anteile: Typ | Bei der Rezepturerfassung wird gegen die hier formulierten Werte geprüft. Mit Prüfung heißt, dass eine Verprobung zur rechts dazu einzugebenden Summe erfolgt.<br>• Prozent mit Prüfung: prüft Prozentsumme<br>• Prozent ohne Prüfung: keine Prozentprüfung<br>• je ME ohne Prüfung: Stückliste ohne Mengenprüfung<br>• je ME mit Prüfung: Stückliste mit Mengenprüfung |
| Anteile: Summe | Falls Anteile geprüft werden sollen, muss hier die Prüfsumme eingetragen werden |
| Rezeptgröße | Diese Rezeptgröße beschreibt die produzierte Menge des Rezeptes.<br>Werden zum Beispiel drei Komponenten zu gleichen Anteilen zusammen gemischt, kann für jede Komponente die Menge 1kg und die produzierte Menge 3kg angegeben werden. So werden periodische Kommazahlen bei den Mengenangaben vermieden.<br>Es wird empfohlen, die Rezeptgröße 0 nicht zu verwenden, da es sonst zu Problemen und unerwarteten Ergebnissen bei der Verwendung der Rezeptur kommen kann.<br> <br>Für die NzuM-Produktion erscheint hinter diesem Feld ein Feld auf das sich die Rezeptgröße bezieht. Die Auswahl im Feld Anteile: Typ muss dafür auf ‚je ME ohne Prüfung‘ oder auf ‚je ME mit Prüfung‘ stehen<br>Zur Auswahl steht:<br>0 - nur Hauptprodukt<br> die Rezeptgröße bezieht sich nur auf das Hauptprodukt<br>1 - Summe Produkte<br> die Rezeptgröße bezieht sich auf alle Produkte |
| Mengeneinheit | Zur Berechnung Produkt-Komponente<br>% oder Stückangaben beziehen sich auf 1 ME Rezept (wichtig bei Pauschal-Komponenten!) |
| Bewertungstyp | • 0 - ohne Verprobung: Wert Produkt ungleich Wert Komponenten möglich<br>• 1 - Komponenten addieren: Wert Komponenten bestimmen Wert Produkt<br>• 2 - Produkt anteilgewichtet: Produktwert wird nach (Mengen) Anteilen auf Komponenten verteilt<br>• 3 - Produkt wertgewichtet: Produktwert wird nach Anteilswert auf Komponenten verteilt<br>• 4 - Produkt Preisminderung<br> <br>Bei der NzuM-Produktion stehen Auswahl 3 und 4 nicht zur Verfügung.<br> <br>„Ohne Verprobung“ übernimmt die Preisfindung des Produktes; Komponentenpreise werden nicht berücksichtigt. Die Verknüpfung zwischen Produkt und Komponenten ist für die Preisfindung also ohne Bedeutung.<br>Bei der „Komponenten-Addition“ ergibt sich der Produktpreis aus der Addition der Komponenten. In diesem Fall spielt die Preisfindung des Hauptartikels keine Rolle.<br>Die folgenden Fälle gehen davon aus, dass Hauptartikel und Summe der Komponenten aufgehen muss. Die Preisfindung (automatische Preisfindung oder manuelle Preiseingabe) findet auf dem Hauptartikel statt. Wenn die Summe der Komponenten aus deren Preisermittlung nicht mit dem Hauptartikel übereinstimmt, dann erfolgt eine wertgewichtete Verminderung oder Erhöhung auf der Komponentenseite nach folgenden Methoden:<br>Bei „Produkt anteilgewichtet“ wird der Betrag einer Komponente aus dem Produktbetrag ermittelt. Als Verteilschlüssel dient das Verhältnis der Komponentenanteile untereinander. Das jeweilige Verhältnis multipliziert mit dem Produktgesamtbetrag ergibt den Komponentenbetrag. Der Einzelpreis einer Komponente ergibt sich dann mittels Division durch die Menge. Achtung: Auf Grund der Rundung und Speicherung auf / von 2-Nachkommastellen kann es zu Rundungsdifferenzen kommen!<br>Bei „Produkt wertgewichtet“ wird der Preis einer Komponente aus dem Produktpreis ermittelt. Als Verteilschlüssel dient das Verhältnis der Komponentenpreise zum Zeitpunkt vor der Erfassung – je nach eingestelltem Verfahren zur Bewertung der Komponenten.<br>Das jeweilige Verhältnis multipliziert mit dem Produktpreis ergibt dann den neuen Komponentenpreis. Dieser Preis multipliziert mit der Menge ergibt dann den Betrag der Komponente. In diesem Fall kann es zu Rundungsdifferenzen bei den Beträgen kommen. Um die Identität zwischen Summe der Komponentenbeträge und Produktbetrag sicher zu stellen, wird dieser dann um diese Differenz erhöht.<br> <br>Hinweis<br>Manuell geänderte Preise werden bei gewichteten Verteilungen nicht berücksichtigt.<br>Sind alle Preise manuell verändert, kann nichts mehr verteilt werden.<br>Der Beleg geht evtl. dann nicht mehr zu Null auf. |
| Individuell | Dieses Feld zeigt bei „Ja“ die Felder Planmenge, Erledigt, Disponiert, Offen auf der Maske an. |
| Variable Komponenten | Dieses Feld ist nur sichtbar, wenn der Bewertungstyp auf „ohne Verprobung“ oder „Komponenten addieren“ steht. Stellt man das Feld für diese beiden Fälle auf Ja, dann besteht die Möglichkeit Komponenten bei Verwendung der Rezeptur variabel zu gestalten, also welche auszutauschen oder weitere hinzuzufügen. |
| Währung | Dieses Feld legt die zu verwendende Währung fest. |
| Komponentenläger | Vorbelegung: nicht änderbar<br>Steht dieses Feld auf ‚änderbar‘, ist auf der Maske für die Zusammensetzung das Feld Lager editierbar. |
| Voreinstellung des Mengenkontrollschalters: | Die Vorbelegung des Häkchen-Feldes ‚Mengenkontrolle zwischen Produkt und Komponenten aktiv‘ auf der Komponentenmaske kann hier im Rezept getrennt nach Erfassung und Korrektur entschieden werden. |
| bei Beleg-Erfassung | Zur Auswahl steht für die Erfassung von Belegen:<br>0 - laut Masken-EPA<br>1 - mit Mengenkontrolle<br>2 - ohne Mengenkontrolle |
| bei Beleg-Korrektur | Zur Auswahl steht für die Korrektur von Belegen:<br>0 - laut Masken-EPA<br>1 - mit Mengenkontrolle<br>2 - ohne Mengenkontrolle<br>3 - letzte Vorgangseinstellung |
| Preisnachkommastellen | Nur für die Verwendungstypen Produktion oder Vermahlung eingebbar.<br>Es können maximal 6 Nachkommastellen verwendet werden. |
| Gegenzeilenausgleich | Nur für den Verwendungstype Vermahlung eingebbar.<br>Wird hier „ja“ eingetragen, so wird die Wertdifferenz zwischen Produkt und Komponenten mit der **1\. Gegenposition** ausgeglichen. |
| Produktpartie anteilig | Komponentenpartien mit Produkt anteilig bebuchen<br> <br>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Nein. |
| korrekte Bewertung | Umrechnung von Preiseinheiten der Bewertung auf korrekte Weise.<br> <br>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Ja. |
| Preise aus Tabellen | Bewertungspreise mit Einheiten richtig übernehmen.<br> <br>Das Feld ist nicht sichtbar für den Verwendungstyp NzuMProduktion und vorbelegt mit Ja. |
| Auflösungstyp | Bei den Verwendungstypen Produktion, NzuM-Produktion und Vermahlung wird dieses Feld mit dem Wert „sichtbar“ gefüllt und ist auf der Maske für den Kunden nicht sichtbar.<br> <br>Typangaben beziehen sich auf die Verwendung der Rezepturmechanik für Handels-Stücklisten, werden also im Bereich Produktion nicht ausgewertet:<br>• manuell: Produktion manuell<br>• verdeckt: Stücklistenauflösung verdeckt<br>• sichtbar: Stücklistenauflösung sichtbar<br>• Umbuch verdeckt: belegbez. Umbuchung verdeckt<br>• Umbuch sichtbar: belegbez. Umbuchung sichtbar<br> |
| Buchungstyp | Bei den Verwendungstypen Produktion, NzuM-Produktion und Vermahlung wird dieses Feld mit dem Wert „beide Seiten“ vorbelegt und ist auf der Maske für den Kunden nicht sichtbar.<br> <br>Nur für eine Handelsstückliste kann man sich entscheiden, ob das Produkt oder seine Komponenten gebucht werden sollen<br>0 – beide Seiten<br>1 – Komponenten<br>2 - Produkte |

 

<p class="siehe-auch">Siehe auch:</p>

- [Produktpreise F11](./produktpreise_f11.md)
- [Zusammensetzung F10](./zusammensetzung_f10.md)
