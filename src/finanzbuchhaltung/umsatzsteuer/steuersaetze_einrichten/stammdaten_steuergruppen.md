# Stammdaten Steuergruppen

<!-- source: https://amic.de/hilfe/stammdatensteuergruppen.htm -->

Hauptmenü > Abschlussarbeiten > Umsatzsteuer > Steuern > Funktion Steuergruppe **F6**

Direktsprung **[STS].**

Hierbei handelt es sich um eine Klassifizierung der Steuerbehandlung. In der Fakturierung können dies z.B. Inlands- und Auslandskunden sein, die bei gleichen Artikeln unterschiedlich belastet werden. Darüber hinaus möchte man die Auslandskunden trotz gleicher steuerlicher Behandlung in der Umsatzsteueranmeldung nach Ländern voneinander unterscheiden können. Ein weiterer Fall ist die Behandlung des innerbetrieblichen Warenverkehrs, der separat dokumentiert werden soll. Andere Beispiele sind optierende Landwirte oder die Behandlung von Pauschalsteuersätzen in Reisekostenabrechnungen in der Buchhaltung. Die Verknüpfung von Kunden / Lieferanten an die Steuersätze bei der Fakturierung erfolgt über den Eintrag der Steuergruppe in den Kunden / Lieferantenstamm.

Der Pfleger sieht nur die Erfassung einer Nummer und der Bezeichnung - die zur leichteren Identifizierung dient - vor. Wichtig ist hier, dass die Steuergruppe 0 für Sachkontenbuchungen der Fibu exklusiv vorgesehen ist. Eine einfache Einrichtung könnte so aussehen:

| Gruppe | Beschreibung |
| --- | --- |
| 0 | Sachkonten (Fibu) |
| 1 | Inland |
| 2 | EU innergemeinschaftliche Lieferungen |
| 3 | Drittland |

Weiterhin kann zu jeder Steuergruppe eine Erlösklasse hinterlegt werden. Wenn eine Erlösklasse ungleich 0 eingetragen wird, so wird bei der Bestimmung der Erlös-/Aufwandskonten diese Erlösklasse herangezogen.

Ebenfalls kann im Pfleger das Feld „Intrastat“ gepflegt werden. Dieses Feld gibt an, ob Vorgänge mit dieser Steuergruppe zum Intrastat Export freigegeben sind oder nicht.
