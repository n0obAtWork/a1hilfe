# Vorgangsimport (LVS)

<!-- source: https://amic.de/hilfe/_lvs20_vimp.htm -->

Hauptmenü > Systempflege > Mandantenserver > Mandantenserverprozesse

oder Direktsprung [[MSP]](../../systempflege/mandantenserver/mandantenserver_prozesse.md)

Alle LVS-Buchungen werden als Vorgangsimport mit der Vorgangsklasse 5150 geschrieben. Diese können je nach Bedarf in mehr oder weniger kurzen aber regelmäßigen Abständen gebucht werden.

Regelmäßige Buchungen können über [Mandantenserverprozesse](../../systempflege/mandantenserver/mandantenserver_prozesse.md) erreicht werden.

#### Mandantenserver LVS Buchungsprozess

Der Mandantenserver kann LVS-Buchungen in kurzen Zeitabständen für alle LVS-Buchungen vornehmen. Dazu ist es notwendig, ein Makro 2.0 auszuführen, das alle LVS-Vorgangsimporte im Status 2 abarbeitet.

Nachteil des synchronen Mandantenserverprozesses: Läuft eine andere Buchung des Mandantenservers längere Zeit, so werden erst danach die LVS-Buchungssätze abgearbeitet.

#### Asynchroner Mandantenserverprozess

Der Asynchrone Mandantenserverprozess (Typ 2 – Asynchron (Managed)) prüft in regelmäßigen Abständen, ob ein asynchroner Prozess läuft, der LVS-Buchungen vollzieht. Dazu ist im Controlstring

Folgendes einzugeben:

```text
VIMP_Automat 5150 -1 1
```

Nun wird im Sekunden-Rhythmus nach zu verarbeitenden Buchungssätzen des Typs LVS geguckt und diese werden abgearbeitet.
