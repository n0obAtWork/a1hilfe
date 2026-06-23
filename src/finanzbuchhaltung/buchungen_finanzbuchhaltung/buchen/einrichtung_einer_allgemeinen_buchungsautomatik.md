# Einrichtung einer allgemeinen Buchungsautomatik

<!-- source: https://amic.de/hilfe/einrichtungeinerallgemeinenbuc.htm -->

Hauptmenü > Systempflege > Mandantenserver > Mandantenserver Protokoll

oder Direktsprung **[MSP]**

Um einen allgemeinen automatischen Buchungslauf einzurichten kann ein Mandantenserver-Prozess verwendet werden.

Hierzu dient der Menüpunkt „Mandantenserver Protokoll“ (Direktsprung MSP):

Über die erste Variante „Prozesse im Mandantenserver“ kann nun mit „F8 – Neu“ ein neuer Mandantenserver-Prozess eingerichtet werden.

Dazu einfach einen **Namen** vergeben (z.B.: „automatische Buchung“).

Das zu verwendende **Control** ist folgendes: **^jpl FIBUCH_EXTERN**

Bei **Sekunden** wird ein Intervall eingetragen (in Sekunden!), in dem der automatische Buchungslauf abgearbeitet werden soll.

Alle anderen Einstellungen bleiben unberührt. Nach dem Speichern wird die automatische Buchung vom Mandantenserver in dem angegeben Intervall ausgeführt.

**Was wird da überhaupt gebucht?**

Die allgemeine Buchungsautomatik führt in dem angegebenen Intervall eine Buchung Vergleichbar mit den Grundeinstellungen des Pflegers für die [Buchung erfasster Belege](./index.md):

- Erfasser: Alle Vorgänge
- Herkunft: Alle Vorgänge

Es werden jedoch sämtliche erfassten Belege des aktuellen Jahres und des vorangegangenen Jahres gebucht.

Dies kann beim ersten Durchlauf ggf. etwas länger dauern. Alle nachfolgenden Buchungen werden entsprechend schneller abgearbeitet.
