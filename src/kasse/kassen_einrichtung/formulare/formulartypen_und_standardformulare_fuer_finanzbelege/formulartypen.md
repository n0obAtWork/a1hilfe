# Formulartypen

<!-- source: https://amic.de/hilfe/formulartypen.htm -->

###### Typ 10 (EC Lastschrift)

Lastschriftbestätigung

Formular 30 (Druck der EC-Lastschrift auf dem Schacht des Bondruckers)

(Bem.: die Nummer des Lastschriftformulars ist flexibel und wird in den Kasseneinstellungen in der Gruppe „Formulare“, Nummer 2 zugeordnet)

| Variablenname | Druckposition | Druckbereich | Bedeutung |
| --- | --- | --- | --- |
| EC_Firma | 3 TextVariable  
 | 950 Hauptteil EC_Lastschrift | Mandanten / Firmenname |
| EC_Betrag | 4 ZahlVariable  
6250 | 950 Hauptteil EC_Lastschrift | Betrag der Lastschrift in erfaßter Währung |
| EC_Waehrung | 3 TextVariable  
6251 | 950 Hauptteil EC_Lastschrift | Währungskürzel, in der Lastschrift erfaßt wurde |
| EC_Datum | 11 Tagesdatum  
6252 | 950 Hauptteil EC_Lastschrift | Tagesdatum, an dem Lastschrift erfaßt wurde |
| EC_Zeit | 3 TextVariable  
6253 | 950 Hauptteil EC_Lastschrift | Uhrzeit, an der Lastschrift erfaßt wurde |
| EC_KartNr | 3 TextVariable  
6254 | 950 Hauptteil EC_Lastschrift | Kartennummer der EC_Karte |
| EC_KontoNr | 3 TextVariable  
6255 | 950 Hauptteil EC_Lastschrift | Kontonummer der EC_Karte |
| EC_IBAN | 3 TextVariable  
6263 | 950 Hauptteil EC_Lastschrift | IBAN der EC_Karte |
| EC_BLZ | 3 TextVariable  
6256 | 950 Hauptteil EC_Lastschrift | Bankleitzahl der EC_Karte |
| EC_BIC | 3 TextVariable  
6264 | 950 Hauptteil EC_Lastschrift | BIC der EC_Karte |
| EC_GueltigBis | 3 TextVariable  
6257 | 950 Hauptteil EC_Lastschrift | Gültigkeitsdatum der EC_Karte |
| EC_BelegNr | 4 ZahlVariable  
6258 | 950 Hauptteil EC_Lastschrift | Referenz auf die Belegnummer, bei dem mit Karte bezahlt wurde |
| EC_BonNr | 4 ZahlVariable  
6259 | 950 Hauptteil EC_Lastschrift | Laufende Ident-Nummer des Zahlungsmittelsatzes |
| EC_Kasse | 4 ZahlVariable  
6260 | 950 Hauptteil EC_Lastschrift | Nummer der Kasse, an der mit dieser Karte gezahlt wurde |

###### Typ 49 (Scheckdruck)

Formular -50 / Scheckdruck

(Bem.: die Nummer des Scheckformulars ist flexibel und wird in den Kasseneinstellungen in der Gruppe „Formulare“, Nummer 1 zugeordnet)

| **Variablenname** | **Druckposition** | **Druckbereich** | **Bedeutung** |
| --- | --- | --- | --- |
| BelegBetrag | 4 ZahlVariable  
6382 | [822 Kasse Positionen](./druckbereiche.md) | Der Betrag der Scheckzahlung in gewählter Währung |
| BelegWaehrung | 3 TextVariable  
6381 | [822 Kasse Positionen](./druckbereiche.md) | Die gewählte Währung des Schecks |
| BelegDatum | 5 DatumVariable  
6203 | 822 Kasse Positionen | Das Tagesdatum |
| BelegKunde | 3 TextVariable  
6370 | 822 Kasse Positionen | Der eingegebene Kundenname |
| SQLK_BELEGBETRAG,Wortbetrag | 9 SQLVariable | 822 Kasse Positionen | Der Belegbetrag in Worten |

###### Typ 51 (Kassenformular Finanzbeleg)

Das Kassenformular verfügt über folgende Druckbereiche:

| **Bereich** | **Bezeichnung** | **Anmerkung** |
| --- | --- | --- |
| 820 | Kassenkopf | Einmaliger Druck |
| 822 | Kasse Positionen | Iterierter Druck über mehrere Positionen falls vorhanden |
| 823 | Kasse Zahlungen | Iterierter Druck über alle Zahlungswege |
| 824 | Kasse Zahlungsmittel | Optionaler Zusatz zu 823 zum Ausdruck der Bankverbindung etwa einer EC Lastschrift |
| 828 | Einleitung Positionsteil | Optionaler Bereich zur optischen Abtrennung des Kopfes gegen den Positionsteil |
| 829 | Einleitung Zahlungswege | Optionaler Bereich zur optischen Abtrennung des Positionsteils gegen die Zahlungswege |
| 860 | Kassenfuß | |

Druckreihenfolge:

820 Kopf

828 Einleitung Positionsteil

822 Positionsteil (Iteriert über alle Positionen des Finanzbelegs)

829 Einleitung Zahlungswege

Iteriert über alle Zahlungen 823 Zahlungen und optional 824 Zahlungsmittel als Zusatzbereich

860 Fuß

Formular -51 / Geldeinzahlung/Geldauszahlung/Geldentnahme

Siehe Einrichtung der Druckbereiche 820, 822, 823, 824, 828,829, 860. Zusätzlich können für Entnahmen die folgenden Positionen eingerichtet werden:

| **Variablenname** | **Druckposition** | **Druckbereich** | **Bedeutung** |
| --- | --- | --- | --- |
| SteuerSatz | 4 ZahlVariable  
6300 | 822 Kasse Positionen | Optional bei Entnahmen wird der Steuersatz der Entnahme gedruckt |
| Steuertext | 3 TextVariable  
6301 | 822 Kasse Positionen | Optional bei Entnahmen wird der Festtext „Steuersatz:“ gedruckt |
| Steuersumme | 4 ZahlVariable  
6302 | 822 Kasse Positionen | Optional bei Entnahmen wird die Steuersumme gedruckt |
| Steuersummetext | 3 TextVariable  
6303 | 822 Kasse Positionen | Optional bei Entnahmen wird der Festtext „Steuersumme:“ gedruckt |
| Nettosumme | 4 ZahlVariable  
6304 | 822 Kasse Positionen | Optional bei Entnahmen wird die Nettosumme gedruckt |
| Nettosummetext | 3 TextVariable  
6305 | 822 Kasse Positionen | Optional bei Entnahmen wird der Festtext „Nettosumme:“ gedruckt |

Formular -52 / Einreichung

Siehe Einrichtung der Druckbereiche 820, 822, 823, 824, 828, 829, 860.

Im Positionsteil 822 können die folgenden Positionen zusätzlich eingerichtet werden:

| **Variable** | **Druckposition** | **Drucktyp** | **Bedeutung** |
| --- | --- | --- | --- |
| BankBezeich | 3 TextVariable  
6310 | 822 Kasse Positionen | Bankbezeichnung, an die eingereicht wird |
| BankBLZ | 3 TextVariable  
6311 | 822 Kasse Positionen | Bankleitzahl der Bank, an die eingereicht wird |
| BankBIC | 3 TextVariable  
6315 | 822 Kasse Positionen | BIC der Bank, an die eingereicht wird |
| BankBem | 3 TextVariable  
6312 | 822 Kasse Positionen | Bemerkungstext zu dieser Einreichung |
| BankKonto | 3 TextVariable  
6313 | 822 Kasse Positionen | Nummer des Kontos bei der Bank, auf das eingereicht wird |
| BankIBAN | 3 TextVariable  
6314 | 822 Kasse Positionen | IBAN des Kontos bei der Bank, auf das eingereicht wird |

Formular -53 / Sortenwechsel

Siehe Einrichtung der Druckbereiche 820, 822, 860.

Im Positionsteil 822 können die folgenden Positionen zusätzlich eingerichtet werden:

| **Variable** | **Druckposition** | **Drucktyp** | **Bedeutung** |
| --- | --- | --- | --- |
| SorwZamiNr  
 | 3 TextVariable  
6320 | 822 Kasse Positionen | Zahlungsart-abhängig: die eingegebene Nummer des zu wechselnden Zahlungsmittels |
| SorwBank | 3 TextVariable  
6322 | 822 Kasse Positionen | Zahlungsart-abhängig: der Bankname des zu wechselnden Zahlungsmittels lt. Eingabe |
| SorwKonto | 3 TextVariable  
6323 | 822 Kasse Positionen | Zahlungsart-abhängig: Bankkontonummer des zu wechselnden Zahlungsmittels lt. Eingabe |
| SorwIBAN | 3 TextVariable  
6333 | 822 Kasse Positionen | IBAN des zu wechselnden Zahlungsmittels lt. Eingabe |
| SorwBLZ | 3 TextVariable  
6321 | 822 Kasse Positionen | Zahlungsart-abhängig: die BLZ des zu wechselnden Zahlungsmittels lt. Eingabe |
| SorwBIC | 3 TextVariable  
6334 | 822 Kasse Positionen | BIC des zu wechselnden Zahlungsmittels lt. Eingabe |
| SorwBem | 3 TextVariable  
6324 | 822 Kasse Positionen | Zahlungsart-abhängig: Eingegebene Bemerkung des Sortenwechsels |
| SorwKunde | 3 TextVariable  
6325 | 822 Kasse Positionen | Zahlungsart-abhängig: Eingegebener Kundenname |
| ZahlartGegeben | 3 TextVariable  
6329 | 822 Kasse Positionen | Zahlungsart des wechselnden Zahlungsmittels |
| BetragGegeben | 4 ZahlVariable  
6328 | 822 Kasse Positionen | Betrag der Wechselsumme in gewählter Währung |
| WaehrungGegeben | 3 TextVariable  
6327 | 822 Kasse Positionen | Währungskürzel der gewählten Währung |
| ZahlartZurueck | 3 TextVariable  
6332 | 822 Kasse Positionen | Zahlungsart des zurückgegebenen Zahlungsmittels |
| BetragZurueck | 4 ZahlVariable  
6331 | 822 Kasse Positionen | Betrag der Wechselsumme in Rückgabewährung |
| WaehrungZurueck | 3 TextVariable  
6330 | 822 Kasse Positionen | Rückgabewährung |

Formular -54 / Zahlungsmeldungen

Siehe Einrichtung der Druckbereiche 820, 822, 823, 824, 828, 829, 860.

Im Positionsteil 822 können die folgenden Positionen zusätzlich eingerichtet werden:

| **Variable** | **Druckposition** | **Drucktyp** | **Bedeutung** |
| --- | --- | --- | --- |
| ZameText | 3 TextVariable  
6340 | 822 Kasse Positionen | Text einer einzelnen Zahlungsmeldung |
| ZameWaehrung | 3 TextVariable  
6341 | 822 Kasse Positionen | Währungskürzel der Buchwährung |
| ZameBetrag | 4 ZahlVariable  
6342 | 822 Kasse Positionen | Einzelbetrag der jeweiligen Zahlungsmeldung |

Formular -57 / Geldübergabe / Geldübernahme

Siehe Einrichtung der Druckbereiche 820, 823, 824, 829, 860.

Formular -55 / Zahlungsmittel storniert /umgewandelt

Siehe Einrichtung der Druckbereiche 820, 822, 828, 860.

Im Positionsteil 822 können die folgenden Positionen zusätzlich eingerichtet werden:

| **Variablenname** | **Druckposition** | **Druckbereich** | **Bedeutung** |
| --- | --- | --- | --- |
| Zahlart | 3 TextVariable  
6356 | 822 Kasse Positionen | Zahlungsart vor Zahlungsmittelumwandlung |
| ZamiNr | 3 TextVariable  
6357 | 822 Kasse Positionen | Nummer des Zahlungsmittels vor Umwandlung |
| BLZ | 3 TextVariable  
6358 | 822 Kasse Positionen | Bankleitzahl des Zahlungsmittels vor Umwandlung |
| BIC | 3 TextVariable  
6361 | 822 Kasse Positionen | BIC des Zahlungsmittels vor Umwandlung |
| Kontonummer | 3 TextVariable  
6359 | 822 Kasse Positionen | Kontonummer des Zahlungsmittels vor Umwandlung |
| IBAN | 3 TextVariable  
6360 | 822 Kasse Positionen | IBAN des Zahlungsmittels vor Umwandlung |
| ZahlartNeu | 3 TextVariable  
6350 | 822 Kasse Positionen | Zahlungsart nach Zahlungsmittelumwandlung |
| ZamiNrNeu | 3 TextVariable  
6351 | 822 Kasse Positionen | Nummer des Zahlungsmittels nach Umwandlung |
| BLZNeu | 3 TextVariable  
6352 | 822 Kasse Positionen | Bankleitzahl des Zahlungsmittels nach Umwandlung |
| BICneu | 3 TextVariable  
6363 | 822 Kasse Positionen | BIC des Zahlungsmittels nach Umwandlung |
| KontonummerNeu | 3 TextVariable  
6353 | 822 Kasse Positionen | Kontonummer des Zahlungsmittels nach Umwandlung |
| IBANneu | 3 TextVariable  
6362 | 822 Kasse Positionen | IBAN des Zahlungsmittels nach Umwandlung |
| Zahlbetrag | 4 ZahlVariable  
6354 | 822 Kasse Positionen | Betrag des geänderten Zahlungsweges |
| ZahlWaehrung | 3 TextVariable  
6355 | [822 Kasse Positionen](./druckbereiche.md) | Währungskürzel des geänderten Zahlungsweges |

###### Typ 50 (Kassensturzformular)

Basisformular -56 / Kassensturz

Das Kassenformular verfügt über folgende Druckbereiche:

| **Bereich** | **Bezeichnung** | **Anmerkung** |
| --- | --- | --- |
| 820 | Kassenkopf | Wie Formulartyp 51 |
| 823 | Kassen Zahlungen | Wie Formulartyp 51 |
| 828 | Einleitung Positionsteil | Wie Formulartyp 51 |
| 829 | Einleitung Zahlungen | Wie Formulartyp 51 |
| 850 | Kassensturz Stückelung | |
| 851 | Kassensturz Bareingänge | |
| 852 | Kassensturz Zami Eingänge | |
| 853 | Kassensturz Barausgänge | |
| 854 | Kassensturz Zami Ausgänge | |
| 855 | Kassensturz Stornobeträge | |
| 856 | Kassensturz Zählungssumme | |
| 859 | Kassensturz Einleitung Unterbericht | |
| 860 | Kassenfuß | Wie Formulartyp 51 |

Druckreihenfolge:

1. 820 Kopf

2. 828 Einleitung Positionsteil

3. 850 Stückelung

4. Iteriert über alle Zahlungen 823 Zahlungen und optional 824 Zahlungsmittel als Zusatzbereich

5. 856 Zählungssumme

6. Summen Bargeldeingänge 851 mit vorangehendem Unterberichtstitel 859

7. Summen Zahlunsgmitteleingänge 852 mit vorangehendem Unterberichtstitel 859

8. Summen Bargeldausgänge 853 mit vorangehendem Unterberichtstitel 859

9. Summen Zahlunsgmittelausgänge 854 mit vorangehendem Unterberichtstitel 859

10. Stornosummen 855 mit vorangehendem Unterberichtstitel 859

11. 860 Fuß
