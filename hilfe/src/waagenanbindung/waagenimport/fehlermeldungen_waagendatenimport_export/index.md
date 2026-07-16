# Fehlermeldungen (Waagendatenimport-/-export)

<!-- source: https://amic.de/hilfe/fehlermeldungenwaagendatenimpo.htm -->

Die meisten Fehlermeldungen werden direkt in das Fehlerprotokoll geschrieben und werden nicht unmittelbar ausgegeben. Es gibt allerdings einige wenige Fehler, die zu sofortiger Anzeige einer Meldung führen:

Kann Datei nicht finden/öffnen C:\\TEMP\\WAAGE.DAT

Existiert das im Parameter WAAGEDAT angegebene Verzeichnis überhaupt?

Unter Windows 95 wird u. U. eine DOS-Box nicht korrekt beendet. Sobald die DOS-Box auf der Taskleiste erscheint, sollte man sie anklicken. Sie wird nun beendet, und das Programm arbeitet ordnungsgemäß weiter.

Wurde unter DATEINAME eine Datei festgelegt, die sich nicht auf dem Datenträger befindet und MULTI_FILES=0, dann kann es nicht klappen.

Wurde die Datei evtl. im Editor geöffnet, oder ist die Importdatei im Windows-Explorer markiert, dann kann sie nicht kopiert werden und der Zugriff auf des Kopierergebnis schlägt fehl.

Wenn nach dieser Fehlermeldung eine Datei, z. B. A:\\WAGGE.DAT auf der Importdatenträger fehlt, dann wurde im Parameter WAAGEDAT fälschlicherweise der Name der/einer Importdatei angegeben und nicht ein Dateiname auf der Festplatte wie C:\\TEMP\\WAAGE.DAT als Ziel für das Kopieren der Importdatei/en.

<p class="siehe-auch">Siehe auch:</p>

- [SKRIPT FALSCH PARAMETRISIERT!](./skript_falsch_parametrisiert.md)
