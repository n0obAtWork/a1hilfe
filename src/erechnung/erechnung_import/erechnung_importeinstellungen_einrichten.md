# eRechnung Importeinstellungen einrichten

<!-- source: https://amic.de/hilfe/erechnungimporteinstellungenei.htm -->

In der Anwendung eRechnung **[XRE],** Variante ***Import-Vorgänge*** hat die Funktion Importeinstellungen bearbeiten.

Hier richten Sie die Importeinstellungen der eRechnung ein.

| Felder |
| --- |
| Fehlerbehandlung HTML | Gibt an, ob eine nicht erfolgreiche Erstellung einer HTML-Visualisierung als Fehler gelten soll (Default **Ja**) |
| Fehlerbehandlung Kunde | Gibt an, ob eine nicht erfolgreiche Findung eines Kunden/Lieferanten als Fehler gelten soll (Default **Nein**) |
| Fehlerbehandlung Validierung | Gibt an, ob eine nicht erfolgreiche Validierung eines Imports als Fehler gelten soll (Default **Ja**) |
| Fehlerfunktion | Gibt eine Datenbankfunktion an, die die Fehlermeldungen eines Imports aufnehmen und z. B. per E-Mail weiterleiten soll.<br> <br>Als Eingabeparameter wird die ImportId gegeben.<br> <br>Als Vorlage kann hier die ausgelieferte Funktion „AMIC_DEMO_XRE_ImportFehlerFunc“ dienen. |
| Kundenfindungsfunktion | Gibt eine Datenbankfunktion an, die aus den importierten Daten einen Kunden/Lieferanten ermitteln soll.<br> <br>Als Eingabeparameter wird die ImportId gegeben, als Ausgabe wird die KundId des Kunden/Lieferanten erwartet.<br> <br>Als Vorlage kann hier die ausgelieferte Funktion „AMIC_STD_XRE_ImportKundensuche“ dienen. |
| Belegflusspostfach Warenwirtschaft | Standardbelegflusspostfach für eRechnungsimporte im Bereich Warenwirtschaft |
| Belegflusspostfach Finanzbuchhaltung | Standardbelegflusspostfach für eRechnungsimporte im Bereich Finanzbuchhaltung |
