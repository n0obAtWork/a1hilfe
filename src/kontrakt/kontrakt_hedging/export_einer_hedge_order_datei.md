# Export einer Hedge-Order-Datei

<!-- source: https://amic.de/hilfe/_exporteinerhedgeorde.htm -->

Die Order-Datei wird in dem Verzeichnis erstellt, das im [Einrichterparameter](../../firmenstamm/einrichterparameter/kontraktstamm_epa_ktrstam.md) eingetragen wird. Wurde kein Pfad festgelegt, so wird der Pfad „..\\user\\“ relativ zur A.eins-Applikation verwendet.

Der Order-String wird von einer Datenbankfunktion erstellt, die [Einrichterparameter](./einstellungen.md) eingetragen wird. Wurde dieser Parameter nicht festgelegt, so wird „AMIC_HEDGE_GETORDERSTRING“ verwendet.

Zusätzlich wird die Datei im Formulararchiv mit der Referenz auf diesen Kontrakt archiviert.
