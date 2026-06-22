# Vorgang erzeugen F6

<!-- source: https://amic.de/hilfe/_waage_vorgangerzeugen.htm -->

Normalware

Vorgänge können nur erzeugt werden, wenn es sich nicht um Rohwarewiegungen handelt.

Vorgänge können hier nicht im Stapel erzeugt werden, sondern nur für jeden Waagedatensatz einzeln. Für die Vorgangserzeugung im Stapel muss man die Funktion [Vorgänge erzeugen](../funktionen_in_der_auswahlliste/vorgaenge_erzeugen_cf9.md) in der Auswahlliste anwählen.

Es wurde jetzt die Möglichkeit geschaffen, eine alternative Vorgangserzeugung an der Waage aufzurufen, dazu wird im [Waagenprozess](../prozess_einrichten/registerkarte_vorgang.md) eine J-Datei oder ein Makro angegeben. Dieser Datei wird dann die OwaageId übergeben.

Bei der Vorgangserzeugung kann jetzt der Vorgang direkt mit ausgedruckt werden. Dazu muss im Waagenprofil auf der [Registerkarte Vorgang](../prozess_einrichten/registerkarte_vorgang.md) der Schalter „Vorgang nach der Erzeugung drucken“ auf Ja gestellt werden. Bei der Funktion Vorgang erzeugen editieren wird der Schalter nicht ausgewertet.

Rohware

Für abgeschlossene Wiegungen mit dem Wiegetyp Rohwareneingang, Rohwarenausgang oder Lohnwägung kann man in der Auswahlliste für mehrere Wiegungen und in der Maske für die aktuelle Wiegung Rohwarenbelege erzeugen.

Beim Start dieser Funktion erscheint eine Abfrage, ob man Rohwarenbelege erzeugen möchte.

Mit dem Einrichterparameter ‚Feuchte muss zum Erzeugen von Rohwarenbelegen angegeben werden’ kann man bewirken, dass keine Rohwarenbelege für Datensätze erzeugt werden, wenn die Feuchte nicht eingetragen wurde.

Der Status der Wiegung wird auf „mit Vorgang“ gesetzt.  
    
Ist der Wiegetyp Rohwareneingang, wird ein Rohwarenbeleg mit dem EK/VK Kennzeichen gleich Einkauf erzeugt, den man sich in den EK Rohwarenbelegen anschauen kann.  
Ist der Wiegetyp Rohwarenausgang, wird ein Rohwarenbeleg mit dem EK/VK Kennzeichen gleich Verkauf erzeugt, den man sich in den VK Rohwarenbelegen anschauen kann.  
Ist der Wiegetyp Lohnwägung, wird ein Rohwarenbeleg mit einem EK/VK Kennzeichen abhängig von der in der Vorlage eingegebenen Lohnklasse erzeugt, den man sich dann entsprechend in den EK oder VK Rohwarenbelegen anschauen kann.  
    
Wenn in einer abgeschlossenen Wiegung keine Sorte eingetragen wurde, wird beim Erzeugen des Rohwarebeleges die Defaultsorte für die Rohwarengruppe des Artikels genommen. Die Defaultsorte muss dafür bei den Rohwarengruppen eingetragen sein.

In den Anwendungen EK-Waage-RWLieferungen oder VK-Waage-RWLieferungen (zu finden im Hauptmenü unter Rohwarenabrechnung) ruft man dann die Funktion ***Lieferungen erzeugen*** für die erzeugten Rohwarenbelege auf, um Rohwarenlieferscheine zu erstellen.  
Mit dem Punkt [Rohwarenlieferscheine sofort erzeugen](../prozess_einrichten/registerkarte_rohware.md) in der Vorlage kann man einstellen, dass die Funktion ***Lieferungen erzeugen*** beim Aufruf der Funktion ***Rohwarenbeleg erzeugen*** sofort mit ausgeführt wird.
