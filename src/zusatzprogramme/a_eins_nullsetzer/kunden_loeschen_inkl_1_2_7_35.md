# Kunden löschen (inkl. 1+2+7+35)

<!-- source: https://amic.de/hilfe/kundenlscheninkl12735.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

Anschriftstamm unter der Bedingung: where (AdressTyp = 11) or (AdressTyp = 12)

AnschriftAddon unter der Bedingung: where (AdressTyp = 11) or (AdressTyp = 12)

BEMERKPOSITION unter der Bedingung: where (BemerkTyp = 0) or (BemerkTyp = 1)  
BEMERKPOSITIONWERTE unter der Bedingung: where (BemerkTyp = 0) or (BemerkTyp = 1)  
BEMERKSTAMM unter der Bedingung: where (BemerkTyp = 0) or (BemerkTyp = 1)

Bemerkung unter der Bedingung: where (BemerkTyp = 0) or (BemerkTyp = 1)

KundenMatchcode

KundenAusland

KundenBank

KundenBonus

KundenBonusSteu

KundenGrLink

KundenMitglied

KundenNotizen

KundenOberKunde

KundenSuchBegr

KundenSummen

KundenZahlBed

KundenZahlKunde

KundenVersAnschr

KundenStamm

KundenAddon

kundengruppe

kundenmatchcode

kontostamm unter der Bedingung: where kontotyp=2

KONTRAKTGRKUNDE

KONTRGRUPPE

PartieKundListe

PartieLiefListe

Wiedervorlage unter der Bedingung: where (WiederTyp = 2)

BesuchsBericht

kundennachhaltigkeit

amic_sharepoint

UmsatzsteueridListe_SteuerGruppe unter der Bedingung MandantKunde nicht geich 0

UmsatzsteueridListe unter der Bedingung Mandantkunde nicht gleich 0

disposition

MitarbeiterGruppeLink

Anschriftennichtvererben

zeiterfassung

KundenMitglAktion

WarenrueckBASIS

RemoteKunden

KundenTankKarte

KundeMaskeDaten

KundenKredit

KundenInfoZeile

EZG_KundBeitrag

EZG_KundenListe

AHOI

wunschliste

BAUSTKUNDLISTE

BAUSTLIEFLISTE

SPEDISTAMM

KREDITLIMITPROTOKOLL

Beim Löschen der Kunden werden automatisch die [Vorgänge Ware](./vorgaenge_ware_warenstatistiken_loeschen_einschl_kassenbeweg.md), [Vorgänge Finanzbuchhaltung](./vorgaenge_finanzbuchhaltung_loeschen_inkl_29.md) , [Kontrakte](./kontrakte_loeschen.md) und [gelöschte Kunden](./geloeschte_kunden_entfernen_inkl_1_2.md) mit gelöscht.
