# LOAD

<!-- source: https://amic.de/hilfe/load.htm -->

#### Syntax

LOAD;

Insert into (feld,feld,...) values (%s)

DATEN

.

.

.

LOAD;

#### Purpose

Beladen einer Tabelle.

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[IDENTLOAD](./identload_statement.md), [READ](./read.md), [DBFLOAD](./dbfcreate_statement_ab_version_5_0.md)

#### Beschreibung

Um ganze Tabellen von einer Datenbank in eine andere Datenbank zu transportieren, und um nicht jedes Mal das gesamte Insert -Statement mitzuschleppen, wurde beschlossen, nur in die erste Zeile das Insert - Statement zu schreibe ( ohne Zeilenumbruch) und in die Folgezeilen die Daten zu belassen. Durch das Schlüsselwort LOAD erkennt das System, dass in der Folge eine so aufgebaute Datenstructur folgt und kann somit im Programm das eigentliche Statement zusammenbauen.

 Derartige Dateien können mit einem Utility - das unter OSQL unter F8 zu finden ist – erstellt werden.

#### Beispiel

LOAD;

insert into FIBUVORGKLASSE( FiBuV_Klasse,FiBuV_KlBezeich, FiBuV_KlBeaKennz,fibuv_klKurzBez) values (%s)

1,'Zahlungsverkehr Banken',0,'ZA'

2,'Ausgangsrechnung',0,'AR'

3,'Ausgangsgutschrift',0,'AG'

4,'Eingangsrechnung',0,'ER'

5,'Eingangsgutschrift',0,'EG'

6,'Sonstige Belege',0,'SO'

7,'Restposten',0,'RP'

8,'Skonto',0,'SK'

9,'Ausbuchungen',0,'AB'

10,'Wechselerfassung',0,'WE'

11,'Kursgewinn/Kursverlust',0,'KD'

12,'Jahreswechsel',0,'JW'

13,'Rohwarenzugang',0,'RZ'

14,'Rohwarenausgang',0,'RA'

15,'Eröffnungsbuchung',0,'EB'

16,'Teilzahlung',0,'TZ'

17,'Interne Umbuchung',0,'IU'

18,'Kostenstellenumbuchung',0,'KU'

19,'Scheckeinreicher',0,'SE'

LOAD;
