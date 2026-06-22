# Privatisierung

<!-- source: https://amic.de/hilfe/privatisierung.htm -->

Für den Import von Bestellungen kann die Prozedur „AMIC_FutterApp_BelegImport“ angepasst werden (z.B. können hier Informationen in UFLD-Felder geschrieben werden). Die Prozedur ist im Steuerparameter „FutterApp Optionen und Ausprägungen“ (SPA 1047) unter dem Punkt „SQL-Prozedur zum Beleg-Import“ (2) eingetragen werden. Ohne Eintrag wird die Standardprozedur gezogen.

Für den Import von Siloinformationen kann die Prozedur „AMIC_FutterApp_Siloverwaltung“ angepasst werden. Die Prozedur ist im Steuerparameter „FutterApp Optionen und Ausprägungen“ (SPA 1047) unter dem Punkt „SQL-Prozedur zur Siloverarbeitung“ (3) eingetragen werden. Ohne Eintrag wird die Standardprozedur gezogen.

Für das Maschinentagebuch, welches die Einträge in der Tabelle „AMIC_AenderungsProtokoll“ vornimmt, kann die Prozedur „AMIC_FutterApp_MaschinentagebuchVersorgung“ angepasst werden. Die Prozedur ist im Steuerparameter „FutterApp Optionen und Ausprägungen“ (SPA 1047) unter dem Punkt „Privater Abschnitt des Maschinentagebuch“ (1) eingetragen werden. Ohne Eintrag wird die Standardprozedur gezogen.

Zum vereinfachten Finden von fehlerhaft importierten Dateien wird die Warningfunction „FutterAppWarnungAuftrag“ zur Verfügung gestellt. Diese kann in der Auswahlliste [AUB] hinterlegt werden und blendet ein gelbes Ausrufezeichen im Hintergrund der neuen Auswahlliste ein, sollten sich Dateien im Verzeichnis „\\Import\\Fehler“ befinden (Auf Zugriffsrechte muss geachtet werden).
