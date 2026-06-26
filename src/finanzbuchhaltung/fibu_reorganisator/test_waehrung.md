# Test Währung

<!-- source: https://amic.de/hilfe/testwhrung.htm -->

Hauptmenü > Abschlussarbeiten > Reorganisation > Fibureorganisation > Funktion ***Test Währung***

Direktsprung **[FIREO]**

Im ***Test Währung*** werden die Belege geprüft, ob die Währungsinformationen in ihnen korrekt sind.

**Buchwährung**  
Es muss in jedem Fall ein Eintrag im Währungsstamm für die Buchwährung vorhanden sein. Wenn es sich bei der Buchwährung um Euro handelt wird noch zusätzlich geprüft, ob als Nachkommastelle für den Betrag eine 2 und für die Rundung die kleinste zugelassene Einheit 0,01 eingetragen ist. Ist dies nicht der Fall, wird eine entsprechende Fehlermeldung ausgegeben.

**Nachkommastellen**  
Man kann im Währungsstamm die Anzahl der Nachkommastellen verändern. Tut man dies unbedacht und ändert die Währung so, dass sich die Nachkommastellen verringern, so kann es zu Rundungsproblemen kommen. Hier werden alle Belege angezeigt, die mehr Nachkommastellen haben, als hinterlegt.

**Währungsbetrag auf 0**  
Prüft, ob für einen Beleg in Fremdwährung auch ein Betrag in Fremdwährung eingetragen ist. 

**Währungskurse**  
Ist der Währungskurs, der zum Zeitpunkt der Erfassung aktiv war, für diesen Beleg hinterlegt?

**Währungsformel**  
Ist die Währungsformel in diesen Beleg hinterlegt?

**Währungsbeträge**  
Für Belege in Fremdwährung wird auch eine Kontrollsumme geführt. Diese Belege lassen sich nicht reorganisieren und müssen gegebenenfalls neu erfasst werden.

**Auszifferung**

Auch für Fremdwährungsbelege gilt, dass währungsseitig ein Auszifferung auf Null aufgehen muss (siehe Kursdifferenzen). Sollte - durch was für Umstände auch immer - eine Auszifferung nicht aufgehen, müssen Sie manuell eingreifen und in der OP-Verwaltung diese Auszifferung mit **F7** zurücksetzen und danach wieder ausziffern. Wenn sich der Fehler dadurch nicht beseitigen lässt, wenden Sie sich bitte mit einer genauen Fehlerbeschreibung an AMIC.
