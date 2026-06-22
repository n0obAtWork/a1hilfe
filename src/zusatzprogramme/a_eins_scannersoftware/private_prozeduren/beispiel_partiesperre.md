# Beispiel Partiesperre

<!-- source: https://amic.de/hilfe/_cescannerpartiespe.htm -->

Hier finden Sie die Beispiel Prozedur für die Partie Sperre. Die Prozedur besteht aus dem Header und eine Abfrage, ob das Partiesperrkenz gesetzt ist. Die Funktion gibt 1 oder 0 zurück.

// Priv. Prozedur p_partie_sperre

//

// Beschreibung:

//

//

//

// Ausgabe 1 gescannte Partie wird gelöscht

// Ausgabe 0 Partie ist in Ordnung

```sql
CREATE PROCEDURE
p_partie_sperre (  in in_vklasse   integer,

       in in_vuklasse
integer,

in in_menge   numeric (15,6),

in in_me   integer,

in in_ArtikelId    integer,

       in in_KundId   integer,

in in_LagerNummer   integer,

in in_varengruppe    integer,

           in
in_EKVK   integer,

in in_PeriodisDato   date,

        in in_ArtiStammId
integer,

in in_partie   integer,

in in_KontraktId   integer,

in in_lagerplatz   integer,

in in_Belegnummer   integer,

 out dc_out integer)
BEGIN
   declare dc_partiesperrkenz integer;
   set dc_partiesperrkenz = 0;
   select partiesperrkennz into
dc_partiesperrkenz  from partiestamm
      where Lagernummer =
in_LagerNummer   and Partieid = in_partie;
   if dc_partiesperrkenz = 0 then
     set dc_out = 0;
   else
     set dc_out = 1;
   end if;
END
```
