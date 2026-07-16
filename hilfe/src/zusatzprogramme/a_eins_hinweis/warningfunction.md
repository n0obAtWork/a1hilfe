# WARNINGFUNCTION

<!-- source: https://amic.de/hilfe/_.htm -->

Mit „Warningfunction“ ist ein individuell designbares System entstanden, welches es ermöglicht auf der [neuen Auswahlliste](../auswahlliste_2_0/datentabelle.md#Warnung) ein Symbol im Hintergrund einzublenden, wenn gewisse Bedingungen erfüllt sind. Hier soll eine Beispieleinrichtung dargestellt werden.

### Einstufige-Version

Man kann eine Prüffunktion direkt an eine Auswahlliste hängen. Dies ermöglicht das direkte Abprüfen eines Kriteriums.

Vorteile: 

- einfache Anbindung
- schnelle Konsistenzprüfung (bei jedem Refresh der Auswahlliste)

Workflow:

- Warnsymbol erscheint im Hintergrund.
- Fehler erkennen und beheben.
- Danach verschwindet das Warnsymbol mit dem nächsten Aktualisieren der Auswahlliste.

Beispiel:

In der Anwendung „Fehlerprotokoll“ Variante „Benutzerhinweis“ ist die Funktion „AuswahllisteWarnung“ hinterlegt. Diese prüft ab, ob ein Fehlerprotokolleintrag existiert. Es wird empfohlen diese als Vorlage zu nutzen.

### Zweistufige-Version

Im Gegensatz zu der einstufigen Version hat man hier die Möglichkeit ein Verhalten bzw. einen Zustand zu fest definierten Zeitpunkten zu überprüfen. Schlägt die Überprüfung fehl, so wird der Benutzer durch den Hinweis bei der entsprechenden Auswahlliste aufmerksam gemacht. Dabei wird außerdem wird eine Meldung im Fehlerprotokoll erzeugt, welche genauere Informationen über die Art des Hinweises enthält.

Vorteile: 

- zeitgesteuerte Abfrage
- Formulierung des Fehlers und ein Hinweis wie dieser behoben werden kann.
- Verlauf wird im Fehlerprotokoll dokumentiert
- Hinweis kann abgestellt werden, ohne Daten zu ändern

Workflow:

- Warnsymbol erscheint im Hintergrund.
- Direktsprung [**FEHLH**].
- Abarbeitung der angezeigten Meldungen.
- Meldung auf „erledigt“ setzen.
- Mit „ESC“ zurück zur ursprünglichen Auswahlliste.
- Danach verschwindet das Warnsymbol mit dem nächsten aktualisieren der Auswahlliste.

Einrichtung:

- Eintragen der Funktion „AuswahllisteWarnungEvent“ bzw. eine Ableitung in einem Event **[EVT]**.
  - Hierin werden die Zuständigkeit und das Aussehen der Fehlermeldung definiert.
- Eintragen der Funktion „AuswahllisteWarnung“ bzw. eine Ableitung in die Auswahlliste, in der das Symbol für den zuständigen Mitarbeiter erscheinen soll.
  - Der FehlProtBereich sollte als Identifikator genutzt werden, um eine Verbindung zwischen Fehlerprotokolleintrag und Auswahlliste zu schaffen.

Beispiel:

Jeden Morgen sollen die Bestände überprüft werden. Wenn eine definierte Menge unterschritten wird, kann so der Mitarbeiter darauf aufmerksam gemacht werden und kann nachbestellen.
