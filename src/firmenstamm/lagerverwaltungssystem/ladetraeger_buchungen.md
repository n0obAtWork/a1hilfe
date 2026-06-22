# Ladeträger Buchungen

<!-- source: https://amic.de/hilfe/_ladetraegerbuchungen.htm -->

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Ladeträger

Direktsprung [LVSLT]

Mit dieser Erfassungsmaske können einzelne Positionen von einem Silo/Ladeträger gelöscht werden. Ein Silo/Ladeträger kann auf eine Lokalität gefahren werden. Des Weiteren kann die Menge an einer Position auf dem Ladeträger geändert werden.

Datentabelle Ladeträgerübersicht

| Erfassungsfeld | Bedeutung |
| --- | --- |
| Ladeträgernummer/Silo | In diesem Feld wird die Ladeträgernummer/Silo angezeigt. Per F3 Auswahl kann zwar ein neuer Ladeträger/Silo hinzugefügt werden, da in der Positionsübersicht nicht die einzelne Position angezeigt werden, sollte man immer den Ladeträger/ Silo in der Auswahlliste auswählen und dann per F9 diese Maske starten. |
| Menge | In diesem Feld kann eine Menge angegeben werden, diese kann dann per Funktion auf die ausgewählte Position in der Positionsübersicht gebucht werden. Hierbei ist zum Empfehlen nur ein Silo/Ladeträger in der Auswahlliste auszuwählen. |
| Gewicht | In diesem Feld wir das Bruttogewicht des Ladeträgers aus dem Artikelstamm angezeigt. |
| Lokalität/Silostand | In diesem Feld wird die Lokalität/Silostand angezeigt auf welchem sich der Ladeträger oder das Silo gerade befindet. Die Lokalität/Silostand kann in diesem Feld verändert werden und mit der Funktion Ladeträgerbewegung / Silobewegung wird dann der Ladeträger / Silo auf diese Lokalität / diesen Silostand umgebucht. |

Positionsübersicht

| Feld | Bedeutung |
| --- | --- |
| Ladeträger | Silo/Ladeträger zur [Ladeeinheit](./ladeeinheiten.md) |
| Ladenr. | Aktuelle [Ladeeinheitsnummer](./ladeeinheiten.md) des Silo/Ladeträgers |
| Position | Position in der Ladeeinheit |
| Artikelnr. | Artikelnummer der Position auf der [Ladeeinheit](./ladeeinheiten.md) |
| Artikelbezeichnung | Bezeichnung des Artikels |
| Partienummer | Partienummer der Position auf der [Ladeeinheit](./ladeeinheiten.md) |
| Partiebezeichnung | Bezeichnung der Partie |
| Menge | Menge der Position auf der [Ladeeinheit](./ladeeinheiten.md) |
| ME | Mengeneinheit |
| Owaage Nummer | Waagennummer zu einer Ladeeinheitsposition |

Funktionen

Lösche Ladeeinheit [F7]

Um eine Ladeeinheit zu löschen, müssen erst alle Positionen von der [Ladeeinheit](./ladeeinheiten.md) gelöscht werden. Dazu wird eine Position in der Positionsdatentabelle markiert. Diese wird dann in Rot dargestellt, dann wird über die Funktion Lösche Ladeeinheit [F9] diese Position gelöscht. Dies wird solange wiederholt, bis alle Positionen gelöscht worden sind. Sind alle Positionen gelöscht worden, so bleiben in der unteren Datentabelle nur noch die Ladeeinheitsnummer und die Ladeträgernummer/ Silonummer stehen. Um die Ladeeinheit komplett vom Ladeträger/Silo zu löschen muss noch einmal in das Feld Ladeträgernummer/Silo in der unteren Datentabelle geklickt werden. Jetzt kann die Ladeeinheit gelöscht werden.

Leermeldung [SF7]

Die Leermeldung ist [hier](../siloverwaltung/silo_silobestand/leermeldung.md) beschrieben.

Ladeträgerbewegung/Silobewegung[F9]

Mit dieser Funktion ist es möglich einen Silo/Ladeträger von einer [Lokalität](./lokalitaeten/index.md) / Silostand auf eine andere [Lokalität](./lokalitaeten/index.md) / Silostand umzubuchen. Dazu wird in das Feld Lokalität / Silostand eine Lokalität eingetragen und dann die Funktion ausgeführt.

Dabei ist zu beachten, dass bei einer Lokalitätsumbuchung mit unterschiedlichen zugeordneten Lägern auch eine Lagerumbuchung durchgeführt wird. Das bedeutet, dass vorm Umbuchen geprüft werden muss, ob alle Artikel auf dem Silo/Ladeträger auch im Ziellager der Lokalität vorhanden sind.

Menge verteilen Netto

Mit dieser Funktion kann die Menge einer Position an der [Ladeeinheit](./ladeeinheiten.md) verändert werden. Dazu wird in der oberen Datentabelle die gewünschte Menge eingetragen. In der unteren Datentabelle wird dann die gewünschte Position ausgewählt, jetzt wird die eingetragene Menge an dieser Position verändert. Die eingetragene Menge wird als Nettomenge gebucht.

Menge verteilen Brutto

Mit dieser Funktion kann die Menge einer Position an der [Ladeeinheit](./ladeeinheiten.md) verändert werden. Dazu wird in der oberen Datentabelle die gewünschte Menge eingetragen. In der unteren Datentabelle wird dann die gewünschte Position ausgewählt, jetzt wird die eingetragene Menge an dieser Position verändert. Von der eingetragenen Menge wird noch das Gewicht des Silos\\Ladeträgers abgezogen.
