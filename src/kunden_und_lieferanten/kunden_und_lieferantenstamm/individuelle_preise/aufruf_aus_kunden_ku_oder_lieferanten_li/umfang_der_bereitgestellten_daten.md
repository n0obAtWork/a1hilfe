# Umfang der bereitgestellten Daten

<!-- source: https://amic.de/hilfe/umfangderbereitgestelltendaten.htm -->

Von der mitgelieferten Ladeprozedur **HoleIndividuellePreiseKunde** werden nachfolgende Daten bereitgestellt:

| Tabellenspalte | Prozedurfeld | Feldtyp | Beschreibung |
| --- | --- | --- | --- |
| Artikel | ArtikelNummer | char(30) | Artikelnummer des Artikels |
| Artikelbezeichnung | ArtikelBezeich | char(255) | Bezeichnung des Artikels |
| Lager | lagernummer | integer | Lagernummer des Artikels |
| Warengruppe | Warengruppe | integer | Nummer der Warengruppe des Artikels |
| gültig ab | Datum | date | Gültig-Ab Datum des Individualpreises. Sollte das aktuelle Datum in mehr als einem Zeitraum enthalten sein, wird immer der Preis mit dem größten Gültig-Ab Datum herangezogen. |
| gültig bis | ArtiIndPrBisDat | date | Gültig-Bis Datum des Individualpreises. Muss für alle Einträge mit dem gleichen Gültig-Ab Datum identisch sein. Die Vorbelegung lässt sich in den Einrichterparametern pflegen. |
| ab Menge | ArtiIndAbMenge | numeric(15,4) | Ab welcher Menge der Preis für den Artikel gilt |
| pro Menge | PreisEinheit | numeric(15,4) | Preiseinheit: für wie viele Einheiten des Artikels der Preis gilt |
| Preis zum Datum 1 (individuelles Datum) | Preis1 | numeric(15,4) | Preis zum im Spaltenkopf angegebenen Datum |
| Preis zum Datum 2 (individuelles Datum) | Preis2 | numeric(15,4) | Preis zum im Spaltenkopf angegebenen Datum  
Standard: nächster folgender Montag  
Kann individuell eingerichtet werden. |
| Preis zum Datum 3 (individuelles Datum) | Preis3 | numeric(15,4) | Preis zum im Spaltenkopf angegebenen Datum  
Standard: übernächster folgender Montag  
Kann individuell eingerichtet werden. |
| individuelles gültig bis Datum | IndivPrBisDatum | date | Erlaubt das Gültig-Bis Datum für einen kompletten Gültigkeitszeitraum in Form eines Mengenänderns zu überschreiben. Wird hier ein neues gültig bis Datum eingetragen, ersetzt es das gültig bis Datum des kompletten Preisbandes. So können Preisbänder verlängert werden, wenn ein zusätzlicher Preispunkt aufgenommen werden soll. |
| ME-Nr. Menge | MENummerAbMenge | integer | Mengeneinheitsnummer und Bezeichnung der ab-Menge. Muss für alle Einträge mit dem gleichen Gültig-Ab Datum identisch sein. |
| ME-Bezeichnung | MENumAbMenBez | char(40) | Bezeichnung der ab-Menge. |
| ME-Nr. Preis | MENummer | integer | Mengeneinheitsnummer und Bezeichnung der pro-Menge. Muss für alle Einträge mit dem gleichen Gültig-Ab Datum identisch sein. |
| ME-Bezeichnung | MEBezeichnung | char(40) | Bezeichnung der pro-Menge. |
| Brutto-Kennzeichen | ArtiIndKennzBru | integer | F3 Auswahl Ja/Nein. Es handelt sich um einen Bruttopreis. Das Feld kann nur für den ersten Eintrag eines Preiszeitraums – gekennzeichnet durch „ab Menge“ 0,00 – geändert werden. Es wird dann für den gesamten Zeitraum geändert. |
| Steuergruppe | ArtiIndSteuerGruppe | integer | Insofern das Brutto-Kennzeichen auf Nein gesetzt wurde, also ein Netto-Preis vorliegt, kann hier eine bestehende Steuer-gruppe mittels F3 Auswahl hinterlegt werden. Muss für alle Einträge mit dem gleichen Gültig-Ab Datum identisch sein. |
| Empfohlener Verkaufspreis | empfVKPreis | numeric(15,4) | Empfohlener Verkaufspreis |
| Sperre | ArtiIndPrSperr | smallint | F3 Auswahl Ja/Nein: Möglichkeit der (vorübergehenden) Sperrung des Individualpreises. |
| Zu-/Abschlag | ArtiIndZuAbKennz | smallint | F3 Auswahl Ja/Nein: Einstellung, ob Zu-/Abschlägen trotz Individualpreises gezogen werden sollen. Die Einstellung gilt für alle Einträge unabhängig von Ab-Datum und Ab-Menge. |
| Rabatt | ArtiIndRabaKennz | smallint | F3 Auswahl Ja/Nein: Einstellung, ob Rabatte trotz Individualpreises gezogen werden sollen. Die Einstellung gilt für alle Einträge unabhängig von Ab-Datum und Ab-Menge. |
| Fracht | ArtiIndFraKennz | smallint | F3 Auswahl Ja/Nein: Einstellung, ob Frachten trotz Individualpreises gezogen werden sollen. Die Einstellung gilt für alle Einträge unabhängig von Ab-Datum und Ab-Menge. |
| Verpackung | ArtiIndVerpKennz | smallint | F3 Auswahl Ja/Nein: Einstellung, ob Verpackungskosten trotz Individualpreises gezogen werden sollen. Die Einstellung gilt für alle Einträge unabhängig von Ab-Datum und Ab-Menge. |
| Fremdartikel | liefartikelnummer | char(30) | Artikelnummer im Fremdsystem |
| Fremd-EAN | LiefEAN | char(40) | EAN Code im Fremdsystem |
| Bestellgröße | Bestellgroesse | numeric(15,4) | Ab welcher Menge der Artikel angeboten wird |

Viele dieser Felder lassen sich über die [Profileinstellungen](../preis_profile.md) aus- oder ein-blenden und können somit an die jeweiligen Erfordernisse angepasst werden.
