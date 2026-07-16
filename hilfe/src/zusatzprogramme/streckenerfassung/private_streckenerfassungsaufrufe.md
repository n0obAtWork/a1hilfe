# Private Streckenerfassungsaufrufe

<!-- source: https://amic.de/hilfe/_vorgangsmappe_privateaufrufe.htm -->

Um die Streckenerfassung aufzurufen kann eine private Funktion eingerichtet werden. Dabei hat der Controlstring folgende Form:

```text
jpl streckenerfassung_auftragaufruf ":pEinkaufVerkauf" ":h.klammernr$" ":pProfil"
```

Diese jpl-Funktion hat 3 Parameter:

| Parameterbezeichnung | Beschreibung |
| --- | --- |
| EINKAUFVERKAUF | 1 => Einkauf<br>2 => Verkauf |
| MODUS | "KONTRAKTSTAMM" => Mit diesem Parameter wird die Streckenerfassung mit der ersten gefundenen Streckennummer des Kontrakts aufgerufen. Die Kontrakt ID wird dafür aus dem ersten Feld (ID1) der Auswahlliste geholt. Zu dem Kontrakt wird dann die Streckennummer ermittelt, wird keine Nummer gefunden wird die Streckenmaske nicht geöffnet.<br> <br>"AUFTRAGMITMAPPE" => Mit diesem Parameter kann die Streckenerfassung für einen Vorgang geöffnet werden. Befindet sich der Vorgang bereits in einer Strecke, wird diese aufgerufen. Ansonsten wird der Vorgang einer Strecke mit der Nummer des Vorgangs zugeordnet und aufgerufen.<br> <br>Leerstring, ID1, ID2, ID3, ID4 => Die Streckennummer wird aus dem angegebenen ID Feld der Auswahlliste ermittelt. Bei einem Leerstring wird die ID1 verwendet.<br> <br>Numerischer Wert => Die Streckenerfassung wird mit der übergebenen Nummer aufgerufen. |
| PROFIL | Hier wird das zu verwendende Streckenerfassungsprofil festgelegt.<br>Wird keins übergeben, wird versucht dieses über die Einrichteparameter STRECKENPROFIL_EINKAUF für Einkauf bzw. STRECKENPROFIL_VERKAUF für Verkauf zu bestimmen. |
