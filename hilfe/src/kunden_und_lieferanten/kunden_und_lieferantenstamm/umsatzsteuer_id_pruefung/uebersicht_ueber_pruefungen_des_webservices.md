# Übersicht über Prüfungen des Webservices:

<!-- source: https://amic.de/hilfe/_ustidprfguebersicht.htm -->

Mit dem Direktsprung [USTID] erreicht man die Historie der Umsatzsteuer-Id-Prüfungen.

Hier kann nach Umsatzsteuer-Id, Vorgang oder Kunde gefiltert werden.

#### Aufruf aus anderen Auswahllisten

Ein Aufruf des Direktsprungs aus der Kundenauswahlliste belegt die Auswahl mit dem markierten Kunden vor.

Ein Aufruf des Direktsprungs aus einer Vorgangsauswahlliste belegt die Auswahl mit dem markierten Vorgang vor.

#### Informationen

Die Auswahlliste enthält die folgenden Informationen:

| Feld | Beschreibung |
| --- | --- |
| Prüfung | Diese Spalte gibt an, ob die Prüfung aus einem Kundeneintrag oder aus einem Vorgang heraus initiiert wurde. |
| UstId | Umsatzsteuer-Id, die geprüft werden sollte |
| Kunde | Kundennummer des zu prüfenden Kunden |
| Vorgang | Vorgangsnummer, wenn Vorgangsanfrage |
| Zeitstempel | Zeitstempel der letzten Änderung des Eintrags |
| Status | <ul><li>Neu – ein noch nicht verarbeiteter Auftrag</li><li>Erledigt – ein bearbeiteter Auftrag</li></ul> |
| Name | Prüfergebnis zum Namen \*) |
| Straße | Prüfergebnis zur Straße \*) |
| PLZ | Prüfergebnis zur Postleitzahl \*) |
| Ort | Prüfergebnis zum Ort \*) |
| Code | Ergebniscode des Webservices<br>Mehr dazu auf der Webseite [https://evatr.bff-online.de/eVatR/xmlrpc/codes](https://evatr.bff-online.de/eVatR/xmlrpc/codes) |
| Info | Eine Zusatzinfo, die ggf. angibt, ob der Kunden oder der Vorgang (noch) nicht existieren – dies sollte i.d.R. leer sein |
| Prüfauftrag | Eine Prüfauftrags-Guid kann optional angezeigt werden. Diese dient dem Support zur Identifikation des Eintrags in der Datenbank bei einer Datenrecherche. |

\*) Nicht immer ist das Ergebnis für Name, Straße, PLZ oder Ort eindeutig Okay oder falsch.

Der Webservice stellt hier vier Antwortmöglichkeiten bereit:

- A = stimmt überein
- B = stimmt nicht überein
- C = nicht angefragt
- D = vom EU-Mitgliedsstaat nicht mitgeteilt
