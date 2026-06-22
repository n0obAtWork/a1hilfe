# Drucksteuerung

<!-- source: https://amic.de/hilfe/drucksteuerung.htm -->

Barverkaufsbelege gehen über Drucker und Formular laut Einstellung [VRGD], falls dort nicht eingerichtet über [DRZ] / [FRZ].

Zahlungsbelege gehen über die Formulare 51 bis 55, falls in der Zahlungsmaske der EPA Formulardruck auf ja geschaltet ist. Anderenfalls werden fest programmierte Formulare (für Kassensturz gibt es derzeit nur ein solches!) über die in der Einrichtung des Kassensystems eingestellten Drucker gedruckt.

In welcher Weise nun die Einstellungen aus den Kassensystemeinstellungen bzgl. LPT oder COM Schnittstelle, zugeordnetem Druckertyp und seiner Steuerzeichen in dieser Mechanik Verwendung finden, bleibt zu erforschen. Wie es scheint, kann man in den Steuerzeichen eintragen was man will, sie werden beim Drucken nur dann ausgewertet, wenn feste Zahlungsformulare gedruckt werden. In allen anderen Fällen scheint darauf gar nicht zugegriffen zu werden. Veränderung des Partial Cut jedenfalls haben keine Verhaltensänderung beim Drucker gezeigt. Es sieht so aus, als würden nur die Steuerzeichen für ASCII Druck, die dem Druckertyp zugeordnet sind, gezogen. Die speziellen Steuerzeichen lt. Einstellung des Kassensystems jedenfalls scheinen nicht benutzt zu werden.

Wer druckt also wo hin:

1. BV über VRGD, wenn nicht vorhanden über FRZ Ascii Druck

2. Finanzbelege Kasse über FRZ wenn Formular zugeordnet ist. Auf fest codiertes Formular auf dem im Kassenstamm eingetragenen Schnittstelle, wenn kein Formular zugeordnet ist.

3. Die Journalrolle im Bondrucker wird nur und ausschließlich über fest codierte Einrichtung auf den eingetragen Schnittstelle ausgegeben.

4. Wiederholungsdruck von Finanzbelegen erfolgt nur auf feste Formulare auf der im Kassenstamm eingetragenen Schnittstelle.

5. Der Zählbericht beim Kassensturz ist nur über ein internes Formular und nur auf der im Kassenstamm eingetragenen Schnittstelle druckbar.

6. Kassensysteme über Terminalserver nur mit CITRIX, damit lokale LPT Schnittstellen freigegeben werden können. Als Schnittstelle im Kassenstamm für den Bondrucker immer die lokale Schnittstelle eintragen.

7. Journalrolle wird bedient, wenn im Druckertyp des Bondruckers die entsprechenden Steuersequenzen, insbes. Ein- und Ausschalten Journalrolle, bekannt sind (Kassensystemverwaltung – Drucksteuersequenzen).
