# Parameter für Rohware-Datenbank-Prozeduren

<!-- source: https://amic.de/hilfe/_rwdbproc_parameter.htm -->

Die möglichen Parameter, die zur Laufzeit versorgt werden sind:

**PAR_AUFRUFMODUS**  
CHAR(12)  
    
Dieser Parameter wurde implementiert, um innerhalb der Prozedur unterscheiden zu können, zu welchem Zweck die Prozedur aufgerufen wurde.

Einer der Werte ‚FIN_PR‘, ‚ABS_PR‘, ‚WM_PR‘ oder ‚MIN_PR‘ kennzeichnet einen Prozeduraufruf zur Ermittlung eines Final-/Produktpreises, Abschlagpreises, Weltmarktpreises oder Mindestpreises.  
    
Der Wert ‚AW‘ zeigt an, dass die Prozedur zur Qualitäts-Analysewertbestimmung aufgerufen wurde.  
‚AWK1‘ bzw. ‚AWK2‘ kennzeichnen die Verwendung der Prozedur zur Bestimmung des korrigierten Analysewertes in erster bzw. zweiter Stufe (Analysewert-Korrektur 1, Analysewert-Korrektur 2 der Qualitätsdefinitionsmaske).  
Entsprechend stehen ‚UB‘ und ‚OB‘ für die Kennzeichnung des Prozedurergebnisses als unterer bzw. oberer Basiswert.

Wird eine Prozedur als Qualitäts-Abrechnungsmethode aufgerufen, so enthält der Parameter einen der vier Werte:

‚Q_ME_ABR_UB‘ bei Interpretation des Ergebnisses als Mengenänderung und Aufruf bei Unterschreitung des unteren Basiswertes durch den korrigierten Analysewert.  
‚Q_ME_ABR_OB‘ bei Interpretation des Ergebnisses als Mengenänderung und Aufruf bei Überschreitung des oberen Basiswertes durch den korrigierten Analysewert.

‚Q_PR_ABR_UB‘ bei Interpretation des Ergebnisses als Preisänderung und Aufruf bei Unterschreitung des unteren Basiswertes durch den korrigierten Analysewert.  
‚Q_PR_ABR_OB‘ bei Interpretation des Ergebnisses als Preisänderung und Aufruf bei Überschreitung des oberen Basiswertes durch den korrigierten Analysewert.  
    

Bei Aufruf einer Prozedur zur Ermittlung eines Kostensatzes wird dieses durch ‚KO_STZ‘

Als Parameterwert angezeigt, während die Ermittlung einer Kostenpauschale durch ‚KO_PAU‘ kenntlich gemacht wird.

**PAR_BELEGMODUS**  
SMALLINT  
    
Dieser Parameter gibt an, in welchem Bearbeitungsmodus sich der Rohwarebeleg, zu dem die den Prozeduraufruf auslösende Qualitätsposition gehört, sich befindet. Für diesen Parameter werden folgende Werte ermittelt:  
    
 0 – Abrechnen des Beleges  
 1 – Erfassung des Beleges  
 2 – Korrektur des Beleges

**PAR_EINLAGERUNG**  
SMALLINT  
    
Dieser Parameter liefert den Wert des Einlagerungskennzeichens der Anliefer-Warenposition (Referenznummer 1) des Rohwarebelegs, zu dem die den Prozeduraufruf auslösende Position gehört.  
Für diesen Parameter werden folgende Werte ermittelt:  
    

0 – keine Einlagerung  
1 – Einlagerung  
    

**PAR_VEREINNAHMUNG**  
SMALLINT  
    
Dieser Parameter liefert den Wert des Vereinnahmungskennzeichens aus Einlagerung der Anliefer-Warenposition (Referenznummer 1) des Rohwarebelegs, zu dem die den Prozeduraufruf auslösende Position gehört.

Für diesen Parameter werden folgende Werte ermittelt:  
    

0 – keine Vereinnahmung  
1 – Vereinnahmung  
    

**PAR_ANALYSEWERT**  
NUMERIC(15,6)  
    

Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter den Wert, der zum Zeitpunkt des Prozeduraufrufs als Analysewert in der Qualität gespeichert ist.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, der Analysewert der bezogenen Qualität übergeben.  
Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PAR_KORRANALYSEWERT**  
NUMERIC(15,6)  
    
Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter den Wert, der zum Zeitpunkt des Prozeduraufrufs als korrigierter Analysewert in der Qualität gespeichert ist.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, der korrigierte Analysewert der bezogenen Qualität übergeben.  
Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PAR_BASISUNTEN**  
NUMERIC(15,6)  
    
Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter den Wert, der zum Zeitpunkt des Prozeduraufrufs als unterer Basiswert in der Qualität gespeichert ist.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, der unterer Basiswert der bezogenen Qualität übergeben.  
Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PAR_BASISOBEN**  
NUMERIC(15,6)  
    
Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter den Wert, der zum Zeitpunkt des Prozeduraufrufs als oberer Basiswert in der Qualität gespeichert ist.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, der obere Basiswert der bezogenen Qualität übergeben.  
Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!

**PAR_KUNDID**  
INTEGER  
    
Dieser Parameter übergibt die KUNDID des dem Beleg zugeordneten Kunden-/Lieferanteneintrags.  
    

**PAR_RWGRP**  
INTEGER  
    
Dieser Parameter übergibt die Rohwarengruppe des dem Beleg zugeordneten Lieferartikels.  
    

**PAR_SORTEID**  
INTEGER  
    
Dieser Parameter übergibt die ID des dem Beleg zugeordneten Rohware-Abrechnungsschema-Eintrags.  
    

**PAR_ABR_TYP**  
SMALLINT  
    
Dieser Parameter übergibt den Typ des zugrundeliegenden Rohwarebeleges:  
1 : Lieferschein  
2 : Abschlag  
3 : Folgeabschlag  
4 : Finale  
    

**PAR_LIE_DAT**  
DATE  
    
Dieser Parameter übergibt das Lieferdatum des zugrundeliegenden Rohwarebeleges.  
    

**PAR_LIE_NR**  
INTEGER  
    
Dieser Parameter übergibt die Lieferscheinnummer des zugrundeliegenden Rohwarebeleges.  
    

**PAR_BEL_NR**  
INTEGER  
    
Dieser Parameter übergibt die Belegnummer des zugrundeliegenden Rohwarebeleges.  
    

**PAR_OWAAGE_ID**  
INTEGER  
    
Dieser Parameter übergibt den Wert der OWAAGE_ID bei Belegen. Die per Waagenschnittstelle erzeugt werden.  
    

**PAR_ABS_STAT**  
SMALLINT  
    
Dieser Parameter übergibt den Abschlagstatus des zugrundeliegenden Rohwarebeleges:  
0 : Abschlagabrechnung ist nicht vorgesehen  
1 : Abschlag vorgesehen, aber aktuell nicht zur Abrechnung freigegeben  
2 : zur Abschlagabrechnung freigegeben  
3: Abschlag abgerechnet  
    

**PAR_FABS_STAT**  
SMALLINT  
    
Dieser Parameter übergibt den Folgeabschlagstatus des zugrundeliegenden Rohwarebeleges:  
0 : Folgeabschlagabrechnung ist nicht vorgesehen  
1 : F-Abschlag vorgesehen, aber aktuell nicht zur Abrechnung freigegeben  
2 : zur Folgeabschlagabrechnung freigegeben  
3: Folgeabschlag abgerechnet  
    

**PAR_FIN_STAT**  
SMALLINT  
    
Dieser Parameter übergibt den Finalstatus des zugrundeliegenden Rohwarebeleges:  
1 : noch nicht zur Final-Abrechnung freigegeben  
2 : zur Finalabrechnung freigegeben  
3: Finale ist abgerechnet  
    

**PAR_ABS_KENNZ**  
SMALLINT  
    
Dieser Parameter übergibt den Abschlagberechnungsmodus des zugrundeliegenden Rohwarebeleges:  
1 : per Abschlagsatz  
3 : per separatem Abschlagpreis  
    

**PAR_ABS_SATZ**  
NUMERIC(15,4)  
    
Dieser Parameter übergibt den Abschlagprozentsatz zur Abschlagberechnung des zugrundeliegenden Rohwarebeleges, wenn der Abschlag per Abschlagsatz zu berechnen ist.:  
    

**PAR_REFNR**  
SMALLINT  
    
Dieser Parameter übergibt die Referenznummer (Merkmalnummer) des zugehörigen Qualitäts- oder Kostenmerkmals der zugrundeliegenden Abrechnungsschemadefinition.  
    

**PAR_BEZ_REFNR**  
SMALLINT  
    
Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter die Referenznummer (Merkmalnummer) der Warenposition der zugrundeliegenden Abrechnungsschemadefinition, auf die das Abrechnungsergebnis des aktuellen Qualitätsmerkmals wirkt.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, die Referenznummer der bezogenen Qualität übergeben.  
    

**PAR_QU_WAAGENR**  
INTEGER  
    
Bei qualitätsbezogenen Prozeduraufrufen übergibt dieser Parameter die in der Qualitätsdefinition des Abrechnungsschemas zugeordnete Waagenpositionsnummer zur aktuellen Qualität.  
Bei kostenbezogenen Aufrufen wird, sofern im Feld ‚Analysenbezug: Qualitätsmerkmal‘ der Kostendefinition eingestellt, die Waagenpositionsnummer der bezogenen Qualität übergeben.  
    

**PAR_HAUPT_ARTIKELID**  
INTEGER  
    
Dieser Parameter übergibt die Artikelid des der Lieferwarenposition (Referenznummer 1) zugeordneten Artikeleintrags.  
    

**PAR_ARTIKELID**  
INTEGER  
    
Dieser Parameter übergibt die Artikelid des den Prozeduraufruf auslösenden Warenposition zugeordneten Artikeleintrags (Nur für Preisfindungsprozeduren).  
    

**PAR_BEZ_ARTIKELID**  
INTEGER  
    

(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die Artikelid des der bezogenen Warenposition (Referenznummer: PAR_BEZ_REFNR) zugeordneten Artikeleintrags.

**PAR_LAGERNR**  
INTEGER  
    
Dieser Parameter übergibt die dem Beleg zugeordnete Lagernummer.  
    

**PAR_HAUPT_KTRID**  
INTEGER  
    
Dieser Parameter übergibt die KontraktId des der Lieferwarenposition (Referenznummer 1) zugeordneten Kontrakts.  
    

**PAR_KTRID**  
INTEGER  
    
Dieser Parameter übergibt die KontraktId des den Prozeduraufruf auslösenden Warenposition zugeordneten Kontrakts (Nur für Preisfindungsprozeduren).

    
**PAR_BEZART_KTRID**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die KontraktId des der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Kontrakts.  
    

**PAR_HAUPT_KTRPOSIT**  
INTEGER  
    
Dieser Parameter übergibt die Artikelposition im der Lieferwarenposition (Referenznummer 1) zugeordneten Kontrakt.  
    

**PAR_KTRPOSIT**  
INTEGER  
    
Dieser Parameter übergibt die Artikelposition des den Prozeduraufruf auslösenden Warenposition zugeordneten Kontrakts (Nur für Preisfindungsprozeduren).

**PAR_BEZART_KTRPOSIT**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die Artikelposition im der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Kontrakt.  
    

**PAR_HAUPT_PARTIEID**  
INTEGER  
    
Dieser Parameter übergibt die PartieId der der Lieferwarenposition (Referenznummer 1) zugeordneten Partie.  
    

**PAR_PARTIEID**  
INTEGER  
    
Dieser Parameter übergibt die PartieId der den Prozeduraufruf auslösenden Warenposition zugeordneten Partie (Nur für Preisfindungsprozeduren).

**PAR_BEZART_PARTIEID**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die PartieId der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Partie.  
    

**PAR_HAUPT_PARTIEPOSIT**  
INTEGER  
    
Dieser Parameter übergibt die Artikelposition in der der Lieferwarenposition (Referenznummer 1) zugeordneten Partie.  
    

**PAR_PARTIEPOSIT**  
INTEGER  
    
Dieser Parameter übergibt die Artikelposition der den Prozeduraufruf auslösenden Warenposition zugeordneten Partie (Nur für Preisfindungsprozeduren).

**PAR_BEZART_PARTIEPOSIT**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die Artikelposition in der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordneten Partie.  
    

**PAR_HAUPT_MENR**  
INTEGER  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Mengeneinheitsnummer.  
    

**PAR_MENR**  
INTEGER  
    
Dieser Parameter übergibt die Mengeneinheitsnummer, die der Warenposition zugeordnet ist, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

    
**PAR_BEZART_MENR**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Mengeneinheitsnummer.  
    

**PAR_HAUPT_MENRPREIS**  
INTEGER  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Preismengeneinheitsnummer.  
    

**PAR_MENRPREIS**  
INTEGER  
    
Dieser Parameter übergibt die Preismengeneinheitsnummer, die der Warenposition zugeordnet ist, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_BEZART_MENRPREIS**  
INTEGER  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Preismengeneinheitsnummer.  
    

**PAR_HAUPT_BRUTTOMENGE**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete Anfangsmenge ( = erfasste Menge ).  
    

**PAR_BRUTTOMENGE**  
INTEGER  
    
Dieser Parameter übergibt die Bruttomenge, die der Warenposition zugeordnet ist, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_BEZART_BRUTTOMENGE**  
NUMERIC(15,6  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete Anfangsmenge.  
    

**PAR_HAUPT_AKTMENGE**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt die der Lieferwarenposition (Referenznummer 1) zugeordnete im Abrechnungsverlauf aktuelle Menge vor Anwendung der aktuellen Qualitätsabrechnungsmethode.  
    

**PAR_BEZART_AKTMENGE**  
NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt die der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zugeordnete im Abrechnungsverlauf aktuelle Menge vor Anwendung der aktuellen Qualitätsabrechnungsmethode.

**PAR_KOST_MENGE**  
NUMERIC(15,6)  
    
(Nur bei kostensatzbezogenen Aufrufen!)  
Dieser Parameter übergibt die Menge der Kostenposition bei Aufruf der Prozedur zur Ermittlung eines Kosten-/Vergütungssatzes.

**PAR_KOST_MENR**  
INTEGER  
    
(Nur bei kostensatzbezogenen Aufrufen!)  
Dieser Parameter übergibt die Mengeneinheitsnummer der Kostenposition bei Aufruf der Prozedur zur Ermittlung eines Kosten-/Vergütungssatzes.

**PAR_KOST_PRMENR**  
INTEGER  
    
(Nur bei kostensatzbezogenen Aufrufen!)  
Dieser Parameter übergibt die Preismengeneinheitsnummer der Kostenposition bei Aufruf der Prozedur zur Ermittlung eines Kosten-/Vergütungssatzes.

    
**PAR_KOST_PREINH**  
NUMERIC(15,6)  
    
(Nur bei kostensatzbezogenen Aufrufen!)  
Dieser Parameter übergibt die Preiseinheit (pro 10, pro 100 …) der Kostenposition bei Aufruf der Prozedur zur Ermittlung eines Kosten-/Vergütungssatzes.

**PAR_HAUPT_ANFPRODPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_ ANFPRODPREIS**  
NUMERIC(15,6)   
    
Dieser Parameter übergibt den aktuellen Anfangsprodukt-/finalpreis der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_HAUPT_PRODPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen des Produkt-/Tages-/Finalpreises (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘), das der Lieferwarenposition (Referenznummer 1) zugeordnet ist.  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_ PRODPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Produkt-/finalpreises der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_BEZART_ANFPRODPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_ANFABSPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den Abschlagpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_ ANFABSPREIS**  
NUMERIC(15,6)   
    
Dieser Parameter übergibt den aktuellen Anfangsabschlagpreis der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_HAUPT_ABSPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen des Abschlagpreises (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘), das der Lieferwarenposition (Referenznummer 1) zugeordnet ist.  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_ ABSPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Abschlagpreises der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_BEZART_ANFABSPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Preis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Preis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).  
    

**PAR_HAUPT_ANFWMPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den Weltmarktpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_ ANFWMPREIS**  
NUMERIC(15,6)   
    
Dieser Parameter übergibt den aktuellen Anfangsweltmarktpreis der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_HAUPT_WMPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen des Weltmarktpreises (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘), das der Lieferwarenposition (Referenznummer 1) zugeordnet ist.  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_ WMPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Weltmarktpreises der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_BEZART_ANFWMPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den Weltmarktpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTWMPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Weltmarktpreis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTWMPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Weltmarktpreis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).  
    

**PAR_HAUPT_ANFMIPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den Mindestpreis, der der Lieferwarenposition (Referenznummer 1) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_ ANFMIPREIS**  
NUMERIC(15,6)   
    
Dieser Parameter übergibt den aktuellen Anfangsmindestpreis der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).

**PAR_HAUPT_MIPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen des Mindestpreises (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘), das der Lieferwarenposition (Referenznummer 1) zugeordnet ist.  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_ MIPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Mindestpreises der Warenposition, die den Prozeduraufruf auslöst. (Nur für Preisfindungsprozeduren).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PAR_BEZART_ANFMIPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den Mindestpreis, der der bezogenen Warenposition (Referenznummer PAR_BEZ_REFNR) zu Beginn des Abrechnungsverlaufs zugeordnet ist.  
    

**PAR_HAUPT_AKTMIPREIS**  
NUMERIC(15,6)  
    
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Mindestpreis für die Lieferwarenposition (Referenznummer 1).  
    

**PAR_BEZART_AKTMIPREIS  
**NUMERIC(15,6)  
    
(Nicht bei kostenbezogenen Aufrufen!)  
Dieser Parameter übergibt den im Abrechnungsverlauf vor Anwendung der aktuellen Qualität berechneten aktuellen Mindestpreis für die bezogene Warenposition (Referenznummer PAR_BEZ_REFNR).  
    

**PAR_ABR_BEZUG**

NUMERIC(15,6)  
    
Dieser Parameter steht nur bei Verwendung der Prozedur als Qualitätsabrechnungsmethode zur Verfügung. Es wird entsprechend der Einstellung für den Wert ‚Bezugsgröße‘ der Qualitätsdefinition dieser ermittelt und nur bei typgerechter Einstellung (Preisbezug bei Abrechnungstyp ‚ Preiszu-/ab-DB-Prozedur‘ bzw. Mengenbezug bei ‚ Mengenzu-/ab-DB-Prozedur‘) mit diesem Parameter übergeben.  
    
    

**PARREFn_ANALYSEWERT  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Wert, der zum Zeitpunkt des Prozeduraufrufs als Analysewert in der Qualitätsposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition) gespeichert ist (Syntaxbeispiele: PARREF2_ANALYSEWERT, PARREF13_ANALYSEWERT). Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PARREFn_KORRANALYSEWERT  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Wert, der zum Zeitpunkt des Prozeduraufrufs als korrigierter Analysewert in der Qualitätsposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition) gespeichert ist (Syntaxbeispiele: PARREF2_KORRANALYSEWERT, PARREF13_KORRANALYSEWERT). Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PARREFn_BASISUNTEN  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Wert, der zum Zeitpunkt des Prozeduraufrufs als unterer Basiswert in der Qualitätsposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition) gespeichert ist (Syntaxbeispiele: PARREF2_BASISUNTEN, PARREF13_BASISUNTEN). Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PARREFn_BASISOBEN  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Wert, der zum Zeitpunkt des Prozeduraufrufs als oberer Basiswert in der Qualitätsposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition) gespeichert ist (Syntaxbeispiele: PARREF2_BASISOBEN, PARREF13_BASISOBEN). Bei der Verwendung dieses Parameters ist aber zu beachten, dass bei der Erfassung und Korrektur eines Rohwarebelegs bei jeder Eingabe oder Veränderung eines abrechnungsrelevanten Wertes der gesamte Beleg neu durchgerechnet wird!  
    

**PARREFn_QU_WAAGENR  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt die in der Qualitätsdefinition des Abrechnungsschemas zugeordnete Waagenpositionsnummer der Qualitätsposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition Syntaxbeispiele: PARREF2_QU_WAAGENR, PARREF13_QU_WAAGENR).  
    

**PARREFn_ARTIKELID  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt die in der Schema-Haupt- oder Sekundär-Warendefinition zugeordnete Artikelid der Rohwarenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition Syntaxbeispiele: PARREF2_ARTIKELID, PARREF13_ARTIKELID).  
    

**PARREFn_MENR  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt die in der Schema-Haupt- oder Sekundär-Warendefinition zugeordnete Mengeneinheitsnummer der Rohwarenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition Syntaxbeispiele: PARREF2_MENR, PARREF13_MENR).  
    

**PARREFn_MENRPREIS  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt die in der Schema-Haupt- oder Sekundär-Warendefinition zugeordnete Preismengeneinheitsnummer der Rohwarenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition Syntaxbeispiele: PARREF2_MENRPREIS, PARREF13_MENRPREIS).  
    

**PARREFn_BRUTTOMENGE  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt die Anfangsmenge der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition Syntaxbeispiele: PARREF2_BRUTTOMENGE, PARREF13_BRUTTOMENGE).  
    

**PARREFn_ANFPRODPREIS  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Produkt-/Tages-/Finalpreis der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition), der der Position zu Beginn des Abrechnungsverlaufs zugeordnet ist (Syntaxbeispiele: PARREF2_ANFPRODPREIS, PARREF13_ANFPRODPREIS).

**PARREFn_PRODPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Produkt-/finalpreises der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PARREFn_ANFABSPREIS  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Abschlagpreis der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition), der der Position zu Beginn des Abrechnungsverlaufs zugeordnet ist (Syntaxbeispiele: PARREF2_ANFABSPREIS, PARREF13_ANFABSPREIS).  
    

**PARREFn_ABSPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Abschlagpreises der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei der gesetztem Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PARREFn_ANFWMPREIS  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Weltmarktpreis der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition), der der Position zu Beginn des Abrechnungsverlaufs zugeordnet ist (Syntaxbeispiele: PARREF2_ANFWMPREIS, PARREF13_ANFWMPREIS).  
    

**PARREFn_WMPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Weltmarktpreises der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei dem gesetzten Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.

**PARREFn_ANFMIPREIS  
**NUMERIC(15,6)  
    
Dieser Parameter übergibt den Mindestpreis der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition), der der Position zu Beginn des Abrechnungsverlaufs zugeordnet ist (Syntaxbeispiele: PARREF2_ANFMIPREIS, PARREF13_ANFMIPREIS).

**PARREFn_MIPREISFIXKENNZ**  
INTEGER  
    
Dieser Parameter übergibt das Preisfix-Kennzeichen (0 für ‚vorläufiger Preis‘, 1 für ‚Preis fest‘) des Mindestpreises der Haupt- oder Sekundär-Warenposition zur Referenznummer n (aus der Rohwarengruppen-/Schemadefinition).  
**Hinweis:** Die DB-Preisfindungsprozedur wird auch bei dem gesetzten Kennzeichen ‚Preis fest‘ ausgeführt, ein ermittelter Preis jedoch nicht dem Positionspreis zugewiesen.
