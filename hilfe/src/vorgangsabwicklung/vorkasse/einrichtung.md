# Einrichtung

<!-- source: https://amic.de/hilfe/_vorkasseeinrichtung.htm -->

### Einrichtung der Vorkasse

Steuerparameter die von der Vorkasse ausgewertet und Benutzt werden.

- [Kreditlimit-Prüfung mit Auftrag/Bestellung(SPA 234)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_mit_auftrag_bestellg_spa_234.md)
- [Vorkasse Ladescheinunterklasse(SPA 693)](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/mengenbuchung_bei_fibu_uebertrag_spa_720.md)
- [Vorkasse Auftragsunterklasse(SPA 694)](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/vorkasse_auftragsunterklasse_spa_694.md)
- Ladeschein ins Kreditlimit einberechnen(SPA 695)

### Einrichterparameter auf der Vorkasse Erfassungsmaske

- Preisaufschlag/Abschlag für die Lieferungssorte

  Bei Rohwarenlieferungen kann hier schon ein Aufschlag für die Qualitäten eingetragen werden. Dieser wird auf der Maske angezeigt und kann dort abgeändert werden.

### Benötigte Vorgangsklassen und Unterklassen

| Vorgangsklasse | Unterklasse | Bedeutung |
| --- | --- | --- |
| 100 | egal | Angebot |
| 400 | Siehe SPA Einstellung 694 | Auftrag |
| 500 | Siehe SPA Einstellung 693 | Ladeschein |
| 600 | 9999 | Rohwarelieferschein |
| 660 | egal | Kontrakt |
