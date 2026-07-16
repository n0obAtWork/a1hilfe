# Datenbankprozedur für Neuanlage

<!-- source: https://amic.de/hilfe/_frz_partie_dbproc_neuanlage.htm -->

Ermöglicht bei der Neuanlage von Partien die individuelle Vorbelegung diverser Felder mittels einer privaten Datenbankfunktion:

Einrichtung der Prozedur

Diese Prozedur wird in der Vorgangsunterklasse [FRZ] eingerichtet (DB-Prozedur für Neuanlage). Nach Eingabe des Prozedurnamens und anschließender Rückpositionierung auf dieses Feld können die entsprechenden Übergabeparameter eingestellt werden, die der Anwender für seine Prozedur wünscht. Auf der rechten Seite kann ein hieraus erzeugtes Muster für die Datenbankprozedur durch Kopieren übernommen werden.

Ablauf der Neuanlage der Partie

Zunächst werden Standardvorbelegungen durchgeführt. Anschließend werden die Vorbelegungen aus bisherigen Einrichterparametern oder -Einstellungen übernommen. Danach wird der Partiestamm in der Datenbank angelegt. Es werden aber noch keine Partieartikel hinzugefügt!

Anschließend wird dann, sofern eingerichtet, die individuelle Datenbankprozedur aufgerufen. Bitte beachten: Wenn innerhalb dieser Prozedur eine Änderung erfolgt, muss diese mit einem ‚COMMIT’ abgeschlossen werden.

Abschließend wird dieser Partiestamm wieder eingelesen und auf der Erfassungsmaske präsentiert (es gibt allerdings auch Situationen, in denen Partien nur im Hintergrund erzeugt werden, wie z.B. bei der automatischen Belegpartie!).

Parameterübergabe an die Prozedur

An die Prozedur wird eine Reihe von Parametern übergeben, die bei der Vorbelegung hilfreich sein können. Es können aus technischen Gründen nicht immer alle Parameter mit Werten übergeben werden. Folgende Parameter stehen zur Verfügung:

*PartieId:* Die neu erzeugte Partie kann durch die PartieId eindeutig identifiziert werden

*Vorgangsklasse*: Die Vorgangsklasse des aktuellen Beleges (im Fall ‚Waage’ handelt es sich hierbei um die Vorgangsklasse, mit der später der Vorgang erzeugt wird)

*Vorgangsunterklasse*: Wie Vorgangsklasse

*Belegnummer*: Soweit vorhanden, die Belegnummer des Beleges

*Aufruftyp*: Mit diesem Wert wird ein Hinweis gegeben, wofür die Partie angelegt wird. Die Kodierung hierzu: Siehe unten!

*ArikelId*: Welcher Artikel löste hier die Partieerzeugung aus

*KundId*: Welcher Kunde/Lieferant ist hierbei beteiligt

Kodierung des Parameters Aufruftyp

- 0 = undefiniert 
- 1 = Warenerfassung
- 2 = BelegPartie
- 3 = Rohware 
- 4 = Produkt 
- 5 = Komponente 
- 6 = Umbuchung 
- 7 = PartienNachtragen
- 8 = Waage 
- 9 = SchnellKorrektur
