# Vorgangsdruckklassen

<!-- source: https://amic.de/hilfe/_vorgangsdruckklassen.htm -->

Hauptmenü > Administration > Drucker > Vorgangdruckklassen

oder Direktsprung **[VRGD]**

Zuordnung der Vorgangsdruckerklassen zu den einzelnen Vorgangsklassen, wie z.B. Nr. 600 für Lieferschein

Definieren Sie zu welcher Vorgangsklasse/Unterklasse diese Druckklasse gültig sein soll.

| Spalte | Beschreibung |
| --- | --- |
| Vorgangsklasse | Vorgangsklasse |
| | Nummer der Unterklasse |
| Unterklasse | Unterklassenbezeichnung |
| RohwareAbr.Typ | Rohwaren-Abrechnungstyp |
| Verwendung | Verwendung des Drucks |
| Gültig ab | Gültigkeitsdatum dieser Definition |

<details>
<summary>Funktion Formulare / Drucker zuordnen</summary>

Hauptmenü > Administration > Drucker > Vorgangdruckklassen

oder Direktsprung **[VRGD]**

Mit Hilfe der Funktion ***Formulare / Drucker zuordnen F5*** definieren Sie, auf welchem Drucker, mit welchem Schacht, mit welchem Formular der Druck mit/ohne openTRANS mit/ohne Mailversand wie oft gedruckt werden soll.

| Spalte | Beschreibung |
| --- | --- |
| Nr | Laufende Nummer |
| Formular | Formularnummer |
| | Formularbezeichnung |
| Schacht | Druckerschacht. Dieser kann z.B. zur Verwendung von Papieren unterschiedlicher Farben oder Briefköpfe verwendet werden.<br>0) Keine Schachtauswahl<br>1) Schacht 1<br>2) Schacht 2<br>3) Erste Seite auf Schacht 1 weitere auf Schacht 2<br>4) Erste Seite auf Schacht 2 weitere auf Schacht 1 |
| Drucker | Druckernummer |
| | Druckerbezeichnung |
| Effektsteuerung | Es stehen drei Möglichkeiten zur Auswahl:<br>1 keine Effektsteuerung<br>2 Ladeliste (im Standard keine Auswertung)<br>3 Lagerabholschein<br>„keine Effektsteuerung“ ist hier die Vorbelegung für das Feld.<br>Lagerabholschein bewirkt, dass dieses Formular nur dann gedruckt wird, wenn es unter den Artikeln mindestens einen Artikel gibt, der als Lagerartikel gekennzeichnet ist. Siehe dazu [weitere Funktionen der Tresenkasse](../../kasse/das_barverkaufssystem/erfassung_mit_den_verschiedenen_kassensystemen/tresenkasse/weitere_funktionen_der_tresenkasse.md). |
| Raffung | |
| Makro | Makro welches vor dem Druck ausgeführt wird. Übergabe von bis zu 4 Parameter. Makroname und Parameter müssen mit einem Leerzeichen getrennt sein.<br>Beispiel: Makroname Parameter1 Parameter3 Parameter3 Parameter4 |
| openTRANS | Gibt an, ob das PDF dieses Drucks eine openTRANS-Datei angehängt bekommen soll (nur wenn openTRANS aktiv) |
| Belegversand | Gibt an, ob dieser Druck jener ist, das zum Versand verwendet wird (ggf. wird eine Kopie des Drucks nicht erstellt, wenn die Einstellung „statt Rechnungsdruck“ aktiv ist (siehe unten) |
| Anzahl | Anzahl der auf diesem Drucker/Schacht auszudruckenden Kopien |

Für den Belegversand gibt es drei verschiedene Einstellungsmöglichkeiten.

| Wert | Beschreibung |
| --- | --- |
| 0 – Nein | Dieser Druck wird nicht für den Belegversand verwendet |
| 1 – Ja | Dieser Druck wird für den Belegversand verwendet. Ist im Kunden „statt Rechnungsdruck“ eingestellt, dann wird ein Exemplar weniger als angegeben physikalisch gedruckt. |
| 2 – Exklusiv | Dieser Druck wird für den Belegversand verwendet. Jedoch wird dieses Formular nur gedruckt, wenn im Kunden „mit Rechnungsdruck“ angegeben ist. Bei Kunden ohne Belegversand wird dieses Formular nie gedruckt.<br>Dies ist vorgesehen für Formulare mit Briefkopf, der auf dem physikalischen Drucker bereits auf dem Druckerpapier vorhanden ist. |

</details>
