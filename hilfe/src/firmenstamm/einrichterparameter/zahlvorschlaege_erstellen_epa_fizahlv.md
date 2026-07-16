# Zahlvorschläge erstellen (EPA FIZAHLV)

<!-- source: https://amic.de/hilfe/_EPA_FIZAHLV.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Bei Zahlungsausgang Fehlerhinweis bei fehlender Bankverbindung | Ja | Für den DTA ist eine gültige Bankverbindung notwendig. Daher wird die Bankverbindung getestet und am Ende ein entsprechender Fehlerhinweis gebracht. Diesen Test kann man hier ausschalten. |
| Bei Zahlungseingang Fehlerhinweis bei fehlender Bankverbindung | Ja | Für den DTA ist eine gültige Bankverbindung notwendig. Daher wird die Bankverbindung getestet und am Ende ein entsprechender Fehlerhinweis gebracht. Diesen Test kann man hier ausschalten. |
| Bei Fremdwährung auf nicht verrechnete OP´S testen | Nein | Werden Zahlungsvorschläge für eine Fremdwährung erstellt, so werden nur die Belege in dieser Währung verrechnet. Existieren z.B. Belege in Buchwährung, so werden diese nicht mit verrechnet. Stellt man diesen EPA auf JA, so erscheint in diesem Fall am Ende ein Hinweise |
| Manuelle Auswahl erlaubt?<br> | Nein | Es gibt hier die Möglichkeit innerhalb der Eingrenzung die Belege manuell auszuwählen. Diese Methode ist veraltet und wird nicht mehr unterstützt. Aus Gründen der Abwärtskompatibilität kann man sich diese Alte Funktion hier wieder aktivieren. |
| Auswahl Ware + Fibu-Belegklasse SO | Nein | Im Standard kann man als Belegherkunft auswählen, ob nur Belege aus der Ware oder aus der Fibu verwendet werden sollen. Stellt man hier diesen EPA auf **Ja** kann man zusätzlich noch auswählen, ob alle Belege aus „Ware + Fibu Belegklasse SO“ herangezogen werden sollen. |
| Datenbankfunktion zur Bestimmung der Kundenbank | | Hier kann eine Datenbankfunktion hinterlegt werden, mit deren Hilfe die zu verwendende Bank bestimmt wird. Sie hat drei Parameter vom Typ Integer. Dies sind die Fibuv_id, der Fibuv_poszaehler und die Kundid. Als Ergebnis wird der Bankkontozaehler vom Typ Integer erwartet. |
| SEPA Bankarbeitstage vor Fälligkeit bei Erstlastschrift | 5 | Die erste Basis-Lastschrift muss 5 Bankarbeitstage (sogenannte TARGET-Tage) vor Fälligkeit bei der Bank des Zahlungspflichtigen eingereicht werden. Hier kann ggf. die Laufzeit mit berücksichtigt werden. |
| SEPA Bankarbeitstage vor Fälligkeit bei Folgelastschrift | 2 | Folge -Lastschriften müssen 2 Bankarbeitstage (sogenannte TARGET-Tage) vor Fälligkeit bei der Bank des Zahlungspflichtigen eingereicht werden. Hier kann ggf. die Laufzeit mit berücksichtigt werden. |
| SEPA Bankarbeitstage vor Fälligkeit bei Firmenlastschrift | 1 | Bei der SEPA-Firmen-Lastschrift beträgt die Frist mindestens einen Geschäftstag vor Fälligkeit bei Erst- und Folgelastschriften. |
| SEPA Bankarbeitstage vor Fälligkeit bei Eillastschrift | 1 | Bei der SEPA-Basislastschrift mit verkürzter Vorlauffrist beträgt die Frist nur einen Tag. Hier kann ggf. die Laufzeit mit berücksichtigt werden. |
| SEPA Maximale Vordatierung des Ausführdatums(Kalendertage) | 15 | Hier wird festgelegt, wieviel Tage nach Einreichung das Ausführungsdatum liegen darf. Diese Angabe erfolgt in Kalendertagen. Gibt man 0 an, wird keine Prüfung durchgeführt. Ansonsten wird das Datum ggf. nach unten korrigiert. |
| Bei Zahlungsart „DTINT trotz SEPA“ die SEPA Tests durchführen? | Ignorieren | Man kann, wenn man das DTINT-Verfahren verwendet, in den Zahlungsarten einstellen, dass trotz SEPA-Einstellungen die Belege zum DTINT Verfahren herangezogen werden. Ob dann die Test auf existierende IBAN, BIC bzw. Mandat durchgeführt werden, kann hier eingestellt werden. |
