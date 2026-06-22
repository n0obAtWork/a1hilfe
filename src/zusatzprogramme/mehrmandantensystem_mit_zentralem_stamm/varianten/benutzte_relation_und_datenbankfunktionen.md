# Benutzte Relation und Datenbankfunktionen

<!-- source: https://amic.de/hilfe/benutzterelationunddatenbankfu.htm -->

Eine kleine Übersicht und Beschreibung von den benutzten Relationen und Prozeduren, die von unserem System ausgeliefert werden.

| Relationen | Kurzbeschreibung | Mandant |
| --- | --- | --- |
| mms_transfer_speicher | Zwischenrelation für das Einspielen der Daten in die Untermandanten | Zentralmandant |
| mms_transfer | Proxy Tabelle für die zu Importierenden Daten | Untermandant |
| mms_transferzwischenspeicher | Speichert den Altertablebefehl ab, wenn das Ändern ein Tabelle nicht funktioniert hat | Untermandant |
| tabellenstruktur | Temporäre Tabelle für das öffnen und verarbeiten der XML Daten | Untermandant |
| mms_transfer_tabellen | Diese Relation enthält die Namen der Prozeduren oder Views, die auf eine der Relationen im MMS System wirkt. | Untermandant |

| Prozeduren | Kurzbeschreibung | Mandant |
| --- | --- | --- |
| ArtikelExportXML | Sammelt die ganze Artikel Informationen aus den Relationen oder den Privaten Views und speichert die Daten als XML File in der Proxy Tabelle mms_transfer ab | Zentralmandant |
| ArtikelImportXML | Öffnet das XML Objekt welches in mms_transfer liegt, kümmert sich um die Umschlüsselung, sorgt dafür dass die Daten durch die XMLEinfügeprozedur in das System gespielt werden. | Untermandant |
| FruchtartExportXML.sql | Sammelt die ganze Artikel Informationen aus den Relationen oder den Privaten Views und speichert die Daten als XML File in der Proxy Tabelle mms_transfer ab | Zentralmandant |
| FruchtartImportXML.sql | Öffnet das XML Objekt welches in mms_transfer liegt, kümmert sich um die Umschlüsselung, sorgt dafür dass die Daten durch die XMLEinfügeprozedur in das System gespielt werden. | Untermandant |
| GruppenExportXML.sql | Sammelt die ganze Artikel Informationen aus den Relationen oder den Privaten Views und speichert die Daten als XML File in der Proxy Tabelle mms_transfer ab | Zentralmandant |
| GruppenImportXML.sql | Öffnet das XML Objekt welches in mms_transfer liegt, kümmert sich um die Umschlüsselung, sorgt dafür dass die Daten durch die XMLEinfügeprozedur in das System gespielt werden. | Untermandant |
| mms_transfer_stop | Verhindert das Importieren und das Exportieren von Daten | Zentralmandant/  
Untermandant |
| SortenExportXML.sql | Sammelt die ganze Artikel Informationen aus den Relationen oder den Privaten Views und speichert die Daten als XML File in der Proxy Tabelle mms_transfer ab | Zentralmandant |
| SortenImportXML.sql | Öffnet das XML Objekt welches in mms_transfer liegt, kümmert sich um die Umschlüsselung, sorgt dafür dass die Daten durch die XMLEinfügeprozedur in das System gespielt werden. | Untermandant |
| tabellen_abaenderer_xml.sql | Wir über den Event aufgerufen  
Probiert eine Relation abzuändern. Sollte dies nicht gelingen, da Relation gesperrt ist. Wird die Funktion abgebrochen | Untermandant |
| TabellenattributeexportXML.sql | Exportiert Relationseigenschaften für den Artikel und die davon abhängigen Relationen | Zentralmandant |
| Tabellenstrukturimport_xml.sql | Importiert die Relationseigenschaften für den Artikel und die davon abhängigen Relationen. | Untermandant |
| TabellenstrukturfruchtartexportXML.sql | Exportiert Relationseigenschaften für die Fruchtart und die davon abhängigen Relationen | Zentralmandant |
| Tabellenstrukturfruchtartimportxml.sql | Importiert die Relationseigenschaften für die Fruchtart und die davon abhängigen Relationen. | Untermandant |
| TabellenstrukturgruppenexportXML.sql | Exportiert Relationseigenschaften für die Gruppen und die davon abhängigen Relationen | Zentralmandant |
| Tabellenstrukturgruppenimportxml.sql | Importiert die Relationseigenschaften für die Gruppen und die davon abhängigen Relationen. | Untermandant |
| TabellenstruktursorteexportXML.sql | Exportiert Relationseigenschaften für die Sorte und die davon abhängigen Relationen | Zentralmandant |
| Tabellenstruktursorteimport_xml.sql | Importiert die Relationseigenschaften für die Sorte und die davon abhängigen Relationen. | Untermandant |
| xml_columneigenschaft.sql | Holt sich die Eigenschaften der einzelnen Relationsfelder. | Untermandant |
| xml_privateview_vorhanden.sql | Schaut nach, ob eine Prozedur oder View auf eine der Relationen wirken soll. | Zentralmandant/  
Untermandant |
| XMLEinfuegeprozedur.sql | Diese Prozedur versucht einen Datensatz, in die jeweilige Relation zu schreiben. Tritt der Fehler *52003* auf, so wird automatisch probiert dieser Realtion die fehlenden Spalten anzufügen. Sollte dies auch einem Grund nicht möglich sein, so wird das Alter Table Statement und das Insert Statement in die Realation mms_transferzwischenspeicher zwischengespeichert. Diese Relation überwacht ein Event und probiert alle 5 Minuten die Statements auszuführen | Untermandant |
