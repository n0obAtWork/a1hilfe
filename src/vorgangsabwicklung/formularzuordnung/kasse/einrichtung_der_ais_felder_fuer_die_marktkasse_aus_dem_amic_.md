# Einrichtung der AIS-Felder für die Marktkasse aus dem AMIC Muster

<!-- source: https://amic.de/hilfe/einrichtungderaisfelderfrdiema.htm -->

Um aus dem AMIC Muster schnell die AIS Felder der Marktkasse einzurichten, muss man nur folgende Schritt-für-Schritt Anleitung befolgen:

1. In **[FRZ]** die gewünschte Vorgangsklasse und Unterklasse für die die Kasse eingerichtet werden soll öffnen / anlegen.  
In diesem Beispiel nehmen wir die 700 - Vorgang mit Unterklasse 9900 – Barverkauf.

2. Auf dem Tabreiter „Kasse“ sollte das Feld „AIS Gruppe“ leer sein. Wenn das Feld nämlich leer ist, wird der Bereich „Ersteinrichtung“ mit dem Feld „AIS-Muster“ angezeigt.

3. In dem Feld „AIS-Muster“ wählt man nun das gewünschte AMIC Muster aus. In diesem Beispiel „AMIC_Marktks_Maxi“

4. Nun gibt es in der Optionbox die Funktion „**Muster Übernehmen**“. Beim Ausführen dieser Funktion öffnet sich eine Maske, in der eine Vorbelegung für die AIS Gruppen gemacht werden.

a. Hier ist es **SEHR WICHTIG**, dass diese Vorbelegung entweder beibehalten oder ganz konsequent angepasst wird. Das erste Feld gibt hierbei die Namensgebung vor. Im Beispiel wurde „Marktks_Maxi_3“ vorgeschlagen, weil es schon 3 andere Einrichtungen gab.

b. Will man diese Vorbelegung allerdings abändern, z.B. stattdessen „Kasse_1“, so müssen alle anderen Gruppen diesem Schema folgen.

c. Aus „Marktks_Maxi_Korr_Menge_3“ wird also „Kasse_Korr_Menge_1“  
Dieses Schema muss so konsequent beibehalten werden, sonst wird die Einrichtung nicht funktionieren. Man kann also nur den Anfang (das Marktks_Maxi) sowie das Ende (die _3) abändern. Der Rest muss bestehen bleiben!

5. Nun klickt man auf F9 – Start bei der Maske „Muster / Import / Export“ und bestätigt darauffolgende Meldung, ob denn die neuen Felder angelegt werden sollen, mit „JA“.

6. Zurück auf der FRZ-Maske wird nun das Feld „AIS Gruppe“ mit der neuen Gruppe befüllt. In dem Beispiel wäre das „Marktks_Maxi_3“ bzw. „Kasse_1“, je nachdem welcher Name bei Schritt 4 gewählt wurde.

7. Wenn man das Feld „AIS Gruppe“ nun verlässt, in dem man auf ein Feld klickt oder auf TAB drückt, wird in der Optionbox die Funktion „**Musterkopie einrichten**“ freigeschaltet.

8. Ein Klick auf die Funktion „**Musterkopie einrichten**“ und anschließend auf **F9 – Speichern** und die Marktkasse ist fertig eingerichtet!
