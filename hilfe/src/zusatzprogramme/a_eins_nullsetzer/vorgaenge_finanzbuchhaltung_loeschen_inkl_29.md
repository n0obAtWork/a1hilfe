# Vorgänge Finanzbuchhaltung löschen (inkl. 29)

<!-- source: https://amic.de/hilfe/vorgngefinanzbuchhaltunglschen.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

Datevposition

Datevstamm

fibuvorgposition

fibuvorgpostext

fibuvorgposwabew

vorgfibulink

fibuvorgproto

fibuvorgreserv

fibuvorgstamm

FIBUVORGDELPROTO

RFSDTALAUF

RFSZAHLUNG

journal

journalfreigabe

journalposition

Kontosummen

Kontozaehler

KostStelSummen

KOSTENTRAEGERSUMMEN

KOSTENSUMMEN

SteuersatzSummen

Zahlungslauf

Zahlungsbeleg

Zahlungsposition

Zahlvorschlag

Zahlvorschliste

zahlvorschlposit

ZAHLVORSCHPROTOKOLL

MahnPosition

Mahnung

mahnvorschlag

mahnvorschliste

mahnvorschposit

zinsabrechnung

zinsabrposition

zinsliste

offenerposten

wechselabrechnung

wechselabrposit

wechselstamm

wechselprolong

kontoblattstamm

kontoblattposit

kontoblattzaehl

fibuwiedzahlung

FibuWiedZahlPosition

EINGANGSMAPPELEM unter der Bedingung: where EinMappTyp=1

FIBUVORGUNGEBU

FIBUVORGKOSTSTEL

FIBUVORGKOSTENTRAEGER

AMIC_DTAUS_ASATZ

AMIC_DTAUS_CSATZ

DTADISKHEADER

DTADISKPOSITION unter der Bedingung: where 1=1

DTADISKPOSTEXT unter der Bedingung: where 1=1

DTADISKVERTEIL

EBILANZ_GAAP_RESULT_HEADER unter der Bedingung: where 1=1

FIBUVORGEXPORT

OPAUSLAND

OPAUSLANDSTAT

fibumailversand

meinfibumailversand

fibuwaehrsumme

Es werden die Daten in folgenden Tabellen aktualisiert:

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'ZAHLUNGSLAUF'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'MAHNVORSCHLISTE'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'ZINSLISTE'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'ZAHLVORSCHLISTE'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'MAHNUNG'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'WECHSELSTAMM'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'WECHSELABRECHNUNG'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'FIBUVORGRESERV'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'ZAHLUNGSBELEG'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = 'KONTOBLATTSTAMM'

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = ‘AMIC_DTAUS_ASATZ‘

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = ‘AMIC_DTAUS_CSATZ‘

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = ‘EBILANZ_GAAP_RESULT_HEADER‘

ident mit Aktualisierung: set identident = 1 unter der Bedingung: where identcolumnname = ‘FIBUVORGEXPORT‘

Beim Löschen der Vorgänge der Finanzbuchhaltung werden automatisch die Anlagenkartei [Bewegungsdaten](./anlagenkartei_bewegungsdaten_loeschen.md) mit gelöscht.
