# Zahlungssätze zu Kassenbelegen weichen ab

<!-- source: https://amic.de/hilfe/zahlungsstzezukassenbelegenwei.htm -->

Zu jedem Kassenbeleg (AcashBelg) können mehrere Zahlungssätze (AcashBelgZhlg) existieren, etwa wenn Skonto gewährt wurde oder Rückgeld ausgezahlt wird. Die Daten sind nicht stimmig, wenn Zahlungssätze zu Belegen fehlen oder umgekehrt es zu Zahlungssätzen keine Belege gibt oder aber in Zahlungssätzen und Belegen unstimmige Beträge gespeichert sind.

Zur Bereinigung gibt es keine maschinelle Unterstützung. Nachfolgende SQL Ausdrücke helfen, Fehlern auf die Spur zu kommen. Fehler werden individuell berichtigt.

Fehlende oder abweichende Zahlungen zu Belegen:

| 
```sql
select
       (select sum( zahlbetrag) from acashbelgzhlg z1

      where z1.zahlks=belegks and
      z1.zahlbelegid=belegid
         and
      z1.filialnummer=a.filialnummer and zahlart in (1,2,3,4,5,12)) gegeben,

       (select sum( zahlbetrag) from acashbelgzhlg z2

      where z2.zahlks=belegks and z2.zahlbelegid=belegid
         and
      z2.filialnummer=a.filialnummer and zahlart=10) zurueck,
       (select sum( zahlbetrag) from acashbelgzhlg z3

      where z3.zahlks=belegks and z3.zahlbelegid=belegid
         and
      z3.filialnummer=a.filialnummer and zahlart=11) skonto,
       isnull(gegeben,0) - isnull(zurueck,0)  as
      Betrag,
       if (gegeben is
      null and zurueck is null and belegsummebrutto != 0) then 'fehlt'

       else if
      abs(belegsummebrutto) != abs(Betrag) then 'abweichend' else '' endif endif
      watdenn,
       BelegKs,
      BelegKsi,
       cast(BelegDatum
      as date) Belegdatumdatum, BelegNr,
       (select
      FormLstBezeich from Formatlist

      where FormLstKennung = 'AcashBelegAr' and FormLstWert = Belegart

      and SprachNummer = 0) BelegArtBez,
       Belegart,

      BelegSummeBrutto, BelegKunde
     from AcashBelg a
     where watdenn != ''

     order by
      a.FilialNummer, a.BelegKs, a.BelegKsi, a.Belegart,
      a.Belegdatum
```

 |
| --- |

Fehlende Belege zu Zahlungssätzen:

| 
```sql
select
       ZahlKs, ZahlKsi,

       today(*)
      Belegdatumdatum, ZahlBelegNr,
       (select
      FormLstBezeich from Formatlist

      where FormLstKennung = 'AcashBelegAr' and FormLstWert = zahlBelegart

      and SprachNummer =0) BelegArtBez,
       zahlbelegart
      Belegart,
       zahlbetrag
      BelegSummeBrutto, filialnummer, Zahlkonto , *
from acashbelgzhlg z
where not exists
(select belegid from Acashbelg a
      where belegid=zahlbelegid and belegks=zahlks and
      a.filialnummer=z.filialnummer)
```

 |
| --- |
