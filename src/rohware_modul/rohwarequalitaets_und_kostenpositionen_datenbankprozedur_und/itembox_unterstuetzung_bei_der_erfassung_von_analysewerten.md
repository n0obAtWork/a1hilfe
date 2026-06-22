# Itembox-Unterstützung bei der Erfassung von Analysewerten

<!-- source: https://amic.de/hilfe/itemboxuntersttzungbeidererfas.htm -->

Hauptmenü > Rohwarenabrechnung \> Rohwaren-Verwaltung > Bearbeiten > Abrechnungsschema > Merkmal-Definition

Direktsprung **[RWG]**

Hauptmenü > Administration \> Werkzeuge > SQL Textmanager

Direktsprung **[SQLM]**

Für die Erfassung und Korrektur von Rohwarebelegen können zur Unterstützung für manuell zu erfassende Analysewerte in den zugehörigen Qualitätsdefinitionen der Abrechnungsschemata auch (private) Itembox-Zuordnungen eingetragen werden.

Bei der Ausführung des SQL-Statements der Itembox kann über eine Reihe festgelegter Parameter auf die zum Zeitpunkt der Ausführung aktuellen zugehörigen Werte zurückgegriffen werden.

Die Namen und Inhalte der Parameter sind nachfolgend erklärt:

**PAR_BELEGMODUS**  
    
Dieser Parameter gibt an, in welchem Bearbeitungsmodus sich der Rohwarebeleg, zu dem die den Itembox-Aufruf auslösende Qualitätsposition gehört, sich befindet. Für diesen Parameter werden folgende Werte ermittelt:  
    
 1 – Erfassung des Beleges  
 2 – Korrektur des Beleges

**PAR_EINLAGERUNG**  
    
Dieser Parameter gibt liefert den Wert des Einlagerungskennzeichens der Anliefer-Warenposition (Referenznummer 1) des Rohwarebelegs, zu dem die den Itembox-Aufruf auslösende Position gehört.

Für diesen Parameter werden folgende Werte ermittelt:  
    

0 – keine Einlagerung  
1 – Einlagerung  
    
    

**PAR_VEREINNAHMUNG**  
    
Dieser Parameter gibt liefert den Wert des Vereinnahmungskennzeichens aus Einlagerung der Anliefer-Warenposition (Referenznummer 1) des Rohwarebelegs, zu dem die den Itembox-Aufruf auslösende Position gehört.  
Für diesen Parameter werden folgende Werte ermittelt:  
    

0 – keine Vereinnahmung  
1 – Vereinnahmung  
    

**PAR_ANALYSEWERT**  
    

Dieser Parameter liefert den aktuellen Analysewert der die den Itembox-Aufruf auslösenden Qualitätsposition.  
    

**PAR_KORRANALYSEWERT**  
    
Dieser Parameter liefert den aktuellen „korrigierten“ Analysewert der die den Itembox-Aufruf auslösenden Qualitätsposition.  
    

**PAR_BASISUNTEN**

    
Dieser Parameter liefert den aktuellen unteren Basiswert der die den Itembox-Aufruf auslösenden Qualitätsposition.  
    

**PAR_BASISOBEN**

    
Dieser Parameter liefert den aktuellen oberen Basiswert der die den Itembox-Aufruf auslösenden Qualitätsposition.  
    

**PAR_KUNDID**  
    
Dieser Parameter übergibt die KUNDID des dem Beleg zugeordneten Kunden-/Lieferanteneintrags.  
    

**PAR_RWGRP**  
    
Dieser Parameter übergibt die Rohwarengruppe des dem Beleg zugeordneten Lieferartikels.  
    

**PAR_SORTEID**  
    
Dieser Parameter übergibt die ID des dem Beleg zugeordneten Rohware-Abrechnungsschema-Eintrags.  
    

**PAR_LIE_NR**  
    
Dieser Parameter übergibt die Lieferscheinnummer des zugrundeliegenden Rohwarebeleges.  
    

**PAR_BEL_NR**  
    
Dieser Parameter übergibt die Belegnummer des zugrundeliegenden Rohwarebeleges.

    
**PAR_LIE_DAT**  
    
Dieser Parameter übergibt das Lieferdatum des zugrundeliegenden Rohwarebeleges.

    
**PAR_OWAAGE_ID**  
    
Dieser Parameter übergibt den Wert der OWAAGE_ID bei Belegen. Die per Waagenschnittstelle erzeugt werden.

    
**PAR_ABR_TYP**  
    
Dieser Parameter übergibt den Typ des zugrundeliegenden Rohwarebeleges:  
1 : Lieferschein  
2 : Abschlag  
3 : Folgeabschlag  
4 : Finale  
    

**PAR_ABS_STAT**  
    
Dieser Parameter übergibt den Abschlagstatus des zugrundeliegenden Rohwarebeleges:  
0 : Abschlagabrechnung ist nicht vorgesehen  
1 : Abschlag vorgesehen, aber aktuell nicht zur Abrechnung freigegeben  
2 : zur Abschlagabrechnung freigegeben  
3: Abschlag abgerechnet  
    

**PAR_FABS_STAT**  
    
Dieser Parameter übergibt den Folgeabschlagstatus des zugrundeliegenden Rohwarebeleges:  
0 : Folgeabschlagabrechnung ist nicht vorgesehen  
1 : F-Abschlag vorgesehen, aber aktuell nicht zur Abrechnung freigegeben  
2 : zur Folgeabschlagabrechnung freigegeben  
3: Folgeabschlag abgerechnet  
    

**PAR_FIN_STAT**  
    
Dieser Parameter übergibt den Finalstatus des zugrundeliegenden Rohwarebeleges:  
1 : noch nicht zur Final-Abrechnung freigegeben  
2 : zur Finalabrechnung freigegeben  
3: Finale ist abgerechnet

**PAR_ABS_KENNZ**  
    
Dieser Parameter übergibt den Abschlagberechnungsmodus des zugrundeliegenden Rohwarebeleges:  
1 : per Abschlagsatz  
3 : per separatem Abschlagpreis  
    

**PAR_ABS_SATZ**  
NUMERIC(15,4)  
    
Dieser Parameter übergibt den Abschlagprozentsatz zur Abschlagberechnung des zugrundeliegenden Rohwarebeleges, wenn der Abschlag per Abschlagsatz zu berechnen ist.

**PAR_QU_WAAGENR**  
    
Dieser Parameter übergibt die in der Qualitätsdefinition des Abrechnungsschemas zugeordnete Waagenpositionsnummer zur aktuellen Qualität.  
    

**PAR_REFNR**  
    
Dieser Parameter übergibt die Referenznummer (Merkmalnummer) des zugehörigen Qualitätsmerkmals des zugrundeliegenden Abrechnungsschemas.  
    

**PAR_BEZ_REFNR**  
    
Dieser Parameter übergibt die Referenznummer (Merkmalnummer) der Warenposition des zugrundeliegenden Abrechnungsschemas, auf die das Abrechnungsergebnis des aktuellen Qualitätsmerkmals wirkt.  
    

**PAR_LAGERNR**  
    
Dieser Parameter übergibt die dem Beleg zugeordnete Lagernummer.  
    

**PAR_HAUPT_ARTIKELID**  
    
Dieser Parameter übergibt die Artikelid des der Lieferwarenposition (Referenznummer 1) zugeordneten Artikeleintrags.

    
**PAR_BEZ_ARTIKELID**  
    

Dieser Parameter übergibt die Artikelid des der bezogenen Warenposition (Referenznummer: PAR_BEZ_REFNR) zugeordneten Artikeleintrags.

**PAR_HAUPT_KTRID**  
    
Dieser Parameter übergibt die KontraktId des der Lieferwarenposition (Referenznummer 1) zugeordneten Kontrakts.  
    

**PAR_BEZART_KTRID**  
    
Dieser Parameter übergibt die KontraktId des der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Kontrakts.  
    

**PAR_HAUPT_KTRPOSIT**  
    
Dieser Parameter übergibt die Artikelposition im der Lieferwarenposition (Referenznummer 1) zugeordneten Kontrakt.  
    

**PAR_BEZART_KTRPOSIT**  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die Artikelposition im der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Kontrakt.  
    

**PAR_HAUPT_PARTIEID**  
    
Dieser Parameter übergibt die PartieId der der Lieferwarenposition (Referenznummer 1) zugeordneten Partie.  
    

**PAR_BEZART_PARTIEID**  
    
Dieser Parameter übergibt die PartieId der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Partie.  
    

**PAR_HAUPT_PARTIEPOSIT**  
    
Dieser Parameter übergibt die Artikelposition in der der Lieferwarenposition (Referenznummer 1) zugeordneten Partie.  
    

**PAR_BEZART_PARTIEPOSIT**  
    
Dieser Parameter übergibt die Artikelposition in der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Partie.  
    

**PAR_HAUPT_MENR**  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Mengeneinheitsnummer.  
    

**PAR_BEZART_MENR**  
    
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Mengeneinheitsnummer.  
    

**PAR_HAUPT_MENRPREIS**  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Preismengeneinheitsnummer.  
    

**PAR_BEZART_MENRPREIS**  
    
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Preismengeneinheitsnummer.  
    

**PAR_HAUPT_BRUTTOMENGE**  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Anfangsmenge ( = erfasste Menge ).  
    

**PAR_BEZART_BRUTTOMENGE**  
    
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Anfangsmenge.  
    

**PAR_HAUPT_AKTMENGE**  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete im Abrechnungsverlauf aktuelle Nettomenge.  
    

**PAR_BEZART_AKTMENGE**  
    
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete im Abrechnungsverlauf aktuelle Nettomenge.

**PAR_HAUPT_ANFPRODPREIS**  
    
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_BEZART_ANFPRODPREIS  
**  
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_ANFABSPREIS**  
    
Dieser Parameter übergibt den Abschlagpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_BEZART_ANFABSPREIS  
**  
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTPREIS**  
    
Dieser Parameter übergibt den aktuellen berechneten Preis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTPREIS  
**  
Dieser Parameter übergibt den aktuellen berechneten Preis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).  
    

**PAR_HAUPT_ANFWMPREIS**  
    
Dieser Parameter übergibt den Weltmarktpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_BEZART_ANFWMPREIS  
**  
Dieser Parameter übergibt den Weltmarktpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTWMPREIS**  
    
Dieser Parameter übergibt den aktuellen berechneten Weltmarktpreis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTWMPREIS  
**  
Dieser Parameter übergibt aktuellen berechneten Weltmarktpreis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).  
    

**PAR_HAUPT_ANFMIPREIS**  
    
Dieser Parameter übergibt den Mindestpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_BEZART_ANFMIPREIS  
**  
Dieser Parameter übergibt den Mindestpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTMIPREIS**  
    
Dieser Parameter übergibt den aktuellen berechneten Mindestpreis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTMIPREIS  
**  
Dieser Parameter übergibt den aktuellen berechneten Mindestpreis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).
