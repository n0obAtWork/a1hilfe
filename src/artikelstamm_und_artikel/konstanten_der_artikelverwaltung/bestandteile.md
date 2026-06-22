# Bestandteile

<!-- source: https://amic.de/hilfe/_bestandteile.htm -->

Hauptmenü > Stammdatenpflege > Konstanten Artikelstamm > Bestandteile

oder Direktsprung **[ABST]**

Häufig sollen zum Artikel zusätzliche Qualitätsmerkmale und/oder Bestandteile wie zum Beispiel Inhaltsstoffe (unter anderen auch Nähr- und Schadstoffe) erfasst werden. Hier können mittels Nr. und Bezeichnung Bestandteile grundsätzlich definiert werden. Die Zuordnung erfolgt im Artikel­stamm mit der Funktion [Zusammensetzung](../parameter_des_artikelstamms/zusammensetzung.md) (s. dort) unter Angabe spezifischer Werte.

| Maskenfeld | Bedeutung |
| --- | --- |
| Bestandteilnummer | Nummer des Bestandteils. Die Nummer kann eigenständig vergeben werden. |
| Bezeichnung | Bezeichnung des Bestandteils |
| Grenzwert | |
| Format | |
| Einheit | |
| Nutzung in | Folgendes Auswahlmöglichkeiten stehen zur Verfügung<br>1. egal<br>2. Ackerschlagkartei<br>3. Qualitätsdaten<br>4. Partieartikelanalyse<br> |
| Typ Schad/Nährstoff | Folgendes Auswahlmöglichkeiten stehen zur Verfügung<br>1. beides<br>2. Schadstoff<br>3. Nährstoff<br>4. Keins von beiden |
| Sortierung | |
| Feldname(Analyse) | |
| Feldtyp(Analyse) | |
| Qualitätsnummer Waage | Hier wird die Nummer des Feldes Waagenqualität in dem Qualitätsmerkmal des Abrechnungsschemas der Sorte eingetragen. |
| Stoffstrom-Art | (nur bei gültiger Stoffstromdaten-Lizenz)<br>Art des Stoffs für die Berücksichtigung in Stoffstromdaten (per F3-Auswahl) |
| ME-Nummer | (nur bei gültiger Stoffstromdaten-Lizenz)<br>Nummer der Mengeneinheit für die Berechnung von Stoffstrom-Mengen |
| [Stoffstrom-DB-Prozedur](../../zusatzprogramme/stoffstrom_bilanz_daten/stoffstromdatenberechnung_per_datenbankprozedur.md) | (nur bei gültiger Stoffstromdaten-Lizenz)<br>Für die stoffartspezifische Berechnung der Stoffstrommenge einer Position kann an dieser Stelle eine private Datenbankprozedur angegeben werden. |

Für die Gewinnung von Daten zur Unterstützung stromstoffbilanzpflichtiger Betriebe (siehe [Stoffstrom-Bilanz-Daten](../../zusatzprogramme/stoffstrom_bilanz_daten/index.md)) sind die bilanzierungspflichtigen Stoffe in dieser Liste einzutragen und den jeweiligen Artikelstamm-Einträgen unter Angabe der jeweiligen Anteile nach Bedarf über deren [Zusammensetzung](../parameter_des_artikelstamms/zusammensetzung.md) zuzuordnen.  
Notwendige Angaben für Stoffstrom-Bestandteile sind an dieser Stelle lediglich die frei wählbare Bestandteilnummer, Bezeichnung, die Stoffstrom-Art und die Mengeneinheit für die Berechnung der einzelnen Stoffstrom-Mengen.
