# openTRANS

<!-- source: https://amic.de/hilfe/_ebillingopentrans.htm -->

**openTRANS-Kunde**

Gibt an, ob es sich um einen Kunden handelt, dessen Vorgänge beim Drucken als openTRANS exportiert werden sollen und ob ggf. ein Druck gegen die Erstellung eines openTRANS-Exports ersetzt wird.

| Wert | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 0 | Nein | Es wird kein Belegversand verwendet |
| 1 | Mit Rechnungsdruck | Der openTRANS-Export erfolgt analog zum Rechnungsdruck |
| 2 | Statt Rechnungsdruck | Der openTRANS-Export erfolgt anstelle des Rechnungsdrucks<br>Hinweis:<br>Bitte beachten Sie, dass diese Einstellung „Statt Rechnungsdruck“ unter Umständen die Einstellung „mit Rechnungsdruck“ im Bereich Belegversand überschreibt. Ist eine dieser Einstellungen auf „statt Rechnungsdruck“ gestellt, so entfällt der Druck! |

**PDF Signierung**

Dieses Kennzeichen gibt an, ob Vorgänge für diesen Kunden im Formulararchiv signiert werden sollen und ob ggf. im PDF die openTRANS-Information in den PDF-Daten enthalten sein soll.

**Warnungen**

Nicht alle Besonderheiten der Warenwirtschaft, die in A.eins abgebildet werden, sind openTRANS-kompatibel. Da openTRANS sowohl für die Kommunikation von A.eins zu A.eins eingesetzt wird, als auch für die Kommunikation über die e-Billing-Schnittstelle, die die Einhaltung des openTRANS-Standards verlangt, gibt es verschiedene Vorgehensweisen, wie mit diesen Unwägbarkeiten umgegangen werden soll:

| Wert | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 0 | anzeigen - bereinigter Export | Die Meldung wird als Fehler im Fehlerprotokoll ins Fehlerprotokoll eingetragen. Die inkompatiblen Elemente werden ausgelassen. |
| 1 | anzeigen - voller Export | Die Meldung wird als Warnung ins Fehlerprotokoll eingetragen. Die inkompatiblen Elemente werden exportiert. |
| 2 | nicht anzeigen - voller Export | Es werden keine Meldung ins Fehlerprotokoll geschrieben. Die inkompatiblen Elemente werden exportiert. |
