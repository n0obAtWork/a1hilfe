# Protokoll

<!-- source: https://amic.de/hilfe/_db_protokoll.htm -->

Hauptmenü > Systempflege > Abstimmung > Protokollauswertung

oder Direktsprung **[PROTO]**

Mit der Anwendung Protokoll lassen sich leicht Änderungen von Daten mitverfolgen. Dafür können für alle Tabellen unterschiedliche Spalten mitprotokolliert werden.

Hierbei ist zu beachten, dass immer Protokolleinträge für eine Tabelle mit aktivierter Protokollierung erfasst werden, selbst wenn die Tabellenspalten, die Änderungen enthalten, nicht explizit in die Erfassungsfeldliste eingetragen wurden.

Auf der Erfassungsmaske stehen zwei Datentabellen zum Erfassen zur Verfügung und die Funktionen zum Anlegen der Protokolltrigger.

[Tabellen](./index.md#datentabelle_tabellen)

[Spalten](./index.md#datentabelle_spalten)

[Funktionen](../../vorgangsabwicklung/formularzuordnung/funktionen.md)

Datentabelle zum Erfassen der Tabellen.

| Feld | Bedeutung |
| --- | --- |
| Protokolltabellen | Name der Tabelle die protokolliert werden soll. |
| XML | Hiermit kann angegeben werden, ob die Daten in einer XML-Struktur gespeichert werden sollen oder ob die Daten einfach hintereinander weg geschrieben werden. (siehe [Beispiel](./protokollstruktur.md)) |
| Änderung | Steht dieses Feld auf „Ja“, wurde eine Änderung an den Spalten gemacht. Deswegen sollte die Überwachung neu gestartet werden. |
| Update | Dieses Feld sagt aus, ob zurzeit ein Updatetrigger zum Protokollieren existiert. |
| Insert | Dieses Feld sagt aus, ob zurzeit ein Inserttrigger zum Protokollieren existiert. |
| Delete | Dieses Feld sagt aus, ob zurzeit ein Deletetrigger zum Protokollieren existiert. |
| Einzelfeldüberwachung | Liste der Spalten, die beim Aufruf der Maske für die Tabelle schon zugeordnet sind. |

Datentabelle zum Erfassen der Spalten.

| Feld | Bedeutung |
| --- | --- |
| PK | Das Feld gibt an, ob es sich um ein Primärschlüsselfeld handelt.<br>Diese Felder dürfen nicht entfernt werden und werden bei jedem speichern der Tabelle wieder hinzugefügt. |
| Feldliste | Name der Spalte die überwacht werden soll. |
| Bezeichnung | Hier kann eine Bezeichnung für die Spalte hinterlegt werden. Verwendet wird dieses Feld jedoch nur, wenn die Tabelle im XML-Modus protokoliert wird. |

Funktionen zum Anlegen der Trigger

| Funktion | Bedeutung |
| --- | --- |
| Überwachung starten | Mit dieser Funktion lassen sich die Protokolltrigger der aktuellen Tabelle anlegen. |
| Überwachung beenden | Mit dieser Funktion werden die Protokolltrigger der aktuellen Tabelle entfernt. |

*Hinweis:*

*Die einzelnen Trigger fangen mit „p_pro_u_“, „p_pro_i_“, „p_pro_d_” an, dahinter befindet sich dann immer der Tabellenname (z.B. „p_pro_u_kontraktstamm“). Sollte es schon private Trigger mit der Bezeichnung geben, werden diese gelöscht/überschrieben.*

*Angeschaut werden können sich die Trigger unter den privaten Datenbanktriggern (Direktsprung SQLPT).*

<p class="siehe-auch">Siehe auch:</p>

- [Löschen einer Zeile in der Einrichtung fürs Protokoll](./loeschen_einer_zeile_in_der_einrichtung_fuers_protokoll.md)
- [Protokollstruktur](./protokollstruktur.md)
