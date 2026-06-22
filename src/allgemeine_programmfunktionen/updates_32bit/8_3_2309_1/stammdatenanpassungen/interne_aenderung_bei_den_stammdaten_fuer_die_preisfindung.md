# Interne Änderung bei den Stammdaten für die Preisfindung

<!-- source: https://amic.de/hilfe/_83_33716.htm -->

Die automatische Vorbelegung der Nummern für individuelle Preisklassen, individuelle Rabattklassen, individuelle Zu-/Abschlagsklassen und individuelle Preisgruppen wurde auf das Ident-Verfahren umgestellt. Bisher wurde für die Klassen die Nummer des Kunden und für die Gruppe die Id des Artikels genommen. Dieser wurde dann entweder eine 0 oder 1 angehangen. Dies konnte unter Umständen zu Fehlern führen, weswegen das Verfahren umgestellt wurde. Wenn eine der genannten Klassen oder die Gruppe automatisiert angelegt wird, wird eine eindeutige Nummer jenseits des Wertebereichs von 100.000.000 verwendet und in einer entsprechenden Tabelle der Datenbank gespeichert. Wenn Sie die Klassen/Gruppe manuell anlegen, wird der Wert analog dem zuvor beschriebenen Verfahren vorgeschlagen. Sie können diese Nummer aber auch ändern und eine eigene eindeutige Nummer verwenden. Wir raten hierbei unterhalb des Werts von 100.000.000 zu bleiben. Sobald Sie das Feld verlassen haben, wird die eingetragene Nummer festgeschrieben und kann nachträglich nicht mehr geändert werden. Sollten Sie sich für eine eigene Nummer entschieden haben, wird die zuvor vorgeschlagene Nummer bei dem nächsten Objekt gleichen Typs nicht mehr wiederverwendet! Die Nummer gilt intern als verbraucht, auch wenn sie nicht verwendet wurde. Da der mögliche Wertebereich jedoch bis ca. 2,4 Milliarden geht, sind ausreichend Nummern für die Zukunft vorhanden.

<p class="just-emphasize">Releasenote Kategorie:</p>

Ticket: 722744[33716]

Version: 8.3.2309.1

Datum: 30.09.2023

Anwendung: [PRIK], [PRI], [PRIE], [RAK], [ZABK]

Variante: Individualpreisklasse

Funktion/Report: -

[Weitere Informationen](http://www.amic.de/hilfe/_indivPreisKlasse.htm)

Tags:

Releasenote, 8.3.2309.1, 33716, 722744
