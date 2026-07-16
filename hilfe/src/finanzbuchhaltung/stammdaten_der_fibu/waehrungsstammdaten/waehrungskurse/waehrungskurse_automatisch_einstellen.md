# Währungskurse automatisch einstellen

<!-- source: https://amic.de/hilfe/whrungskurseautomatischeinstel.htm -->

Sind die ISO-Bezeichnungen der von Ihnen verwendeten Währungen im Währungsstamm korrekt eingegeben worden, wurde das Feld Kursdienst mit **Ja** belegt und wurde die Währung nicht mit einer Kurssperre versehen, so können Sie die aktuellen Kurse der Währungen über einen Webservice erfragen und automatisch eintragen lassen. Dabei werden alle drei Multiplikator-Felder auf den Wert des aktuellen Bankenwechselkurses gesetzt, der im Internet erfragt wurde.

Zur Verfügung stehen Wechselkurse von derzeit 31 Währungen, die von der Europäischen Zentralbank (EZB) auch auf der Webseite [http://www.ecb.int/stats/exchange/eurofxref/html/index.en.html#latest](http://www.ecb.int/stats/exchange/eurofxref/html/index.en.html#latest) zur Verfügung gestellt werden.

Die Daten werden am Nachmittag des Handelstages ab ca. 15:00 Uhr bereitgestellt. An Samstagen und Sonntagen findet wie auch an Feiertagen kein Handel statt. An diesen Tagen werden keine Daten veröffentlicht.

Mit den Daten aus dem Internet werden Ankauf-, Verkauf- und Mittlerer Kurs gleichermaßen belegt.

**Einstellungen**

Mit dem [Steuerparameter 675](../../../../firmenstamm/steuerparameter/optionen_global/waehrungskurs_mit_webdaten_ueberschreiben_spa_675.md) (Währungskurs mit Webdaten überschreiben) stellen Sie ein, ob es erlaubt sein soll, dass bestehende Kurse mit den Daten aus dem Internet überschrieben werden.

Mit dem [Steuerparameter 676](../../../../firmenstamm/steuerparameter/optionen_global/waehrungskurs_x_tage_zurueck_web_abrufen_spa_676.md) (Währungskurs x Tage zurück Web abrufen) stellen Sie ein, wie viele Tage in die Vergangenheit die Daten abgerufen werden sollen.

Bitte beachten Sie hierbei, dass für den Samstag und Sonntag keine Daten ausgegeben werden, da an diesen Tagen kein Handel stattfindet. An einem Montag ist also der jüngste Kurs 3 Tage alt.

Die historischen Daten der EZB werden sich nicht ändern. Ausgenommen einer unwahrscheinlichen Korrektur eines falsch ausgewiesenen Kurses werden diese Daten gleich bleiben. Wenn Sie also regelmäßig Kurse abrufen, ist die Historie für viele Tage (max. 90) nicht sinnvoll.

Bestehende Kurse werden auch nur bei eingeschaltetem [Steuerparameter 675](../../../../firmenstamm/steuerparameter/optionen_global/waehrungskurs_mit_webdaten_ueberschreiben_spa_675.md) überschrieben.

GGf. muss die Webseite [https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml](https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml) auf dem Datenbankserver als vertrauenswürdige Seite hinzugefügt werden.

**Einmaliger Eintrag**

Dazu rufen Sie in der Anwendung „Währungskurse“ die Funktion ***Währungskurse aktualisieren* SF10** auf. Es erscheint ein Abfragefenster mit dem Sie entscheiden, ob die Währungskurse aktualisiert werden sollen oder nicht. Wenn Sie mit ‚Ja‘ bestätigen werden die Kurse des Vortags von der Europäischen Zentralbank gelesen und für den Vortag eingetragen.

**Regelmäßiger Eintrag**

Wenn Sie täglich zu einer bestimmten Zeit die Kurse automatisch erfragen und eintragen lassen wollen, so können Sie einen Event nutzen. Rufen Sie dazu die Funktion ***Währungskurse Event*** **SF12** auf.  
Es öffnet sich ein Fenster in dem Sie die Uhrzeit festlegen können zu der die Kurse täglich aktualisiert werden sollen. Wenn Sie den Button ‚Event erzeugen‘ drücken wird der Event neu erzeugt. Existiert bereits ein Event bekommen Sie einen Hinweis und können entscheiden, ob der Event überschrieben werden soll oder nicht.

**Einträge ins Fehlerprotokoll**

Sollte ein Umrechnungskurs nicht ermittelbar sein, so wird ein Eintrag ins Fehlerprotokoll eingefügt, der die ISO-Bezeichnung der betreffenden Währung nennt.
