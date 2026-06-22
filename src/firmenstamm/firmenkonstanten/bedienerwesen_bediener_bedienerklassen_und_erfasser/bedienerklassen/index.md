# Bedienerklassen

<!-- source: https://amic.de/hilfe/_einrichtungbedienerk.htm -->

Hauptmenü > Administration > Firmenkonstanten > Bedienerklassen

oder Direktsprung **[BDKL]**

Bedienerklassen fassen Mitarbeiter mit gleichen Rechten zusammen; sie dienen also somit einer Strukturierung der Rechtevergabe.

<details>
<summary>Felder der Bedienerklasse:</summary>

In der Variante „Bedienerklasse“ werden folgende Felder behandelt:

| Felder | Beschreibung |
| --- | --- |
| Bedienerklasse | Im Normalfall eine nicht-negative Zahl die Bediener gruppiert.<br>Als Spezialfälle gibt es technische Bedienerklassen [Bedienerklasse: Defaultklasse](./standard_bedienerklassen.md). |
| Bezeichnung | Bezeichnung der Bedienerklasse |
| Betriebsstätte | Bei angeschlossenem Filialsystem Zuordnung der Bedienerklasse zur Betriebsstätte.<br>Standard: 0, ohne Filiale |
| Bezeichnung Betriebsstätte | Bezeichnung der Betriebsstätte |
| Gesperrt | JA/NEIN<br>Login-Sperre aller Bediener dieser Bedienerklasse |
| Sicherheitsklasse | [Sicherheitsklasse](./standard_bedienerklassen.md) |
| Controllerklasse | Controllerklassen werden gelb hervorgehoben. |
| Bedienerlisten-Info | Liste der zugehörigen Bediener.<br>Angezeigt werden nur Bediener, die weder gelöscht oder gesperrt sind, und die in der Datenbank als User verzeichnet sind. |

</details>

<details>
<summary>Suchmöglichkeiten der Bedienerklasse:</summary>

| Suchkriterien |
| --- |
| Bedienerklasse | von … bis … |

</details>

<details>
<summary>Funktionen der Bedienerklasse</summary>

In der Variante „Bedienerklasse“ stehen folgende Funktionen zur Verfügung.

| Funktionen | Beschreibung |
| --- | --- |
| Ändern **(F5)**, Ansehen **(F6)**, Löschen **(F7)**, Neu **(F8)** | Ruft den [Bedienerklasse: Pfleger](./bedienerklasse_pfleger.md) auf. Innerhalb dieser Funktion lässt sich mit ***Speichern unter…*** eine neue Bedienerklasse erzeugen.<br>Dabei werden dann auch sämtliche Schutzeinstellungen der Funktionen ([Zugriffsrechte Funktionen](../../zugriffsrechte_funktionen.md)) übernommen. |
| EPAs zeigen **(F10)** | Individuelle Steuerungen von Abläufen können in Anwendungen über Einrichterparameter (EPA) vorgenommen werden.<br>Diese Funktion ruft die entsprechende Anwendung zur Ansicht und Pflege der Einrichterparameter auf.<br>Hauptmenü \> Administration \> Steuerung \> EPAs zeigen<br>Direktsprung **[EPAZ]** |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Bedienerklasse: Pfleger](./bedienerklasse_pfleger.md)
- [Standard Bedienerklassen](./standard_bedienerklassen.md)
