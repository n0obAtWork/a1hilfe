# WIN2DOS

<!-- source: https://amic.de/hilfe/win2dos.htm -->

#### Syntax

WIN2DOS table-name [Dateiname der Umsetztabelle];

#### Purpose

Wandelt die Umlaute der Windows Codepage in Umlaute der DOS Codepage um.

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[DOS2WIN](./dos2win_statement.md)

#### Beschreibung

Sollen Daten aus Aeins exportiert und in eine DOS basierende Fremdsoftware eingebaut werden, tritt das Problem auf, dass die Deutschen Umlaute hier unterschiedlich dargestellt werden. Dieser Befehl schnappt sich eine Relation(table-name) und nimmt sich alle Textfelder vor um dort gegebenenfalls die Umlaute umzuwandeln. Es erfolgt nur ein Update, wenn auch Umlaute in den Datensätzen vorhanden sind. Die Umsetzungstabelle (Siehe DOS2WIN) braucht nicht extra angepasst werden(darf nicht). Nach wie vor muss das DOS Zeichen an stelle 1 stehen und das Windows Zeichen an stelle 2.

#### Beispiel

```text
WIN2DOS ANSCHRIFTSTAMM c:\AEINS\BIN\UMLAUT.TXT
```
