# Ware abstimmen

<!-- source: https://amic.de/hilfe/_wareabstimmen.htm -->

Hauptmenü > Systempflege > Ware abstimmen

oder Direktsprung **[WABST]**

Zum Einlesen der Werte wird die Funktion ***Periodenwerte einlesen*** gewählt. Je nach Datenvolumen und Rechnerleistung kann dieser Vorgang einige Zeit in Anspruch nehmen. Danach werden zeilenweise die Ergebnisse nach gewählter Einstellung dargestellt.

Sollten Differenzen auftreten, stehen unter ***Konsistenz prüfen*** Analysefunktionen zur Verfügung, mit denen die Ursache gefunden werden kann.

<p class="just-emphasize">Funktionsknöpfe</p>

| Beschriftung | Funktion |
| --- | --- |
| ***Belegtyp wechseln*** | Wechsel der Datenbereiche:  
Die Spaltenwerte werden je Einstellung mit den unterschiedlichen Datenbereichen gefüllt. Es ist also eine Vergleichsmöglichkeit je erwarteter Fragestellung möglich. |
| ***+*** / ***\-*** | Blättern Jahre und Perioden:  
Es kann hiermit zwischen den jeweiligen Zeiträumen gewechselt werden. |
| ***Einzelsumme*** / ***kumulierte Summe*** | Einzelsumme stellt den Wert der oben angezeigten Periode, kumulierte Summe den Wert bis einschließlich o.a. Periode dar. |
| ***Periode*** / ***Vorperiode*** / ***Vorjahr*** | Zu Vergleichszwecken kann damit in die rechte Spalte der Inhalt eines anderen Zeitraums eingestellt werden. |
| ***Wertvergleich*** / ***ProzentAnteil*** / ***Differenz*** | Wertvergleich stellt die nackten Werte, ProzentAnteil die relative, Differenz die absolute Abweichung der rechten zur linken Spalte dar. |
| Erstelle WABST.TXT | Wertvergleich stellt die nackten Werte, ProzentAnteil die relative, Differenz die absolute Abweichung der rechten zur linken Spalte dar. |

Die Datenaufbereitung (Einlesen) mit WABST nimmt zurzeit noch relativ viel Zeit in Anspruch, da in der jetzigen Version stets der gesamte Datenbestand (WARE / FIBU) eingelesen wird. In einer zukünftigen Version sollen dann abgeschlossene Perioden gespeichert werden, so dass dann lediglich die aktuell offenen Perioden neu eingelesen werden!

**Was tut WABST?** Es wertet folgende Datenbestände aus:

• **WARENBELEGE**: Summensätze der Warenvorgänge

• **WARENPOSITION**: aus einzelnen Warenbewegungen errechnete Summen

• **FIBUBELEGE**: Summen aus allen Belegen in der Fibu, die aus der Ware stammen

• **ARTIKELSUMMEN**: Die Artikelsummensätze aus der Periodenstatistik = fakturierte Summe

sinnvoll ist der Vergleich:

**WARENBELEGE WARENPOSITION**

 **WARENBELEGE FIBUBELEGE**

 **WARENBELEGE ARTIKELSUMMEN**

im **Verkauf** (Rechnung, Rechnungs-Storno, Gutschrift, Gutschrift-Storno)

im **Einkauf** (E-Rechnung, E-Rechnungs-Storno, E-Gutschrift, E-Gutschrift-Storno)

pro Periode lfd. Jahr / Vorjahr

in Einzelsummen / kumulierte Summen

als DM-Wert, %-Anteil oder Differenz

Für die interne Abstimmung ist der Vergleich WARENBELEG → FIBUBELEGE von besonderer Wichtigkeit, denn eine Übereinstimmung dieser Zahlen ist aus der Sicht der ordnungsgemäßen Buchführung notwendig.

Die Abstimmung WARENBELEGE → WARENPOSITION sowie  
WARENBELEGE → ARTIKELSUMMEN gibt Auskunft über die interne Stimmigkeit (Konsistenz) der Warenbuchführung und ist daher Grundlage der Vertrauenswürdigkeit diverser Statistiken.

**Hinweis:**

WABST markiert an einigen Stellen die Ausgabe Felder mit ‚XXXXXXXXXX‘. Diese Daten stehen aus technischen Gründen nicht zur Verfügung oder sind in den unterschiedlichen Bereichen nach anderen Kategorien gruppiert, so dass sie nicht untereinander vergleichbar sind. In diesen Fällen sind nur die SUMMEN-Felder aussagekräftig bzw. vergleichbar!
