# Bereichsauswahl über JPP Vorbelegen

<!-- source: https://amic.de/hilfe/_JPPANWCOND.htm -->

Wird eine Anwendung programmgesteuert (z.B. über Makro) aufgerufen, so kann man die Bereichsauswahl vorbelegen. Dazu dient das JPP-Objekt „JAnwCond“. Fett geschriebene Parameter sind Pflichtangaben.

| Funktion | Parameter | |
| --- | --- | --- |
| Init | **Profil** | Diese Funktion muss zu Beginn aufgerufen werden. Die drei Pflicht-Parameter identifizieren die Bereichsauswahl. Wird der Parameter WithLastCond mit 1 übergeben, dann wird das angegeben Profil als Vorbelegung geladen, ansonsten wird jedes Mal die Standardeinstellung als Basis verwendet. |
| **CondId** |
| **Besitzer** |
| WithLastProf | Sollen die letzten Werte dieses Profils als Basis verwendet werden? Standardeinstellung ist **0** für **Nein** |
| CondAktiv | Hier kann eingestellt werden, ob standardmäßig alle aktivierbaren Häkchen aus sind (Wert = 0) oder gesetzt sind (Wert = 1). Wird dieser Parameter nicht angegeben, werden die Häkchen so gesetzt, wie es in der Anwendung vorgegeben ist. Dies ist die Standardeinstellung.<br>Die Zeilen in der Bereichsauswahl, die mit den Funktionen SetVon und SetBis angegeben werden, sind immer aktiv.<br> |
| SetVon | **Idx** | Der Index, wie er in der Einrichtung der Bereichsauswahl angegeben wurde. |
| **Von** | Der Wert, der in der Bereichsauswahl verwendet werden soll.<br> |
| SetBis | **Idx** | Der Index, wie er in der Einrichtung der Bereichsauswahl angegeben wurde. |
| **Bis** | Der Wert, der in der Bereichsauswahl verwendet werden soll.<br> |
| Finit | | Der Aufruf erfolgt als letztes, bevor die Anwendung aufgerufen wird. Wird das JPP-Object vorher abgeräumt (JPP_DEL), dann wird diese Funktion automatisch aufgerufen, wenn dies noch nicht geschehen ist.<br> |

Beispiel:
