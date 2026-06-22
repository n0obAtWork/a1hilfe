# Bedienerstamm

<!-- source: https://amic.de/hilfe/_bedienerstamm.htm -->

Hauptmenü > Administration > Firmenkonstanten > Bediener

oder Direktsprung [BD]

In dieser Variante können Sie A.eins-Bediener definieren.

<details>
<summary>Felder des Bedienerstamm:</summary>

| Felder | Beschreibung |
| --- | --- |
| Kurzname | Benutzerkennung  
A.eins Login  
 |
| ID | Eindeutige Bediener-Id  
 |
| KlNr | [Bedienerklasse](./bedienerstamm_pfleger.md)  
Mitglieder der [Controllerklasse](../../zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenstamm/index.md#Controllerklasse) werden gelb hervorgehoben.  
 |
| Bedienerklasse | Bezeichnung der Bedienerklasse  
 |
| Name | Name  
 |
| e-Mail | E-Mail-Adresse  
 |
| KlAdmin | Bedienerklassenadministrator  
 |
| Protokoll | Kennzeichen für Systemhinweise  
 |
| Betrieb | [Filialnummer](../../../../filialsystem/index.md)  
Mitglieder des aktuellen Mandanten werden grün hervorgehoben.  
 |
| Bezeichnung | Bezeichnung des Betriebes  
 |
| User in DB | Aktivitätskennzeichen des Users in der SQL-Anywhere Datenbank  
Bei nicht-aktiven Bedienern wird der Kurzname rot hervorgehoben.  
 |
| Sperre | Kennzeichen für logische Sperre  
 |
| Verwendeter Name (Druck) | Angezeigter Name für externe Verwendung (z.B. Druck)  
 |
| Windows Login Name | Name Sicherheit  
Aktivierte „Name Sicherheiten“ werden grün hervorgehoben.  
 |
| Gelöscht | Kennzeichen für logische Löschung  
 |
| Prüfer | Kennzeichen für Prüfangelegenheiten (z.B. Vieraugen-Prinzip)  
 |
| [Codepage](./bediener_clonen.md) | Selektionsmöglichkeit der Codepage des Bedieners.  
 |

</details>

 

<details>
<summary>Suchmöglichkeiten des Bedienerstamm:</summary>

| Suchen | Beschreibung |
| --- | --- |
| Status | nach Kennzeichen für logische Löschung  
\- Aktiv  
\- Inaktiv  
\- Gelöscht  
\- Neu |
| Bedienerklasse | von … bis … |
| Kurzname | % |
| Bediener-Id | von … bis … |
| Betrieb/Filiale | von … bis … |

</details>

<details>
<summary>Funktionen des Bedienerstamm:</summary>

Es stehen folgende Funktionen zur Verfügung:

| Funktionen | Beschreibung |
| --- | --- |
| Ändern (F5), Ansehen (F6), Neu (F8) | Ruft dem [Pfleger](./bedienerstamm_pfleger.md) des Bedieners auf. |
| Abgleichen mit Systemtabelle (Shift + F10) | Replikation  
Abgleich Systemtabelle **SysRemoteUser** mit Bedienerstamm/Filialstamm mit Bestätigungsabfrage |
| Fremdserver Rechte zuordnen (Shift + F9) | Externes Login der Bediener überarbeiten |
| Bediener clonen… | Vervielfältigt ausgewählte Bediener ([Bediener clonen](./bediener_clonen.md)) |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Bedienerstamm: Pfleger](./bedienerstamm_pfleger.md)
- [Bediener clonen](./bediener_clonen.md)
- [Bediener – Codepage](./bediener_codepage.md)
- [Der Bediener „AMIC“](./der_bediener_amic.md)
