# Rohware-Einrichterparameter

<!-- source: https://amic.de/hilfe/rohwareeinrichterparameter.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Schema-/Kundenänderung > Einrichterparameter

Direktsprung **[RWB]**

**Artikelauswahl auf Rohwarengruppe des Beleges beschränkt**

Vorbelegung Ja

Entscheidet man sich dafür, dass die Artikelauswahl nicht auf die Rohwarengruppe des zu korrigierenden Beleges beschränkt ist, dann kann man Rohwarenbelege auch rohwarengruppenübergreifend korrigieren.

Bitte beachten Sie dazu den [Hinweis zum Wechsel der Rohwarengruppe](./wichtiger_hinweis_zum_wechsel_der_rohwarengruppe.md)

Vorgangsunterklasse des Zwischenbeleges

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Schema-/Kundenänderung

Direktsprung **[RWB]**

Vorbelegung 0  
Hier legt man die Vorgangsunterklasse fest mit der die Zwischenbelege/Lieferscheine erzeugt werden. Für diese Unterklasse sollte das Feld RohwareVorerfassung auf ungleich ‚ohne’ stehen, ansonsten wird nach der ersten ‚gültigen’ Vorgangsunterklasse gesucht.  
Wird keine Vorgangsunterklasse mit dem Feld RohwareVorerfassung ungleich ‚ohne’ gefunden, dann wird der Datensatz nicht in die Maske für die Verarbeitung geladen.

Rohwarebeleg danach zur normalen Korrektur öffnen

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Schema-/Kundenänderung

Direktsprung **[RWB]**

Vorbelegung Nein

Möchte man an den korrigierten Rohwarenbelegen noch weitere Änderungen vornehmen (die nicht über diese Spezial Korrektur möglich sind, aber über die normale Korrektur), dann kann man mit Hilfe dieses Einrichterparameters im Anschluss die Belege zur normalen Korrektur öffnen lassen.
