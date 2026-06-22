# Aufladen

<!-- source: https://amic.de/hilfe/_lvs20_aufladen.htm -->

Sind alle Waren in den Warenausgang gebracht worden, so kann aufgeladen werden.

Die Einzelnen Ladeträger können mit Hilfe der Datenbankfunktion „AMIC_LVS_AUFLADEN“ dem aufzuladenden Vorgang zugeordnet werden und es werden LVS-Vorgangsimporte der Vorgangsunterklasse 90 erzeugt.

Erst am Ende des Aufladens wird daraus ein Lieferschein mit Hilfe der Datenbankfunktion „AMIC_LVS_AUFLADENENDE“ die Positionen und erstellt einen zweiten Satz mit den Importdaten des Ladescheins.

**Empfohlener Arbeitsablauf Scanner:**

• Scan „AUFLADEN

• Scan Ladescheinnummer

• Scan der NVE

o Anzeige der NVE-Info

• Scan AUFLADENENDE

o Prüfung auf Vollständigkeit

• Erzeugen eines Belegs im VIMP
