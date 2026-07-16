# Ausgehende Telefonie über smx_call 1100

<!-- source: https://amic.de/hilfe/ausgehendetelefoniebersmxcall1.htm -->

Um ein ausgehendes Telefonat einzuleiten muss die Software mitgeteilt bekommen wo und wie sie die zu wählende Telefonnummer bekommen kann.

Dazu ist es in den meisten Fällen notwendig an den betreffenden Stellen eine private Funktion anzulegen die per Controlstring die A.eins-Programmfunktionalität ^smx_call 1100 mit geeigneten weiteren Parametern aufruft. Die A.eins-Methode verwendet Standard-Windows-Methoden für den Telefonanruf. Spezielle Systeme müssen über die unten aufgeführte Script-Methodik angesprochen werden.

| Parameter ^smx_call 1100 Modus Param1 Param2 | |
| --- | --- |
| Modus=0 („Auswahlliste“) | Wenn Param1 angeben ist, dann dieser als Name der Auswahllisten-Spalte interpretiert und der dortige Wert als **Telefonnummer** verwendet.<br>Wenn Param1 nicht angeben ist, dann wird die **Telefonnummer** vom System wie folgt ermittelt:<br>Es wird die KundId aus der Returnliste gesucht, sollte das nicht gehen wird die KundNummer versucht zu ermitteln und daraus die KundId. Letztlich dann mit der KundId aus dem Adressstamm das AdressTelefon.<br> <br>Als Vereinfachung kann die A.eins-Funktion<br>Kundenanruf_Aufruf verwendet werden, der Controlstring lautet ^jpl Kundenanruf_Aufruf.<br> |
| Modus=1 („Maskenmodus“) | In Param2 wird der Name des Maskenfeldes übertragen.<br>Param1=1:Das Maskenfeld enthält die **KundNummer**<br>Param1=2:Das Maskenfeld enthält die **Kundid**<br>(bei 1 und 2 wird wie oben das Adresstelefon ermittelt)<br>Param1=3: Das Maskenfeld enthält die **Telefonnummer**. |

Beispiele:

| ^smx_call 1100 1 1 KundNummerAnz | Das Maskenfeld „KundNummerAnz“ enthält die KundNummer |
| --- | --- |
| ^smx_call 1100 1 1 k.KontoNummer$ | Das Maskenfeld „k.KontoNummer$“ enthält die KundNummer |
| ^smx_call 1100 1 2 KundId$ | Das Maskenfeld „KundId$“ enthält die KundId |
| ^jpl mboxcall 1100 | Die Telefonnumer wird aus der Auswahlliste ermittelt ( wie oben unter Modus=0 ) beschrieben. |

Für noch individuellere Möglichkeiten steht ein Template für eine völlig scriptfähige Lösung bereit.

So ist z.B. das ausgelieferte VBA-Script „**AMIC_TAPI_CALL**“ ein Beispiel für einen möglichen Lösungsweg. Dort wird ein Ansatzpunkt demonstriert, um spezielle Hardware-Umgebungen berücksichtigen zu können und/oder Fremdsoftware aufzurufen.

Hierzu muss man dann allerdings AMIC_TAPI_CALL privatisieren.

Es gibt Umgebungen in denen die Privatierung und entsprechende Einbindung des Scripts unabdingbar ist. Beispiele sind:

1) Unmögliche bzw. nicht zufriedenstellende Linespezifikation mit der internen smx_call-Methode.

2) USER-bezogene oder Arbeitsplatz-Technische Besonderheiten.
