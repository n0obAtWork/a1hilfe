# Anlegen einer Partie über die Vorgangserfassung

<!-- source: https://amic.de/hilfe/_anlegeneinerpartiebe.htm -->

Am Beispiel „erfassen Lieferscheine“ wird im Folgenden beschrieben, wie ein Partiestamm in der Vorgangserfassung angelegt wird. Diese Funktion steht im Einkauf, Verkauf sowie in Umbuchung zur Verfügung. Steuerungsparameter **[SPA]** erlauben diese Neuanlage in der Vorgangserfassung (siehe Steuerungsparameter [SPA] Partieverwaltung)

Aufruf z.B. mit:

Hauptauswahlmenü > Wareneinkauf > Eingangslieferschein > Eingangslieferschein erfassen

oder Direktsprung **[ELE]**

Wenn mit ***Positionsteil*** **F5** und dann ***Artikel*** **F4** die Erfassung einer Artikelposition gewechselt wird, steht (bei entsprechender SPA-Einstellung (20,21)) die Funktion ***Partieauswahl*** **CF7** zur Verfügung.

Über diese Funktion können bestehende Partien dieser Position zugeordnet (F3) und neue Partien angelegt (F8) werden. Dieses Partieauswahlgitter zeigt sich automatisch auf der Maske, wenn für den in der Position erfassten Artikel bereits Partien existieren. Somit wird der Benutzer automatisch darauf hingewiesen, dieser Position Partien zuzuordnen (Einkauf sowie auch Verkauf).

Über die Funktion ***neue Partie F8*** wird ein neuer Partiestamm angelegt. Es öffnet sich die Partiestammdatenmaske. Für die Erläuterung der Felder siehe [Anlegen einer Partie über die Partiestammdatenverwaltung.](./anlegen_einer_partie_ueber_partiestammdatenverwaltung.md) Nachdem die notwendigen Eingaben erfolgt sind, kann diese Partie mit ***Speichern*** **F9** abgespeichert werden.

Der Artikel mit dem erfassten Lieferscheingewicht wird dieser Partie automatisch zugeordnet.

Diese Vorgehensweise der Partieanlage steht für Lieferscheine und Rechnungen im Einkauf sowie im Verkauf zur Verfügung.
