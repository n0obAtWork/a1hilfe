# SEPA in den Zahlungsarten

<!-- source: https://amic.de/hilfe/sepaindenzahlungsarten.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Stammdaten > Zahlungsarten

Direktsprung **[FIZAH]**.

1) DTA-Typ

Im Zuge der SEPA-Einführung wurden die DTA-Typen Einzugsermächtigung und Abbuchungsauftrag durch Basislastschrift, Basislastschrift mit verkürzter Vorlauffrist und Firmenlastschrift ersetzt. Diese werden nicht mehr in den Stammdaten für [Zahlungsarten](../stammdaten_zahlungsverkehr/zahlungsart.md) gepflegt sondern im [Mandat](./sepa_mandat_fuer_lastschriften.md). Die Basislastschrift mit verkürzter Vorlauffrist steht erst ab SEPA-Version 2.7 zur Verfügung. Diese wurde zum 4.November 2013 deutschlandweit eingeführt, wird aber aller Voraussicht nach erst zum 01.Februar 2014 von den Banken umgesetzt. Außerdem ist es möglich, dass die Kreditinstitute auch über diesen Zeitraum hinaus die Version 2.5 annehmen.

2) Echtzeitüberweisung

Bei Zahlungsarten mit Formularklasse **Zahlungsausgang** kann man Echtzeitüberweisung auf Ja stellen. Es werden dann im automatischen Zahlungsverkehr beim Erstellen der Zahlungsvorschläge (Direktsprung [ZHVE]) alle Belege gezogen, deren Fälligkeit vor dem **nächsten Stichtag** liegt und als Ausführungsdatum wird der **Stichtag** verwendet.
