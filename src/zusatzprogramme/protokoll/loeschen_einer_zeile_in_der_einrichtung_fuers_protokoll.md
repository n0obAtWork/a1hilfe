# Löschen einer Zeile in der Einrichtung fürs Protokoll

<!-- source: https://amic.de/hilfe/lscheneinerzeileindereinrichtu.htm -->

Es kann eine aktuell markierte Einrichtungszeile des Protokolls über Shift+Strg+Entf oder Entfernen des Tabellennamens in der Spalte Protokolltabellen gelöscht werden.  
Es erscheint eine Sicherheitsabfrage die wie folgt lautet:

Sollen der Tabellenname und die zugehörigen Trigger ‚*Name der Tabelle*‘ wirklich aus der Protokollierung entfernt werden?  
Für eine erneute Protokollierung müsste alles neu eingetragen werden.

Bestätigt man diese Abfrage mit Ja werden

\- Alle Protokolltrigger (Insert, Update, Delete) der ausgewählten Tabelle gelöscht.

\- Der Inhalt aus den Tabellen Protokoll_Einrichtung und Protokoll_Einrichtungstamm für die ausgewählte Tabelle gelöscht.

Für den gelöschten Eintrag findet keine Protokollierung mehr statt und es kann auch keine Überwachung über den Funktionsaufruf gestartet werden.

Möchte man diese Tabelle zu einem späteren Zeitpunkt wieder überwachen muss eine neue Einrichtung stattfinden.

Protokollierungen/Aufzeichnungen zu dieser Tabelle die bis zu dem Zeitpunkt der Löschung vorgenommen wurden werden nicht aus der Relation Protokoll entfernt.
