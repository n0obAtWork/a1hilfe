# SQLK Nachweisvorlage

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_einr_sqlknachweis.htm -->

Diese Methode liefert einen einzeiligen Resultset zur Warenbewegung:

| Feld | Beschreibung |
| --- | --- |
| Label | 'aus nachhaltigem Anbau.‘ |
| Text1 | 'Zertifizierungssystem für Nachhaltigkeit:‘ + Text lt. af_zertmeth |
| Text2 | 'REDcert, Zertifikatsnummer: ' + eingerichteter Bemerkungstext |
| Text3 | 'Die Ware entspricht den Nachhaltigkeitsverordnungen (BioSt-NachV u. Biokraft-NachV)! ' |
| Text4 | 'Für die Berechnung der Treibhausgasbilanzierung soll der Standardwert verwendet werden‘ |
| Text5 | '(§ 8 u. Anlage 2 der Nachhaltigkeitsverordnungen). ' |
| Zertifizierung | Text lt. af_zertmeth |
| Zertifikatsbemerkung | Eingerichteter Bemerkungstext |
| Zertifikat_BLE | Normierte Zertifikatsausgabe gemäß BLE:  
DE-B-BLE-BM-10-100-20100009-00000001  
Stehend für Deutschland-Bund-BLE-Biomasse-  
Zertifizierungssystem(10=ISCC)-  
Zertifizierungsstelle-Zertifikatsnummer-Liefernummer;  
Zur Nutzung dieses Formats trägt man bitte alles bis zur Liefernummer in das Textfeld zum Zertifikat ein |
| Zustand | Integer Codierung entsprechend Format NACHHSTAT |
| THG Wert | Ermittelter THG Wert |
| THG Herkunft | 1= Kontrakt  
2=Warenbewegung  
3=Erzeuger (EK) / Mandant (VK)  
4=Artikel |

Das Label wird zur Darstellung in der Auswahlliste „Bewegungsübersicht“ verwendet.

Je nach Eingangsparametrisierung kann, die ist nachhaltig-Methode benutzt werden, um Nachhaltigkeitsinformation für verschiedene Zwecke zu erhalten.

```sql
create procedure
ist_nachhaltig
(
  in
in_wabewid        integer default 0
  ,in   in_wabewgruppe
integer default 90
  ,in
in_ArtikelId      integer default null
  ,in
in_KundId         integer default
null
  ,in
in_KtrId          integer default
null
  ,in   in_KtrArtiPosit
integer default null
  ,in
in_Klasse         integer default 0
  ,in
in_Date
date    default today()
)
RESULT (

zustand
integer

,farbe
integer

,label
char(255)

,text1
char(100)

,text2
char(100)

,text3
char(100)

,text4
char(100)

,text5
char(100)

,zertifizierungsmethode char(100)

,zertifikatsbemerkung   char(255)

,Zertifikat_BLE         char(255)

,ThgWert
numeric(15,4)

,ThgWertHerkunft        integer
      )
--
-- zustand über Format nachhstat
-- 0 = vererbt aus Stammsatz
(Kunde<->Artikel)
-- 1 = Ware ist nachhaltig (individuell gesetzt)
-- 2 = nicht nachhaltig (individuell gesetzt)
-- 3 = vererbt nicht nachhaltig
-- 4 = vererbt und nachhaltig
--
```

Nicht alle Kombinationen von Eingabeparametern sind sinnvoll. Anwendungsbereiche:

| Notation | Datenquelle; Verwendung |
| --- | --- |
| Ist_nachhaltig(in_wabewid=###) | Nachhaltigkeitsstatus für eine Warenbewegung; Formulardruck auf Basis gespeicherter Daten |
| Ist_nachhaltig(in_wabewid=NULL,  
 in_artikelid=###,  
 in_klasse=0) | Standard Nachhaltigkeit des Artikels im Verkauf;  
Info-Bildschirm, Formulardruck auf Basis nicht gespeicherter Daten |
| Ist_nachhaltig(in_wabewid=NULL,  
 in_artikelid=###,  
 in_klasse=1000,  
 in_KundId=###) | Standard Nachhaltigkeit des Artikels im Einkauf bei anzugebendem Lieferanten;  
Info-Bildschirm, Formulardruck auf Basis nicht gespeicherter Daten |
| Ist_nachhaltig(in_wabewid=NULL,  
 in_artikelid=###,  
 in_klasse=0,  
 in_ktrid=###) | Bestimmung der Nachhaltigkeit aus Kontrakt, ersatzweise Bestimmung Standard Verkauf;  
Info-Bildschirm, Formulardruck auf Basis nicht gespeicherter Daten |
| Ist_nachhaltig(in_wabewid=NULL,  
 in_artikelid=###,  
 in_klasse=1000,  
 in_KundId=###,  
 in_ktrid=###) | Bestimmung Nachhaltigkeit aus Kontrakt, ersatzweise Bestimmung Standard Lieferant;  
Info-Bildschirm, Formulardruck auf Basis nicht gespeicherter Daten |

Verwendungsbeispiel als SQLK:

```sql
select *
from ist_nachhaltig(:ID_WABEWID)
```
