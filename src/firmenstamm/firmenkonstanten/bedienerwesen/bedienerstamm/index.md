# Bedienerstamm

Hauptmenü > Administration > Firmenkonstanten > Bediener oder Direktsprung **[BD]**

In dieser Variante können Sie A.eins-Bediener definieren.

<details>
  <summary>Felder des Bedienerstamm:</summary>

  | Felder | Beschreibung |
  | :----- | :----------- |
  | Kurzname | <p>Benutzerkennung</p><p>A.eins Login</p> |
  | ID | Eindeutige Bediener-Id |
  | KlNr | <p>Bedienerklasse</p><p>Mitglieder der Controllerklasse werden gelb hervorgehoben.</p> |
  | Bedienerklasse | Bezeichnung der Bedienerklasse |
  | Name | Name |
  | e-Mail | E-Mail-Adresse |
  | KlAdmin | Bedienerklassenadministrator |
  | Protokoll | Kennzeichen für Systemhinweise |
  | Betrieb | <p>Filialnummer</p><p>Mitglieder des aktuellen Mandanten werden grün hervorgehoben.</p> |
  | Bezeichnung | Bezeichnung des Betriebes |
  | User in DB | <p>Aktivitätskennzeichen des Users in der SQL-Anywhere Datenbank.</p><p>Bei nicht-aktiven Bedienern wird der Kurzname rot hervorgehoben.</p> |
  | Sperre | Kennzeichen für logische Sperre |
  | Verwendeter Name (Druck) | Angezeigter Name für externe Verwendung (z.B. Druck) |
  | Windows Login Name | <p>Name Sicherheit</p><p>Aktivierte „Name Sicherheiten“ werden grün hervorgehoben.</p> |
  | Gelöscht | Kennzeichen für logische Löschung |
  | Prüfer | Kennzeichen für Prüfangelegenheiten (z.B. Vieraugen-Prinzip) |
  | Codepage | Selektionsmöglichkeit der Codepage des Bedieners. | 
</details>

<details>
  <summary>Suchmöglichkeiten des Bedienerstamm:</summary>

  | Suchen | Beschreibung |
  | :----- | :----------- |
  | Status | <p>nach Kennzeichen für logische Löschung</p><ul><li>Aktiv</li><li>Inaktiv</li><li>Gelöscht</li><li>Neu</li></ul> |
  | Bedienerklasse | von … bis … |
  | Kurzname | % |
  | Bediener-Id | von … bis … |
  | Betrieb/Filiale | von … bis … |
</details>

<details>
  <summary>Funktionen des Bedienerstamm:</summary>

  Es stehen folgende Funktionen zur Verfügung:

  | Funktionen | Beschreibung |
  | :--------- | :----------- |
  | Ändern **(F5)**, Ansehen **(F6)**, Neu **(F8)** | Ruft dem Pfleger des Bedieners auf. |
  | Abgleichen mit Systemtabelle **(Shift + F10)** | <p>Replikation</p><p>Abgleich Systemtabelle SysRemoteUser mit Bedienerstamm/Filialstamm mit Bestätigungsabfrage</p> |
  | Fremdserver Rechte zuordnen **(Shift + F9)** | Externes Login der Bediener überarbeiten |
  | Bediener clonen… | Vervielfältigt ausgewählte Bediener (Bediener clonen) |
</details>

<p class="siehe-auch">Siehe auch:</p>

- [Bedienerstamm: Pfleger](bed_pfleger.md)