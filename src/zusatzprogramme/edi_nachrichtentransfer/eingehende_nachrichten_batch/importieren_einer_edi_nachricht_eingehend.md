# Importieren einer EDI-Nachricht (eingehend)

<!-- source: https://amic.de/hilfe/importiereneineredinachrichtei.htm -->

1. Eine Datei mit einer „ORDERS“-EDI-Nachricht in das Verzeichnis „..\\Rosi-Test“ hineinlegen.

2. Das Import-Programm mit dem Befehl „GSCEdiImport.exe db 0“ aufrufen.  
db => Name der Datenbank

3. Jetzt das Makro „C#AMIC_ROSI_EDI_ORDRSP_FRESSNAPF“ ausführen.  
\=> Es wird ein Vorgang (Auftrag) angelegt.
