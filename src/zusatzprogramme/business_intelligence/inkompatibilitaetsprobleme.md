# Inkompatibilitätsprobleme

<!-- source: https://amic.de/hilfe/inkompatibilittsprobleme.htm -->

• Es kann bei einem schlecht formulierten SQL Befehl dazu führen, dass Feldnamen doppelt vergeben worden sind. Diese unsaubere Programmierung hat schon jetzt zu der Situation geführt, dass niemand weiß, welchen der beiden Felder das System auf den Bildschirm bringt. In einer BI Umgebung sind aber doppelte Felder nicht mehr erlaubt, hierzu ist die Anwendung zu korrigieren und alle doppelten Felder sind zu entfernen.

• In eine Bereichsauswahl war es bisher möglich auf interne Strukturvariablen zuzugreifen, dieser Zugriff ist ab jetzt nicht mehr möglich und alle entsprechend formulierten Bereichsauswahlen (:\*) sind zu korrigieren und auf Datenbankvariablen (z.B. USER) umzustellen.

• Achtung: Nur weil eine Auswahlliste funktioniert, heißt es nicht, dass diese mit Business Intelligence funktioniert. Die Auswahlliste korrigiert teilweise inkorrektes SQL während Excel das SQL-Statement unkorrigiert ausführt. Beliebte Fehler sind zum Beispiel ein Komma vor dem FROM.
