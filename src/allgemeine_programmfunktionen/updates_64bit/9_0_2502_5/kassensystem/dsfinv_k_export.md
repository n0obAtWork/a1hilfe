# DSFinV-K Export

<!-- source: https://amic.de/hilfe/_90_37461.htm -->

Beim Export der Kassendaten gemäß DSFinV-K kam es bislang zu einem Fehler im Bereich Referenzen, wenn mehrere offene Posten in einer einzigen Zahlung mit der Option „Zahlungsmeldung für Kreditrechnungen“ verarbeitet wurden. Dieses wurde nun korrigiert. Der REF_TYP wird beim Erzeugen der Daten berücksichtigt. Es wird nur noch ein Eintrag pro REF_TYP erzeugt unabhängig von der Anzahl der Offenen Posten in der Zahlung. Das REF_DATUM wird ausschließlich eingetragen, wenn der REF_TYP auf Transaktion steht.

Releasenote Kategorie:

Ticket: 747791[37461]

Version: 9.0.2502.5

Datum: 15.10.2025

Anwendung: DSFinV-K Export

Variante: DSFinV-K Export

Funktion/Report: Export erzeugen

[Weitere Informationen](http://www.amic.de/hilfe/_dsfinvkexport.htm)

Tags:

Releasenote, 9.0.2502.5, 37461, 747791
