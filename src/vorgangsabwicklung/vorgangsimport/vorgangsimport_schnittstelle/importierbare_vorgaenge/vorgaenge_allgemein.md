# Vorgänge allgemein

<!-- source: https://amic.de/hilfe/vorgngeallgemein.htm -->

**Vorgang teildisponieren**

Vorgänge können Positionen aus anderen Vorgängen teildisponieren. Dazu wird im Feld interneReferenz der Position die WabewGuid der Vorgängerposition eingetragen. Aus dieser Position wird dann in diese neue teildisponiert.

**Vorgang ändern**

Vorgänge können auf zweierlei Weisen geändert werden:

1. Changing

Hier werden nur die im Import gegebenen Komponenten geändert.

Nicht vorhandene Positionen werden hinzugefügt.

Nicht gegebene Positionen werden unverändert bleiben.

Zu diesem Zweck wird der ImportTyp im Vorgangstamm auf 10 gesetzt.

2. Explizit –

Jede Komponente muss gegeben werden. Jede nicht gegebene Komponente wird entfernt.

Nicht vorhandene Positionen werden hinzugefügt.

Zu diesem Zweck wird der ImportTyp im Vorgangstamm auf 11 gesetzt.

Als Referenz für die im Beleg zu ändernder Position gilt die WabewGuid des aktuellen Beleges.

<p class="just-emphasize">Daten Manipulation vorm Erzeugen eines Vorgangs:</p>

Es besteht die Möglichkeit mittels einer privaten Prozedur/Funktion die Daten im Vorgangsimport noch zu manipulieren. Diese private Prozedur wird im Steuerparameter „Allgemeiner Steuerparameter für die Vorgangsimportschnittstelle“ ([928](../../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/allgemeiner_steuerparameter_fuer_die_vorgangsimportschnittst.md)) in der Option „GlobaleAenderungsProzedur“ hinterlegt. Nach dem Aufruf dieser Prozedur wird die eigentliche Vorgangserstellung gestartet. Die Prozedur wird für jeden zu erzeugendem Vorgang aufgerufen.

Der Kopf der Prozedur / Funktion muss genauso aussehen wie der Beispielkopf der privaten Prozedur. Der Rückgabewert wird nicht ausgewertet.

Beispielkopf der privaten Prozedur.

```text
create function NameDerFunktion (
in in_UebernahmeId integer,
in in_SatzId integer
)
returns integer
```
