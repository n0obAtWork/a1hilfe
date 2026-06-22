# Lagerumbuchung

<!-- source: https://amic.de/hilfe/_celagerumbuchung.htm -->

Eine Lagerumbuchung kann nur dann erfolgreich mit dem Scanner abgearbeitet werden, wenn diese im System erfasst worden ist. Eine Lagerumbuchung wird unter [LGU] angelegt.

Des Weiteren ist zu beachten, dass der Scanner auf dem Ziellager arbeitet. Das Lager kann unter [VKONS] eingerichtet werden. Dazu muss man sich mit dem Bediener des Scanners in A.eins anmelden und das Lager umstellen.

Es muss ein AMIC Etikettendruck Dokument eingerichtet werden, welcher die Scancodes in EAN 128 Codiert für die Lagerumbuchung enthält.

Folgende Scancodes werden für die Lagerumbuchung benötigt.

1. LGU + V_numnummer + eventuell die Lokalitätsnummer z.B. LGU 4711 123

2. STORNO um den letzten abgesetzten Befehl zu stornieren

3. LGUENDE um die Lagerumbuchung abzuschließen

Folgende [Einrichtungen](../anwendung_scanner_in_aeins/einrichtung_des_scanners_an_der_zentral_datenbank/server_starten.md) haben direkten Einfluss auf die Bearbeitung einer Lagerumbuchung mit dem Scanner.

| Felder auf Registerkarte Vorgangseinstellungen | Bedeutung |
| --- | --- |
| Warenbewegungsaddonfeld | Als Sonderfunktion steht die Möglichkeit bereit, die Originalmenge des Beleges vor Korrektur zu sichern, hierbei kann ein beliebiges Feld in dem Warenbewegungsaddon definiert werden. |
| Unbearbeitete Position auf 0 setzen | Mit diesem Parameter kann eingestellt werden, ob alle nicht bearbeiteten Positionen auf 0 gesetzt werden sollen. Dies ist nur für den Fall Interessant wenn keine Teildisposition gemacht wird. |
| Teildisposition Lagerumbuchung | Mit dieser Einstellung kann eingestellt werden, ob der Beleg der mit dem Scanner bearbeitet wird korrigiert werden soll, oder ob eine Teildisposition vorgenommen werden soll. Dies bedeutet, dass ein neuer Vorgang mit der nicht gelieferten Ware erstellt wird. |
| Buchungstyp Lagerumbuchung | Hier kann hinterlegt werden, welcher Buchungstyp der Lagerumbuchung zugeordnet werden soll, nach dem diese erfolgreich Bearbeitet worden ist. |
| Lagerumbuchung Addon Speichern | Sollen die im Addon Daten gespeichert werden. |
| Leervorgang Löschen | Hier kann eingestellt werden, ob ein Auftrag mit leeren Positionen gelöscht werden soll. |
| Lösche 0 Positionen | Mit dieser Funktion kann eingestellt werden, ob Positionen mit einer 0 Menge aus dem Vorgang gelöscht werden sollen. |
| Gebindefaktor aus Vorgang | Hier kann eingestellt werden, ob der Gebinde Faktor aus dem Vorgang gelesen werden soll. |

Um eine Lagerumbuchung mit dem Scanner durchzuführen wird wie folgt vorgegangen.

1. Scannen des Befehles LGU

2. Erfassen der Ware(Artikelean, Menge und eventuell noch eine Partie )

3. Die Lagerumbuchung wird mit dem Befehl LGUENDE beendet.

Ablauf

• Als erstes wird der Startscancode erfasst wie z.B. LGU 4711. Beim Startscancode muss immer zwischen dem LGU und der Vorgangsnummer ein Leerzeichen stehen. Nach dem der Startscancode erfasst worden ist, werden im unteren Teil des Scanner Bildschirmes alle Position der Lagerumbuchung angezeigt. Enthält die Lagerumbuchung mehr als neun Positionen so kann mit den Pfeil hoch und Pfeil runter Tasten geblättert werden.

• Jetzt kann eine Position aus der Lagerumbuchung eingescannt werden. Die Suche, der Position in der Lagerumbuchung funktioniert wie folgt.

1 Wird nur der Artikel pro Position erfasst wird der erste gefundene Artikel in der Lagerumbuchung bearbeitet.

2 Wird jetzt eine Partie zu diesem Artikel erfasst, so wird jetzt in der Lagerumbuchung nach der Kombination Artikel und Partie gesucht.

• Die Menge kann entweder per Hand eingegeben oder eingescannt werden. Es kann pro Position im Auftrag nur eine Partie geben.

• Der Scanner unterstützt beim Erfassen keine Partieverteilung. Dies bedeutet, wenn mehrere Partien eines Artikels umgebucht werden sollen, muss in der Lagerumbuchung pro Partie eine Position angelegt werden.

• Nach dem abarbeiten der Positionen wird LGUENDE eingescannt.

Gebinde

Für das Erfassen des Gebindes gibt es mehrere Einstellungsmöglichkeiten. In dem Standardfall kann nur die Ergebnismengeneinheit angegeben werden. Wird der Schalter Gebindefaktor aus Vorgang auf ja gestellt, so wird die eingegebene Menge als Anzahl genommen und der Gebindefaktor aus dem Auftrag gelesen. Des Weiteren kann an der Mengeneinheit selbst noch einmal hinterlegt werden, ob die Mengenangabe als Gesamt Menge oder als Gebinde Anzahl gewertet werden soll.
