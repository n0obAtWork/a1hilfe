# Änderungsprotokoll

<!-- source: https://amic.de/hilfe/_modreport_einleitung.htm -->

Menü: Administration > Werkzeuge: Projekte (Direktsprung: [SUPP]) Variante: Änderungsprotokoll

Mit der Variante *Änderungsprotokoll* können aus archivierten Logfile-Daten Änderungen von Datensätzen der A.eins-Datenbank nachvollzogen werden. Es werden damit Antworten auf die Fragestellungen der Art „Wer hat wann ein bestimmtes Attribut einer bestimmten Relation geändert, gelöscht oder eingefügt?“ geliefert.

Grundsätzlich erfolgt die Logfile-Archivierung auf einem Fremdserver, um das A.eins-System durch die schnell wachsende Datenmenge und gegebenenfalls der Komplexität und damit durchaus möglichen längeren Laufzeiten von Anfragen nicht unnötig zu belasten. Die Einrichtung eines entsprechenden Systems zur Logfile-Archivierung wird durch den A.eins-Support unter Berücksichtigung der individuellen Gegebenheiten durchgeführt. 

Änderungsabfragen werden den Eingaben entsprechend mittels Datenbankfunktionen der A.eins-Datenbank generiert und einer “entfernten“ Prozedur der Fremdserverdatenbank zur Ausführung übergeben. Als Ergebnis werden die Statements der gefundenen Logfile-Einträge, ergänzt durch den verursachenden Bediener und dem Archivierungsdatum, in die Auswahlliste übernommen.

Die Funktion **Query zusammenstellen** ruft eine Maske zur Eingabe der gewünschten Daten auf. Auf verschiedenen Tabs können unterschiedliche Anfragevarianten realisiert werden:

<p class="siehe-auch">Siehe auch:</p>

- [Allgemein](./allgemein.md)
- [Artikel](./artikel.md)
- [Kunden](./kunden.md)
- [Kontrakt](./kontrakt.md)
