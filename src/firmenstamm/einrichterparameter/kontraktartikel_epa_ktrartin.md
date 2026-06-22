# Kontraktartikel (EPA KTRARTIN)

<!-- source: https://amic.de/hilfe/_EPA_KTRARTIN.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Bezeichnung für die Felder allgemeiner Wert / Bemerkung | | Mit diesem Parameter kann die Bezeichnung für das Feld „Wert / Bemerkung“ individuell gesetzt werden. |
| Itembox Artikel (lagerspezifisch) | IB_ARTIKEL_NU | |
| Itembox Artikel (lagerunspezifisch) | IB_Artikel_Lg_UnSpezifisch_NU | |
| Eigene Artikel Itembox für lagerunspezifische Kontrakte | Nein | |
| Soll das lagerspezifische Kennzeichen in Kontrakt übernommen werden? | Nein | Wird dieser EPA gesetzt, so kann ein Eintrag „lagerspezifisch“ im Kontraktstamm eingesetzt werden. |
| Soll das Artikellager beim ersten Artikel ins Ziellager übernommen werden? | Nein | |
| Sollen die Felder für Planlieferzeit und -datum angezeigt werden? | Ja | Mit diesem Parameter kann man die Felder Planlieferzeit / -datum anzeigen oder verschwinden lassen. |
| Kontraktpreis in Weltmarktpreis übernehmen? | Keine Übernahme | |
| Soll das Rohwarengruppenfeld bei Rohwarenkontrakten gesperrt sein? | Ja | Hiermit kann das Rohwarengruppenfeld aktiviert oder deaktiviert werden. Dieser Parameter wirkt jedoch nicht, wenn der [Steuerparameter 612](../steuerparameter/kontraktwesen/rohwarengruppe_und_sorte_aus_artikel_vor.md) auf „Nein“ steht. |
| Berechnung der maximalen Unter-/Überschreitung auch bei Mengenänderung | Nein | |
| Soll die Nachhaltigkeitsüberprüfung ins Fehlerprotokoll geschrieben werden? | Nein | Bei der Nachhaltigkeitsüberprüfung wird eine Fehlermeldung ausgegeben, so dass der Artikel nicht eingegeben werden kann.  
Wird dieser Parameter auf „Ja“ gesetzt, erscheint die Meldung nicht mehr, sondern wird ins Fehlerprotokoll eingetragen. |
| Soll die Gebindeinfos angezeigt werden? (Nur für Testzwecke) | Nein | Mit diesem Parameter kann man einen Gebinderechner einschalten. Dies dient aktuell aber nur zu Testzwecken. |
| Prozedur zur Berechnung des allgemeinen Wertes | | Hier kann eine Prozedur zum Berechnen des allgemeinen Wertes und der Bemerkung hinterlegt werden. Die Prozedur wird beim Verlassen bestimmter Felder aufgerufen (z.B. Fracht, Preis, …).  
   
Die Übergabeparameter an die Prozedur sind folgende:  
in in_ktrid integer default 0  
,in in_klasse integer default 1  
,in in_preis numeric(15,4) default 0  
,in in_fracht numeric(15,4) default 0  
,in in_handling numeric(15,4) default 0  
,in in_matif numeric(15,4) default 0  
,in in_anwwert numeric(15,4) default 0  
,in in_future numeric(15,4) default 0  
,in in_hedgeartikelid integer default 0  
,in in_hedgemonat char(10) default ''  
Das Resultset der Prozedur ist wie folgt:  
wert numeric(15,4)  
,bezeichnung char(40) |
