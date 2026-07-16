# Rabatte löschen (inkl. 1)

<!-- source: https://amic.de/hilfe/rabattelscheninkl1.htm -->

In folgenden Relationen werden die Datensätze entfernt:

Rabattklasse (ohne die 0 (ohne Rabatt) zu entfernen)  
Artirabattgruppe (ohne die 0 (kein Rabatt) zu entfernen)

ArtiRabattArt

ArtiRabattTyp  
ArtiRabattSatz  
ArtiRabattText

In folgenden Relationen wird die 0 für ‚ohne Rabatt‘ eingetragen:

Kundenstamm (Felder RabKlNummerEK, RabKlNummerEKI, RabKlNummerVK, RabKlNummerVKI)  
Artikel (Felder ArtiRabGrupEK, ArtiRabGrupEKI, ArtiRabGrupVK, ArtiRabGrupVKI)

BaustArtikel (Felder ArtiRabGruppeVK, ArtiRabGruppeEK)

Beim Löschen der Rabatte werden automatisch die [Vorgänge Ware](./vorgaenge_ware_warenstatistiken_loeschen_einschl_kassenbeweg.md) mit gelöscht.
