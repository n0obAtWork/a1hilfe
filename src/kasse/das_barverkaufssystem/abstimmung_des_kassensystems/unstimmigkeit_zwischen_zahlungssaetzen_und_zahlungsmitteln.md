# Unstimmigkeit zwischen Zahlungssätzen und Zahlungsmitteln

<!-- source: https://amic.de/hilfe/unstimmigkeitzwischenzahlungss.htm -->

Zu jedem Zahlungssatz (AcashBelgZhlg) einer unbaren Zahlungsart (Zahlungsarten 2, 3, 4, 5) muss ein Zahlungsmittelsatz (AcashBelgZami) existieren.

Zur Bereinigung gibt es keine maschinelle Unterstützung. Nachfolgende SQL Ausdrücke helfen, Fehlern auf die Spur zu kommen. Fehler werden individuell berichtigt.

Fehlende oder abweichende Zahlungsmittel:

| 
```sql
select
       ZahlKs, ZahlKsi,

       today(*)
      Belegdatumdatum, ZahlBelegNr,
     (select zamibetrag from
      acashbelgzami where zamiidnr = zahlzamiidnr) zamibetrag,

       (select
      FormLstBezeich from Formatlist

      where FormLstKennung = 'AcashBelegAr' and FormLstWert = zahlBelegart

      and SprachNummer =0) BelegArtBez,
       zahlbelegart
      Belegart,
       zahlbetrag
      BelegSummeBrutto, filialnummer, Zahlkonto , *
from acashbelgzhlg z
where zahlart in (2,3,4,5) and
      (zamibetrag is null or zamibetrag != zahlbetrag)
```

 |
| --- |

Fehlende Zahlungssätze zu Zahlungsmitteln:

| 
```sql
select
       ZamiKs as
      BelegKs, ZamiKsi as BelegKsi,
       Zamidatum as BelegDatumDatum,ZamiBelegNr as BelegNr,

       (select
      FormLstBezeich from Formatlist

      where FormLstKennung = 'ZamiArt' and FormLstWert = ZamiArt

      and SprachNummer =0) BelegArtBez,
       ZamiArt as
      Belegart,
       zamibetrag
      BelegSummeBrutto, filialnummer
     from acashbelgzami z
     where zamiidnr not in
      (select zahlzamiidnr from acashbelgzhlg)

      order by FilialNummer, BelegKs,
      BelegKsi, Belegart
```

 |
| --- |
