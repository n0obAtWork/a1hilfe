# Registerkarte Spezialitäten

<!-- source: https://amic.de/hilfe/_prozess_spezialitaeten.htm -->

| Feld | Bedeutung |
| --- | --- |
| Scheinmenge für die erste Wiegung | Hier kann eine Scheinmenge für die erste Wiegung eingetragen warden, die bei der Netto Wiegung verwendet warden soll. |
| Waagenprofil | Hier kann man festlegen, welches Terminal man verwenden möchte.  
Normalerweise wird das Feld Terminal beim Öffnen der Waagenmaske mit dem zuletzt verwendeten Profil gefüllt. Dies ist auch weiterhin so, wenn man eine Vorlage verwendet, in der kein Terminal angegeben ist.  
Neue Wiegung  
    
Öffnet man die Waagenmaske im Neufall mit einer Vorlage, für die ein Terminal hinterlegt ist, dann wird dieses Terminal in die Maske übernommen und das Feld Terminal deaktiviert.  
Wechsel der Vorlage  
    
Wechselt man die Vorlage, dann wird das Feld Terminal entsprechend mit angepasst, wenn die Vorlage ein Terminal enthält. Enthält sie keines, dann wird das Feld Terminal wieder zum Editieren freigegeben.  
Bearbeiten  
    
Öffnet man die Waagenmaske mit F5 zum Bearbeiten, dann wird das Feld Terminal abhängig von der Wiegung bzw. vom Status gefüllt.  
Status eröffnet:  
Es wurde noch keine Wiegung durchgeführt, deshalb wird zunächst das zuletzt verwendete Terminal geladen. Enthält die verwendete Vorlage ein Terminal, so wird dieses Terminal in die Maske übernommen und das Feld deaktiviert.   
Status 1te Wiegung:  
Wurde die 1te Wiegung durchgeführt, dann wird die Maske mit dem dort verwendeten Terminal gestartet.  
Status 2te Wiegung:  
Wurde die 2te Wiegung durchgeführt, dann wird die Maske mit dem dort verwendeten Terminal gestartet.  
   
 |
| Nacherfassung von Wiegungen | Hier kann für die nachträgliche manuelle Erfassung von Wiegungen das Wiegedatum über das Belegdatum gesteuert werden, so dass nicht die eigentliche Eingabezeit der Wiegung als Wiegedatum festgehalten wird, sondern z.B. der Tag vorher.  
Für die Nacherfassung empfiehlt es sich eine eigene Vorlage einzurichten und diese als Menüpunkt (mit Hilfe von ‚[Vorlage als Menüpunkt SF9](./funktion_vorlage_als_menuepunkt_sf9.md)’) in die Waage einzubinden. Diese Funktion ist dann bei Bedarf nur für bestimmte Bedienerklassen freizuschalten.  
 |
| Nummernkreis manuell | Hier kann ein Nummernkreis angegeben werden, mit dem die Wiegenummer der Waage bei manuellen Wiegungen vorbelegt werden soll. Das ist dann hilfreich, wenn auf den Wiegeschein eine eindeutige Wiegenummer aufgedruckt werden muss.  
Bei automatischen Wiegungen wird die Wiegenummer von der Waage an das System übertragen. Bei manuellen Wiegungen kann man eine beliebige Nummer vergeben.  
    
    
Dies ist nur eine Vorbelegung. Die Nummer kann nachträglich abgeändert werden, außer man schützt für manuelle Wiegungen das zweite Eingabefeld mit Hilfe des Einrichterparameters ‚[Wiegenummer bei manuellen Wiegungen ausblenden](../funktionen_auf_der_waagenmaske/einrichterparameter_in_der_waage.md)’.  
Wenn die Wiegenummer nach der Vorbelegung durch den Nummernkreis geändert wird, wird die Nummer nicht in den Nummernkreis zurückgegeben. Auch nicht, wenn eine manuelle Wiegung erneut manuell überschrieben und somit eine neue Nummer gezogen wird.  
    
 |
