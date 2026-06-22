# Labordaten

<!-- source: https://amic.de/hilfe/_labordaten.htm -->

Hauptmenü > Saatzucht > Saatenlabor > Labordaten 

oder Direktsprung **[LABOR]**

Funktionen der Anwendung Labor

| Funktion | Bedeutung |
| --- | --- |
| Neue Probe | Öffnet die Labormaske zum Erfassen einer neuen Probe.  
 |
| Probendaten bearbeiten | Öffnet die Ausgewählte Probe zum Bearbeiten.  
 |
| Probenuntersuchung bearbeiten |   
 |
| Probenzusatzdaten bearbeiten |   
 |
| Probendaten ansehen | Öffnet die Ausgewählte Probe nur zum Ansehen.  
 |
| Nachuntersuchung | Ermöglicht das Nacherfassen einzelner Verfahren.  
 |
| Methoden | Öffnet die Anwendung zur Pflege der [Methoden](./lwk_uebertrage.md)  
 |
| Verfahren | Öffnet die Anwendung zur Pflege der [Verfahren](../laborverfahren.md)  
 |
| Löschen |   
 |
| Drucke Prüfbericht | Druckt ein oder mehrere [Prüfberichte](../labormethoden.md#Methode_Preufbericht) aus, die an einer [Methode](./lwk_uebertrage.md) hinterlegt worden sind.  
 |
| Drucke Teilprobenetikett | Druckt alle [Teilprobenetiketten](../labormethoden.md#Methode_TeilprobenEtikett) aus, die an der [Methode](./lwk_uebertrage.md) hinterlegt wurde  
 |
| Drucke Untersuchungsetiketten | Druckt alle Etiketten, die im [Verfahren](../laborverfahren.md) auf der Registerkarte [Allgemein](../laborverfahren.md#UEB_LaborverfahrenAllgemein) hinterlegt wurden.  
 |
| Archiv Ansehen | Öffnet die Archiv-Anwendung für Labor.  
 |
| Probeteilen  
   
und  
    
    
Probeteilen und Druck | Mit dieser Funktion wird für jedes [Teilprobenetikett](../labormethoden.md#Methode_TeilprobenEtikett), welches in der [Methode](./lwk_uebertrage.md#Labormethoden) des ausgewählten Labordatensatzes hinterlegt ist, ein Eintrag in die Tabelle Saatgutetiketten gemacht. Bei der Funktion „Probedruck und Druck“ wird nach dem Anlegen der Datensätze die entsprechenden Etiketten ausgedruckt.  
   
In dem Steuerparameter [1043 „Allgemeiner Steuerparameter für das Labor“](../../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/allgemeiner_steuerparameter_fuer_das_labor_spa_1043.md) kann für die Ausprägung „ProzedurProbeTeilen“ eine alternative Datenbankprozedur zur Probenteilung hinterlegt werden. Diese muss als Eingangsparameter und Rückgabeparameter dieselben Parameter haben wie die Standardprozedur Laborprobeteilen:  
    
    
   
 |
| Wiederholdruck | Mit dieser Funktion werden die zu diesem Labordatensatz hinterlegten Etiketten in der Tabelle Saatgutetiketten erneut ausgedruckt.  
 |

Folgende Einrichterparameter gibt es auf diesem Pfleger: [Einrichterparameter Labordaten](../../../firmenstamm/einrichterparameter/qualitaetslabor_nach_ista_epa_labordaten.md)

Funktionen auf der Maske

Die Funktionen zu dieser Maske erreicht man nur über das Kontextmenü durch Drücken der rechten Maustaste. Folgende Funktionen können aus der Maske aufgerufen werden.

| Funktion | Bedeutung |
| --- | --- |
| Speichern | Speichert den Datensatz ab.  
**Achtung:** *Die Verfahrensdaten auf den Registern werden sofort bei Eingabe gespeichert!*  
 |
| Silozuordnung | Öffnet die Maske Silopartien  
 |
| Archiv Ansehen | Öffnet die Archiv Zuordnung zu der Probe  
 |
| Drucke Prüfbericht | Druckt ein oder mehrere [Prüfberichte](../labormethoden.md#Methode_Preufbericht) aus, die in der verwendeten [Methode](./lwk_uebertrage.md) unter „Prüfberichte“ hinterlegt wurden  
 |
| Drucke Teilprobenetikett | Druckt alle [Teilprobenetiketten](../labormethoden.md#Methode_TeilprobenEtikett) aus, die in der verwendeten [Methode](./lwk_uebertrage.md) unter „Etikett Teilproben“ hinterlegt wurden  
 |
| Drucke Untersuchungsetiketten | Druckt alle Etiketten, die im [Verfahren](../laborverfahren.md) auf der Registerkarte [Allgemein](../laborverfahren.md#UEB_LaborverfahrenAllgemein) unter „Druckoptionen“ hinterlegt wurden.  
 |

Auf der Labordaten-Maske befinden sich im oberen Teil die Kopfdaten eines Probensatzes sowie darunter ein Register mit den Daten der einzelnen Untersuchungsverfahren.

Im Kopfteil der Maske befindliche Felder:

| Name | Bedeutung |
| --- | --- |
| Typ | Hier kann der Probentyp (Zweck) der Probenentnahme eingetragen werden. Mit der Taste **F3** kann hier eine Auswahl (AF_QualArt) aufgerufen werden.  
 |
| Probe | Die Probennummer wird über den Nummernkreis vorgeschlagen, der in der Methode mit demselben Probentyp(Zweck) hinterlegt wurde.  
 |
| Eing.datum | Das Eingangsdatum der Probe.  
 |
| Anerkennungsnr./Partie | Die Partiebezeichnung.  
 |
| Art | Hier wird die Bezeichnung der Saatfruchtart angezeigt.  
 |
| Sorte | Hier wird die Bezeichnung der Saatfruchtsorte angezeigt.  
 |
| Kateg. | Hier wird die Kurzbezeichnung der Saatkategorie angezeigt.  
 |
| Behandlung | Hier kann die Laborbehandlung eingetragen werden, sie ergibt sich aus der zugeordneten Partie bzw. des Artikel. Es ist jedoch auch möglich diese manuell auszuwählen. Mit der Taste **F3** kann hier eine Auswahl (AF_BEHANDLUN) aufgerufen werden.  
 |
| Probedatum | Das Datum der Durchführung der Probe.  
 |
| Probengew. | Das Gewicht der Probe. Es darf nicht kleiner als 213 g sein.  
 |
| Prob.Nehmer | Hier wird eingetragen von wem die Probe entnommen wurde.  
 |
| Lager | Die Lagernummer.  
 |
| Aufbereiter | Die bundesweite gültige Aufbereiterkennziffer.  
 |
| VO-Kennz | Das Vermehrerorganisation-Kennzeichen.  
 |
| Nob | Hier kann angegeben werden, ob eine „nicht obligatorische Beschaffenheitsprüfung“ durchgeführt werden soll.  
 |
| Q-Stat | Der Qualitätsstatus. Mit der Taste **F3** kann hier eine Auswahl (AF_QUALSTAT) aufgerufen werden.  
 |
| Norm | Die Norm wird im Benutzerformat „BF_QualKl“ gepflegt und kann via Taste **F3** **ausgewählt werden**.  
 |
| Partiereferenz |   
 |
| Methode | Die Methode beschreibt Art und Abfolge des in dieser Saatgutprüfung anzuwendenden Verfahrens. Zu beachten ist bei der Auswahl der Methode die Bedeutung von Probentyp, Fruchtart, Kategorie, Sortentyp, Norm, Anbauart und Behandlung (siehe [Methode](./lwk_uebertrage.md)). Nachdem die Methode ausgewählt wurde, werden in der darunterliegenden Tabelle die Verfahren, die dieser Methode zugewiesen sind, angezeigt.  
 |
| Bemerkung | Hier kann eine Bemerkung zu diesem Stammsatz eingetragen werden.  
 |
| Verfahren | In dieser Tabelle werden die Verfahren zur Methode angezeigt und können um weitere Verfahren ergänzt werden. Steht der Einrichterparameter „Erweiterte Einstellungen“ auf **Ja**, so kann zum Entfernen eines Verfahrens auf der entsprechenden Zeile „Shift-Strg+Entf“ gedrückt werden. Verfahren können nur entfernt werden, wenn noch kein Untersuchungsergebnis eingetragen wurde.  
 |
| §15 | In diesem Feld kann eingetragen werden, ob Paragraph 15 hier Anwendung fand.  
 |
| Partie | Die Partienummer.  
 |
| SAP Probe | Die Probennummer innerhalb des SAP Systems.  
 |
| Bemerkung | Hier wird der zur Partie gehörige Matchcode angezeigt.  
 |
| Gewicht |   
 |
| Satznr | Die Nummer des Probensatzes (neu = 1), Nachuntersuchungen erhöhen den Wert jeweils um eins.  
 |
| SAP-Prüflos | Die Prüflosnummer (SAP QPLOS) kann hier eingetragen werden.  
 |
| Artikel | Hier wird die Artikelnummer und die Artikelbezeichnung aus dem Artikelstamm angezeigt.  
 |
| Artikelbezeichnung | Hier wird die Artikelbezeichnung aus dem Artikelstamm angezeigt.  
 |
| Vermehrer | Hier wird die Vermehrernummer (die Kundennummer aus dem Kundenstamm) und die Bezeichnung angezeigt.  
 |
| Attest | |

Sind alle notwendigen Kopfdaten angegeben, so wird die Erfassung dieses Teils der Daten mit der Taste **F9** (Speichern der Kopfdaten) abgeschlossen. Nun können die Daten zu den ausgewählten Verfahren in den jeweiligen Registerkarten bearbeitet werden.  
Dabei ist **zu beachten**, dass, für jedes Verfahren getrennt, nach jeder Eingabe auf der Verfahrensregisterkarte die zum Verfahren gehörenden Daten unmittelbar gespeichert und wieder eingelesen und angezeigt werden. Dadurch ist es möglich, individuelle Berechnungen zum Verfahren mittels privater Update-Trigger auf den zum Verfahren gehörenden Relationen durchführen zu lassen und die resultierenden Ergebnisse sofort auf der Maske sichtbar zu machen. Diese Realisierung der Implementation von Berechnungen ermöglicht unter anderem auch die Berechnungen bei der Erzeugung der Datensätze außerhalb der Bearbeitung per Labordaten-Maske (zum Beispiel durch Scanner-Datenübernahme).

Felder auf der Registerkarte KF ung. (Keimfähigkeit ungebeizt), KF geb. (Keimfähigkeit gebeizt) und Derm(Keimfähigkeit)

Bei der Keimfähigkeitsuntersuchung werden eine bestimmte Anzahl Reine Samen eingekeimt und nach einer festgelegten Anzahl an Tagen (Keimdauer) ausgewertet. Die Samen und Keimlinge werden in folgende Kategorien eingeteilt: Normale und anomale Keimlinge; frische, harte und tote Samen. Der Prozentsatz der normalen Keimlinge bildet die Keimfähigkeit. Man spricht von gebeizten Saatgut, wenn es mit Pflanzenschutzmitteln gegen bspw. Pilzbefall oder Schädlinge behandelt wurde.

Es ist möglich den Eingabefeldern in diesem Verfahren eine Vordergrundfarbe und Hintergrundfarbe zuzuordnen. Diese müssen in der Tabelle „Keimfähigkeit“ im Feld „Feldfarbe“ z.B. per Trigger hinterlegt werden. Die Farbe wird dann bei jedem einlesen der Daten aktualisiert. Beispiel: „k.wh1_frisch$, ROT, GRÜN**;** k.wh2_frisch$, BLAU, GELB“

Die Kombination aus Feldnamen, Hintergrundfarbe, Vordergrundfarbe wird mit Komma getrennt. Jede weitere Kombination wird mit einem vorangestellten Semikolon hinzugefügt. Möglich Farben sind: BLACK, SCHWARZ, BLUE, BLAU, GREEN, GRÜN, GRUEN, CYAN, TÜRKIS, TUERKIS, RED, ROT, MAGENTA, YELLOW, GELB, WHITE, WEISS, WEIß, GREY, GRAY, GRAU.

| Name | Bedeutung |
| --- | --- |
| Medium | Hier wird das Labormedium angezeigt aus dem Format „AF_MEDIUM“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Behandlung | Hier wird die Laborbehandlung angezeigt aus dem Format „AF_BEHANDLUN“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Menge | Hier wird die Vorbelegung der Menge zur Behandlung angezeigt aus dem Format „AF_BEHAMENGE“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Körner | Die Anzahl der Körner wird hier angezeigt.  
 |
| Vork./Temp | Hier wird die Temperatur der Vorkühlung angezeigt.  
 |
| Keimung | Hier wird die Keimtagevorbelegung angezeigt aus dem Format „AF_KEIMTAGE“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Temp. | Hier wird die Keimtemperatur angezeigt aus dem Format „AF_KEIMTEMP“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Ansetzdatum | Das Datum der Ansetzung wird hier eingetragen.  
 |
| Auswertdatum | Das Datum der Auswertung wird hier eingetragen.  
 |
| Abschlussdatum | Das Abschlussdatum wird hier eingetragen.  
 |
| Norm | Die Qualitätsklasse kann hier eingetragen werden.  
 |
| WH1 – WH8 | Wiederholung 1 bis 8  
 |
| ges. | Gesamt (als Fließkommazahl)  
 |
| ger. | Gerundet (als Ganze Zahl).  
 |
| Zählung | Das Ergebnis der Zwischenzählung kann hier eingetragen werden.  
 |
| Normal | Das Ergebnis der Zählung mit dem Status „normal“ kann hier eingetragen werden.  
 |
| Anomal | Das Ergebnis der Zählung mit dem Status „anomal“ kann hier eingetragen werden.  
 |
| Tot | Das Ergebnis der Zählung mit dem Status „tot“ kann hier eingetragen werden.  
 |
| Hart | Das Ergebnis der Zählung mit dem Status „hart“ kann hier eingetragen werden.  
 |
| frisch gek. | Das Ergebnis der Zählung mit dem Status „frisch gekeimt“ kann hier eingetragen werden.  
 |
| Summe | Hier wird die Summe eingetragen.  
 |
| Zähl. % |   
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ per Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Ges. KF % | Die Gesamtkeimfähigkeit in Prozent kann hier eingetragen werden.  
 |
| Bemerkung | Hier kann eine Bemerkung für die Untersuchung eingetragen.  
 |
| Bemerkung intern | Hier kann eine interne Bemerkung für die Untersuchung eingetragen.  
 |

Felder auf der Registerkarte Lufa

Untersuchungen, die bei der Lufa (Landwirtschaftliche Untersuchungs- und Forschungsanstalt) in Auftrag gegeben wurden, werden hier eingetragen.

| Name | Bedeutung |
| --- | --- |
| Institut | Die Kundennummer des Lufa-Labor wird hier eingetragen. |
| AuftragNr. Int | Die interne Auftragsnummer wird hier eingetragen. |
| AuftragNr. Ext | Die externe Auftragsnummer wird hier eingetragen. |
| Pr.-Eing.datum | Das Eingangsdatum der Probe kann hier eingetragen werden. |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden. |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen. |
| Masse | |
| Masse Mengeneinheit | Hier kann die Mengeneinheit der Masse eingetragen werden. |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen. |
| Inhaltsstoffe (Tabelle) | |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**. |

Felder auf der Registerkarte Reinheit/Besatz

Bei der Reinheitsuntersuchung wird durch mechanische Auftrennung der Untersuchungsprobe in Reine Samen (Samen, die augenscheinlich zu derselben, angegebenen Fruchtart gehören), Samen anderer Arten (Unkrautsamen, Samen anderer Kulturarten) und Unschädliche Verunreinigungen unterschieden.

Bei der Besatzuntersuchung wird eine vorgegebene Menge Saatgut auf Samen anderer Arten untersucht, die gezählt und mit botanischem Namen angegeben werden.

| Name | Bedeutung |
| --- | --- |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden. |
| UMenge (g) | Die Untersuchungsmenge in Gramm der Probe kann hier eingetragen werden. |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen. |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**. |
| KU1 – KU3 | Kulturart 1 - 3 |
| KUSum | Kulturart Gesamtsumme |
| UK1 – UK3 | Unkraut/Wildart 1 – 3 |
| UKSum | Unkraut Gesamtsumme |
| UV | |
| Scler. | |
| g | |
| % | |
| g.% | |
| AW % | |
| GK % | |
| SF % | |
| Reinheit (Tabelle) | In der Tabelle „Reinheit“ können in der Spalte „g“ das Gewicht in Gramm eingegeben werden. Die Spalte „%“ dient der prozentualen Eingabe. Die Anzahl kann in der Spalte „Anz“ angegeben werden. In der Spalte „Besatz“ wird die Besatzbezeichnung eingetragen. Die Spalte „T“ wird zur Pflege der Besatzarten-Gruppierung genutzt und ist an das Format „AF_BESATZART“ angeschlossen via Taste **F3** **kann hier eine Auswahl aufgerufen werden****.**  
 |
| Besatz (Tabelle) | In der Tabelle „Besatz“ können in der Spalte „%“ prozentualen Eingaben vorgenommen werden. Die Anzahl kann in der Spalte „Anz“ angegeben werden. In der Spalte „Besatz“ kann die Besatzbezeichnung eingetragen werden.  
 |

Felder auf der Registerkarte Sonstiges

Bei der Untersuchung Sortierung soll die Kalibrierung von pilliertem (mit einer Hülle aus neutraler Masse ummantelt, die Alternative hierzu wäre das Beizen) oder unbehandeltem Saatgut geprüft werden. Diese Art der Untersuchung soll mit zwei Wiederholungen durchgeführt werden.

Die Untersuchung der Feuchte von Saatgut dient der Beurteilung der Lagerfähigkeit. Feuchtes Saatgut wird leicht von Schädlingen und Mikroorganismen angegriffen. Ebenso kommt es schneller zu physiologischen Abbauprozessen, daher ist Saatgut mit einem hohen Feuchtigkeitsgehalt nur sehr begrenzt haltbar.

| Name | Bedeutung |
| --- | --- |
| TKM g | Prozentualer Wert der Triebkraft von ungebeizten Saatgut. Wiederholung 1 bis 8.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Anzahl Samen | Prozentualer Wert der Triebkraft von gebeizten Saatgut. Wiederholung 1 bis 8.  
 |
| TKM g (TKM extern) | Tausend Korn Masse in Gramm.  
 |
| Gew. r.S. | Das Gewicht der Reinen Samen kann hier eingetragen werden.  
 |
| Status (TKM extern) | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Sortiernorm | Nach welcher Norm die Sortierung durchgeführt wurde kann hier eingetragen werden.  
 |
| Wert in % | Der prozentuale Wert kann hier eingetragen werden.  
 |
| Beizgrad | Die Zusammensetzung und die Konzentration des Beizmittels kann hier eingetragen werden.  
 |
| Status (Sortierung) | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Feuchte |
| Schroten | Folgende Ausprägungen sind möglich.  
• Nein  
• Grob  
• Fein  
Die Ausprägungen werden im Anwenderformat AF_FESCHROTE gespeichert  
 |
| Dauer | In dem Feld Dauer wird die Anzahl der Stunden eingetragen. Diese sind in dem Anwenderformat „AF_FEDAUER“ hinterlegt.  
 |
| Temperatur | In diesem Feld wird die Temperatur eingetragen. Folgende Ausprägungen sind möglich  
• Niedrig (101-105°C)  
• Hoch (130-133)  
Die Daten werden im Anwenderformat „AF_FETEMP“ hinterlegt.  
 |
| U-Datum | Das Untersuchungsdatum kann hier eingetragen werden. Mit Taste F3 kann hier eine Auswahl aufgerufen werden.  
 |
| Uhrzeit | Die Uhrzeit der Untersuchung kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| 1.Wied Cont | 1\. Wiederholung Cont  
 |
| 1.Wied Tara | 1\. Wiederholung Tara-Gewicht  
 |
| 1.Wied Brutto | 1\. Wiederholung Bruttogewicht  
 |
| 1.Wied Trock | 1\. Wiederholung Trockengewicht  
 |
| 1.Wied Erg. % | 1\. Wiederholung Ergebnis in Prozent.  
 |
| 2.Wied Cont | 2\. Wiederholung Cont  
 |
| 2.Wied Tara | 2\. Wiederholung Tara-Gewicht  
 |
| 2.Wied Brutto | 2\. Wiederholung Bruttogewicht  
 |
| 2.Wied Trock | 2\. Wiederholung Trockengewicht  
 |
| 2.Wied Erg. % | 2\. Wiederholung Ergebnis in Prozent.  
 |
| Feuchte % | Die Feuchte in Prozent kann hier eingetragen werden.  
 |
| Bemerkung | Hier kann eine Bemerkung zur Untersuchung eingetragen werden.  
 |
| Status (Feuchte) | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| TKMG Leguminosen |
| WH 1 - 8 (1.Satz) | Wiederholung 1 bis 8 (1.Satz)  
 |
| WH 1 – 8 (2.Satz) | Wiederholung 1 bis 8 (2.Satz)  
 |
| TKM g | Tausend Korn Masse in Gramm von Leguminosen (Hülsenfrüchtlern).  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |

Felder auf der Registerkarte Techn.Prüfung

| Name | Bedeutung |
| --- | --- |
| Untersuchungsdatum | Das Untersuchungsdatum wird hier eingetragen.  
 |
| Hohlmaß | Das Prüfmittel zum Verfahren Hektoliter Gewicht (HLG) kann hier angegeben werden. Eine Auswahl ist mit der F3 Taste möglich. Hier stehen die Werte aus dem Format „AF_LABHOHLM“ zur Verfügung.  
 |
| Hektoliter Gewicht | Masse kann hier angegeben werden.  
 |
| Mengeneinheit | Mengeneinheit zur angegebenen Masse.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Status | Das Druckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**. |

Felder auf der Registerkarte Triebkraft

Die Triebkraftuntersuchung wird durchgeführt, wenn die gebeizte Keimfähigkeit über einem festgelegten Schwellwert gegenüber der ungebeizten Keimfähigkeit liegt. Sie ist ein wichtiger zusätzlicher Indikator für die Gesundheit des Saatgutes.

| Name | Bedeutung |
| --- | --- |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Triebkraft ungeb. % (1 – 8) | Prozentualer Wert der Triebkraft von ungebeizten Saatgut. Wiederholung 1 bis 8.  
 |
| ges. |   
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Triebkraft ungeb. % (1 – 8) | Prozentualer Wert der Triebkraft von gebeizten Saatgut. Wiederholung 1 bis 8.  
 |
| ges. |   
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |

Felder auf der Registerkarte Silo

| Name | Bedeutung |
| --- | --- |
| Bezeichnung | Die Bezeichnung der Partie kann hier eingetragen werden.  
 |
| Menge in dt | Die Partiemenge in Dezitonnen.  
 |
| Partiemng kg | Die Partiemenge in Kilogramm.  
 |
| PartieNr | Hier kann die Partienummer eingetragen werden.  
 |
| Mischung | Hier kann die Spalte Mischung aus Tabelle Silopartie gepflegt werden.  
 |
| Gesamtmenge | Hier kann die Gesamtmenge in Dezitonnen und Kilogramm eingetragen werden.  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |

Felder auf der Registerkarte Tetra (Tetrazolium)

Biochemische Prüfung von Samen auf Lebensfähigkeit nach dem topographischen Tetrazolium-Verfahren gemäß den ISTA-Vorschriften. Es finden hierbei bis zu vier Wiederholungen statt, wobei die Samen hierfür präpariert sein können (dies erleichtert das Eindringen der Färbelösung).

| Name | Bedeutung |
| --- | --- |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| 1\. Rolle | Hier werden die Ergebnisse der ersten Untersuchung eingetragen.  
 |
| 2\. Rolle | Hier werden die Ergebnisse der zweiten Untersuchung eingetragen.  
 |
| 3\. Rolle | Hier werden die Ergebnisse der dritten Untersuchung eingetragen.  
 |
| 4\. Rolle | Hier werden die Ergebnisse der vierten Untersuchung eingetragen.  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Bemerkung | Bemerkungsfeld, hier können Bemerkungen zur Untersuchung eingetragen werden.  
 |

Felder auf der Registerkarte Zusammensetzung

Die Untersuchung der Zusammensetzung wird zum Beispiel bei Saatgutmischungen mit vorhandenen Soll-Werten für die einzelnen in der Mischung enthaltenen Bestandteile genutzt. Neben der Angabe der Untersuchungsmenge, eines abweichenden Untersuchungsdatums und des Prüfers werden die ermittelten Bestandteile per F3-Auswahl in der Spalte ‚Zusammensetzung‘ aufgenommen sowie der zugehörige Soll-Wert und der ermittelte Ist-Wert als Gewicht (gemäß den ISTA-Vorschriften) oder prozentual erfasst. Im Bemerkungsfeld kann zum Beispiel das Untersuchungsergebnis kommentiert werden.

| Name | Bedeutung |
| --- | --- |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| UMenge (g) | Die Untersuchungsmenge wird in diesem Feld festgehalten.  
 |
| Zusammensetzung | Bestandteile aus der Liste der Besatzarten  
 |
| g | Ermitteltes Gewicht des Bestandteils in der Probe (gemäß den ISTA-Vorschriften)  
 |
| Ist% | Ermittelter prozentualer Anteil des Bestandteils in der Probe  
 |
| Soll% | Prozentualer Anteil des Bestandteils  
 |
| Bemerkung | Bemerkungsfeld, hier können Bemerkungen zur Untersuchung eingetragen werden.  
 |
| Status | Der Status der Zusammensetzungsuntersuchung kann hier angegeben werden. Auswählbar über das Anwender-Format „AF_KFDRUCK“ per Taste **F3.**  
 |

Felder auf der Registerkarte Kotrollanbau

Das Verfahren Kontrollanbau ermöglicht die Eingabe von Feldversuchsergebnissen.

| Name | Bedeutung |
| --- | --- |
| Institut | Die Kundennummer des Versuchsansteller wird hier eingetragen.  
 |
| AuftragNr. Int | Die interne Auftragsnummer wird hier eingetragen.  
 |
| AuftragNr. Ext | Die externe Auftragsnummer wird hier eingetragen.  
 |
| Versanddatum | Das Versanddatum der Probe kann hier eingetragen werden.  
 |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Eine Auswahl ist mit **F3** möglich. Hier wird das Anwenderformat „AF_KFDRUCK“ verwendet.  
 |
|   
In der Tabelle können die Merkmale gepflegt werden, die zur Feststellung der Sortenechtheit benötigt werden. |
| Nr | Die Nummer des Qualitätsmerkmals. Eine Auswahl ist mit **F3** möglich. Hinter der Nummer wird die Bezeichnung des Qualitätsmerkmals angezeigt  
 |
| Ist | Der ermittelte Wert des Qualitätsmerkmals kann hier eingetragen werden.  
 |
| Ausprägung | Hier wird die Ausprägung des Qualitätsmerkmals angezeigt. In dem Steuerparameter „[Allgemeiner Steuerparameter für das Labor](../../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/allgemeiner_steuerparameter_fuer_das_labor_spa_1043.md)“ kann über den Schlüssel „ProzedurKontrollanbauAuspraegung“ eine Prozedur hinterlegt werden, die die anzuzeigende Ausprägung zurückliefert. Diese Prozedur muss dieselben Eingangs- und Ausgangsparameter haben wie die Standardprozedur „KontrollanbauAusprägung“.  
   
   
In der Standardprozedur werden der Formatname oder - wenn kein Format angegeben ist - die Mengeneinheit aus den [Qualitätsmerkmalen](../qualitaetsmerkmale.md) verwendet, um die Ausprägung anzuzeigen  
. |

Felder auf der Registerkarte Marker

Das Verfahren Marker ermöglicht die Eingabe von genotypischen Untersuchungen.

| Name | Bedeutung |
| --- | --- |
| Institut | Die Kundennummer des Versuchsansteller wird hier eingetragen.  
 |
| AuftragNr. Int | Die interne Auftragsnummer wird hier eingetragen.  
 |
| AuftragNr. Ext | Die externe Auftragsnummer wird hier eingetragen.  
 |
| Versanddatum | Das Versanddatum der Probe kann hier eingetragen werden.  
 |
| U-Datum | Das Untersuchungsdatum der Probe kann hier eingetragen werden.  
 |
| Prüfer | Der Prüfer dieser Untersuchung wird hier eingetragen.  
 |
| Status | Das Keimfähigkeitsdruckkennzeichen kann hier angegeben werden. Auswählbar über das Format „AF_KFDRUCK“ via Taste **F3** **kann** **eine Auswahl aufgerufen werden**.  
 |
| Tabelle | In der Tabelle können die Analysewerte zum vorgegebenen Merkmal gepflegt werden.  
 |
| Merkmal | Merkmale für die genotypische Untersuchung. Mit der Taste **F3** kann eine Auswahl über die [Qualitätsmerkmale](../qualitaetsmerkmale.md) abgerufen werden, die in dem betreffenden Verfahren untersucht werden soll. Hier können nur Qualitätsmerkmale ausgewählt werden, die den Merkmalstyp „Genotyp“ haben.  
 |
| Marker | Die Bezeichnung der Marker die für die Untersuchung eines Merkmals eingesetzt werden  
. |
| Anzahl | Anzahl an Datenpunkten mit denen das jeweilige Merkmal mit dem entsprechenden Marker untersucht werden soll  
. |
| HOM | Der Analysewert der Ausprägung Homozygot kann hier eingetragen werden.  
 |
| HET | Der Analysewert der Ausprägung Heterozygot kann hier eingetragen werden  
. |
| NON | Der Analysewert der Ausprägung Negativ kann hier eingetragen werden.  
 |
| Ergebnis % | Das Ergebnis in % kann hier eingetragen werden.  
 |

AIS – Erweiterung

Der Laborpfleger lässt sich mit AIS erweitern. Das zu verwendende Ident-Maskenfeld lautet **h.QualitaetsId$**

<p class="siehe-auch">Siehe auch:</p>

- [Untersuchungsetiketten-Druck](./untersuchungsetiketten_druck.md)
- [LWK-Übertrage](./lwk_uebertrage.md)
