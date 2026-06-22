# Kundenindividuelle Adressaufbereitung(SPA 136)

<!-- source: https://amic.de/hilfe/_SPA_136.htm -->

Mit diesem Steuerparameter kann die kundenindividuelle Adressaufbereitung mit Adressmasken aktiviert / deaktiviert werden.

| Wert | Text | Beschreibung |
| --- | --- | --- |
| 0 | Nein | Hier wird eine festgelegte Adressaufbereitung aus dem Urzeiten von A.eins verwendet. |
| 1 | Ja | Hier wird die kundenindividuelle Adressaufbereitung mittels [Adressmaske](../../../kunden_und_lieferanten/konstanten_bearbeitung/adressmaske.md) verwendet. Die verwendete Maske kann in der [Anschrift auf der Registerkarte „Zusätze“](../../../kunden_und_lieferanten/kunden_und_lieferantenstamm/anschriften/registerkarten_in_anschriften/zusaetze.md) eingetragen und in [KUAN] erstellt werden. Bitte beachten Sie, dass mit der Einstellung „AMIC-DEFAULT“ in der Anschrift stets die „Kunden-Default“-Maske (1) verwendet wird. |
| 2 | Ja, ohne Nationalitätskennz im Inland | Wie [1], aber wenn im [Staatstamm](../../staatstamm/index.md) die Zollgruppe auf „Inland“ steht, wird unabhängig von der Einrichtung das Post-Länderkennzeichen und der Staats-Name nicht in der Anschrift verwendet. |
