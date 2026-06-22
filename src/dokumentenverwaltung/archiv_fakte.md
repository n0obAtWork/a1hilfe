# Archiv-Fakte

<!-- source: https://amic.de/hilfe/_archivfakte.htm -->

Hauptmenü > Administration > Archiv > Administration > Archiv-Fakte

Direktsprung **[FAREF]**

Bestimmte Objekte in A.eins (z.B. Vorgänge, Kunden, Artikel, u.a.) haben feste Regeln hinterlegt, mit deren Hilfe sie automatisch Archiv-Referenznummern vorgeschlagen bekommen, wenn sie neu ins System kommen. Diese Objekte zusammen mit ihren Definitionen sind in A.eins die Archiv-Fakte.

| Felder |
| --- |
| Auslieferung | Ja/Nein<br>Gibt an, ob das vorliegende Archiv-Fakt von AMIC ausgeliefert wird.<br>AMIC-Auslieferungen sind solche Archiv-Fakte die mit **fam_ref_** beginnen. |
| Archiv-Fakte | Eindeutiger Schlüsselbegriff der Archiv-Fakte.<br>Es stehen maximal 30 Zeichen zur Verfügung. |
| Beschriftung | Repräsentation des Archiv-Faktes in der Benutzeroberfläche. |
| AMIC-Referenz | Datenbank-Funktion zur Ermittlung der Referenz eines Archiv-Faktes.<br>Die jeweilige Datenbank-Funktion die AMIC ausliefert ist als Beispiel zu sehen. Es kann durchaus sein, dass vor Ort individuelle Anpassungen durchgeführt werden müssen.<br>Allerdings – so zeigt die Praxis – ist das selten von Nöten. |
| Privat-Referenz | Die Möglichkeit die Funktionalität der AMIC-Referenz zu ersetzen.<br>Wenn man hier deine existierende private Datenbank-Funktion einträgt, dann wird zur Ermittlung der Referenz genommen, und nicht die AMIC-Referenz! |
| Relation | Die Relation, in der die Kerndaten sowie die Archiv-Referenz des Archiv-Faktes gespeichert werden. Das ist ein Fakt. |
| Referenz-Spalte | Der Name der Spalte der Relation, in der die Archiv-Referenz gespeichert wird. Das ist ein weiteres Fakt. |
| Aufruf-Parameter | Die Parameter für den Aufruf der jeweiligen Datenbank-Funktion. |
| **Ohne Referenz!** | Eine statistische Aussage darüber, wie viele Exemplare eines bestimmten Archiv-Faktes KEINE eingetragene Archiv-Referenz in der Referenz-Spalte der Relation hinterlegt haben.<br>Dabei handelt es sich vornehmlich um Alt-Bestände vor Aktivierung der Archivierung in A.eins.<br>Beispiel: Eine Angabe von 130000 im Archiv-Fakt „fam_ref_fibu“ bedeutet, dass es 130.000 Einträge in der Relation „fibuvorgstamm“ gibt, die keine eingetragene Archiv-Referenz in der Spalte „fibuv_paginiernr“ haben. |

| Suchkriterien |
| --- |
| Suchen | Like<br>Sucht in allen Feldern außer „Auslieferung“ und „Ohne Referenz!“ nach der Begrifflichkeit. |
| Referenz-Information | Ja/Nein<br>Such-Schalter, aktiviert bei Einstellung „Ja“ die Analyse, wie viele Archiv-Fakte ohne Archiv-Referenz ausgestattet sind und stellt die Information unter „**Ohne Referenz!**“ zur Verfügung. |

| Funktionen |
| --- |
| ***Neu*** **F8** | Anlage eines neuen Archiv-Faktes. Es können private Archiv-Fakte definiert werden!<br>Für Details siehe Archiv-Fakte-Pfleger. |
| ***Ändern*** **F5** | Ändern eines Archiv-Faktes.<br>Hiermit lässt sich die „Privat-Referenz“ einpflegen.<br>Für Details siehe Archiv-Fakte-Pfleger. |
| ***Ansehen*** **F6** | Ansehen eines Archiv-Faktes.<br>Für Details siehe Archiv-Fakte-Pfleger. |
| ***Löschen*** **F7** | Löscht ein Archiv-Fakt.<br>Auslieferungen lassen sich nur durch System-Entwickler löschen.<br>Für Details siehe Archiv-Fakte-Pfleger. |
| Nachreferenzieren … | Mit Hilfe dieser Funktion ist es möglich, selektierte Archiv-Fakte so zu bearbeiten, dass die Archiv-Referenz-Nummer nachreferenziert wird, falls keine angegeben ist.<br>Nach Abschluss der Operation ist die Anzahl der Archiv-Fakte ohne Archiv-Referenz 0. |
