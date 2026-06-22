# Allgemein

<!-- source: https://amic.de/hilfe/_ modreport _allgemein.htm -->

Auf diesem Tabs werden gezielt Änderungen eines Attributs von Datensätzen gesucht, die durch Angabe von mindestens einem Schlüsselattribut-Wert näher spezifiziert werden.

Die maximale Anzahl der Datensätze, die in die Auswahlliste zu übernehmen sind, wie auch der zu untersuchende Zeitraum bezüglich des Logfile-Archivierungsdatums kann angegeben werden. Die Suche erfolgt grundsätzlich beginnend mit dem Bis-Datum hin zum Ab-Datum und bricht bei Erreichen der Maximalzahl der Ergebnissätze ab.

Die Angabe des auszuwertenden Attributnamens wird durch eine Itembox unterstützt, die auch über eine Auflistungsvariante nach Relationsnamen verfügt und nach Auswahl des Attributs auch den zugehörigen Relationsnamen in das entsprechende Maskenfeld schreibt.

Existieren Attribute mit dem angegeben Namen in mehreren Relationen, so kann diese, ebenfalls unterstützt durch eine entsprechende Itembox, angegeben werden.

Ist die Relation angegeben, so werden auf der Maske die zugehörigen Schlüsselattribute mit jeweils einem Eingabefeld dargestellt. Für eine Abfrage muss bei zusammengesetzten Schlüsseln mindestens ein Attribut mit einem Wert versehen werden.

Werden zum Beispiel Informationen über die Änderung des Attributs *KTRPREIS* in der Relation *KONTRAKTPREIS* gesucht, so werden bei Angabe der ID des Kontrakts im Feld *KtrId* alle Logfile-Einträge gefunden, die Änderungen des Attributs *KTRPREIS* und Löschungen oder Einfügungen von Datensätzen in dieser Relation mit der angegebenen Kontrakt-ID dokumentieren. Wird hingegen das weitere Schlüsselattribut *KtrArtiPosit* zusätzlich mit einem Wert versorgt, so wird die Suche auf den dadurch spezifizierten Kontraktartikel eingeschränkt. 

Mit der Funktion **Anfrage starten** wir die Anfrage generiert und zur Abarbeitung an eine entfernte Prozedur übergeben. Die Maske wird nach Ende der Bearbeitung durch den Fremdserver automatisch verlassen und das Ergebnis in der Auswahlliste dargestellt. Werden keine Daten angezeigt, so konnten keine Logfile-Einträge entsprechend der gemachten Angaben gefunden werden.
