# Kassensicherungsverordnung Einrichtung

<!-- source: https://amic.de/hilfe/_kassenSichVeinrichtung.htm -->

Hauptmenü > Barvorgänge > Stammdaten > Kassensicherungsverordnung Einrichtung

Dieser Dialog pflegt Einrichtungsdaten der Kassensicherungsverordnung in A.eins.

Dies ist genauer eine Spezialisierung des Steuerparameter-Pflegesystems in A.eins.

Der Steuerparameter **1056** (Kassensicherungsverordnung) in der Gruppe **53** (Kasse / Barverkauf) ist der Steuerparameter (SPA) der die Einrichtungsparameter vorhält.

Die Einrichtungsparameter sind:

1. TSE-relevante Daten

2. Kassenbarcode-Ermittlungsprozeduren

3. TSE-Kommunikationsprozeduren

<details>
<summary>Kopfdaten</summary>

| Feld | Beschreibung |
| --- | --- |
| Steuerparameter | Nummer des SPA  
Name des SPA |
| SPA-Gruppe | Nummer der SPA-Gruppe  
Name der SPA-Gruppe |
| Lizenz | Nummer der Lizenz  
Name der Lizenz |

</details>

<details>
<summary>Register Prozeduren</summary>

| Feld | Beschreibung |
| --- | --- |
| Gültig ab | Datum ab wann diese Steuerparameter, also die Kassensicherungsverordnungs-Einrichtungen, gültig sind |
| Kassenbarcode | Der Name der Prozedure, die die Daten für den Kassenbarcode liefert. |
| Get Finish | Der Name der Prozedure für den Get / Finish |
| Get Start | Der Name der Prozedure für den Get / Start |
| Set Finish | Der Name der Prozedure für den Set / Finish |
| Set Start | Der Name der Prozedure für den Set / Start |
| Bon-UStId | Ergibt sich aus den Anwendungsfunktionen unter **[Forma]** - Variante 2: Anwendungsfunktionen - ***AF_KSV_UsId.*** Hier können die Parameter angepasst werden. |

</details>

<details>
<summary>Funktionen</summary>

| Funktionen | Beschreibung |
| --- | --- |
| Neu **(F7)** | Setzt ein neues Datum ab dem dieser Steuerparameter (SPA) aktiv ist. |
| Löschen **(F8)** | Löscht den Eintrag des aktuellen Steuerparameter (SPA). |
| TSE pflegen **(F10)** | Ruft die [Auswahlliste](./tse_auswahlliste/index.md) mit allen eingerichteten TSE auf. |
| Formulare pflegen | Ruft die Maske zum [Formulare pflegen](./formulare_pflegen.md) auf. |
| Aufbereitungsstatus ändern **(F3)** | Ändert das Feld in eine private Variante. |
| Aufbereitungsprozedur ändern **(F5)** | Bearbeitet die private Variante. |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [TSE Auswahlliste](./tse_auswahlliste/index.md)
- [Formulare pflegen](./formulare_pflegen.md)
