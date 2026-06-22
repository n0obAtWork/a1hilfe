# Daten aktualisieren (Refresh)

<!-- source: https://amic.de/hilfe/datenaktualisierenrefresh.htm -->

Wenn man sich in einer AIS-Anwendung befindet, von dort einen Pfleger aufruft, der die Daten verändert, und in die darrunterliegende Maske zurückkehrt, kann es wünschenswert sein, dass die Anzeige mit den geänderten Daten neu aufgebaut werden soll. Bei der Datenherkunft SQL kann man einen Schalter „Refresh“ setzen. Es werden dann bei allen Feldern, bei denen der Schalter auf **Ja** steht, beim wiederbetreten der Maske die Daten neu eingelesen und angezeigt.

Zusätzlich steht eine Funktion **dbx_io ("AISREFRESH")** zur Verfügung, die das Aktualisieren der Felder auslöst. Diese kann z.B. auf Pusch-Button als Controlstring eingetragen werden oder in einem Makro verwendet werden. Diese Funktion hat als zusätzlichen Optionalen zweiten Parameter den Feldname. Hat man viele Felder in AIS – darunter eventuell viele Datentabellen - mit dem Refresh-Flag versehen, kann es Sinnvoll sein, nur einzelne dieser Felder mit der Funktion AISREFRESH anzusprechen. Der Feldname muss dabei so lauten, wie er auf der Maske steht, also ggf. mit Handel, Punkt und $.

**HINWEIS:** *So einzeln angesprochene Felder müssen nicht den Schalter Refresh auf **Ja** stehen haben.*
