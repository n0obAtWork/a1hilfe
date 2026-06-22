# Kontrakt

<!-- source: https://amic.de/hilfe/_ modreport _kontrakt.htm -->

Auf diesem Tabs werden gezielt Änderungen eines Attributs von Datensätzen in Relationen gesucht, die einem bestimmten Kontraktstamm zuzuordnen sind. Die Angaben in den Feldern *Kontraktnummer* und *KtrId* sind optional, es muss aber mindestens zu einem dieser Felder eine Eingabe erfolgen. Alle genannten Eingabefelder verfügen über eine unterstützende Itembox-Anbindung.

Wird lediglich die Kontraktnummer angegeben, so ist die Basis für kundenstammbasierte Suchanfragen die Menge aller Kontraktstammeinträge mit dieser Kontraktnummer, also auch diejenigen mit eingetragenem Löschkennzeichen.

Wird ein Kontraktstamm per KtrId spezifiziert, so ist die Basis für kontraktstammbasierte Suchanfragen nur der angegebene Kontraktstammeintrag.

Die Angabe des auszuwertenden Attributnamens wird durch eine Itembox unterstützt, die auch über eine Auflistungsvariante nach den hier erlaubten Relationsnamen verfügt und nach Auswahl des Attributs auch den zugehörigen Relationsnamen in das entsprechende Maskenfeld schreibt.

Existieren Attribute mit dem angegeben Namen in mehreren Relationen, so kann diese, ebenfalls unterstützt durch eine entsprechende Itembox, angegeben werden.

Der zu untersuchende Zeitraum bezüglich des Logfile-Archivierungsdatums, wie auch die maximale Anzahl der Datensätze, die in die Auswahlliste zu übernehmen sind, können angegeben werden. Die Suche erfolgt grundsätzlich beginnend mit dem Bis-Datum hin zum Ab-Datum und bricht bei Erreichen der Maximalzahl der Ergebnissätze ab.

Mit der Funktion **Anfrage starten** wir die Anfrage generiert und zur Abarbeitung an eine entfernte Prozedur übergeben.

Die Maske wird nach Ende der Bearbeitung durch den Fremdserver automatisch verlassen und das Ergebnis in der Auswahlliste dargestellt. Werden keine Daten angezeigt, so konnten keine Logfile-Einträge entsprechend der gemachten Angaben gefunden werden.
