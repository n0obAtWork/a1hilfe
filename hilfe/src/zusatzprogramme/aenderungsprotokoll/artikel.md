# Artikel

<!-- source: https://amic.de/hilfe/_ modreport _artikel.htm -->

Auf diesem Tabs werden gezielt Änderungen eines Attributs von Datensätzen in Relationen gesucht, die bestimmten Artikeln und/oder Artikelstämmen zuzuordnen sind. Die Angaben in den Feldern *Artikelstammnummer, ArtiStammId, Artikelnummer* und A*rtikelId* sind optional, es muss aber mindestens zu einem dieser Felder eine Eingabe erfolgen. Die Angabe einer Lagernummer ist ebenfalls optional. Alle genannten Eingabefelder verfügen über eine unterstützende Itembox-Anbindung.

Wird lediglich ein Artikelstamm durch Angabe der Artikelstammnummer oder ArtiStammId spezifiziert, so ist die Basis für artikelbasierte Suchanfragen die Menge aller Artikel, die zu diesem Artikelstamm gehört. Die Angabe einer Lagernummer schränkt die Basis auf die Artikel des spezifizierten Lagers ein.

Wird ein Artikel per Artikelnummer spezifiziert, so ist die Basis für artikelbasierte Suchanfragen die Menge aller Artikel, die diese Artikelnummer haben. Die Angabe einer Lagernummer schränkt die Basis auf die Artikel des spezifizierten Lagers ein.

Wird ein Artikel per ArtikelId spezifiziert, so ist die Basis für artikelbasierte Suchanfragen nur der angegebene Artikel.

Zu beachten ist, dass gegebenenfalls auch Artikel mit Löschkennzeichen berücksichtigt werden.

Die Angabe des auszuwertenden Attributnamens wird durch eine Itembox unterstützt, die auch über eine Auflistungsvariante nach den hier erlaubten Relationsnamen verfügt und nach Auswahl des Attributs auch den zugehörigen Relationsnamen in das entsprechende Maskenfeld schreibt.

Existieren Attribute mit dem angegeben Namen in mehreren Relationen, so kann diese, ebenfalls unterstützt durch eine entsprechende Itembox, angegeben werden.

Der zu untersuchende Zeitraum bezüglich des Logfile-Archivierungsdatums, wie auch die maximale Anzahl der Datensätze, die in die Auswahlliste zu übernehmen sind, können angegeben werden. Die Suche erfolgt grundsätzlich beginnend mit dem Bis-Datum hin zum Ab-Datum und bricht bei Erreichen der Maximalzahl der Ergebnissätze ab.

Mit der Funktion **Anfrage starten** wir die Anfrage generiert und zur Abarbeitung an eine entfernte Prozedur übergeben.

Die generierte Abfrage kann relativ komplex werden und kann eine mehr oder weniger längere Antwortzeit bewirken. Wird zum Beispiel das Attribut *ArtiZAG_Preis* der Relation *ARTZUABGENSATZ* zu einem Artikelstamm angegeben, so müssen alle Datensätze des Logfilearchivs gefunden werden, die mit Schlüsselwerten zur Relation *ARTZUABGENSATZ* versehen sind, die in Zu-/Abschlag-Zuordnungen vorkommen, deren Zu-/Abschlag-Gruppennummer als normale EK- oder VK-Zu-/Abschlaggruppe oder individuelle EK- oder VK-Zu-/Abschlaggruppe in einem der zum Artikelstamm gehörenden Artikel zugeordnet sind.

Die Maske wird nach Ende der Bearbeitung durch den Fremdserver automatisch verlassen und das Ergebnis in der Auswahlliste dargestellt. Werden keine Daten angezeigt, so konnten keine Logfile-Einträge entsprechend der gemachten Angaben gefunden werden.
