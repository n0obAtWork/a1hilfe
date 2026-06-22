# Erstellung des Exports

<!-- source: https://amic.de/hilfe/erstellungdesexports.htm -->

Diese Funktion sorgt dafür, dass aus dem Kunden der Rechnung das zugehörige Profil ermittelt und die eRechnung in drei Stufen aus den Rechnungsdaten erstellt wird.

1. Daten extrahieren

1. Die im [Exportprofil eingetragenen Prozeduren](../erechnung_profilpfleger/oberflaeche_startseite.md) lesen Daten aus den Rechnungsdaten der Datenbank und speichern diese in einen Tabellensatz zwischen.

2. Diese Prozeduren können mit Hilfe des Profilpflegers auf Funktionalität und Inhalt gestestet werden. Die Zuordnungen der Daten ergeben sich aus der [Spezifikation 3.0.1 der XRechnung](https://xeinkauf.de/dokumente/#xrechnung). Alle Daten, die ermittelt werden, werden in Tabellen gespeichert, deren Namen sich an diese Spezifikation anlehnen.

Die Datenbankprozeduren stellen die einzige Möglichkeit dar, Individualisierungen in den Daten vorzunehmen.

Als Orientierung für die Herkunft von Daten haben wir Standard-Prozeduren mit dem Namenspräfix „AMIC_STD_XRE_“ zur Verfügung gestellt.

2. Xml formulieren

1. Die ermittelten Daten werden nun in das Datenaustauschformat UBL ([Universal Business Language](https://de.wikipedia.org/wiki/Universal_Business_Language)), einem XML-Format, gelesen, erstellt und in eine Datei im vorgegebenen Verzeichnis gespeichert.

2. Dabei werden die Business-Terms gemäß der [Spezifikation für UBL2.1](http://www.datypic.com/sc/ubl21/s-UBL-Invoice-2.1.xsd.html) in die Felder der einzelnen Businessterms (BT) gelesen.

Das Mapping ist vom Standard vorgegeben und es können keine Änderungen an der Zuordnung vorgenommen werden.

3. Archivieren

Zusätzlich wird das erstellte XML auch noch im Archiv als eRechnung-Export (Belegklasse 8040 – eRechnung Xml) gespeichert.

Hinweis:

Eine Erweiterung des eRechnung-Standards um kundenspezifische Felder ist **NICHT** vorgesehen. Es können also außerhalb des Standards keine weiteren Informationen in diesem Format exportiert werden.

Zur Vermeidung von Problemen bei der Konsistenzprüfung warnen wir ausdrücklich vor dem Missbrauch ungenutzter Datenfelder zum Transport von artfremden Daten!
