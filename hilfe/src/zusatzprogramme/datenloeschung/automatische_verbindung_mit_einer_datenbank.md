# Automatische Verbindung mit einer Datenbank

<!-- source: https://amic.de/hilfe/_datloeauto.htm -->

Statt sich jedes Mal manuell mit einer Datenbank zu verbinden, können auch Startparameter benutzt werden, um diesen Schritt zu überspringen. Dabei müssen folgende Werte in beliebiger Reihenfolge angegeben werden:

Benutzer=Benutzer Passwort=Passwort DBN=Datenbankname ENG=Datenbank-Servername

Alternativ kann man auch nur einen Parameter zur Datenbankverbindung nutzen:

Connectionstring=derConnectionstringZurDatenbank

Bei der Angabe von beiden Varianten, hat der Controllstring als Datenbankquelle Vorrang.

Die Groß- und Kleinschreibung oder die Reihenfolge der Parameter ist hierbei unerheblich. Wichtig ist die Rechtschreibung und das es keine Leerzeichen innerhalb eines Parameters gibt (z.B. nicht „Benutzer = ich“).  
Falls die automatische Verbindung fehlschlagen sollte, wird darauf hingewiesen und es wird die Maske geöffnet zur manuellen Verbindung mit der Datenbank [siehe Manuelle Verbindung mit der Datenbank](./manuelle_verbindung_mit_einer_datenbank.md).
