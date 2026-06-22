# Bearbeitung von Kassenbelegen

<!-- source: https://amic.de/hilfe/_bearbeitungvonkassenbelg.htm -->

In der AW Gesamtbarverkauf unter Warenwirtschaftssystem/Barvorgänge gibt es die Möglichkeit, Kassenbelege nachträglich zu bearbeiten:

Dabei besteht die Möglichkeit des Stornierens und Druckens von Belegen für Finanz- und Kassenvorgängen. Während der Druck grundsätzlich möglich ist, muss an dem Arbeitsplatz, an dem storniert werden soll, die zugehörige Kasse eröffnet sein. Kassenstürze sind dabei nicht stornierbar (nur wiederholt druckbar).

Die Bearbeitung ist in der Variante Belegüberblick möglich.

1\. Drucken:

Hier kann der Kassenbeleg nochmals gedruckt werden in der Form wie auch schon während der "Echterfassung". Um ein solches Formular auf den Schacht zu drucken, muss auf dieser Maske der EPA "Sollen Formulare auf den Schacht gedruckt werden" auf Ja gesetzt sein. Dann wird auch auf die eingerichteten Formulare 51-54 zurückgegriffen.

2\. Stornieren:

Ein Beleg kann nur einmal storniert werden, eine Stornierung ist nicht mehr rückgängig zu machen.

a) Der Beleg erhält ein Stornokennzeichen.

b) Es wird ein Stornobeleg gedruckt, wobei EPA-abhängig auf die Formulare 51-54 zurückgegriffen wird. In diesen Formularen ist an entsprechender Position im Kopfteil die TextVariable Storno zu hinterlegen, in der **nur** bei Stornobelegen der Text "Storno-" gedruckt wird.

c) Es werden die Stornobeträge je nach Belegart in Storno-Feldern der Relation AcashBelgKsiz verwaltet bzgl. der Kasse/Sitzung, die diesen Stornovorgang auslöst (nicht auf die Kasse/Sitzung, die den Urbeleg erzeugt hat!!!). Diese Beträge werden in den entsprechenden Varianten im Gesamtbarverkauf unter den "Sto"-Feldern angezeigt.

d) Mit der Stornierung ist eine Barauszahlung/Bareinzahlung verbunden, diese wird auf die Soll/Umsätze der Kasse angerechnet, die die Stornierung durchführt.

e) Wenn der SPA 50 "Aut. Buchung von Kassenvorg. in FiBu" gesetzt ist, wird zusätzlich eine Gegenbuchung zur Buchung erzeugt, die automatisch beim ursprünglichen Finanzvorgang erzeugt wurde.

f) In der Relation AcashBelgStoProto wird ein Protokoll über alle getätigten Stornierungen von Finanz-und Kassenvorgängen geführt.

g) Um Konflikte mit evtl. schon weiterverarbeiteten Zahlungsmitteln zu vermeiden, können nur Belege storniert werden, die nicht auf Zahlungsmittel zurückgreifen.

Evtl. muss dann das Zahlungsmittel, das die Stornierung verhindert, zuerst in "Bar" umgewandelt werden (siehe Bearbeitung von Zahlungsmitteln).

Auch ein Wiederholungsdruck von Stornobelegen ist möglich.

Bem.: Beim Wiederholungsdruck/Stornowiederholungsdruck gibt es auf dem Beleg den Vermerk "Belegkopie". Bei Nutzung der Formulare 51-54 muss dort im Kopfteil die Textvariable Kopie an passender Stelle eingesetzt werden.

Bem.: Die obigen Stornierungen werden auch beim Kassenabschluss berücksichtigt, sowohl in den AMIC_Standard-CRW-Reports als auch bei der Verteilung auf Kostenkonten, wenn diese Automatik über SPA eingeschaltet ist (dabei wird beim Stornieren von Barverkäufen/Bareinkäufen/BarGutschriften gegen das in den Kassenkonten hinterlegte Stornokonto gebucht).
