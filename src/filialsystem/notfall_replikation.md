# Notfall Replikation

<!-- source: https://amic.de/hilfe/_notfall_replikation.htm -->

Hauptmenü > Externe Kommunikation > Notfall

In der Maske Notfall kann ein Standortname angegeben werden für den eine Notfall-Replikation erstellt werden soll. Es können auch mehrere Standortnamen angegeben werden. Diese werden dann nacheinander erstellt. Standortnamen dürfen nicht mit Sonderzeichen oder Zahlen beginnen, sondern müssen mit einen Buchstaben beginnen.

Im Feld Name Datenbank werden vorhandenen Notfall-Publikationen angezeigt. Ebenfalls wird dort vom System eingetragen, ob die Notfall-Datenbank erstellt wurde.

Vor dem Ausführen der Prozedur müssen die Einrichterparameter gesetzt werden.

| Einrichterparameter: | Beschreibung |
| --- | --- |
| ReplikationsPfad: | Pfad in der die Datenbank der Notfall-Replikation angelegt wird. Zusätzlicher Ordner für den Standort wird angelegt. |
| PublisherName: | Name des Publishers der Original-Datenbank. Ist dieser nicht vorhanden wird er angelegt. Existiert bereits ein Publisher für die Original Datenbank wird dieser verwendet. |
| BasisDatenbankPfad: | Pfad zur Basisdatenbank. Diese wird in den Replikationspfad mit der Log-Datei kopiert und in den Standortnamen umbenannt. |
| BasisDatenbankName: | Name der Datei der Basisdatenbank. |
| ServerName: | Name des Datenbankservers mit dem die Originaldatenbank verbunden ist. |
| DatenbankName: | Name der Originaldatenbank. |
| QuellServerPfad: | Pfad in dem die Replkikationsdateien und Log-Dateien angelegt werden. Der Pfad wird mit Ordnern für den Publisher und Remote-Benutzern ergänzt. |
| DatenbankPfad: | Pfad zur Original-Datenbank. |

Alle Pfade müssen Netzwerkfreigabe erhalten. Die Pfade müssen als Netzwerkpfade angegeben werden. Der Benutzer, von dem die Notfall-Replikation gestartet wird, muss Lese- und Schreibberechtigung besitzen für alle genutzten Netzwerkfreigaben.

Die Prozedur wird nach drücken von „F10“ ausgeführt. In die Tabelle Filialstamm werden der Publisher und die Remote-Benutzer der Datenbank eingetragen. Die Tabelle Mandantstamm wird von der Original-Datenbank in die Notfall-Replikation übertragen. Die Mandantenbezeichnung wird mit Notfall_“Standortname“ ergänzt.

Es werden zwei Ereignisse erstellt.

1. Dbrexp_Schedule wird jede Minute ausgeführt. Änderungen der Original-Datenbank werden zur Notfall-Datenbank repliziert. Änderungen an der Notfall-Datenbank werden **nicht** zur Original-Datenbank repliziert.

2. Cloudcontrol wird jede Stunde ausgeführt. Daten werden zur Replikationsüberwachung gesendet.

Wenn die Notfall-Replikation erstellt wurde kann die Notfall-Datenbank gestartet werden. Falls die Notfall-Datenbank keine Replikationsdaten verarbeiten kann, muss eventuell noch ein Remote-Reset aller beteiligten Datenbanken durchgeführt werden.
