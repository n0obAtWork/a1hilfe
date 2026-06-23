# Parameter für den Testmandanten im Mandantprofil

<!-- source: https://amic.de/hilfe/_testmandantenparameter.htm -->

Im Testmandanten werden Verbindungen zu externen Systemen, Events etc. unterbunden um zu verhindern, dass das Testsystem unkontrolliert Kontakt zu Live-Systemen aufnimmt. Für private Funktionen lässt sich die Funktion „AMIC_MandantProfilParameter“ aufrufen.

Diese wird mit einem Parameternamen (Private Parameter haben den Präfix „P_“) und den Vorgabewert gefüllt.

Die Prozedur gibt folgende Möglichkeiten zurück:

| System | Wert |
| --- | --- |
| Livesystem ohne definierten Parameter | Default-Wert (Vorgabewert) |
| Livesystem mit definierten Parameter | Für das Livesystem definierter Parameter aus der Tabelle MandantProfil |
| Testsystem ohne definierten Parameter | NULL |
| Testsystem mit definierten Parameter | Für das Testsystem definierter Parameter aus der Tabelle MandantProfil |

<p class="just-emphasize">Einrichtungsbeispiel:</p>

Sie wollen in einer Prozedur eine Mailadresse verwenden, die „live@me.xy“ heißt. Im Testsystem soll jedoch „test@me.xy“ verwendet werden.

Tragen Sie in der Tabelle Mandantprofil die folgenden Werte ein:

```sql
INSERT INTO
MandantProfil (MandantProfilId, MandantParName, MandantParWert)
VALUES(1,'P_MAILADRESSE','test@me.xy')
```

**Diese Werte können Sie bequem mit dem** [**Mandantenprofil-Pfleger**](./pfleger_fuer_das_mandantenprofil.md) **eintragen!**

Verwenden Sie die Prozedur „AMIC_MandantProfilParameter“ wie folgt:

```text
Set
mailadresse = AMIC_MandantProfilParameter('P_MAILADRESSE',
'live@me.xy');
```

Mit dieser Einrichtung stellen Sie sicher, dass Sie im Testsystem andere Parameter verwenden, als im Livesystem und dennoch nicht auf Funktionalitäten verzichten müssen.
