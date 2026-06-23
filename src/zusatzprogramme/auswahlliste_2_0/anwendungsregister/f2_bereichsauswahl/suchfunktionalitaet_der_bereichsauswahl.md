# Suchfunktionalität der Bereichsauswahl

<!-- source: https://amic.de/hilfe/_weiterefunktionender.htm -->

Bei der Suche innerhalb von Texten – wie z.B. Kundennamen oder Ort – kann häufig auch mit Platzhaltern gearbeitet werden. Die Syntax hält sich hier an die SQL-Norm.

<p class="just-emphasize">Die „%“ – Funktion</p>

Häufig wiederkehrende Fragen sind z.B. „Ich hätte gerne alle Kunden, die mit „M“ beginnen“. Die Antwort erhält man, indem in einer der Suchvarianten nach „Nachname“ folgendes eingegeben wird:

| Auswahl | Ergebnis |
| --- | --- |
| M% | „Ich hätte gerne alle Kunden, die mit „M“ beginnen“<br> |
| %,kiel | Alle Kunden aus „Kiel“<br> |
| m%,k% | Alle Kunden mit „M“ aus „K“<br> |
| me%,ka% | Alle „Me“ aus „Ka“<br> |
| %ma,fritz<br> | Alle, die irgendwo im Namen die Zeichenfolge „ma“ aufweisen und mit Vornamen „Fritz“ heißen |

<p class="just-emphasize">Die „_“ Funktion (Unterstrich „Shift –„)</p>

Diese Funktion dient als Platzhalter für **ein** Zeichen.

| Auswahl | Ergebnis |
| --- | --- |
| _e | Alle Kunden mit „e“ an zweiter Stelle<br> |
| m__er<br> | Alle „Meier“, unabhängig ob sie mit „ei, ey, ay, ai“ etc. an zweiter und dritter Position geschrieben werden<br><strong>Achtung:</strong> <em>Hier wird natürlich auch der Name „Mauer“ etc. ausgewiesen.<br><br></em> |
| m[_]m__er | Um einen echten Unterstrich zu suchen, muss man diesen in eckige Klammern setzen. Dieses Beispiel alle Einträge, die mit „m“ beginnen, dann einen Unterstrich „_“ haben und danach alle „Meier“, unabhängig ob sie mit „ei, ey, ay, ai“ etc. an zweiter und dritter Position geschrieben werden. Also z.B.: m_Mayer.<br> |

Soll der Unterstrich _ gesucht werden und nicht als Platzhalter dienen, so ist er in eckigen Klammern zu setzen: [_]

<p class="just-emphasize">Kombination aus „%“ und „_“</p>

| **Auswahl** | **Ergebnis** |
| --- | --- |
| **m__er,%baden**<br> | Alle mit „M“ an erster Stelle und „er“ an 4.-5. Aus „…...baden“ |
| **_b%er,%baden**<br> | Alle mit „b“ an zweiter Stelle und anschließend irgendwo „er“ aus „…..baden“ |
