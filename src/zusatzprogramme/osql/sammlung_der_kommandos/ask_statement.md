# ASK Statement

<!-- source: https://amic.de/hilfe/askstatement.htm -->

<p class="just-emphasize">Syntax</p>

ASK Beschreibung>VARIABLE[,Beschreibung>Variable[,….]];

<p class="just-emphasize">Purpose</p>

Interaktive Abfrage von Parametern.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

ASKJN, Parameter, Variablen , Envirenmentvariable

<p class="just-emphasize">Beschreibung</p>

Mit dem ASK Statement können Variablen für die Benutzung innerhalb einer Kommandodatei abgefragt werden. Es sind maximal 10 Variablen erlaubt.

Ein zweites ASK Statement löscht alle bis zu diesem Punkt existierenden

Variablen. Drück man den Abbruchbutton beim ASK-Statement innerhalb einer Kommandodatei, wird diese beendet.

<p class="just-emphasize">Beispiel</p>

ASK Belegnr>BNR,Jahrnummer>JNR;

MSG Suchen nach Belegnumer :BNR im Jahr :JNR;

Select \* from fibuvorgposition where Jahrnummer=:JNR and FiBuV_Nummer=’:BNR’;
