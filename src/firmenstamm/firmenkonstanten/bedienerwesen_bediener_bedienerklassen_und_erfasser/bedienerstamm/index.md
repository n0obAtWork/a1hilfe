# Bedienerstamm

<!-- source: https://amic.de/hilfe/_bedienerstamm.htm -->

Hauptmenü > Administration > Firmenkonstanten > Bediener

oder Direktsprung **[BD]**

In dieser Variante können Sie A.eins-Bediener definieren.

<details>
<summary>Felder des Bedienerstamm:</summary>

| Felder | Beschreibung |
| --- | --- |
| Kurzname | Benutzerkennung<br>A.eins Login<br> |
| ID | Eindeutige Bediener-Id<br> |
| KlNr | [Bedienerklasse](./bedienerstamm_pfleger.md)<br>Mitglieder der [Controllerklasse](../../zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenstamm/index.md#Controllerklasse) werden gelb hervorgehoben.<br> |
| Bedienerklasse | Bezeichnung der Bedienerklasse<br> |
| Name | Name<br> |
| e-Mail | E-Mail-Adresse<br> |
| KlAdmin | Bedienerklassenadministrator<br> |
| Protokoll | Kennzeichen für Systemhinweise<br> |
| Betrieb | [Filialnummer](../../../../filialsystem/index.md)<br>Mitglieder des aktuellen Mandanten werden grün hervorgehoben.<br> |
| Bezeichnung | Bezeichnung des Betriebes<br> |
| User in DB | Aktivitätskennzeichen des Users in der SQL-Anywhere Datenbank<br>Bei nicht-aktiven Bedienern wird der Kurzname rot hervorgehoben.<br> |
| Sperre | Kennzeichen für logische Sperre<br> |
| Verwendeter Name (Druck) | Angezeigter Name für externe Verwendung (z.B. Druck)<br> |
| Windows Login Name | Name Sicherheit<br>Aktivierte „Name Sicherheiten“ werden grün hervorgehoben.<br> |
| Gelöscht | Kennzeichen für logische Löschung<br> |
| Prüfer | Kennzeichen für Prüfangelegenheiten (z.B. Vieraugen-Prinzip)<br> |
| [Codepage](./bediener_clonen.md) | Selektionsmöglichkeit der Codepage des Bedieners.<br> |

</details>

 

<details>
<summary>Suchmöglichkeiten des Bedienerstamm:</summary>

| Suchen | Beschreibung |
| --- | --- |
| Status | nach Kennzeichen für logische Löschung<br><ul><li>-&nbsp;&nbsp;&nbsp;&nbsp; Aktiv</li><li>-&nbsp;&nbsp;&nbsp;&nbsp; Inaktiv</li><li>-&nbsp;&nbsp;&nbsp;&nbsp; Gelöscht</li><li>-&nbsp;&nbsp;&nbsp;&nbsp; Neu</li></ul> |
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
| Ändern <strong>(F5),</strong> Ansehen **(F6),** Neu **(F8)** | Ruft dem [Pfleger](./bedienerstamm_pfleger.md) des Bedieners auf. |
| Abgleichen mit Systemtabelle **(Shift + F10)** | Replikation<br>Abgleich Systemtabelle **SysRemoteUser** mit Bedienerstamm/Filialstamm mit Bestätigungsabfrage |
| Fremdserver Rechte zuordnen **(Shift + F9)** | Externes Login der Bediener überarbeiten |
| Bediener clonen… | Vervielfältigt ausgewählte Bediener ([Bediener clonen](./bediener_clonen.md)) |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Bedienerstamm: Pfleger](./bedienerstamm_pfleger.md)
- [Bediener clonen](./bediener_clonen.md)
- [Bediener – Codepage](./bediener_codepage.md)
- [Der Bediener „AMIC“](./der_bediener_amic.md)
