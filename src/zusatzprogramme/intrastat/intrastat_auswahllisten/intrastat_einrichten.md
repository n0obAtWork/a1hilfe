# Intrastat einrichten

<!-- source: https://amic.de/hilfe/intrastateinrichten.htm -->

| Felder | Beschreibung |
| --- | --- |
| Ausgabeformat | 0: ASCII-Format  
1:INSTAT/XML  
**Aus Gründen der Datensicherheit und der ab Bezugszeitraum Januar 2022 verpflichtenden Angabe von Umsatzsteuer-Identifikationsnummer des** EU**\-Handelspartners und des Ursprungslandes der Ware in der Verkehrsrichtung Versendung wird eine Ablösung der Dateimeldungen im Festformat ASCII-Fix zum 30.06.2021 angestrebt. Neuanträge zur Meldung in diesem Format werden daher ab 01.02.2020 nicht mehr genehmigt!**  
 |
| Materialnummer Versand | 5-stellige Materialnummer (Versand)  
Die 5-stellige alphanumerische Materialnummer für den Versand wird nach erfolgreicher Prüfung der gelieferten Testdateien vom Statistischen Bundesamt vergeben. Für die Übermittlung der Testdatei muss hier „XGTEST“ stehen.  
 |
| Materialnummer Einfuhr | 5-stellige Materialnummer (Versand)  
Die 5-stellige alphanumerische Materialnummer für die Einfuhr wird nach erfolgreicher Prüfung der gelieferten Testdateien vom Statistischen Bundesamt vergeben. Für die Übermittlung der Testdatei muss hier „XGTEST“ stehen.  
 |
| Warenbewegung Intrastat | 0: Nein  
1: Ja  
Gibt die Eingabemöglichkeiten der [Intrastat Zusatzdaten](./intrastat_positionen/intrastat_zusatzdaten.md) schon in der Vorgangserfassung frei.  
 |
| Intrastat Dateipfad | Verzeichnis für die Ausgabedateien. Im Format INSTAT/XML werden dort noch zwei weitere Verzeichnisse „Einfuhr“ und „Versendung“ angelegt.  
 |
| Intrastat Dateiname Versand  
und  
Intrastat Dateiname Einfuhr | Diese Felder wird nur im ASCII-Format abgefragt.  
Im INSTAT/XML-Format setzt sich der Name automatisch wie folgt zusammen:  
    
Materialnummer_YYYYMM_YYYYMMDD_HHMM.XML   
    
wobei YYYYMM der Beginn des Auswertungszeitraums ist, YYYYMMDD das Tagesdatum und HHMM die Uhrzeit(Stunden und Minuten), an dem das Dokument erstellt wurde.  
Wenn es eine Testmeldung ist, dann steht statt der Materialnummer der Text „XGTEST“.  
 |
| Intrastat Wert/Masse | Optionale Wert/Masse Datenbank-Funktion  
 |
