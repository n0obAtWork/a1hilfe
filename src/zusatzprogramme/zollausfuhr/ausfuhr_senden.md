# Ausfuhr senden

<!-- source: https://amic.de/hilfe/ausfuhrsenden.htm -->

Wenn Sie die Bearbeitung Ihrer Datenerfassung abgeschlossen haben, können Sie die Ausfuhr beim Zoll anmelden.

Dazu wählen Sie die von Ihnen bearbeitete Zollanmeldung aus der Auswahlliste in der Anwendung Zoll-Lieferscheine mit dem Direktsprung [LIZO] mit dem Status „erfasst“ aus und wählen die Funktion „Ausfuhr senden“.

Es wird die Prozedur *AEBAusfuhrAnmeldungAnlegen* aufgerufen. Diese ruft zunächst, falls vorhanden, die private Datenbankprozedur *p_ZollAusfuhrBelegDefaults(in in_vaId integer)* auf, mittels derer die Daten im Ausgangsvorgang noch automatisiert ergänzt werden können.  
Anschließend wird die Korrektheit der Daten des Ausgangsvorgangs mittels der Prozedur *AMIC_FUNC_VABeleg_Check* weitestgehend geprüft. Ggf. werden Einträge im Fehlerprotokoll erstellt.  
Werden keine Fehler festgestellt, so wird das für den Webservice notwendige XML erzeugt und mittels der Prozedur *createOBTAtlas* an den AEB-Webservice übermittelt.

Die Zollanmeldung wird damit an den Datendienstleister versendet. Bei korrekter Einstellung „sofort senden“ im Register „Zoll“ des Mandantenstamm-Pflegemoduls wird diese Anmeldung, sofern sie korrekt ist, vom Dienstleister an den Zoll weitergeleitet. Anderenfalls bekommen Sie eine Meldung, die Ihnen den Hinweis gibt, im Fehlerprotokoll nach Problemen beim Versand zu suchen.

Der Status der Zeile wechselt von „erfasst“ auf „versendet“. Bei vom Dienstleister festgestellten Fehlern oder Unvollständigkeiten im gesendeten Beleg müssen die Daten auf dem Web-Portal des Dienstleisters angepasst werden.
