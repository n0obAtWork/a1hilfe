# Usage

<!-- source: https://amic.de/hilfe/_tammo_usage.htm -->

Nach der vollständigen Eintragung einer Mail, löst Tammo einen Eintrag in den Datenstrom aus. Der Prozess dazu lautet TammoExecute. Der Kommandoparameter „LOOPSLEEPTIME“ erlaubt eine Dauerschleife des Tammo-Prozesses mit einer Wartezeit zwischen den Mailevents von „LOOPSLEEPTIME“-Sekunden.

### Heartbeat

Um eine Überwachung des Prozesses zu ermöglichen, kann mit dem Parameter „HEARTBEATSECONDS“ festgelegt werden, in welchem Intervall ein Timestamp Eintrag in die Tabelle „TammoInformation“ erfolgen soll.

Dadurch kann man mit Hilfe eines Datenbankevents überprüfen, ob der Prozess schon länger nicht gelaufen ist.

### Formulararchivgruppe

Die Mail und alle Anhänge werden im Formulararchiv in einer Gruppe zusammengefasst. Anhand dieser kann man die Dokumente zusammengehörenden Dokumente identifizieren.

Der Name der Gruppe ist ein vorangestelltes „Tammo“ und eine GUID. Sie könnte wie folgt aussehen:

```text
Tammo-{98cb6768-7fbd-477d-a4fa-1564fd46dc90}
```
