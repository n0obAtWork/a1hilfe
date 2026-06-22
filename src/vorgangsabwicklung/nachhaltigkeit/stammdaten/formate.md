# Formate

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_formate.htm -->

Die Nachhaltigkeit hat einige spezielle AF – Formate, die gepflegt werden müssen.

Nachhaltigkeitsstatus (Format AF_NACHHSTAT)

Auf Anforderung wurde das Nachhaltigkeitskennzeichen am Kunden nicht als ja/nein-Kennung eingebaut. Stattdessen ist untenstehendes Anwenderformat einzurichten, das an folgende feste Bedeutung gebunden ist.

| Wert | Beschreibung |
| --- | --- |
| 0 | undefiniert |
| 1-9 | nicht nachhaltig |
| \>=10 | nachhaltig |

Zertifikatstyp (Format AF_NAHA_ZERT)

Relevant sind hier die Typen 4 und 5, deren numerische Repräsentation als verbindlich anzusehen ist im Hinblick auf die Standard-Zulieferfunktionen für die Nachhaltigkeitsinformation.

| Wert | Beschreibung |
| --- | --- |
| 4 | Alle Warenlieferanten, die eine Selbsterklärung abgegeben haben oder selbst zertifiziert sind. |
| 5 | Das eigene Zertifikat, das bei Warenausgängen auszuweisen ist. Der Typ 5 kann systemweit nur einmal vorkommen bei einem Konto, das als Platzhalter für die eigene Firma steht. |
| Alle anderen | Werden für die Nachhaltigkeit nicht gebraucht. |

Die Nummernangaben im Kommentarfeld müssen mit angegeben werden!

Zertifikate vom Typ 4 gelten für den Einkauf und müssen für den im Beleg angegebenen Kunden auf der Kundenstammmaske auf dem Tabreiter Zertifikate als Zertifikat eingerichtet sein.

Das Zertifikat mit dem Typ 5 wird nur in dem Systemkunden eingetragen. Diesen findet man heraus oder richtet diesen ein auf der Mandantenstammmaske unter dem Direktsprung [MND] im Feld Systemkundennnummer. Die Zertifikateinrichtungen für den Systemkunden regeln den gesamten Verkauf und Belege mit den Vorgangsklassen 5100, 5110, 5120, 5200, 5210 und 5220.

Zertifizierungsmethode (Format AF_ZERTMETH)

Angegeben werden die Zertifizierungsinstitutionen. (z.B. ISCC, REDcert, Selbsterklärung)

Kategorie des Zertifikats (Format AF_ZERTKATEG)

Dieses Format dient der Kategorie des Zertifikats, aktuell wird die Kategorie nur zur Auswertung in der [Bewegungsübersicht](../auswertungen/bewegungsuebersicht.md) verwendet.
