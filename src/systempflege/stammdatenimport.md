# Stammdatenimport

<!-- source: https://amic.de/hilfe/_stammdatenimportpfleger.htm -->

Hauptmenü > Externe Kommunikation > Stammdatenimport > Stammdatenimport

Mit dieser Anwendung können Importe für Kunde, Artikel und Artikelpreise durchgeführt werden. Die Import Dateien müssen im dbf Format vorliegen. Anhand eines Scripts werden die Daten dann in die jeweilige AMIC Tabelle geschrieben. Beim Artikel ist es die Tabelle AMC_Artikel. Aus dieser Tabelle werden dann im zweiten Schritt die Daten in die richtigen Relationen verteilt.

<details>
<summary>Felder des Stammdatenimport</summary>

| Felder | Bedeutung |
| --- | --- |
| Bezeichnung | Bezeichnung des Imports<br> |
| Import Typ | Art des Importes<br> |
| Datei | Pfad zur Datei, welche die zu Importierenden Daten enthält.<br> |

</details>

 

<details>
<summary>Funktionen des Stammdatenimport</summary>

| Funktionen | Bedeutung |
| --- | --- |
| Ändern **(F5)**, Ansicht **(F6)**, Neu **(F7)**, Löschen **(F8)** | Ruft den Pfleger auf<br> |
| Ausführen **(F9)** | Führt den Import aus<br> |

</details>

 

### Neuanlage eines Imports

Mit ***Neu*** oder **F8** kann ein neuer Import angelegt werden.

Es gibt drei Arten des Imports

1. Artikelimport

2. Artikelpreisimport

3. Kundenimport

| Eingabefelder | Bedeutung |
| --- | --- |
| Name | Hier wird der Name des Imports hinterlegt z.B. Artikelimport<br> |
| Import Typ | Art des Importes<br> |
| Datei | Pfad zur Datei welche die zu Importierenden Daten enthält.<br> |
| Scriptdatei | Die Scriptdatei wird in Abhängigkeit des Importtyps gesetzt. Soll nicht die Standard-Importdatei genutzt werden, so kann hier ein eigenes SQL oder Makro eingetragen werden.<br> |
| Scripttyp | Hier wird der Scripttyp eingetragen zur Auswahl stehen.<br>1. SQL Script<br>2. A.eins Makro<br> |
| Importdatei löschen | Soll die Importdatei nach dem erfolgreichen Import gelöscht werden.<br> |

### Endkontrolle für den Stammdatenimport

Hauptmenü > Externe Kommunikation > Stammdatenimport > Endkontrolle/Einspielung Artikel

Hauptmenü > Externe Kommunikation > Stammdatenimport > Endkontrolle/Einspielung Preise

Hauptmenü > Externe Kommunikation > Stammdatenimport > Endkontrolle/Einspielung Kunden
