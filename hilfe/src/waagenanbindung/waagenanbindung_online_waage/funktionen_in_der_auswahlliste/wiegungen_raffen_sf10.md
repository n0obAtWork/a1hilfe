# Wiegungen raffen SF10

<!-- source: https://amic.de/hilfe/_waage_wiegungraffen.htm -->

Über **SF10** hat man die Möglichkeit, mehrere Wiegungen zusammenzufassen:

Auf der Auswahlliste markiert man die Wiegungen, die man raffen möchte. Dann wählt man die Funktion ***Wiegungen raffen*** und es öffnet sich ein Fenster, in dem im oberen Bereich angezeigt wird, welche Wiegungen gerafft werden sollen. Im unteren Bereich wird der aus dem Raffen entstehende Datensatz angezeigt. (Ist dies nicht der Fall hilft ein Return im Feld Raffen, wo der Cursor steht oder ein Klick mit der Maus in das darunterliegende Feld).

In der Spalte Raffen hat man nun noch die Möglichkeit, Datensätze vom Raffen auszunehmen, indem man die entsprechende Zeile mit Nein versieht. Der Datensatz im unteren Bereich wird dann entsprechend neu berechnet.  
Mit **F10** startet man das Raffen.  
Nach einer kurzen Sicherheitsabfrage wird durch Bestätigen mit Ja der zusammengefasste Datensatz erzeugt und die anderen Datensätze werden auf gelöscht gesetzt. Man erhält einen Hinweis, dass die Raffung durchgeführt wurde.

    
Über den Einrichterparameter „Funktion für Raffen der Wiegungen“ der Maske „Wiegungen raffen“ kann man festlegen, wie die Datensätze zusammengefasst werden sollen. Es stehen zwei Funktionen zur Verfügung.

AMIC_WAAGE_RAFFEN : Die Menge wird summiert und die Feuchte gemittelt

AMIC_WAAGE_RAFFEN_GEWM: Die Menge wird summiert und die Feuchte wird gewichtet gemittelt

Hier sind auch private Anpassungen über neue Funktionen möglich, die dann im Einrichterparameter angegeben werden.
