# ODBC Anschluss zur Datenbank

<!-- source: https://amic.de/hilfe/odbcanschlusszurdatenbank.htm -->

Der ODBC Anschluss des Benutzers zu seinen Mandanten muss korrekt im ODBC Einrichtungsmodul eingetragen sein. Es muss weiterhin darauf geachtet werden, dass NICHT der User ADMIN diese ODBC Verbindung steuert, es muss IMMER der aktuelle User diese Verbindung steuern. Das BI Interface nimmt von sich aus die Setzung der ODBC Verbindung vor, wird aber über einen Externen CITRIX Applikation Zugriff oder einen SharePoint Zugriff eine Verbindung zur Datenbank realisiert, so muss die ODBC Verbindung korrekt gepflegt sein.
