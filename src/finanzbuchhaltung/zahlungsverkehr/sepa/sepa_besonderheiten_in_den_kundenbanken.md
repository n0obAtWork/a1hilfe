# SEPA-Besonderheiten in den Kundenbanken

<!-- source: https://amic.de/hilfe/sepabesonderheitenindenkundenb.htm -->

Hauptmenü > Stammdaten > Kunden-/Lieferanten > Kundenstamm bzw. Lieferantenstamm

Direktsprung **[KU]** bzw. **[LF]**

Die Pflege der Kundenbank ändert sich dergestalt, dass es nicht mehr möglich ist bestehende Bankverbindungen zu löschen oder zu ändern, sobald bei dieser Bankverbindung ein SEPA-Mandat hinterlegt ist. Geändert werden kann bei Bankverbindungen mit benutztem Mandat immer noch die Sperre, das Bis-Datum, die Soll- und Haben Obergrenzen, die Währung des Kontos sowie der Empfänger/Zahlungspflichtige.

Weiterhin muss in den Kundenbanken das neue Feld für die IBAN gepflegt werden. Die IBAN kann nachträglich über eine Funktion „Generiere IBAN“ im Pfleger für Kundenbanken (Direktsprung **[KUBA]**) für alle Kundenbanken mit eingetragener Bank und Kontonummer erzeugt werden. Diese vorgeschlagene IBAN ist in jedem Fall zu überprüfen. Die IBAN wird ausschließlich von der kontoführenden Bank vergeben.

Bei der Neuerfassung der Banken wird anhand der IBAN – für Deutschland, Österreich und Belgien - sowohl Kontonummer als auch die Bank eingetragen. Geschieht dies nicht, so weicht entweder der Aufbau der Iban ab, sie ist falsch oder die Banken-Stammdaten sind nicht gepflegt.

Kennt man die IBAN nicht, so kann man nach wie vor über Kontonummer und Bank die IBAN generieren lassen. Man kann in den [Steuerparametern](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) die IBAN-generierung abschalten. Wird die IBAN trotz aktivem SPA nicht vorgeschlagen, so kann dies daran liegen, dass der im Bankenstamm hinterlegte Staat nicht Deutschland, Österreich bzw. Belgien ist oder der eingetragene ISO-Code im Staatenstamm (Direktsprung **[Staat]**) nicht DE, AT bzw. BE ist.

Diese vorgeschlagene Nummer ist in jedem Fall zu überprüfen, da es sich hier nur um einen Vorschlag handelt. Die IBAN wird ausschließlich von der kontoführenden Bank vergeben. Sollte es notwendig sein, die vorgeschlagene IBAN zu ändern, da eine abweichende IBAN vom Kreditinstitut vergeben wurde, so ist dies durchaus möglich. Sie wird jedoch noch einmal anhand des Prüfzifferverfahrens geprüft und es erscheint ggf. eine Warnung.

Eine Änderung der IBAN wirkt sich sofort auf die Zahlungsvorschläge aus. Bei bereits Freigegebenen Zahlungen wird die IBAN nicht mehr verändert.

Für das Lastschriftverfahren ist ein gültiges Mandat notwendig. Dieses [Mandat](./sepa_mandat_fuer_lastschriften.md) kann pro Kundenbank und Kundenkonto hinterlegt werden.
