# Kasse

<!-- source: https://amic.de/hilfe/kasse.htm -->

Hauptmenü > Administration > Formulare / Abläufe > Formularzuordnung/Vorgansunterklasse > Register Kasse

Direktsprung **[FRZ]**

Auf dem Register Kasse werden die AIS-Gruppen und Felder dem Kassenmodul zugeordnet. Dabei ist zu beachten, dass hier nicht pro Formularzuordnung, sondern pro AIS-Gruppe die Zuordnung erfolgt. Ändert man hier also z.B. für die der Unterklasse 1 zugeordneten AIS-Gruppe die Feldzuordnung, so gilt dies für alle Formularzuordnungen, die dieselbe AIS-Gruppe verwenden. Um einen Überblick zu haben, wo diese Gruppe überall verwendet wird, existiert ein Anzeigegrid, in der alle Klasse/Unterklassen angezeigt werden, in der die AIS-Gruppen verwendet wird.

Da das System nicht weiß, wie die AIS-Feldnamen vom Anwender vergeben wurden, es jedoch wissen muss, wo die zu verarbeitenden Daten dargestellt werden sollen, muss eine Zuordnung erfolgen. Es wird dabei zwischen Pflicht und Zusatzfeldern unterschieden. Die Pflichtfelder sind das Minimum, welches angegeben werden muss.

Diese Zuordnung kann auch halb-automatisch erfolgen.  
Siehe: [Einrichtung der AIS-Felder für die Marktkasse aus dem AMIC Muster](./einrichtung_der_ais_felder_fuer_die_marktkasse_aus_dem_amic_.md).

| Feld | Beschreibung |
| --- | --- |
| AIS-Gruppe | Hier muss die AIS-Gruppe eingetragen werden, in der die Definition der Kasse steht oder zu der die Definition erstellt werden soll. Eine Auswahl aller AIS-Gruppen kann mit F3 aufgerufen werden. Wird eine Gruppe ausgewählt, für die bereits eine Kassen-Definition existiert, so wird diese sofort geladen. |
| Pflichtfeld Kassenfeldname | Es wird in der Kasseneinrichtung zwischen Pflichtfeldern und Zusatzfeldern unterschieden. Pflichtfelder sind z.B. Artikelnummer, Menge und Preis. |
| Pflichtfeld AIS-Feldname | Hier wird der frei wählbare AIS-Feldname eingetragen. Diese können mit F3 ausgewählt werden. Zusätzlich werden dann noch – falls es ein Feld in einem Grid ist - das AIS-Grid und die tatsächliche Gruppe - falls die AIS-Gruppe aus mehreren Gruppen besteht – angezeigt. |
| Zusatzfeld Kassenfeldname | Hier stehen alle Felder, die zusätzlich angezeigt werden können. |
| Zusatzfeld AIS-Feldname | Die Zuordnung eines AIS-Feldnamens ist für Zusatzfelder freigestellt. |

<p class="just-emphasize">Zusatzangaben</p>

<p class="just-emphasize">Leergutverarbeitung</p>

Gibt an, wie Leergut in der Kasse verarbeitet werden soll.

| Feld | Beschreibung |
| --- | --- |
| Kein Leergut | Leergut wird über die normale Artikelerfassung (F4) fakturiert. |
| Leergut nur für fakturierte Artikel | Es wird nur Leergut für zu fakturierende Artikel im Leergutdialog angezeigt. Leergut als Folgeartikel wird bei der normale Artikelerfassung übersprungen. |
| Alle Leergutartikel anzeigen | Es werden alle zu fakturierende Leergutartikel im Leergutdialog angezeigt. Leergut als Folgeartikel wird bei der normale Artikelerfassung übersprungen. |

<p class="just-emphasize">Kassenbelegdruck sofort</p>

Gibt an, on der Druck des Kassenbelegs sofort bei der Erfassung oder beim Abschluss der Zahlung erfolgen soll.

<p class="just-emphasize">Externes Kassendisplay</p>

Wird in einem externen Kassendisplay ein BON eingerichtet, so wird dieses Formular dazu verwendet, diesen Bon darzustellen. Wird kein Formular eingetragen, so wird das Druck-Formular verwendet.

<p class="just-emphasize">Unterklasse Lieferschein und Unterklasse Rechnung</p>

Diese beiden Felder werden herangezogen, wenn man bei der Marktkasse die Dialogmaske ‚Speichern unter‘ zum Wechsel in eine Rechnung oder in einen Lieferschein aufgefordert wird. 

<p class="just-emphasize">Prozedur Verkaufsbeschränkung</p>

Hier kann eine Prozedur eingefügt werden, welche die Standardbehandlung übersteuert. Als Übergabeparameter benötigt die Prozedur die „Vekaufsbeschränkungsnummer“. Der Rückgabeparameter heißt „meldung“.

Wenn die Meldung leer ist, wird der Vorgang durchgelassen. Steht in der Meldung ein Text, so wird dieser angezeigt und der Vorgang wird nicht abgeschlossen.

```sql
CREATE PROCEDURE
p_verkaufsbeschraenkungMitMeldung ( in in_typ integer)
--
BEGIN
   select 'Achtung! Verkaufsbeschränkung' as
meldung;
EXCEPTION
  when others then
    call amic_exception( ERRORMSG() ||
CHAR(10) || CHAR(13) || TRACEBACK(), SQLCODE , SQLSTATE ,
'p_verkaufsbeschraenkungMitMeldung' , -1 );
END
```

<p class="siehe-auch">Siehe auch:</p>

- [Einrichtung der AIS-Felder für die Marktkasse aus dem AMIC Muster](./einrichtung_der_ais_felder_fuer_die_marktkasse_aus_dem_amic_.md)
