# Allgemeine Erläuterung

<!-- source: https://amic.de/hilfe/allgemeineerluterung.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Schema-/Kundenänderung

Direktsprung **[RWB]**

Diese Funktion bietet die Möglichkeit den Artikel, das Abrechnungsschema oder den Kunden eines Rohwarenbeleges zu ändern.  
Diese Änderungen kann man mit der normalen Korrektur nicht vornehmen.

Datensätze die korrigiert werden sollen, müssen zunächst in der Auswahlliste markiert werden. Bei Anwahl der Funktion ‚Schema-/Kundenänderung’ öffnet sich ein Fenster, welches die markierten Datensätze in einem Grid anzeigt, wenn diese mit der Funktion bearbeitet werden können.  
In folgenden Fällen können die Rohwarenbelege nicht mit dieser Funktion bearbeitet werden und werden nicht in die Maske übernommen:

Der Rohwarenbeleg ist kein Lieferschein (Vorgangsklasse 600 / 1600)

Der Lieferschein ist gesperrt.

Es wurde keine passende Vorgangsunterklasse zur Vorgangsklasse des Beleges gefunden. Das kann z.B. der Fall sein,  
wenn im [Einrichterparameter](./rohware_einrichterparameter.md) eine Vorgangsunterklasse eingetragen ist, die für die Vorgangsklasse nicht existiert  
oder/und  
zur Vorgangsklasse keine Unterklasse mit der Einstellung RohwareVorerfassung ungleich ‚ohne’ vorhanden ist.

Im Grid auf der Maske kann man die gewünschten Änderungen vornehmen.  
Die Korrektur für alle Belege auf der Maske startet man mit der Funktion [Start Korrektur F9](./start_korrektur_f9.md).

Die zu korrigierenden Rohwarenbelege werden storniert und über die Erzeugung eines neuen Lieferscheines, der in einen Rohwarenbeleg gewandelt wird, neu erzeugt.  
Tritt bei der Neuerzeugung ein Fehler auf, dann ist das Original schon storniert und die Daten damit verloren.  
Deshalb ist es wichtig sich die Rohwarenbelege vorher ins Archiv zu drucken.

Die Steuergruppe eines Rohwarenbeleges bleibt erhalten, wenn der Rohwarenparameter [rwpa] ‚*Steuergruppenvorbelegung*‘ (aus der Parametergruppe Erfassung Seite 6) auf ‚*aus Kundenstamm*’ steht.  
Bei der Einstellung *‚fester Wert*’ wird die Steuergruppe aus dem Rohwarenparameter ‚*Steuergruppenfestwert*‘ verwendet.
