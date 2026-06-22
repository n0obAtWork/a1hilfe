# Einrichterparameter für SEPA

<!-- source: https://amic.de/hilfe/einrichterparameterfrsepa.htm -->

Hauptmenü > Mahn-,Zahl-, Zinswesen > Zahlungsverkehr > Zahlungen bearbeiten > Funktion ***DTA*** **F9**

Direktsprung **[ZHB]**

In der Dialogmaske, die den Datenträgeraustausch zusammenstellt, existieren Einrichterparameter, die nur für SEPA zuständig sind.

1) Der Einrichterparameter „SEPA: Folgende Version verwenden:“ ist nicht mehr aktiv. Die Version kann bzw. muss nun pro Hausbank eingerichtet werden.  
    

2) SEPA: Nur eine Art des Lastschriftverfahrens zulassen?  
    
Um eine Mischung von Basis- und Firmenlastschriften zu vermeiden wird vor jedem Erstellen geprüft, ob unterschiedliche Lastschriftverfahren in der Auswahl vorkommen und dann ggf. die Verarbeitung abgebrochen. Wenn man diesen Einrichterparameter auf **nein** stellt, wird diese Prüfung nicht durchgeführt.

In der Dialogmaske zum Zusammenstellen der Zahlungsvorschläge (Direktsprung **[ZHVE]**) existieren Einrichterparameter, die nur für SEPA zuständig sind.

1) SEPA Bankarbeitstage vor Fälligkeit bei Erstlastschrift. Standardeinstellung ist 5.

2) SEPA Bankarbeitstage vor Fälligkeit bei Folgelastschrift. Standardeinstellung ist 2.

3) SEPA Bankarbeitstage vor Fälligkeit bei Firmenlastschrift. Standardeinstellung ist 1.

4) SEPA Bankarbeitstage vor Fälligkeit bei Eillastschrift. Standardeinstellung ist 1.

5) SEPA Maximale Vordatierung des Ausführdatums(Kalendertage). Standardeinstellung ist 14.

**Achtung**: *Ab Version 3.0 wurde die Vorlauffrist für Erst-, Folge, Letzt – und Einmallastschriften auf einen Bankarbeitstag verkürzt.*
