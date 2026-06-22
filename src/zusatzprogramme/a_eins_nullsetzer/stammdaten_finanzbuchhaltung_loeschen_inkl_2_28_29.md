# Stammdaten Finanzbuchhaltung löschen (inkl. 2+28+29)

<!-- source: https://amic.de/hilfe/stammdatenfinanzbuchhaltunglsc.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

SachKontStamm

SachKontStammAddon

OberSachKonto

SachKontoZins

SachKontDruckPos

KontoSortierPos

Kontostamm unter der Bedingung where (KontoTyp=1) or (KontoTyp=3)

Beim Löschen der Stammdaten der Finanzbuchhaltung werden automatisch die [Vorgänge Finanzbuchhaltung](./vorgaenge_finanzbuchhaltung_loeschen_inkl_29.md) und die Anlagenkartei [Stammdaten](./anlagenkartei_stammdaten_loeschen_inkl_29.md) und [Bewegungsdaten](./anlagenkartei_bewegungsdaten_loeschen.md) mit gelöscht.
