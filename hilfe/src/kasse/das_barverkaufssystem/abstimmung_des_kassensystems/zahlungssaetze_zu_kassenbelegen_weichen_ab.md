# Zahlungssätze zu Kassenbelegen weichen ab

<!-- source: https://amic.de/hilfe/zahlungsstzezukassenbelegenwei.htm -->

Zu jedem Kassenbeleg (AcashBelg) können mehrere Zahlungssätze (AcashBelgZhlg) existieren, etwa wenn Skonto gewährt wurde oder Rückgeld ausgezahlt wird. Die Daten sind nicht stimmig, wenn Zahlungssätze zu Belegen fehlen oder umgekehrt es zu Zahlungssätzen keine Belege gibt oder aber in Zahlungssätzen und Belegen unstimmige Beträge gespeichert sind.

Zur Bereinigung gibt es keine maschinelle Unterstützung. Nachfolgende SQL Ausdrücke helfen, Fehlern auf die Spur zu kommen. Fehler werden individuell berichtigt.

Fehlende oder abweichende Zahlungen zu Belegen:

| <pre><code>select&#10; (select sum( zahlbetrag) from acashbelgzhlg z1&#10; where z1.zahlks=belegks and z1.zahlbelegid=belegid&#10; and z1.filialnummer=a.filialnummer and zahlart in (1,2,3,4,5,12)) gegeben,&#10; (select sum( zahlbetrag) from acashbelgzhlg z2&#10; where z2.zahlks=belegks and z2.zahlbelegid=belegid&#10; and z2.filialnummer=a.filialnummer and zahlart=10) zurueck,&#10; (select sum( zahlbetrag) from acashbelgzhlg z3&#10; where z3.zahlks=belegks and z3.zahlbelegid=belegid&#10; and z3.filialnummer=a.filialnummer and zahlart=11) skonto,&#10; isnull(gegeben,0) - isnull(zurueck,0) as Betrag,&#10; if (gegeben is null and zurueck is null and belegsummebrutto != 0) then 'fehlt'&#10; else if abs(belegsummebrutto) != abs(Betrag) then 'abweichend' else '' endif endif watdenn,&#10; BelegKs, BelegKsi,&#10; cast(BelegDatum as date) Belegdatumdatum, BelegNr,&#10; (select FormLstBezeich from Formatlist&#10; where FormLstKennung = 'AcashBelegAr' and FormLstWert = Belegart&#10; and SprachNummer = 0) BelegArtBez,&#10; Belegart,&#10; BelegSummeBrutto, BelegKunde&#10; from AcashBelg a&#10; where watdenn != ''&#10; order by a.FilialNummer, a.BelegKs, a.BelegKsi, a.Belegart, a.Belegdatum</code></pre> |
| --- |

Fehlende Belege zu Zahlungssätzen:

| <pre><code>select&#10; ZahlKs, ZahlKsi,&#10; today(*) Belegdatumdatum, ZahlBelegNr,&#10; (select FormLstBezeich from Formatlist&#10; where FormLstKennung = 'AcashBelegAr' and FormLstWert = zahlBelegart&#10; and SprachNummer =0) BelegArtBez,&#10; zahlbelegart Belegart,&#10; zahlbetrag BelegSummeBrutto, filialnummer, Zahlkonto , *&#10;from acashbelgzhlg z&#10;where not exists&#10;(select belegid from Acashbelg a where belegid=zahlbelegid and belegks=zahlks and a.filialnummer=z.filialnummer)</code></pre> |
| --- |
