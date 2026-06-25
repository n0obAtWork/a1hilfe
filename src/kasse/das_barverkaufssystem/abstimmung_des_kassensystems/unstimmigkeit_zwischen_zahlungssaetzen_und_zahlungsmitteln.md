# Unstimmigkeit zwischen Zahlungssätzen und Zahlungsmitteln

<!-- source: https://amic.de/hilfe/unstimmigkeitzwischenzahlungss.htm -->

Zu jedem Zahlungssatz (AcashBelgZhlg) einer unbaren Zahlungsart (Zahlungsarten 2, 3, 4, 5) muss ein Zahlungsmittelsatz (AcashBelgZami) existieren.

Zur Bereinigung gibt es keine maschinelle Unterstützung. Nachfolgende SQL Ausdrücke helfen, Fehlern auf die Spur zu kommen. Fehler werden individuell berichtigt.

Fehlende oder abweichende Zahlungsmittel:

| <pre><code>select&#10; ZahlKs, ZahlKsi,&#10; today(*) Belegdatumdatum, ZahlBelegNr,&#10; (select zamibetrag from acashbelgzami where zamiidnr = zahlzamiidnr) zamibetrag,&#10; &#10; (select FormLstBezeich from Formatlist&#10; where FormLstKennung = 'AcashBelegAr' and FormLstWert = zahlBelegart&#10; and SprachNummer =0) BelegArtBez,&#10; zahlbelegart Belegart,&#10; zahlbetrag BelegSummeBrutto, filialnummer, Zahlkonto , *&#10;from acashbelgzhlg z&#10;where zahlart in (2,3,4,5) and (zamibetrag is null or zamibetrag != zahlbetrag)</code></pre> |
| --- |

Fehlende Zahlungssätze zu Zahlungsmitteln:

| <pre><code>select&#10; ZamiKs as BelegKs, ZamiKsi as BelegKsi,&#10; Zamidatum as BelegDatumDatum,ZamiBelegNr as BelegNr,&#10; (select FormLstBezeich from Formatlist&#10; where FormLstKennung = 'ZamiArt' and FormLstWert = ZamiArt&#10; and SprachNummer =0) BelegArtBez,&#10; ZamiArt as Belegart,&#10; zamibetrag BelegSummeBrutto, filialnummer&#10; from acashbelgzami z&#10; where zamiidnr not in (select zahlzamiidnr from acashbelgzhlg)&#10; order by FilialNummer, BelegKs, BelegKsi, Belegart</code></pre> |
| --- |
