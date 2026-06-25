# Pflegerstamm

<!-- source: https://amic.de/hilfe/pflegerstamm.htm -->

Administration > Werkzeuge > Pflegerstamm verwalten

Stammdatenpflege > Stammdatenpfleger > Pflegerstamm verwalten

oder Direktsprung [**PST**]

Der Pflegerstamm verwaltet Metadaten zum automatisieren Aufruf von Stammdatenpflegern. Die bereitgestellten Informationen zu einem Pfleger ermöglichen es,

- die zugehörige Anwendung zu starten
- den Pfleger für Testdatensätze zu begutachten
- mit Hilfe des JPP-Objekts "JPfleger" den Stammdatenpfleger programmatisch aus JPL, MAKRO oder VBA aufzurufen

| **Funktionen der Auswahlliste** | |
| --- | --- |
| Pflege-Funktionen | Neu, Ändern, Ansehen, Löschen<br>Außer „Ansehen“ nur Entwicklung! |
| Test | Test-Aufruf des Stammdaten-Pflegers mit dem unter ‚Test Select‘ zugeordneten SQL Statement ( nur Entwicklung) |
| Anwendung | Start der Anwendung ( nur Entwicklung ) |
| Erzeuge Quelltext-Snippet | Öffnet Editor mit Snippet für Pflegerstamm-Aufruf (Copy&Paste Verwendung) zur Verwendung in JPL ( bei MAKRO Verwendung: ähnlicher Aufbau) |

| **Suchen** | |
| --- | --- |
| Name wie | Suche in „Name“ |
| Maske wie | Suche in „Maske“ |
| Interface | Auswahl nach Interface |
| Rollen zugeordnet? | Ja/Nein |
| Entwickler | Suche in „Entwickler“ |

| **Felder des Pflegers** | |
| --- | --- |
| Name | Pflegerstamm-Name, eindeutiger Bezeichner |
| Bezeichnung | Weiteres Feld für Erläuterungen. |
| Maske | Name der Maske |
| Anwendung | In welcher Anwendung ist dieser Pfleger eingebettet |
| zugehörige Optionbox | Optionbox des Pflegers |
| zuständiger Entwickler | Ansprechpartner für AMIC |
| Version | Festlegung welches Stammdaten-Verfahren intern verwendet wird.<br><ul><li>&nbsp;&nbsp;&nbsp; Jpl_Interface</li><li>&nbsp;&nbsp;&nbsp; Kontext-Interface</li></ul> |
| Ident Select | Mit diesem SQL-Statement wird der zu pflegende Datensatz eindeutig bestimmt. Die Versorgung der optional 4 möglichen Identifizierungsparameter (ID1!..ID4) erfolgt beim Aufruf über das JPP Objekt mit den unter ‚interne Idents‘ festgelegen Parametern |
| Extern Select | Für Relationen, deren Primärschlüssel von der sichtbaren Identifizierung abweicht ( Beispiel Kundenstamm: Kundid = Primär, Kundnummer = sichtbarerer Schlüssel) dient dieses Statement zur Ermittlung der Identifikationen für das eigentliche Ident Select. Also im Beispiel Kundenstamm:<br> select Kundid from Kundenstamm where KundNummer = ':ID1‘ and KundLoeKennz = 0<br> <br>Man kann also im Beispiel Kundenstamm einen Stammsatz über die Kundid oder über die KundNummer ansprechen.<br>Die Versorgung der optional 4 möglichen Identifizierungsparameter (ID1!..ID4) erfolgt beim Aufruf über das JPP Objekt mit den unter ‚externe Idents‘ festgelegen Parameter |
| Test Select | In der Auswahlliste vom Pflegerstamm wird bei Anwahl der Funktion ‚Test‘ dieses Statement zum Lokalisieren eines Datensatzes ausgeführt. Es werden keine ‚:‘-Identifier versorgt. |
| interne Idents | Parameternamen für die Versorgung der IDs von ‚Ident Select‘ beim Aufruf mit dem JPP-Objekt. |
| externe Idents | Parameternamen für die Versorgung der IDs von ‚Extern Select‘ beim Aufruf mit dem JPP-Objekt. |
| Rollenbindung | Optionale Rollenbindungen für die Methoden-Aufrufe<br><ul><li>&nbsp;&nbsp;&nbsp; Ändern</li><li>&nbsp;&nbsp;&nbsp; Ansehen</li><li>&nbsp;&nbsp;&nbsp; Löschen</li><li>&nbsp;&nbsp;&nbsp; Neu<br>Siehe <a href="../firmenstamm/firmenkonstanten/zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenpflegerstamm.md">Rollenpflegerstamm</a></li></ul> |

| **Spezialfunktionen** | |
| --- | --- |
| Rollenbindungshelper (**F10**) | Diese Funktion versucht automatisch die Rollenbindungen zu ermitteln.<br>Sollte das nicht zum Erfolg führen, muss die Rollenbindung „manuell“ erfolgen. Auf jeden Fall ist die Sinnhaftigkeit der ermittelten Ergebnisse zu überprüfen.<br> <br>Siehe [Rollenpflegerstamm](../firmenstamm/firmenkonstanten/zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenpflegerstamm.md) |
