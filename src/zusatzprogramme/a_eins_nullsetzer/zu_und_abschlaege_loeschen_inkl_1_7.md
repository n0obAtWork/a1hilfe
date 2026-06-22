# Zu- und Abschläge löschen (inkl. 1+7)

<!-- source: https://amic.de/hilfe/zuundabschlgelscheninkl17.htm -->

In folgenden Relationen werden die Datensätze entfernt:

ZUABSCHKLASSE (ohne die 0 (ohne Zu-/Abschlag) zu entfernen)

ArtiZuAbGruppe (ohne die 0 (keine Zu-/Abschlaggruppe) zu entfernen)

ARTZUABTYP

ARTZUABTEXT

ARTZUABZAHLART

ARTZUABZAHLSATZ

ARTZUABVERSART

ARTZUABVERSSATZ

ARTZUABGENERELL

ARTZUABGENSATZ

ArtiZuAbGewicht

ArtZuAbGewiSatz

ArtiZuAbGebinde

ArtZuAbGebiSatz

ArtZuAbLaufzeit

ArtZuAbLaufsatz

ArtZuAbLiefMen

ArtZuAbLiefSatz

In folgenden Relationen wird die 0 eingetragen:

Kundenstamm (Felder ZuAbKlNummerEK, ZuAbKlNummerEKI, ZuAbKlNummerVK, ZuAbKlNummerVKI)

Artikel (Felder ArtiZuAbGrupEK, ArtiZuAbGrupEKI, ArtiZuAbGrupVK, ArtiZuAbGrupVKI)

BaustArtikel (Felder ArtiZAGrNummerVK, ArtiZAGrNummerEK)

Beim Löschen der Zu- und Abschläge werden automatisch die [Vorgänge Ware](./vorgaenge_ware_warenstatistiken_loeschen_einschl_kassenbeweg.md) und [Kontrakte](./kontrakte_loeschen.md) mit gelöscht.
