# Intrastat einrichten

<!-- source: https://amic.de/hilfe/intrastateinrichten.htm -->

| Felder | Beschreibung |
| --- | --- |
| Ausgabeformat | 0: ASCII-Format<br>1:INSTAT/XML<br>**Aus Gründen der Datensicherheit und der ab Bezugszeitraum Januar 2022 verpflichtenden Angabe von Umsatzsteuer-Identifikationsnummer des** EU**\-Handelspartners und des Ursprungslandes der Ware in der Verkehrsrichtung Versendung wird eine Ablösung der Dateimeldungen im Festformat ASCII-Fix zum 30.06.2021 angestrebt. Neuanträge zur Meldung in diesem Format werden daher ab 01.02.2020 nicht mehr genehmigt!**<br> |
| Materialnummer Versand | 5-stellige Materialnummer (Versand)<br>Die 5-stellige alphanumerische Materialnummer für den Versand wird nach erfolgreicher Prüfung der gelieferten Testdateien vom Statistischen Bundesamt vergeben. Für die Übermittlung der Testdatei muss hier „XGTEST“ stehen.<br> |
| Materialnummer Einfuhr | 5-stellige Materialnummer (Versand)<br>Die 5-stellige alphanumerische Materialnummer für die Einfuhr wird nach erfolgreicher Prüfung der gelieferten Testdateien vom Statistischen Bundesamt vergeben. Für die Übermittlung der Testdatei muss hier „XGTEST“ stehen.<br> |
| Warenbewegung Intrastat | 0: Nein<br>1: Ja<br>Gibt die Eingabemöglichkeiten der [Intrastat Zusatzdaten](./intrastat_positionen/intrastat_zusatzdaten.md) schon in der Vorgangserfassung frei.<br> |
| Intrastat Dateipfad | Verzeichnis für die Ausgabedateien. Im Format INSTAT/XML werden dort noch zwei weitere Verzeichnisse „Einfuhr“ und „Versendung“ angelegt.<br> |
| Intrastat Dateiname Versand<br>und<br>Intrastat Dateiname Einfuhr | Diese Felder wird nur im ASCII-Format abgefragt.<br>Im INSTAT/XML-Format setzt sich der Name automatisch wie folgt zusammen:<br><br>Materialnummer_YYYYMM_YYYYMMDD_HHMM.XML <br><br>wobei YYYYMM der Beginn des Auswertungszeitraums ist, YYYYMMDD das Tagesdatum und HHMM die Uhrzeit(Stunden und Minuten), an dem das Dokument erstellt wurde.<br>Wenn es eine Testmeldung ist, dann steht statt der Materialnummer der Text „XGTEST“.<br> |
| Intrastat Wert/Masse | Optionale Wert/Masse Datenbank-Funktion<br> |
