# Produktion

<!-- source: https://amic.de/hilfe/_FRZ_Produktion.htm -->

| Feld | Beschreibung |
| --- | --- |
| Komponentenzeilenprozedur | Im Produktionssystem kann über eine Prozedur die automatische Rezeptur-Mengenverteilung der Komponenten mit dieser Prozedur übersteuert werden. Als Beispiel wird folgender Prozedur mitgeliefert:  
create procedure ProduktionsZeilen ( in in_status_vorgang integer, in in_produktartikelnummer char(40), in_Gesamtmenge numeric(15,4), in in_zeile integer default 1, in in_artikelId integer, in in_menge numeric(15,4) )  
result ( menge numeric(15,4) )  
begin  
 declare dc_diesenr char(40);  
 if ( in_status_vorgang &lt;> 2 ) then  
 if ( in_Gesamtmenge &lt;> 0 ) then  
 select first artikelnummer into dc_diesenr from artikel where artikelid \= in_artikelid and substring(artikelnummer,2,3)=substring(in_produktartikelnummer,2,3);  
 if dc_diesenr is not NULL then  
 select 5;  
 else  
 select NULL;  
 end if;  
 else  
 select NULL from dummy where 1\=2;  
 end if;  
 else  
 select NULL from dummy where 1\=2;  
 end if;  
end |
| Gridbreite in PIXEL | Die Breite der drei Grids in der Produktionserfassungsmaske kann mit diesem Wert festgelegt werden, insbesondere kann hierdurch bei großen Bildschirmen eine bessere Anzeige der Informationen erreicht werden. |
| Itembox Artikel Zugang | Hier kann eine alternative Itembox für die Zugangsartikel hinterlegt werden. |
| Itembox Artikel Abgang | Hier kann eine alternative Itembox für die Abgangsartikel hinterlegt werden. |
| Partie angeben | Ist im Komponenten Grid die Partienummer eingebbar? Ja/Nein  
Dieses Kennzeichen kann durch die Partieführung aus dem Artikel, Tab - Reiter Partie „Behandlungs Kz ausw.“ mit „Ja“ übersteuert werden. |
