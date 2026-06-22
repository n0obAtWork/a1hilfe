# Privateinrichtungen

<!-- source: https://amic.de/hilfe/privateinrichtungen.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

SQL_Text unter der Bedingung: where SQL_TextBesitzer = 1

AnwendVariante unter der Bedingung: where AnwVarBesitzer = 1

AnwendCondition unter der Bedingung: where AnwCondBesitzer = 1

Optionbox unter der Bedingung: where AnwFunkId in (select AnwFunkId from AnwendFunktion where AnwFunkBesitzer = 1 and isnull(AnwRptId,'') = '')

AnwendFunktion unter der Bedingung: where AnwFunkBesitzer = 1 and isnull(AnwRptId,'') = ''

AnwendCondition unter der Bedingung: where AnwCondBesitzer = 1 and AnwCondId in (select AnwCondId from AnwendReport where AnwRptBesitzer=1)

AnwendReport unter der Bedingung: where AnwRptBesitzer = 1

Optionbox unter der Bedingung: where AnwFunkId in (select AnwFunkId from AnwendFunktion where AnwFunkBesitzer = 1 and isnull(AnwRptId,'') != '')

AnwendFunktion unter der Bedingung: where AnwFunkBesitzer = 1 and isnull(AnwRptId,'') != ''

SQL_Text unter der Bedingung: where SQL_TextBesitzer in (12,13,14)

UserFelder

QReportDefine unter der Bedingung: where QRNummer &lt; 1000
