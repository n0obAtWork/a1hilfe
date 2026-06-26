# ASK Statement

<!-- source: https://amic.de/hilfe/askstatement.htm -->

#### Syntax

ASK Beschreibung>VARIABLE[,Beschreibung>Variable[,….]];

#### Purpose

Interaktive Abfrage von Parametern.

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

ASKJN, Parameter, Variablen , Envirenmentvariable

#### Beschreibung

Mit dem ASK Statement können Variablen für die Benutzung innerhalb einer Kommandodatei abgefragt werden. Es sind maximal 10 Variablen erlaubt.

Ein zweites ASK Statement löscht alle bis zu diesem Punkt existierenden

Variablen. Drück man den Abbruchbutton beim ASK-Statement innerhalb einer Kommandodatei, wird diese beendet.

#### Beispiel

ASK Belegnr>BNR,Jahrnummer>JNR;

MSG Suchen nach Belegnumer :BNR im Jahr :JNR;

Select \* from fibuvorgposition where Jahrnummer=:JNR and FiBuV_Nummer=’:BNR’;
