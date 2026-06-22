# AIS Felder auf dem Wiegeschein andrucken

<!-- source: https://amic.de/hilfe/_waage_addonandrucken.htm -->

Um AIS Felder auf dem Wiegeschein anzudrucken, verwendet man im [Formulareinrichter](../../../zusatzprogramme/formulareinrichtung_und_zuordnung/index.md) [FRM] für den Bereich 1000/Wiegekopf die Formulardruckposition 454/Zugriff auf OWAAGEADDON Daten. In der Spalte Text kann man dann mit F3 aus den Feldern im OwaageAddOn das Feld auswählen, welches man andrucken möchte. Siehe auch [Wiegeschein drucken](../funktionen_auf_der_waagenmaske/wiegeschein_drucken.md). Des Weiteren besteht die Möglichkeit, per SQLK sich Daten dazu zuladen. Der Name der ID, welche die OWaage_id enthält, heißt ID_OWAAGE_ID.

Beispiel:

```sql
select
 Kundnummer from OWaage as ow
 join Kundenstamm as ks on (ks.KundId =
ow.OWaage_Kundeid)
 where ow.OWaage_id =
:ID_OWAAGE_ID
```
