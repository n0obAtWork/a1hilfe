# Rohware-Wandlung

<!-- source: https://amic.de/hilfe/_svli_rwli_wandlung.htm -->

Mit der Funktion ***Rohware-Wandlung*** können Lieferscheine in Rohware-Lieferscheine gewandelt werden, wenn sie die dafür notwendigen Bedingungen erfüllen.  
Zunächst einmal muss die Wandlung durch das Kennzeichen *Rohware Vorerfassung* in der Anwendung [Vorgangsunterklasse](../../../formularzuordnung/index.md) [FRZ]mit einem der Werte **‚möglich‘** oder **,geprüft‘** für die Unterklasse des Lieferscheins belegt sein. Im ersten Fall werden nachfolgende Bedingungen für eine Rohware-Wandlung beim Aufruf der Wandlungsfunktion geprüft und die Wandlung gegebenenfalls abgelehnt. Im zweiten Fall erfolgt die Prüfung bereits bei Abschluss der Lieferschein-Erfassung. Dadurch ist dann aber für diese Unterklasse kein Lieferschein erfassbar, der die Wandlungsvoraussetzungen verletzen würde, obwohl er vielleicht ohnehin nicht zur Wandlung vorgesehen ist. Daher empfiehlt sich die Einstellung **‚geprüft‘** genau dann, wenn eine eigens für die zur Rohware-Wandlung vorgesehenen Lieferscheine geschaffene Vorgangsunterklasse vorhanden ist.

Es können nur Lieferscheine in Rohware-Lieferscheine gewandelt werden, wenn

\- der Lieferschein genau eine Warenposition enthält

\- der Artikel der Warenposition ein Rohwareartikel ist (eingetragene Rohwarengruppe)

\- der Lieferschein über keine Zeilen- und Gruppen-Rabatte, -Frachten und -Zu-/Abschläge verfügt

\- ein im Lieferschein bereits zugeordneter Kontrakt ein Rohwarekontrakt ist

\- keine Gebinde-Mengeneinheit zur Erfassung genutzt wurde

\- der Lieferschein keine Gefahrgutinformation enthält

\- keine Streckenzuordnung vorhanden ist

\- keine oder genau eine Partie zugeordnet ist

\- eine per UFLD-Feld erfasste abweichende Schemanummer zur Rohwarengruppe des Artikels passt 

Nach erfolgreicher Durchführung der Wandlung ist der Lieferschein nicht mehr in den Lieferschein-Auswahllisten vorhanden, da er ja nun aufgrund seines Rohware-Charakters im Rohwaremodul dargestellt wird.

ACHTUNG: VorgangsAddOn-Daten werden grundsätzlich bei der Wandlung beibehalten, werden aber im Rohwaremodul nicht dargestellt. Dennoch kann zum Beispiel per *SQLK-*Nutzung in der Formulareinrichtung auf diese Daten zugegriffen werden. Um diese Daten auch bei der Weiterverarbeitung (Abschlag-, Finalabrechnung) nutzen zu können, muss in den Vorgangsunterklassen der Rohwarevorgangsklassen das Feld *bei Einzelumwandlung Addon kopieren* mit dem Wert **‚Ja‘** eingestellt werden.
