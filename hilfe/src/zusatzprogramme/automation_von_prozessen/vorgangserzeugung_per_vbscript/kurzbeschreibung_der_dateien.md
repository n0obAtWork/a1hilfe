# Kurzbeschreibung der Dateien

<!-- source: https://amic.de/hilfe/kurzbeschreibungderdateien.htm -->

bestellung_include.vbs

Diese Datei dient als Funktionssammlung sowie Klassendefinition die von den beiden anderen VBS-Dateien verwendet werden. Sie ist auch für später folgende VB-Skripte oder Projekte zu vorgesehen.

bestellung.vbs

Sie beinhaltet die Erzeugung der Vorgänge. Sie kann auch ohne Automatismus über Parameter Vorgänge zu Testzwecken (Fehlersuche/Entwicklung) erzeugen.

bestellung_start.vbs

Diese Datei liest aus den XML-Dateien die zu generierenden Vorgänge, überprüft die Daten und stößt die Vorgangserstellung/-bearbeitung in bestellung.vbs an.

autom_bestellung.xml

In dieser Datei sind zurzeit die zusammenhängen Vorgänge „Neuer Vorgang“, „Neue Position“ und „Position zusammenführen“ beschrieben. Sie wird von dem Script bestellung_start.vbs eingelesen.

auftrag.xml

Wird zurzeit nicht verwendet. Sie dient zur dynamischen Erzeugung von Aufträgen mit x-Warenpositionen und jeweils y-Partien sowie deren Mengen. Ihre Auswertung ist ebenfalls in bestellung_start.vbs implementiert.
