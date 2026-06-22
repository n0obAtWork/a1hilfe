# Import per Controlstring

<!-- source: https://amic.de/hilfe/importpercontrolstring.htm -->

Beispielsweise durch das Absetzen eines Controlstrings als Anweisung für den mandantenserver kann folgendes JPL abgesetzt werden:

```text
^jpl
vimperzeugebelege <VorgangsKlasse> <VorgangsArt> <Automatik>
<UebernahmeId> <SatzId> <Test> <Status> <Drucken>
<CallBackToken>
```

| Parameter | Typ | Bemerkung |
| --- | --- | --- |
| VorgangsKlasse | Integer | Die Vorgangsklasse, die importiert werden soll |
| VorgangsArt | integer | 0 = Neu (Standard)  
1= Bearbeitung  
Dieser Wert hat bei useCS=1 keine Bedeutung |
| Automatik | integer | 0 – manueller Aufruf mit GUI  
1 – automatischer Aufruf ohne GUI (Empfohlen für Aufrufe aus dem Mandantenserver oder anderer Automatismen) |
| UebernahmeId | integer | Übernahmeid, die importiert werden soll |
| SatzId | integer | Satzid, die importiert werden soll |
| Test | Integer | Immer 0 – nur für interne Zwecke |
| Status | Integer | Der Status, in dem sich der zu importierende ImportStammsatz befindet |
| Drucken | Integer | NUR bei usecs=1 Vorgangsdruck auslösen, wenn Wert = 1 |
| CallBackToken | string | Wenn gegeben, wird nach Abschluss dieses Token als Datenbank-Message an alle Clients mit dem Namensteil „(callback)“ gesendet. |

Beispielaufruf:

```text
^jpl
vimperzeugebelege ^400 0 1 123456 1 0 2
```
