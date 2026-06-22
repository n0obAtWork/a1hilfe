# Speicherung einer Wiegung

<!-- source: https://amic.de/hilfe/_waage_speichern_wiegung.htm -->

Normalerweise werden Datensätze in der Waage beim Verlassen der Waagemaske gespeichert.  
Es gibt aber ein paar Spezialfälle:

Fall: Öffnen der Waagenmaske im Neufall und direktes Anwählen der 1ten Wiegung

Waagedatensätze ohne relevante Eingaben (z.B. Kunde, Artikel) wurden bisher nur gespeichert, wenn beim Durchführen der ersten Wiegung eine Wiegenummer ungleich Null angegeben wurde. Der Speichermechanismus wurde erweitert, so dass der Datensatz nun auch gespeichert wird, wenn das Gewicht der ersten Wiegung ungleich Null und die Wiegenummer dabei 0 ist.

Fall: Öffnen der Waagenmaske im Neufall und sofortiges Verlassen mit **ESC**

In diesem Fall wurde der Datensatz bisher nicht gespeichert.

Es gibt aber Firmen, die Datensätze mit Status „Eröffnet“, aber ohne eine weitere Eingabe speichern möchten. Sie benötigen nur die Belegnummer, um schon Etiketten drucken zu können, bevor die LKWs an der Waage ankommen. Wenn der LKW dann kommt, erhält der Fahrer einen Aufkleber und der Datensatz mit der Belegnummer, die auf dem Aufkleber ist, wird aus den Datensätzen rausgesucht und entsprechend gefüllt.

Dafür wurde eine Abfrage beim Verlassen der Maske eingebaut, die erscheint, wenn keine Daten eingegeben wurden. Man hat dann die Möglichkeit, den Datensatz trotzdem zu speichern.
