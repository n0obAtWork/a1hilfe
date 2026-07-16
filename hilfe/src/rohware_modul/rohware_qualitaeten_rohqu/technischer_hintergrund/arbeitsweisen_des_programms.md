# Arbeitsweisen des Programms

<!-- source: https://amic.de/hilfe/arbeitsweisendesprogramms.htm -->

- Wird ein Labordatensatz erfasst oder geändert und als gültiger Datensatz des Belegtyps erstellt, so werden aus der Tabelle „Rohware_Qual_Nachtrag“ bestehende Daten dieses Belegtyps geladen, ergänzt, bzw. abgeändert und in diese Tabelle zurückgeschrieben. Von dort werden die Daten zur Erstellung der Belegsabrechnungen verwendet.
- Wird ein Beleg gelöscht, so werden aus der Tabelle „Rohware_Qual_Nachtrag“ bestehende Daten dieses Belegtyps geladen. Die Werte des bestehenden Datensatzes werden entfernt und verbliebene Daten in die Tabelle zurückgeschrieben.
- Die Änderungen in der Tabelle „Rohware_Qual_Nachtrag“ erfolgen ohne Sicherungssperren. Ist der betreffende Satz zum Zeitpunkt der Änderung bereits mit dem Einspielkennzeichen (abgerechnet) versehen, so wird der Anwender bei Eingabe der Änderung gewarnt. Beim Speichern erfolgt ein Eintrag ins Fehlerprotokoll als Warnung.
