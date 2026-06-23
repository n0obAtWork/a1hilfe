# Quadriga Anlagenbuchhaltung

<!-- source: https://amic.de/hilfe/quadrigaanlagenbuchhaltung.htm -->

Der Import der Daten aus der Quadriga-Anlagenbuchhaltung ist nicht in den Standardimport integriert.

Es existiert ein Hilfsprogramm, das man als private Funktion einbinden kann. Dieses Programm hat zwei Parameter. Der erste Parameter ist der Nummernkreis, der zur Belegnummernvergabe herangezogen wird. Die Inventurnummer aus der Anlagebuchhaltung wird als Referenznummer übernommen. Der zweite Parameter ist das Verzeichnis, auf das die Fileselectionbox zeigt.

<strong>Tipp</strong><em>: Man kann sich eine private Funktion einrichten, die als Controlstring „^jpl quadriga nummer c::\\verzeichnis“ enthält. Dabei ist zu beachten, dass der Doppeltpunkt beim Verzeichnis zweimal angegeben werden muss und der Backslash (‚\\’) nur einmal. Damit entfällt die Einrichtung des Hilfsprogramms.</em>

<p class="just-emphasize">Stammdaten</p>

Die von Quadriga übergebenen Sachkonten müssen in A.eins eingerichtet sein

Die Kostenstellen müssen in A.eins eingerichtet sein. Wird von der Quadriga-Software keine Kostenstelle übergeben, so wird die im Sachkontenstamm hinterlegte Kostenstelle verwendet.

Im Mandantenstamm muss das Umbuchungskonto eingerichtet sein.

Die in der Quadriga-Software vergebene Inventarnummer darf nur numerisch sein. Diese Nummer kann auch alphanumerisch sein.

<p class="just-emphasize">Vorgehensweise</p>

Die Daten der Anlagenbuchhaltung werden über den Menüpunkt „LISTEN“ / „Buchungen“ / „Finanzbuchhaltung“ exportiert, indem man dort als Ausgabemedium „Datei“ wählt. Das Ausgabeformat muss dBase sein.

ACHTUNG:

Einmal ausgelagerte Werte werden von der Quadriga-Anlagenbuchhaltung nicht gekennzeichnet, so dass ein versehentliches doppeltes Übertragen der Daten möglich ist. Es muss also durch organisatorische Maßnahmen sichergestellt werden, dass eine doppelte Übertragung nicht vorkommt.

In A.eins wählt man nun das Hilfsprogramm an und wählt die Datei aus. Die dann importierten Daten werden zuerst in der Relation Quadriga zwischengespeichert von der sie anschließend in die Primanota geschrieben werden. Alle Belege bekommen den Herkunftstypen (fibuvp_herktyp) 32. Dort können die Daten kontrolliert und gegebenenfalls geändert oder gelöscht werden.

Fehler, die beim Erstellen der Belege auftreten, werden ins Fehlerprotokoll (FEHLP) geschrieben. Die Fehler werden gegebenenfalls direkt nach dem Durchlauf angezeigt und anschließend automatisch gelöscht.
