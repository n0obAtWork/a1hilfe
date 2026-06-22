# Prozedur für Intrastat / Zollabwicklung

<!-- source: https://amic.de/hilfe/prozedurfrintrastatzollabwickl.htm -->

In dieses Feld kann eine private Prozedur eingetragen werden, welche Einfluss auf die beiden Felder HerkunftZielLand und HerkunftZielRegion in der Warenbewegung nimmt. Ist in diesem Feld keine Private Prozedur eingetragen, so wird die Vorbelegung aus den UFLD Feldern in der Vorgangserfassung Maske für jede einzelne Warenposition herangezogen. Die Vorbelegung für das Herkunft Ziel Land und Herkunft Ziel Region wird aus der Hauptadressen Anschrift des Kunden gezogen. Das Bundesland entspricht der Herkunft Ziel Region diese gilt aber nur für Deutschland. Das Land des Kunden entspricht Herkunft Ziel Region. In der Warenposition gibt es die Möglichkeit die Vorbelegung für die Felder manuell zu überschreiben.

Soll für die Bestimmung von Herkunft Ziel Land und Herkunft Ziel Region eine private Datenbankprozedur benutzt werden, so müssen einigen Richtlinien eingehalten werden.

```sql
create procedure
p_landregion (  in
in_ufld_land integer default 0,

in in_ufld_region integer default
null,

in in_wabew_land integer default 0,

in in_wabew_region integer default
null,

in in_ArtikelID integer,

in in_PartieID integer,

in in_KontraktID integer,

in in_Menge Numeric(15,4),

in in_KundID integer,

in in_LagerNummer integer

)

RESULT(

"HerkunftZielLand" integer,

"HerkunftZielRegion" integer

)
Begin
 declare dc_HerkunftZieLand
integer;
 declare dc_HerkunftZielRegion integer;
--Bestimmung des
HerkunftZielLand und HerkunftZielRegion
  select
dc_HerkunftZieLand as HerkunftZielLand, dc_HerkunftZielRegion as    HerkunftZielRegion from dummy;
END
```
