# Löschverhalten bei Vorgangsmappen (SPA 796)

<!-- source: https://amic.de/hilfe/_SPA_796.htm -->

Mit diesem Steuerparameter kann das Löschverhalten für Vorgangsmappen eingestellt werden. Steht der Steuerparameter „Locking in der Vorgangsmappe“ ([SPA 795](./locking_in_der_vorgangsmappe_spa_795.md)) auf „Nein“ wird nur der Stammsatz gelöscht, die Verbindungen an den Belegen bleiben dabei erhalten.

Für ein anderes Löschverhalten muss der Steuerparameter „Locking in der Vorgangsmappe“ auf „Ja“ steht.

Zur Einstellung stehen dann verschiedene Typen zur Verfügung.

| Typ | Wert |
| --- | --- |
| EKKLASSE | Einkaufsklasse ab der kein Beleg mehr vorhanden sein darf.<br>*Beispiel: Bei 1700 dürfen sich keine Belege in der Mappe befinden die eine Rechnung oder höher sind.* |
| VKKLASSE | Verkaufsklasse ab der kein Beleg mehr vorhanden sein darf.<br>*Beispiel: Bei 700 dürfen sich keine Belege in der Mappe befinden die eine Rechnung oder höher sind.* |

Wird bei beiden Werten nichts eingetragen, erfolgt keine Stornierung der Belege. Es wird dann nur der Stammsatz gelöscht und die Verbindung an den Belegen entfernt.
