# Preise - Preisfindungseinstellungen pro Vorgang

<!-- source: https://amic.de/hilfe/preisepreisfindungseinstellung.htm -->

Auf der Registerkarte „Preisfindung“ stehen folgende Felder zur Verfügung

| Feld | Beschreibung |
| --- | --- |
| Fixe Preisliste | Steht hier eine gültige Preislistennummer, wird die zugehörige Preisliste zur Preisermittlung herangezogen. |
| Fixe Steuergruppe | ---- nicht mehr verwenden ---- |
| Brutto-Vorgänge | Vorbelegung bei ‚Ja‘ als Bruttobeleg |
| Preis laut Bewertung | Bei der Preisvorbelegung per Bewertungspreis ( zum Beispiel bei Lagerumbuchungen) gibt man hier das Verfahren zur Ermittlung des Bewertungspreises ein. |
| Preisfindungsprozedur | Der Name Datenbankprozedur, die zur Preisermittlung aufgerufen werden soll. Ein Beispiel für die (möglichen) Parameterübergaben findet sich in der Datenbank in der Prozedur ‚Amic_MusterPreisAusDatenbank“. Der Prozedur werden diverse Informationen übergeben, wobei einige Parameter nur bei bestimmten Vorgangsklassen sinnvoll belegt sind. Die Rückgabe der Werte erfolgt über eine (optionale) Ergebnismenge. Liefert die Prozedur keine Ergebnismenge, wird die übliche Preisfindung aktiviert. Die Ergebnismenge muss mindestens folgende Datenbankfelder enthalten:  
Preis numeric(15,6) // der Preis ( bei Währung in der Währung)  
 Einheit numeric(15,4) // Einheit = per 1, per 100 oder ähnlich  
 ME_Nummer integer) // Mengeneinheit des Preises  
Die Prozedur kann für alle Vorgangsklassen außer Rohware hinterlegt werden.  
    
 |
| Aufrufebene | |
| Preislimit berücksichtigen | Hier kann angegeben werden, ob das Preislimit aus dem Artikelstamm bei der Eingabe eines Preises bei der Erfassung geprüft werden soll. |
