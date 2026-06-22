# Suchfunktionalität der Bereichsauswahl

<!-- source: https://amic.de/hilfe/_weiterefunktionender.htm -->

Bei der Suche innerhalb von Texten – wie z.B. Kundennamen oder Ort – kann häufig auch mit Platzhaltern gearbeitet werden. Die Syntax hält sich hier an die SQL-Norm.

Die „%“ – Funktion

Häufig wiederkehrende Fragen sind z.B. „Ich hätte gerne alle Kunden, die mit „M“ beginnen“. Die Antwort erhält man, indem in einer der Suchvarianten nach „Nachname“ folgendes eingegeben wird:

| Auswahl | Ergebnis |
| --- | --- |
| M% | „Ich hätte gerne alle Kunden, die mit „M“ beginnen“  
 |
| %,kiel | Alle Kunden aus „Kiel“  
 |
| m%,k% | Alle Kunden mit „M“ aus „K“  
 |
| me%,ka% | Alle „Me“ aus „Ka“  
 |
| %ma,fritz  
 | Alle, die irgendwo im Namen die Zeichenfolge „ma“ aufweisen und mit Vornamen „Fritz“ heißen |

Die „_“ Funktion (Unterstrich „Shift –„)

Diese Funktion dient als Platzhalter für **ein** Zeichen.

| Auswahl | Ergebnis |
| --- | --- |
| _e | Alle Kunden mit „e“ an zweiter Stelle  
 |
| m__er  
 | Alle „Meier“, unabhängig ob sie mit „ei, ey, ay, ai“ etc. an zweiter und dritter Position geschrieben werden  
**Achtung:** *Hier wird natürlich auch der Name „Mauer“ etc. ausgewiesen.* |
| m[_]m__er | Um einen echten Unterstrich zu suchen, muss man diesen in eckige Klammern setzen. Dieses Beispiel alle Einträge, die mit „m“ beginnen, dann einen Unterstrich „_“ haben und danach alle „Meier“, unabhängig ob sie mit „ei, ey, ay, ai“ etc. an zweiter und dritter Position geschrieben werden. Also z.B.: m_Mayer.  
 |

Soll der Unterstrich _ gesucht werden und nicht als Platzhalter dienen, so ist er in eckigen Klammern zu setzen: [_]

Kombination aus „%“ und „_“

| **Auswahl** | **Ergebnis** |
| --- | --- |
| **m__er,%baden**  
 | Alle mit „M“ an erster Stelle und „er“ an 4.-5. Aus „…...baden“ |
| **_b%er,%baden**  
 | Alle mit „b“ an zweiter Stelle und anschließend irgendwo „er“ aus „…..baden“ |
