# DSGVO Objektdefinition

<!-- source: https://amic.de/hilfe/dsgvoobjektdefinition.htm -->

Hauptmenü > Stammdatenpflege > Anschriften > DSGVO

Direktsprung <strong>[DSGVO]</strong> Variante DSGVO-Objekte.

In A.eins existieren verschiedene Anschriftenarten für die jeweils Objekte definiert wurden. Hierzu gehören Kundenanschriften, Lageranschriften, Filialanschriften und viele mehr. Diese Objekte fassen alle Tabellen zusammen, welche die DSGVO betreffen. Zu dem Kunden/Lieferanten-Objekt gehören beispielsweise folgende Tabellen:

- Kundenstamm
- Anschriftstamm
- Kundenmatchcode
- Kundenaddon

AMIC liefert bereits vorgegebene Definitionen solcher Objekte aus. Bearbeiten Sie diese mit der Anwendung „DSGVO“ in der Variante „DSGVO-Objekte“.

| | Bedeutung |
| --- | --- |
| Objekt | Kennung des verwendeten Objekts.<br>Alle Objekte sind von AMIC vorgegeben und es können keine eigenen Objekte angelegt werden. Es können aber jederzeit Tabellen entfernt oder hinzugefügt werden. Diese werden mit dem Zusatz „Privat“ vermerkt und gespeichert. |
| Referenztabelle | Nur Anzeige:<br>Jedes Objekt bezieht sich auf mehrere Tabellen. Die hier angezeigte Tabelle ist die Basis, auf die sich alle anderen Tabellen des Objekts beziehen.<br> |
| Referenzfeld (IDENT)/(ADRESSID) | Nur Anzeige:<br>Der Feldname aus der Referenztabelle, auf den sich der „IDENT“- oder der „ADRESSID“- Wert beziehen.<br> |
| Tabelle | Zum Objekt gehörende Tabellen.<br> |
| Bestimmt durch | Durch diesen Spaltennamen wird bestimmt, welche Daten der DSGVO zugeordnet werden. |
| Aktion | Was soll mit den Feldern/der Tabelle geschehen? Mögliche Aktionen sind:<br>• anonymisieren: Die Daten in den Feldern werden mit \* überschrieben.<br>• löschen: Die Daten in den Feldern werden gelöscht.<br>Beispiel:<br>Die Tabelle „KundenMatchcode“ enthält ein Feld. In diesem Fall löschen Sie diese Daten. |
| WHERE-Bedingung für die Liste | Um die Datensätze, zu bestimmen, muss eine verknüpfende WHERE-Bedingung angegeben werden.<br>Für die Tabelle „Anschriftstamm“ des Objektes „Anschrift Vertreter“ sieht die verknüpfende WHERE-Bedingung wie folgt aus:<br><pre><code>where adresstyp=4 and adressid=(select adressid from&#10; vertstamm where vertnummer=:IDENT)</code></pre><br> <br>Das Feld IDENT enthält jeweils die Nummer des ausgewählten Datensatzes. In den von AMIC ausgelieferten Objektdefinitionen finden Sie zu allen Objekten Beispiele.<br> |
| WHERE-Bedingung für das Anonymisieren | Es kann vorkommen, dass die Anonymisierung mehr Daten enthält als die Liste. Zum Beispiel, wenn Anschriften archiviert werden. Für diese Fälle ist es möglich eine abweichende verknüpfende WHERE-Bedingung anzugeben.<br>Bleibt das Feld leer, verwendet die Anonymisierung die verknüpfende WHERE-Bedingung der Liste.<br> |

Vor dem Speichern werden beide WHERE-Bedingungen ausgeführt. Bei einer auftretenden Fehlermeldung wird nicht gespeichert. Bessern Sie nach und speichern erneut.

Um von AMIC vorgegebene Tabellen aus einem Objekt zu entfernen, löschen Sie diese Objekte mit der Funktion „***Löschen/wiederherstellen***“ (**F7**). In der Form behandelte Objekte werden als „gelöscht“ gekennzeichnet. Zum Wiederherstellen verwenden Sie dieselbe Funktion.

Ein mit dem Löschkennzeichen versehenes Objekt wird nicht mehr ausgewertet. In der Auswahlliste ist dies in Spalte „gelöscht“ hinterlegt.
