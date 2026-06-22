# Schecks über Formulartyp 201 drucken

<!-- source: https://amic.de/hilfe/schecksberformulartyp201drucke.htm -->

Es existieren zu diesem Typ folgende Formularbereiche:

• 500 Kopf Scheckschreibung Formularkopf

• 502 Folgekopf Scheck Folgekopf

• 503 Positionszeile Scheck Zeilentyp

• 504 Alternativteil Scheckdruck Zeilentyp

• 508 Zwischenabschluss Scheck Fuß

• 510 Abschluss Scheck Abschluss  
    

Folgende Variablen sind in allen Teilen (Kopf, Fuß und Zeilentyp) verfügbar. Formularbereiche, die nicht separat mit aufgeführt werden, enthalten nur Festtext oder diese Felder!

| Bezeichnung | Typ | Nr. | Bedeutung |
| --- | --- | --- | --- |
| Datum | | 5 | Datum der Scheckschreibung. |
| Schecknummer | Num. | 4 | Fortlaufende Nummer des Schecks, die in der Maske erfasst wird. |
| Betrag | Num. | 4 | Gesamtbetrag in der Form 1.000,00 |
| Betragstern | Text | 3 | Gesamtbetrag in der Form \*\*\*\*\*\*1.000,00\* |
| Betragbuchst | Text | 3 | Gesamtbetrag in der Form /Eins/Null/Null/Null//Null/Null |
| Betragbuchst1 | Text | 3 | Wenn Betragbuchst nicht ausreicht, kann man hier eine zweite Zeile, in der dann der Rest steht, einrichten. |
| BetragbuchstOCX | Text | 3 | Wie Betragbuchst nur ohne Umlaute |
| BetragbuchstOCX1 | Text | 3 | Wie Betragbuchst1 nur ohne Umlaute |
| AbsKontoNummer | Num. | 4 | Kontonummer des Absenders also der Hausbank |
| AbsKontoName | Text | 3 | Name des Absenders |
| AbsBLZ | Num. | 4 | Bankleitzahl des Absenders |
| AbsBezeichnung | Text | 3 | Bezeichnung des Absenders |
| AbsSwift | Text | 3 | Swift/BIC des Absenders |
| AbsIBAN | Text | 3 | IBAN des Absenders |
| Adresse | Block | 6 | Adressfeld |
| KundenNummer | Num. | 4 | Kundennummer wie im Kundenstamm hinterlegt |
| GegenNummer | Text | 3 | Kundennummer beim Kunden wie im Kundenstamm hinterlegt |
| KundUStStatKennz | Text | 3 | UST.-Ident wie im Kundenstamm hinterlegt |
| EmpfKontoNummer | Text | 3 | Kontonummer des Empfängers |
| EmpfBankName | Text | 3 | Name der Bank des Empfängers |
| EmpfBLZ | Num. | 4 | Bankleitzahl des Empfängers |
| EmpfSwift | Text | 3 | Swift/BIC des Empfängers |
| EmpfIBAN | Text | 3 | IBAN des Empfängers |
| EmpfBezeich | Text | 3 | Entweder die Kundenbezeichnung oder, wenn im Kundenstamm hinterlegt, die abweichende Bezeichnung aus den Kundenbanken. |
| ZahlSumme | Num. | 4 | Gesamt Betrag |
| SkontoSumme | Num. | 4 | Gesamt Skonto Betrag |
| BelegSaldoSumme | Num. | 4 | Gesamt Betrag - gesamt Skonto Betrag |
| Anzahl | Num. | 4 | Anzahl Positionen auf dem Scheck |
| VWZBELEGNUMMER1 | Text | 4 | Nur im Bereich 500 bzw. 510. Textliche Ausgabe aller Belegnummern und Belegdaten in einer Zeile |
| VWZBELEGNUMMER2 | Text | 4. | Nur im Bereich 500 bzw. 510. Fortführung der textliche Ausgabe aller Belegnummern und Belegdaten in einer Zeile |
| VWZREFNUMMER1 | Text | 4 | Nur im Bereich 500 bzw. 510. Textliche Ausgabe aller Referenznummern und Belegdaten in einer Zeile |
| VWZREFNUMMER2 | Text | 4. | Nur im Bereich 500 bzw. 510. Fortführung der textliche Ausgabe aller Referenznummern und Belegdaten in einer Zeile |

503 Positionszeile Scheck

| Bezeichnung | Typ | Nr. | Bedeutung |
| --- | --- | --- | --- |
| ZahlungId | | | Interne Identifikation des Schecks |
| ZahlPosZaehler | | | Interne Identifikation der Scheckzeile |
| FiBuV_id | | | Interne Identifikation des gebuchten Zahlungsbelegs |
| ZahlPosBetrag | | | Betrag der Rechnung |
| ABS_ZahlPosBetrag | | | Betrag der Rechnung , jedoch immer mit positivem Betrag |
| ZahlPosSkonto | | | Skonto auf obigen Betrag |
| ABS_ZahlPosSkonto | | | Skonto auf obigen Betrag, jedoch immer mit positivem Betrag |
| BelegPosSaldo | | | Beleg Betrag - Beleg Skonto |
| ABS_BelegPosSaldo | | | abs (Beleg Betrag - Beleg Skonto), jedoch immer mit positivem Betrag |
| ZahlPosSH | | | Sollhabenkennzeichen dargestellt als ‘-‘ für Haben und ‘ ‘ für Soll,<br>kann jedoch per Einrichtungsparameter eingestellt werden. Mögliche Formate wären: MINUSPLUS, SH, SOLLHABEN |
| ABS_ZahlPosSH | | | Sollhabenkennzeichen dargestellt als ‘-‘ für Haben und ‘ ‘ für Soll,<br>kann jedoch per Einrichtungsparameter einstellbar. Bei negativen Beträgen (S.o.) werden diese Kennzeichen gedreht. Also:<br>\-1.000,00 S è 1.000,00 H bzw.<br>\-1.000,00 H è 1.000,00 S |
| FiBuV_Nummer | | | Interne Belegnummer |
| FiBuV_FremdNr | | | Externe Belegnummer ( Belegnummer des Lieferanten) |
| FiBuV_Datum | | | Belegdatum |
| FiBuVP_Text | | | Belegtext |
| FiBuVP_SkoDatum | | | Skonto Gültigkeitsdatum |
| FiBuVP_ValDatum | | | Fälligkeitsdatum |
| FiBuV_NumNummer | | | Numerischer Anteil der Belegnummer |
| NumKreisNummer | | | Nummernkreis |
| FilialNummerFil | | | Nummer der Filiale, falls hinterlegt |
| FilialNummerZen | | | Nummer der Zentrale, falls hinterlegt |
| Jahrnummer | | | Jahr, dem der Beleg zugeordnet ist |
| Perinummer | | | Nummer der Periode wie im Periodenstamm hinterlegt |
| Waehrnummer | | | Nummer der Währung in der der Beleg erfasst wurde. |
| KostStelNummer | | | Nummer der Kostenstelle |
| ZahlBedNummer | | | Zahlungsbedingung |
| FiBuV_Klasse | | | Nummer der Vorgangsklasse in der Finanzbuchhaltung (z.Zt. 1-15) |
| FibuV_KLKurzBez | | | Kurzform der Vorgangsklasse (z.B.: ZA ER AR.....) |
| FiBuV_KlBezeich | | | Bezeichnung der Vorgangsklasse ( z.B. Zahlung ....) |

504 Alternativteil Scheckdruck

Dieser Bereich wird nur gedruckt, wenn kein Folgekopf eingerichtet ist und die Anzahl der zu druckenden Zeilen kleiner ist als der vorhandene Platz. In diesem Bereich kann dann z.B. ein Festtext eingerichtet werden, in dem ein Hinweis auf die Avise, die dann gedruckt werden kann, steht. Die Avise wird nur dann erstellt, wenn dieser Bereich eingerichtet ist. Ist dieser Bereich nicht eingerichtet, wird dieser Scheck nicht gedruckt.
