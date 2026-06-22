# Stoffstromdatenberechnung per Datenbankprozedur

<!-- source: https://amic.de/hilfe/_stoffstromberperproc.htm -->

Die Berechnung der Stoffstrommenge erfolgt grundsätzlich im Stoffstrommodul durch Ermittlung der resultierenden Menge aus der Warenpositionsmenge und dem angegebenen Anteil in Verbindung mit dem Anteiltyp (% oder als Anzahl Mengeneinheit / Stoffstrom-Grundmengeneinheit). Das Ergebnis dieser Berechnung kann jedoch durch Einsatz einer privaten Datenbank-Prozedur geändert werden.

Zum einen kann eine solche Prozedur für die eine oder andere Stoffstromart in der Anwendung Bestandteile für den betreffenden Bestandteil angegeben werden. Für die Namensgebung derartiger privater Prozeduren ist lediglich zu beachten, dass diese mit **‚P_‘** beginnen und ohne Parameter anzugeben sind.  
Ist eine Datenbankprozedur für die zu berechnende Stoffstromart angegeben, so wird diese mit Versorgung der im Prozedurkopf angegeben Parametern nach der internen Wertberechnung aufgerufen und anschließend das Prozedurergebnis verarbeitet.  
Ist keine stoffartspezifische Datenbankprozedur angegeben, so wird stattdessen, falls vorhanden, die ebenfalls private Datenbankprozedur **‚p_StoffStromBerechnung‘** aufgerufen.  
Während letztere Datenbankprozedur nicht in der Datenbank vorhanden sein muss, wird bei nicht Vorhandensein der angegebenen stoffstromspezifischen Datenbankprozedur eine Fehlermeldung im Fehlerprotokoll erzeugt.

Das Berechnungsmodul erwartet als Ergebnis einer derartigen Datenbankprozedur ein RESULT mit den Attributen

\- Aktion integer

\- Anteil decimal(20,8)

\- MEAnteil integer

\- Menge decimal(20,8)

\- BerechneMenge integer

\- SetHerkunft integer

Bedeutung der Werte:

\- **Aktion**  
Nur wenn der Wert dieses Attribut = **1** ist werden die zurückgegebenen Daten berücksichtigt. Andernfalls wird das vor Prozeduraufruf ermittelte Ergebnis beibehalten.

\- **Anteil  
**Ist der Wert des Attributs **Aktion = 1**, so ersetzt der im Attribut **Anteil** zurückgegebene Wert den vor Prozeduraufruf ermittelten Wert des Stoffstrom-Anteils.

\- **MEAnteil  
**Ist der Wert des Attributs **Aktion = 1**, so ersetzt der im Attribut **MEAnteil** zurückgegebene Wert den vor Prozeduraufruf ermittelten Wert des Anteiltyps (0 für % oder Mengeneinheitsnummer) für den Stoffstrom-Anteil.

\- **Menge  
**Ist der Wert des Attributs **Aktion = 1**, so ersetzt der im Attribut **Menge** zurückgegebene Wert den vor Prozeduraufruf ermittelten Wert der Stoffstrom-Menge.

\- **BerechneMenge  
**Ist der Wert des Attributs **Aktion = 1** und der Wert des Attributs **BerechneMenge = 1**, so wird anschließend die Mengenberechnung mit dem gegebenenfalls durch die Prozedur geänderten Rückgabewerten für Anteil und Anteiltyp erneut durchgeführt.

\- **SetHerkunft  
**Ist der Wert des Attributs **Aktion = 1** und der Wert des Attributs **SetHerkunft = 0, 10 oder 20,** so wird der Herkunftsschalter der Position auf den entsprechenden Wert gesetzt  
\- **0:** aus Artikelstamm  
\- **10:** Anteil manuell  
\- **20:** Menge manuell  
    
Bei jedem anderen SetHerkunft-Rückgabewert bleibt die ursprüngliche Einstellung des Herkunftsschalters erhalten.  
**Die Rückgabe des Wertes 20 bewirkt, dass diese Position bei zukünftigen Änderungen der zugehörigen Warenpositionsdaten nicht neu berechnet wird!**

Die zu verwendende Datenbankprozedur muss über eine Auswahl erlaubter Parameter verfügen, von denen die Parameter

\- PAR_ANTEIL decimal(20,8)  
zur Übermittlung des bereits ermittelten Stoffstrom-Anteils

\- PAR_MEANTEIL integer  
zur Übermittlung des bereits ermittelten Stoffstrom-Anteiltyps

\- PAR_ERGMENGE decimal(20,8)  
zur Übermittlung der bereits ermittelten Stoffstrom-Menge

**zwingend erforderlich!**

Es stehen jedoch weitere Parameter zur Verfügung, die in der Datenbankprozedur genutzt werden können:

\- PAR_MEMENGE integer  
Mengeneinheitsnummer zur Stoffstrom-Menge

\- PAR_STOFFSTROMART integer  
Die Nummer der Stoffstrom-Art (Achtung: nicht die Bestandteilnummer!)

\- PAR_HERKUNFT integer  
Die Codierung des Herkunftsschalters (0, 10)

\- PAR_VORGKLASSE integer  
Die Vorgangsklassennummer des zugehörigen Belegs

\- PAR_VORGTYP integer  
Die Codierung des Vorgangstyps des zugehörigen Belegs  
\- 1 = EINKAUF  
\- 2 = VERKAUF  
\- 3 = UMBUCHUNG  
\- 4 = PRODUKTION  
\- 5 = INVENTUR

\- PAR_ROHWARESTUFE integer  
Die Codierung der Belegstufe bei Rohwarebelegen  
\- 1: Lieferschein  
\- 2: Abschlagrechnung  
\- 3: Folgeabschlagrechnung  
\- 4: Finalabrechnung

\- PAR_KUNDID integer  
Die KundId zum zugehörigen Vorgang

\- PAR_VKLIEFERANT integer  
Bei Verkaufsbelegen: die ID des der aktuellen Position zugeordneten Lieferanten

\- PAR_ARTISTAMMID integer  
Die ArtikelstammID des zugehörigen Artikelstamms

\- PAR_ARTIKELID integer  
Die ArtikelId des zugehörigen Artikels

\- PAR_WABEWTYP integer  
Der Warenbewegungstyp der zugehörigen Warenbewegung

\- PAR_ROHZEILENTYP integer  
Der Typ der Warenposition innerhalb eines Rohwarebelegs  
\- 100: Lieferwarenposition  
\- 200: Sekundär-/Sortierwarenposition  
\- 400: Kosten-/Vergütungsposition

\- PAR_MENGEWARE decimal(20,8)  
Die Menge der zugehörigen Warenbewegung

\- PAR_MEMENGEWARE integer  
Die Mengeneinheitsnummer der Menge der zugehörigen Warenbewegung

\- PAR_GEWICHTWARE decimal(20,8)  
Das berechnete Gewicht der zugehörigen Warenbewegung

Es müssen in der Prozedurdeklaration nur diejenigen Parameter angegeben werden, die in der Prozedur benutzt werden sollen.
