# Technische Hilfestellung

<!-- source: https://amic.de/hilfe/technischehilfestellung.htm -->

Die Dokumentation zu den Business-Terms finden Sie hier: [https://xeinkauf.de/dokumente/#xrechnung](https://xeinkauf.de/dokumente/#xrechnung). Darin finden sich Business-Groups (BG), nach denen die Daten in der Datenbank in Tabellen mit dem Präfix „XRE_“ abgelegt sind. Die Business-Terms (BT) beschreiben einzelne Felder. Diese sind von 1-163 durchnummeriert.

Diese BT-Nummern finden sich auch in den Standard-Prozeduren zur Ermittlung von Daten für den erechnungs-Export wieder. Aus den Tabellen werden die Daten in das Format der Universial Business-Language (UBL) übertragen und als XMl exportiert. Sollte ein Wert an einer bestimmten Stelle erwartet werden, so kann diese im UBL identifiziert und mit Hilfe der [Spezifikation für UBL](http://www.datypic.com/sc/ubl21/s-UBL-Invoice-2.1.xsd.html) oder mit Hilfe der entsprechenden [Stylesheetdefinition](https://github.com/itplr-kosit/xrechnung-visualization/blob/master/src/xsl/ubl-invoice-xr.xsl) gefunden und einem Business-Term zugeordnet werden. Dann lässt sich die Herkunft des Feldes in der Tabelle über die entsprechenden prozeduren bestimmen und der Export durch Individualisierung der Exportprozeduren ergänzen.
