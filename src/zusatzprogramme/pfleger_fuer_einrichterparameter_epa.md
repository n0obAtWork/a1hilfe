# Pfleger für Einrichterparameter (EPA)

<!-- source: https://amic.de/hilfe/pflegerfreinrichterparameterep.htm -->

Hauptmenü > Administration > Steuerung > EPAs zeigen

oder Direktsprung **[EPAZ]**

Aufgabe dieses Pflegers ist es, an zentraler Stelle die bedienerklassenabhängigen EPA-Einstellungen komfortabel und übersichtlich zu administrieren. Bisher war es nur möglich, die EPAs für Bediener derselben Bedienerklasse zu setzen, mit der man sich angemeldet hat. Um die EPAs für jede Bedienerklasse explizit abweichend von der AMIC-Vorbelegung zu setzen, musste man sich als Bediener der Bedienerklasse anmelden, für die man den EPA umsetzen wollte. Außerdem war dieses Umsetzen auch nur direkt auf der Maske möglich.

Wenn man in die Anwendung geht, sind in der dann erscheinenden Auswahlliste alle im System vorhandenen Einrichtungsparameter aufgelistet, diese sind nach Maskenname und Anlagedatum geordnet, d.h. die zuletzt durch AMIC angelegten Einrichterparameter erscheinen in der Liste oben. Als Profilbedingung steht der Maskenname zur Verfügung, ebenso das Anlagedatum des EPAs sowie sein Name bzw. seine Kurzbezeichnung. Für die letzte Bedingung steht ein Suchalgorithmus zur Verfügung, der z.B. bei Eingabe von „Löschen“ alle Einrichtungsparameter anzeigt, in deren für den Anwender sichtbaren Beschreibung die Zeichenkette „Löschen“ vorkommt.

Bem.: den Maskennamen ermittelt man durch Auslösen der Taste SH+STRG+F5 auf der gewünschten Maske 

Als Profilbedingung ist es dann möglich, sich genau die zu einer Maske gehörigen Einrichterparameter durch Eingabe des Maskennamens anzeigen zu lassen, oder durch Abgrenzung des AnlageDatums sich nur die neuesten Einrichterparameter anzeigen zu lassen.

In der Auswahlliste gibt es folgende Felder:

\- Maskenname, hier wird der Name der Maske angezeigt, für die dieser EPA definiert ist.

\- EPAName, hier wird der Name des EPAs angezeigt, so wie sie für den Kunden angezeigt wird.

\- AMIC_Vorbelegung, hier wird die von AMIC festgelegte Default-Einstellung angezeigt.

\- Anlagedatum, hier wird das Datum angezeigt, an dem dieser EPA angelegt wurde ( das Default-Datum für ältere Einträge lautet auf den 01.01.2001)

\- EPA_Kurz, hier ist der Variablenname eingetragen, so wie sie intern benutzt wird

\- Format, hier ist der zugehörige Name des Formatstrings hinterlegt, wenn diesem EPA ein solcher zugeordnet ist.

Um jetzt EPA-Werte zentral für gewisse Bedienerklassen zu setzen, markiert man wie gewohnt die zu ändernden Einrichtungsparameter in der Auswahlliste und löst F5 aus. (Blättermechanismus ist auch möglich)

Oben wird nochmals der Maskenname, der EPA-Name sowie die AMIC-Vorbelegung angezeigt.

In dem Grid auf der linken Seite werden zudem die augenblicklich für die entsprechende Bedienerklasse geltenden EPA-Werte angezeigt.

Um jetzt diese EPA-Vorbelegung zu ändern, stehen zwei Möglichkeiten zur Verfügung:

\- man trägt im weißen Feld des Grids den neuen Wert des EPAs für die zugehörige Bedienerklasse ein. Wenn dort Werte geändert worden sind, kommt beim Speichern / Blättern die Speicherns-Abfrage, die bei Bestätigung das endgültige Wegschreiben in die zugehörigen Tabellen durchführt.

\- Auf der rechten Seite ist es möglich für gewisse Bereiche von Bedienerklassen, einen Wert zu setzen. Hierzu trägt man in den VON / BIS – Bereich die Bedienerklassen ein, für die der EPA auf den gemeinsamen „neuen Wert“ geändert werden soll. Dann startet man den Knopf F5 und es wird der neue Wert in das Array geschrieben für die Bedienerklassen, die in dem VON / BIS – Bereich liegen. Die Vorbelegung der VON-Bedienerklasse geschieht mit der kleinsten Bedienerklasse, die des BIS – Wertes mit der größten im System vorhandenen Bedienerklasse. Der „neue Wert“ ist mit der AMIC-Vorbelegung vorbelegt. Die endgültige Übernahme in die zugehörigen Tabellen geschieht dann nach Bestätigen der Speicherns-Abfrage.

Bem.: Um die AMIC-Vorbelegung für alle Bedienerklassen wiederherzustellen, muss man also nur in die Maske gehen, F5 auslösen und dann abspeichern.

Hinweis bzgl. des Abspeicherns:

Es werden nur Werte im Bedienerprofil für die entsprechende Bedienerklasse abgespeichert, wenn sie sich von der AMIC-Vorbelegung unterscheiden. Wenn sie identisch mit der AMIC-Vorbelegung sind, ex. diese Einträge im Bedienerprofil nicht und es wird automatisch die AMIC-Vorbelegung gezogen.
