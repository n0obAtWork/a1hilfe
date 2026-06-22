# Kunden

<!-- source: https://amic.de/hilfe/_ modreport _kunden.htm -->

Auf diesem Tabs werden gezielt Änderungen eines Attributs von Datensätzen in Relationen gesucht, die einem bestimmten Kunden- oder Lieferantenstamm zuzuordnen sind. Die Angaben in den Feldern *Kundennummer* und *KundId* sind optional, es muss aber mindestens zu einem dieser Felder eine Eingabe erfolgen. Alle genannten Eingabefelder verfügen über eine unterstützende Itembox-Anbindung.

Wird lediglich die Kundennummer angegeben, so ist die Basis für kundenstammbasierte Suchanfragen die Menge aller Kundenstammeinträge mit dieser Kundennummer, also auch diejenigen mit eingetragenem Löschkennzeichen.

Wird ein Kundenstamm per KundId spezifiziert, so ist die Basis für kundenstammbasierte Suchanfragen nur der angegebene Kundenstammeintrag.

Die Angabe des auszuwertenden Attributnamens wird durch eine Itembox unterstützt, die auch über eine Auflistungsvariante nach den hier erlaubten Relationsnamen verfügt und nach Auswahl des Attributs auch den zugehörigen Relationsnamen in das entsprechende Maskenfeld schreibt.

Existieren Attribute mit dem angegeben Namen in mehreren Relationen, so kann diese, ebenfalls unterstützt durch eine entsprechende Itembox, angegeben werden.

Der zu untersuchende Zeitraum bezüglich des Logfile-Archivierungsdatums, wie auch die maximale Anzahl der Datensätze, die in die Auswahlliste zu übernehmen sind, können angegeben werden. Die Suche erfolgt grundsätzlich beginnend mit dem Bis-Datum hin zum Ab-Datum und bricht bei Erreichen der Maximalzahl der Ergebnissätze ab.

Mit der Funktion **Anfrage starten** wir die Anfrage generiert und zur Abarbeitung an eine entfernte Prozedur übergeben.

Die generierte Abfrage kann relativ komplex werden und kann eine mehr oder weniger längere Antwortzeit bewirken. Wird zum Beispiel das Attribut *ArtiZAG_Preis* der Relation *ARTZUABGENSATZ* zu einem Kundenstamm angegeben, so müssen alle Datensätze des Logfilearchivs gefunden werden, die mit Schlüsselwerten zur Relation *ARTZUABGENSATZ* versehen sind, die in Zu-/Abschlag-Zuordnungen vorkommen, deren Zu-/Abschlag-Kassennummer als normale EK- oder VK-Zu-/Abschlagklasse oder individuelle EK- oder VK-Zu-/Abschlagklasse im ausgewählten Kundenstamm zugeordnet sind.

Die Maske wird nach Ende der Bearbeitung durch den Fremdserver automatisch verlassen und das Ergebnis in der Auswahlliste dargestellt. Werden keine Daten angezeigt, so konnten keine Logfile-Einträge entsprechend der gemachten Angaben gefunden werden.
