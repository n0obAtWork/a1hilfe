# Archiv-Import über JPP-Methode Free_Import

<!-- source: https://amic.de/hilfe/archivimportberjppmethodefreei.htm -->

Die Aufgabe der JPP-Methode Free_Import aus dem JPP-Objekt JFA_Import ist es Dateien gemäß den in [FAI einrichtbaren Import-Profilen](../archiv_import/index.md) in das Archiv zu verbringen.

Diese Methode wird von diversen Aeins-internen Applikationen aufgerufen, u.a. Mandantenserver (Profile mit Automatik-Kennzeichnung), aber auch Abwicklungen in den „Bereichen“ FAI und FAA.

Im Mandantenserver-Betrieb werden automatisch die Schalter „Protokoll“ und „Start-Abfrage“ auf „Nein“ gesetzt.

| Parameter: |
| --- |
| fai_id | Pflichtfeld | „Schlüssel“ der Relation fa_import |
| fai_pfad | Optional | Standard: …  
Ist dieser Pfad angegeben und ungleich …, so überschreibt dieser Wert die Profil-Vorgabe fai_pfad.  
Unterstützt werden hier JVARS, d.h. es wird zur Laufzeit der Methode der Inhalt einer JVAR herangezogen. Ein Beispiel für die Syntax ist: @jvars(5004,userpath) |
| receiver | Optional | Standard: … |
| mandser | Optional | Standard: FALSE |
| olderas | Obsolete | Versorgung über das Feld „Wartezeit in Minuten“  
Siehe im gleichen Zusammenhang auch die nun mögliche Parametrisierung „Max. Anzahl pro Durchlauf“ |
