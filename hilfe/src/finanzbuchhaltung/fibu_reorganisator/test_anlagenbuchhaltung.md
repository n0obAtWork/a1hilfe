# Test Anlagenbuchhaltung

<!-- source: https://amic.de/hilfe/testanlagenbuchhaltung.htm -->

Hauptmenü > Abschlussarbeiten > Reorganisation > Fibureorganisation > Funktion ***Test Anlagenbuchhaltung***

Direktsprung **[FIREO]**

Im ***Test Analgenbuchhaltung*** werden die Einträge auf Konsistenz geprüft. Es werden folgende Tests durchgeführt:

**Fehlerhafte Umbuchungsverweise**

Wird ein Anlagegut umgebucht, wird intern das Ursprüngliche Anlagegut vermerkt. Dieser Test listet alle Anlagegüter auf, bei denen entweder der Eintrag im ursprünglichen Anlagegut fehlt oder der Umbuchungsbeleg nicht existiert. Sollte dieses Problem auftauchen, muss entweder der Umbuchungsbeleg gelöscht werden oder der die Umbuchungszeile im Originalbeleg gelöscht werden.

**Fehlerhafter Verweis zur Fibu**

Wenn aus der Anlagenbuchhaltung heraus Belege erstellt werden oder Finanzbuchhaltungsbelege bestimmten Anlagegütern zugewiesen wurden, wird ein Verweis erstellt. Zeigt der Verweis auf einen nicht existierenden Beleg, so kann man Fibu und Anlagenbuchhaltung nicht mehr abstimmen. Alle Fehlerhaften Verweise werden hier aufgelistet.

**Restbuchwert**

Hier wird getestet, ob der Restbuchwert des Anlagengutes eventuell kleiner als 0 ist. Dies kann nur dann geschehen, wenn man ein Anlagegut mit Gewinn verkauft hat und die Aufwands/Ertragszeile noch fehlt. Es erscheint dann am Ende der Zeile der Hinweis: „Verkauft ohne Erlös/Aufwandseintrag“. Ist dies nicht der Fall, so erscheint der Hinweis „Fehler!“. Anlagegüter, deren Restbuchwert kleiner als 0 ist, werden in der Auswahlliste Anlagestamm (Direktsprung **[ANKAS]**) rot gekennzeichnet.
