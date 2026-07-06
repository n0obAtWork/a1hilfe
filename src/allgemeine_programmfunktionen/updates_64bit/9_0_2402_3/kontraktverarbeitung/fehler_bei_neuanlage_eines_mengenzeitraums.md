# Fehler bei Neuanlage eines Mengenzeitraums

<!-- source: https://amic.de/hilfe/_90_35655.htm -->

Bei der Erfassung eines neuen Mengenzeitraums konnte es, bei Eingabe einer Menge oder Summe von mindestens 1.000 oder mehr, zu einem Fehler bei der internen Verarbeitung des Tausender-Trennzeichens kommen. In diesem Fall wurde der Punkt als das englische Dezimaltrennzeichen interpretiert, wodurch die Summe um den Faktor 1000 zu niedrig in der Datenbank abgespeichert wurde. Der Fehler wurde nun behoben.

### Releasenote Kategorie:

Ticket: 737982[35655]

Version: 9.0.2402.3

Datum: 08.11.2024

Anwendung: Kontrakte [KTR]

Variante: Kontrakte

Funktion/Report: Mengenzeiträume

[Weitere Informationen](../../../../kontrakt/kontraktstammdaten/zeitraeume_festlegen.md)

#### Tags:

Releasenote, 9.0.2402.3, 35655, 737982
