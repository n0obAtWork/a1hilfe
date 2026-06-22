# Block Leerzeilen

<!-- source: https://amic.de/hilfe/blockleerzeilen.htm -->

Hier kann hinterlegt werden, ob bei der Ausgabe von Textblöcken (dazu gehören Adressen, Bemerkungen, Zahlungsbedingungen) pro Ausgabezeile je eine zusätzliche Leerzeile erzeugt werden soll.

Diese Einstellung kann für folgende Konstellation hilfreich sein:

Bei einem Windows-Druckformular wird der Zeilenabstand durch Anpassung der Zeilenskalierung in der Fonttabelle sehr klein eingestellt.

Alle gewöhnlichen Text- und Datenausgaben werden in einem Font erstellt, der doppelt so hoch ist wie das zugrunde liegende Zeilenraster.

Die Druckpositionen werden in jeder zweiten Zeile positioniert.

Nur ein geringer Teil der Angaben (wie z.B. Bankverbindung, juristische Firmenbezeichnung etc.) werden in kleiner Schriftart erstellt.

Durch diesen Trick können also unterschiedliche Zeilenabstände simuliert werden.

Ein Problem tritt jedoch bei blockorientierten Ausgaben (z.B. Adresse) auf, da diese auf fortlaufenden Zeilen gedruckt werden. Mit Hilfe der im Formulareinrichter eingetragenen Block-Leerzeilen kann nun hier der richtige Zeilenabstand wiederhergestellt werden.

Druckaufbereitung von Adressen

Um den Druck von Adressen anzupassen, wurden für die Vorgangsbearbeitung zwei neue Steuerparameter erstellt:

Parameter 586: „Leerzeilen bei Adressen entfernen“

Bei der Einstellung „Ja“ werden alle Leerzeilen aus Adressen entfernt (z.B. leere Zusätze oder Namensteile)

Parameter 587: „Adressen von unten aufbauen“

Bei der Einstellung „Ja“ wird die Adresse ausgehend von der untersten Zeile des Ausgabeblockes aufgefüllt. Die Adresse ist also immer an den unteren Rand ausgerichtet. Ist der Ausgabeblock auf dem Formular kleiner als die Anzahl der Adresszeilen, so werden bei dieser Einstellung die obersten Zeilen unterdrückt.
