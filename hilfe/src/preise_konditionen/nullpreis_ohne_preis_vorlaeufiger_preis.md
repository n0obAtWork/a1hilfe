# Nullpreis / ohne Preis / vorläufiger Preis

<!-- source: https://amic.de/hilfe/_Nullpreis.htm -->

In der Artikelerfassung der Vorgangsbearbeitung hat man die Möglichkeit über Signalfelder eine Warenposition ohne Preis oder mit einem vorläufigen Preis zu kennzeichnen. Am Signalfeld „[Nullpreis OK](../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/signalfelder.md#NullpreisOK)“ kann man dann in der Maske der Warenposition erkennen, dass ein Preis 0 absichtlich eingegeben wurde und seine Berechtigung hat. Am Signalfeld „[Nicht Endpreis](../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/signalfelder.md#NichtEndpreis)“ erkennt man, dass der Preis noch nicht der endgültige ist und weiterer Pflege bedarf.  
***Wichtig zu wissen***: Diese beiden Signalfelder haben keine direkten Auswirkungen auf Umwandlungen. Sie dienen dem Anwender nur als Hinweis in der Artikelerfassungsmaske.

Diese Signalfelder können über die Funktionen „[Nullpreis Okay an/aus](../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/merkmale.md#Nullpreis_anaus)“ und „[Vorläufiger Preis an/aus](../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/merkmale.md#VorlaeufigerPreis_anaus)“ vom Anwender jederzeit an- und ausgeschaltet werden. Hat man beide Merkmale für eine Warenposition angewählt, dann wird nur das Signal „Nicht Endpreis“ angezeigt.

Das Label hinter dem Feld für den Gesamtpreis in der Maske der Artikelerfassung zeigt an, woher der eingegebene Preis ([Preisherkunft](../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/herkunft_des_preises.md)) kommt. Der angezeigte Text kommt aus dem Format PR_HERKUNFT. Steht z.B. im Preisfeld der Wert 0, dann erscheint im Label der Text „ohne Preis“. Bei manueller Eingabe eines Preises steht in diesem Label der Text „manuelle Eingabe“.

Der [Steuerparameter 253](../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/unbepreiste_lieferscheine_umwandelsperre_spa_253.md) (unbepreiste Lieferscheine = Umwandelsperre) bietet die Möglichkeit Lieferscheine in denen eine Warenposition ohne Preis vorhanden ist für die Umwandlung zu sperren.  
***Wichtig zu wissen***: Das Signalfeld „Nullpreis OK“ hat keine Auswirkung auf diese Sperre!

­
