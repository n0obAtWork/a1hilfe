# Einrichtung eines openTRANS-Imports

<!-- source: https://amic.de/hilfe/_OTVorgimport_einr.htm -->

Externe Kommunikation > openTRANS > openTRANS

Direktsprung [OT]

In der Variante Importprofile finden Sie die Einstellungsmöglichkeiten für die Importe.

| Vorgangsimport-Profil |
| --- |
| Ident | Fortlaufende Nummer zur internen Identifikation |
| Quelle | Textfeld zur Repräsentation der Quelle – dieses Feld wird nur für Datenanzeigen verwendet. |
| Aktiv | Gibt an, ob dieses Profil beim Import von Dateien verwendet werden soll. |
| Pfad | Dateipfad, der angibt, wo die zu importierenden Dateien zu finden sind. |
| Archivpfad | Dateipfad, der angibt, wohin die importierten Dateien abzulegen sind. Ist diese Angabe leer, werden die Dateien nach der Verarbeitung gelöscht. |
| Lagernummer | Nummer des Lagers, das als Vorgabe für den Import verwendet werden soll, wenn sich nicht durch die Verwendung eines Makros eine andere Semantik-basierte Lagernummer ergibt. |
| Kunde | Kundennummer, die für die Interpretation der Artikelnummern und anderer Absenderspezifischen Bezeichner im Mapping verwendet werden soll, wenn sich nicht durch ein Makro eine andere Semantik-basierte Kundennummer ergibt. |
| Präprozessor-Makro | C#-Makro, das der Interpretation der zu importierenden Daten dient. |
| Postprozessor-Makro | C#-Makro, das nach dem erfolgreichen Import aufgerufen wird. |
| Stylesheets | Liste von Stylesheets, deren Anwendung für den Import der Dateien dieses Profils in Frage kommt.  
So können z.B. Bestellungen eines externen Systems in openTRANS-Aufträge, Rechnungen in Rechnungen und Lieferavise in Bestellungen gewandelt werden. |
