# Wichtige Tabellen des Kassensystems

<!-- source: https://amic.de/hilfe/wichtigetabellendeskassensyste.htm -->

| Tabelle | Beschreibung | Schlüssel |
| --- | --- | --- |
| AcashBelg | Kassenbelege | BelegKs (Kassennummer)  
BelegId (interne Belegidentifikation)  
FilialNummer (Betriebsstättennummer Erzeuger)  
   
Bei Barverkäufen ist der Kassenbeleg über BelegId und V_Id mit einem Vorgang verknüpft. |
| AcashBelgZhlg | Zahlungssätze zu Kassenbelegen. Zu einem Kassenbeleg kann es mehrere Zahlungssätze geben. | ZahlKs (Kassennummer)  
ZahlBelegId (interne Belegidentifikation)  
FilialNummer (Betriebsstättennummer Erzeuger)  
ZahlNummer (fortlaufende Nummer)  
   
Verknüpfung mit AcashBeleg über BelegKs=ZahlKs und BelegId=ZahlBelegId und die Filialnummern. |
| AcashBelgZami | Zahlungsmittel bei unbarer Zahlung | ZamiIdNr (interne Identifikation)  
   
Verknüpfung über AcashBelgZhl.ZahlZamiIdNr |
| AcashBelgKsiz | Kassenbericht. Die Tabelle enthält zusammenfassende Information über einzelne Sitzungen. | KsiKsNr (Kassennummer)  
KsiASK (sitzungsnummer)  
FilialNummer (Betriebsstättennummer Erzeuger) |
| AcashFibuLink | Beschreibt die Verbindungen von Kassenbelegen zur Fibu | BelegKs (Kassennummer)  
BelegId (interne Belegidentifikation)  
FilialNummer (Betriebsstättennummer Erzeuger)  
FibuV_Id (Ident. Des zugeh. Fibu-Vorgangs) |
