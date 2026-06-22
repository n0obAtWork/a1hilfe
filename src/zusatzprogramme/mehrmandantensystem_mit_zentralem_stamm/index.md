# Mehrmandantensystem mit zentralem Stamm

<!-- source: https://amic.de/hilfe/mehrmandantensystemmitzentrale.htm -->

Das A.eins System erlaubt es mehrere Mandanten parallel arbeiten zu lassen.

Mit Hilfe der Datenbank Proxy Technologie ist es möglich, einen Mandanten als Basissystem zu kennzeichnen, in dem die Stammdaten Kundenstamm und Artikelstamm gepflegt werden, um diese dann "online", also direkt nach dem Neueintrag oder dem Änderungsdienst auf den angeschlossenen Mandanten abzubilden. Des Weiteren gibt es zwei Übertragungsarten für das Mehrmandantensystem. Die erste Übertragungsart schreibt, die zu übertragenden Dateien in die Untermandanten Direkt ein. Bei der zweiten Übertragungsart, die per EPA einrichtbar ist, werden die Daten auf dem Zentralen System erst in eine Zwischenrelation gespeichert. Ein Ereignis verteilt die Daten, dann auf die passenden Untermandanten. Von unserer Seite aus empfehlen wir die Möglichkeit des direkten Übertragens zu benutzen. Die Einrichtung finden Sie [hier](./varianten/server.md). 

Zwei Begrifflichkeiten sind hierbei festzuhalten:

| Begrifflichkeiten | Bedeutung |
| --- | --- |
| Zentralmandant | einer der Datenbanken muss als Zentralmandant festgelegt werden, von diesem Mandanten aus werden, die Untermandanten mit den Änderungen versorgt |
| Untermandant | Der Untermandant ist Nutznießer der zentral gepflegten Informationen, und zwar auf Basis der Kundennummer und/oder auf Basis der Kombination Lagernummer/Artikelnummer. |

Untermandanten können selbstverständlich abweichende Kundenstämme oder Artikelstämme führen, nur die im Zentralmandanten als gemeinsam genutzte Kunden oder Artikel werden an die Untermandanten abgegeben.

Es ist sogar auf Relationsbasis möglich, in den Untermandanten "Überschreibungsschutz" festzulegen, und auch Default Vorbelegungen für den Neufall festzulegen.

<p class="siehe-auch">Siehe auch:</p>

- [Varianten](./varianten/index.md)
