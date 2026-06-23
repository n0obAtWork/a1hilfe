# WIN2DOS

<!-- source: https://amic.de/hilfe/win2dos.htm -->

<p class="just-emphasize">Syntax</p>

WIN2DOS table-name [Dateiname der Umsetztabelle];

<p class="just-emphasize">Purpose</p>

Wandelt die Umlaute der Windows Codepage in Umlaute der DOS Codepage um.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[DOS2WIN](./dos2win_statement.md)

<p class="just-emphasize">Beschreibung</p>

Sollen Daten aus Aeins exportiert und in eine DOS basierende Fremdsoftware eingebaut werden, tritt das Problem auf, dass die Deutschen Umlaute hier unterschiedlich dargestellt werden. Dieser Befehl schnappt sich eine Relation(table-name) und nimmt sich alle Textfelder vor um dort gegebenenfalls die Umlaute umzuwandeln. Es erfolgt nur ein Update, wenn auch Umlaute in den Datensätzen vorhanden sind. Die Umsetzungstabelle (Siehe DOS2WIN) braucht nicht extra angepasst werden(darf nicht). Nach wie vor muss das DOS Zeichen an stelle 1 stehen und das Windows Zeichen an stelle 2.

<p class="just-emphasize">Beispiel</p>

```text
WIN2DOS ANSCHRIFTSTAMM c:\AEINS\BIN\UMLAUT.TXT
```
