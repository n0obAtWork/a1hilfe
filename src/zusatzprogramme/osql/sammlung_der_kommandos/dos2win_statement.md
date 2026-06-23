# DOS2WIN Statement

<!-- source: https://amic.de/hilfe/dos2winstatement.htm -->

<p class="just-emphasize">Syntax</p>

DOS2WIN table-name [Dateiname der Umsetztabelle];

<p class="just-emphasize">Purpose</p>

Wandelt die Umlaute der DOS Codepage in Umlaute der Windos-Codepage um.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[WIN2DOS](./win2dos.md)

<p class="just-emphasize">Beschreibung</p>

Bei Datenübernahmen aus ehemaligen DOS/ Prolog – Programmen tritt das Problem auf, dass die Deutschen Umlaute auf unterschiedlich dargestellt werden. Dieser Befehl schnappt sich eine Tabelle(table-name) und nimmt sich alle Textfelder vor um dort gegebenenfalls die Umlaute umzuwandeln. Es erfolgt nur ein Update, wenn auch Umlaute in den Datensätzen vorhanden sind. Wird keine Umsetztabelle angegeben, werden nur die gebräuchlichen Umlaute umgewandelt. Diese wären ÄÖÜßäöü. Weiterhin kann es auch Probleme mit Hochkomma in den Tabellen geben. Diese werden auch umgewandelt.

 Die Umsetztabelle hat ein einfaches Format. In jeder Zeile steht ein umzusetzendes Zeichen gefolgt vom Zeichen, wie es unter Windows dargestellt werden soll. Um dies Datei zu erstellen, kann man den MSDOS Editor aufrufen, dort die Zeichen eingeben um anschließend dieselbe Datei unter Windows mit Notepad aufzurufen und dort noch mal die Zeichen in der entsprechenden Zeile einzugeben.  
 Um nicht die Zeichen eingeben zu müssen, was für die Dos-Umlaute doch etwas umständlich ist, kann man auch den ASCII- Wert angeben. Dabei werden die ersten drei Stellen als ASCII-Wert als umzusetzendes Zeichen und die folgenden drei Stellen als Zeichen, in das umgewandelt werden soll. Also würde die Zeile

065097

angeben, dass der n Buchstaben A in a umgewandelt werden soll.

<p class="just-emphasize">Beispiel</p>

DOS2WIN ANSCHRIFTSTAMM c:\\AEINS\\BIN\\UMLAUT.TXT
