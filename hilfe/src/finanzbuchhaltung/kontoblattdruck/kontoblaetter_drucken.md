# Kontoblätter drucken

<!-- source: https://amic.de/hilfe/kontobltterdrucken.htm -->

Hauptmenü > Abschlussarbeiten > Kontoblätter > Kontoblätter bearbeiten > Funktion ***Kontoblätter drucken***

Direktsprung <strong>[KOD]</strong> oder <strong>[KOK]</strong>

Für den Druck des Kontoblattes stehen verschiedene Möglichkeiten zur Verfügung, die sich letztlich nur durch die Möglichkeit der Eingrenzung unterscheiden:

### Kontoblätter drucken:

Man kann hier die Konten eingrenzen, die zu diesem Kontoblatt gehören.

### Kontoblätter Einzelkonten:

Im Gegensatz zum Druck über "Kontenblätter drucken" werden hier zu einem Konto aus allen Kontoblattläufen die Daten herausgesucht. Eine zusätzliche Eingrenzungsmöglichkeit ist hier die Seite des Kontoblattes.

Diese beiden Möglichkeiten basieren auf dem Formulardruck. Für den Kokore und den Konto- / Infoblattdruck existieren Formularvorlagen. Man kann sich jedoch die [Kontoblätter](./formulare_fuer_kontoblaetter_typ_220_einrichten.md) selber im Formulareinrichter gestalten.

Neben dem Formulardruck existiert auch ein fest vorgegebener Crystal Report. Diesen Findet man im Menü unter

Hauptmenü > Abschlussarbeiten > Kontoblätter > Kontoblattdruck

Direktsprung **[KODD]**

Hierbei handelt es sich um einen vordefinierten Crystal-Report, der auf Basis der erstellten Kontoblätter die Informationen zu den Buchungen ausgibt. Man kann hier nach der Laufnummer (KontoBlDruckId) – dann werden nur die Daten dieses Kontoblatts ausgedruckt - oder nach der Jahrnummer eingrenzen – hier werden dann alle Kontoblätter, die in dem ausgewählten Jahr aufgelaufen sind, gedruckt. Wird eine Laufnummer (KontoBlDruckId) angegeben, so wird die eingegebene Jahrnummer ignoriert.

Dieser Report kann auch direkt aus den Auswahllisten „Kontoblätter bearbeiten“ oder „KoKoRe bearbeiten“ über die Funktion ***Kontoblattdruck* SF8** aufgerufen werden.
