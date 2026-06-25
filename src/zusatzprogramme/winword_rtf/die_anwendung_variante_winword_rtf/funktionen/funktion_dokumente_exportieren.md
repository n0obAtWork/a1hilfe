# Funktion "Dokumente exportieren"

<!-- source: https://amic.de/hilfe/funktiondokumenteexportieren.htm -->

Probleme für die die Funktion "Ansehen" keine Hinweise liefert, kann es hilfreich sein die Winword- und RTF-Dokumenten in das Datei-System zu verbringen umso mehr Möglichkeiten zu haben die entsprechenden Dateien zu untersuchen.

Nach dem Export wird das Export-Verzeichnis im Windows-Explorer geöffnet.

Das Export-Verzeichnis befindet sich im A.eins-Verzeichnis unter Export\\word2rtf erweitert um den jeweiligen Tabellen-Namen.

Der Export legt die benötigten Verzeichnisse automatisch an.

Der Export spielt keine RTF-Dateien aus die Null oder die Länge 0 haben.

Der Export löscht keine Dateien.

Exportierte Dateien folgen der Namenskonvention

```text
{Tabelle}_{Pk_Wert}_{Name der Spalte}.{Extension}
```

wobei "Name der Spalte" entweder der Inhalt "Winword-Spalte" oder "Rtf-Spalte" ist und "Extension" jeweils analog entweder "doc" oder "rtf".
