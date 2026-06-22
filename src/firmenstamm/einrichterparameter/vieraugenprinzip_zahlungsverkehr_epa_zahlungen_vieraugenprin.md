# Vieraugenprinzip Zahlungsverkehr (EPA ZAHLUNGEN_VIERAUGENPRINZIP)

<!-- source: https://amic.de/hilfe/_EPA_ZAHLUNGEN_VIERAUGENPRINZIP.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| VBS Script welches die Zahlungen überträgt | | Hier kann ein VBS Skript angeben werden, welches die DTAUS Datei überträgt.  
Es werden folgende Parameter an das Skript übergeben:  
/FILE= (Ausgabedatei)  
/Id= (DTA-Laufnummer)  
 |
| VBA Script welches die Zahlungen überträgt | | Hier kann ein VBA Skript angeben werden, welches die DTAUS Datei überträgt.  
Es werden folgende Parameter an das Skript übergeben:  
/FILE= (Ausgabedatei)  
/Id= (DTA-Laufnummer)  
/DTAProzedur=(Unter Optionen DTA_PROZEDUR angegebene Prozedur)  
 |
| Soll die Datei im Explorer angezeigt werden | Nein | Hier kann entschieden werden, ob die Datei im Explorer angezeigt werden soll.  
 |
| Prozedur zum Beantragen des Rücksetzens der Zahlungsnummer | ZahlungRueckBeantragen | Hier kann eine private Datenbankprozedur hinterlegt werden, die eine Mail an die Bediener der einzurichtenden Bedienerklasse versendet. Dieser Prozedur werden drei Parameter übergeben:  
• DTA-Laufnummer (Zahllaufid)  
• Die Bedienerklasse, die das Kennzeichen zurücksetzen darf. Siehe nächsten EPA  
• SMTP Server. Siehe übernächsten EPA  
   
 |
| Bedienerklasse, die das Kennzeichen zurücksetzten darf | | Hier wird die Bedienerklasse eingetragen, die das Übertragungskennzeichen löschen darf. In der Standardprozedur bekommen alle Bediener dieser Bedienerklasse die Mail zugesendet.  
 |
| SMTP Server | | Mail-Server  
 |
