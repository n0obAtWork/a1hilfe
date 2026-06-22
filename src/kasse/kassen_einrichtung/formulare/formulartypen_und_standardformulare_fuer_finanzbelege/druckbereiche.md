# Druckbereiche

<!-- source: https://amic.de/hilfe/druckbereiche.htm -->

###### Druckbereich 820 / Kassenkopf

| **Variablenname** | **Druckposition** | **Bedeutung** |
| --- | --- | --- |
| Kasse | 4 ZahlVariable<br>6200 | Kassennummer |
| Kassierer | 3 TextVariable<br>6207 | Userkürzel |
| Kassierername | 3 TextVariable<br>6208 | Name des Bedieners lt. Bedienerstamm |
| BelegArt | 3 TextVariable<br>6205 | Belegarttext |
| BelegDatum | 5 DatumVariable<br>6203 | Das Belegdatum |
| BelegNr | 4 ZahlVariable<br>6204 | Die Belegnummer |
| BelegId | 4 ZahlVariable<br>6202 | Beleg ID |
| Sitzung | 4 ZahlVariable<br>6206 | Kassensitzungsnummer |
| Filialnummer | 4 ZahlVariable<br>6201 | Filialnummer |
| FilialBezeich | 3 TextVariable<br>6209 | Die Bezeichnung der Filiale |
| FilialStrasse | 3 TextVariable<br>6210 | Die Straße der Filiale |
| FilialPLZ | 3 TextVariable<br>6212 | Die Postleitzahl der Filiale |
| FilialOrt | 3 TextVariable<br>6211 | Der Ort der Filiale |
| MandantBezeich | 3 TextVariable<br>6221 | Die Bezeichnung des Mandanten |
| MandantStrasse | 3 TextVariable<br>6222 | Die Straße des Mandanten |
| MandantPLZ | 3 TextVariable<br>6223 | Die Postleitzahl des Mandanten |
| MandantOrt | 3 TextVariable<br>6224 | Der Ort des Mandanten |
| Kopie | 3 TextVariable<br>6226 | Bei Wiederholungsdruck wird der Beleg als Kopie markiert |
| Storno | 3 TextVariable<br>6225 | Bei Stornierung wird der Beleg als Storno markiert |
| BelegName | 3 TextVariable<br>6217 | Der Name des Kunden bzw. Kontobezeichnung je nach Belegart |
| BelegText | 3 TextVariable<br>6213 | Bemerkungstext |
| BelegBetrag | 4 ZahlVariable<br>6215 | Die Belegsumme in Buchwährung |
| BelegKWBetrag | <br> | Die Belegsumme in Kassenwährung (derzeit nicht unterstützt) |
| BelegWaehrung | 3 TextVariable<br>6214 | Das Währungskürzel der Buchwährung |
| BelegKW | <br> | Das Währungskürzel der Kassenwährung (derzeit nicht unterstützt) |
| Konto | 4 ZahlVariable<br>6218 | Kontonummer Kunde/Konto je nach Belegart |
| Ort | 3 TextVariable<br>6219 | Wohnort des Kunden lt. Anschriftstamm, falls kundenbezogener Beleg |
| PLZ | 3 TextVariable<br>6220 | PLZ des Kunden lt. Anschriftstamm, falls kundenbezogener Beleg |
| Gegenkasse | 4 ZahlVariable<br>6216 | Kassennummer der Gegenkasse, belegartabhängig |
| | | | | |

###### Druckbereich 822 / Kasse Positionen

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| BelegName | 3 TextVariable<br>6217 | Der Name des Kunden bzw. Kontobezeichnung je nach Belegart |
| BelegText | 3 TextVariable<br>6213 | Bemerkungstext |
| BelegBetrag | 4 ZahlVariable<br>6215 | Die Belegsumme in Buchwährung |
| BelegKWBetrag | <br> | Die Belegsumme in Kassenwährung (derzeit nicht unterstützt) |
| BelegWaehrung | 3 TextVariable<br>6214 | Das Währungskürzel der Buchwährung |
| BelegKW | <br> | Das Währungskürzel der Kassenwährung (derzeit nicht unterstützt) |
| Konto | 4 ZahlVariable<br>6218 | Kontonummer Kunde/Konto je nach Belegart |
| Ort | 3 TextVariable<br>6219 | Wohnort des Kunden lt. Anschriftstamm, falls kundenbezogener Beleg |
| PLZ | 3 TextVariable<br>6220 | PLZ des Kunden lt. Anschriftstamm, falls kundenbezogener Beleg |

Hinweis: die oben aufgeführten Druckpositionen können wahlweise im Kopf oder im Positionsteil gedruckt werden. Eine Einzahlung etwa besitzt nur eine Positionszeile. Hier sollten diese Variablen im Positionsteil eingerichtet werden, denn dessen Einrichtung soll nicht leer sein. Bei einer Zahlungsmeldung hingegen wird im Positionsteil die Begleichung mehrerer Kreditrechnung angezeigt. Hier wäre eine Einrichtung im Kopfteil sinnvoll, damit etwa die Gesamtsummen nicht mehrfach je Zeile gedruckt werden.

Alle weiteren Druckpositionen dieses Bereiches sind Belegart-spezifisch und in der Beschreibung der jeweiligen Basisformulare aufgelistet.

###### Druckbereich 823 / Kasse Zahlungen

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Zahlungsart des Zahlungsweges |
| Waehrung | 3 TextVariable<br>6381 | Buchwährung |
| Zahlungsbetrag | 4 ZahlVariable<br>6382 | Betrag des Zahlungsweges in Buchwährung |
| ZamiId | 4 ZahlVariable<br>6383 | Interne Id Nummer eines Zahlungsmittels, 0 wenn bar |
| ZamiBlz | 3 TextVariable<br>6384 | Zahlart-abhängig die BLZ zum Zahlungsweg |
| ZamiBIC | 3 TextVariable<br>6391 | Zahlart-abhängig die BIC zum Zahlungsweg |
| ZamiKonto | 3 TextVariable<br>6385 | Zahlart-abhängig die Bankkontonummer des Zahlungswegs |
| ZamiIBAN | 3 TextVariable<br>6390 | Zahlart-abhängig die IBAN des Zahlungswegs |
| ZamiNummer | 3 TextVariable<br>6386 | Zahlart-abhängig die Nummer des Zahlungsmittels (z.B. Schecknummer, Gutscheinnummer) |
| ZamiKasse | 4 ZahlVariable<br>6388 | Kasse, an der das Zahlungsmittel vereinnahmt wurde |
| ZamiSitzung | 4 ZahlVariable<br>6389 | Zugehörige Sitzung, in deren Verlauf des Zahlungsmittel vereinnahmt wurde |
| ZamiBelegNr | 4 ZahlVariable<br>6387 | Nummer des Belegs, der mit diesem Zahlungsmittel beglichen wurde |

###### Druckbereich 824 / Kasse Zahlungsmittel

Mittels des Druckbereichs 824 wird die Möglichkeit gegeben, eine Zusatzzeile zu 823 zu drucken. Wird z.B. in 823 der Zahlungsweg EC ausgewiesen, so kann speziell zu diesem Zahlungsweg auch die Bankverbindung gedruckt werden. 824 wird unterdrückt, wenn BLZ, Kontonummer und ZamiNummer alle leer sind (also z.B. bei Barzahlungen oder EC Zahlungen am Bezahlterminal).

| | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Zahlungsart des Zahlungsweges |
| Waehrung | 3 TextVariable<br>6381 | Buchwährung |
| Zahlungsbetrag | 4 ZahlVariable<br>6382 | Betrag des Zahlungsweges in Buchwährung |
| ZamiId | 4 ZahlVariable<br>6383 | Interne Id Nummer eines Zahlungsmittels, 0 wenn bar |
| ZamiBlz | 3 TextVariable<br>6384 | Zahlart-abhängig die BLZ zum Zahlungsweg |
| ZamiBIC | 3 TextVariable<br>6391 | Zahlart-abhängig die BIC zum Zahlungsweg |
| ZamiKonto | 3 TextVariable<br>6385 | Zahlart-abhängig die Bankkontonummer des Zahlungswegs |
| ZamiIBAN | 3 TextVariable<br>6390 | Zahlart-abhängig die IBAN des Zahlungswegs |
| ZamiNummer | 3 Textvariable<br>6386 | Zahlart-abhängig die Nummer des Zahlungsmittels (z.B. Schecknummer, Gutscheinnummer) |
| ZamiKasse | 4 ZahlVariable<br>6388 | Kasse, an der das Zahlungsmittel vereinnahmt wurde |
| ZamiSitzung | 4 ZahlVariable<br>6389 | Zugehörige Sitzung, in deren Verlauf des Zahlungsmittel vereinnahmt wurde |
| ZamiBelegNr | 4 ZahlVariable<br>6387 | Nummer des Belegs, der mit diesem Zahlungsmittel beglichen wurde |

###### Druckbereich 828 / Trennbereich zwischen Kopf und Positionsteil

Derzeit keine besonderen Druckpositionen vorgesehen. Der Trennbereich kann zur optischen Abgrenzung des Kopfes gegen den Positionsteil verwendet werden.

###### Druckbereich 829 / Trennbereich zwischen Positionsteil und Zahlungswegen

Derzeit keine besonderen Druckpositionen vorgesehen. Der Trennbereich kann zur optischen Abgrenzung des Positionsteils gegen die anschließend zu druckenden Zahlungswege eingesetzt werden.

###### Druckbereich 850 / Kassensturz - Stückelung

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Einheit | 3 TextVariable<br>6400 | Währungseinheit |
| Anzahl | 4 ZahlVariable<br>6401 | Anzahl der gezählten Einheiten |
| Betrag | 4 ZahlVariable<br>6402 | Anzahl \* Einheit |
| BelegWaehrung | 3 TextVariable<br>6214 | Währungseinit |
| | | | | |

###### Druckbereich 851 / Kassensturz - Bareingänge

Bargeldeingänge verdichtet nach Zahlungsarten und Währungen

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Bar, Rückgeld, Fremdwährung |
| Waehrung | 3 TextVariable<br>6214 | Währung des Satzes |
| Betrag | 4 ZahlVariable<br>6420 | Betrag des Satzes |
| | | | | |

###### Druckbereich 852 / Kassensturz - Zahlungsmitteleingänge

Unbare Eingänge verdichtet nach Zahlungsarten und Währungen

Bereich 852 druckt je nach SPA-Einstellung (Kasse/Barverkauf: Druck der Zamis beim Kassenabschluss gruppiert nach Zahlungsart und Währung entsprechend oft; d.h. entweder werden alle eingegangenen Zahlungsmittel einzeln aufgelistet oder als Gesamtsumme innerhalb einer Währung

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Zahlungsart des eingegangenen unbaren Zahlungsmittels (Scheck, EC-Karte,...) |
| Waehrung | 4 TextVariable<br>6214 | Währung des eingegangenen unbaren Zahlungsmittels (Scheck, EC-Karte,...) |
| Betrag | 3 TextVariable<br>6420 | Betrag des eingegangenen unbaren Zahlungsmittels |
| | | | | |

###### Druckbereich 853 / Kassensturz - Barausgänge

Bargeldausgänge verdichtet nach Zahlungsarten und Währungen

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Bar, Rückgeld, Fremdwährung |
| Waehrung | 3 TextVariable<br>6214 | Währung des Satzes |
| Betrag | 4 ZahlVariable<br>6420 | Betrag des Satzes |
| | | | | |

###### Druckbereich 854 / Kassensturz - Zahlungsmittelausgänge

Unbare Ausgänge verdichtet nach Zahlungsarten und Währungen

Bereich 854 druckt je nach SPA-Einstellung (Kasse/Barverkauf: Druck der Zamis beim Kassenabschluss) gruppiert nach Zahlungsart und Währung entsprechend oft; d.h. entweder werden alle entnommenen Zahlungsmittel einzeln aufgelistet oder als Gesamtsumme innerhalb einer Währung.

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Zahlungsart | 3 TextVariable<br>6380 | Zahlungsart des eingegangenen unbaren Zahlungsmittels (Scheck, EC-Karte,...) |
| Waehrung | 4 TextVariable<br>6214 | Währung des eingegangenen unbaren Zahlungsmittels (Scheck, EC-Karte,...) |
| Betrag | 3 TextVariable<br>6420 | Betrag des eingegangenen unbaren Zahlungsmittels |
| | | | | |

###### Druckbereich 855 / Kassensturz - Stornobeträge

Gesamtsummen von Stornobeträgen im Ein- und Ausgang

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Waehrung | 3 TextVariable<br>6214 | Kurztext der Zentralwährung, da Stornierungssätze in dieser Währung abgelegt ist |
| BetragEin | 4 ZahlVariable<br>6430 | Gesamtbetrag in Zentralwährung über alle Stornierungen von Finanzvorgängen, die in die Kasse gehen |
| BetragAus | 4 ZahlVariable<br>6431 | Gesamtbetrag in Zentralwährung über alle Stornierungen von Finanzvorgängen, die aus der Kasse gehen. |
| EingangAusgang | 3 TextVariable<br>6432 | Festtext Eingang oder Ausgang |
| | | | | |

###### Druckbereich 856 / Kassensturz - Zählungssumme

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Text | 3 TextVariable<br>6440 | Festtext: Überschuss / Manko / Pari |
| Betrag | 4 ZahlVariable<br>6441 | Überschussbetrag / Manko-Betrag / 0 |
| Waehrung | 3 TextVariable<br>6214 | Es wurde in Kassenwährung gezählt, also Kassenwährung-Kurztext |
| | | | | |

###### Druckbereich 859 / Kassensturz - Einleitung Unterbericht

Jeder Unterbericht zum Zählbericht (etwa Auflistung der Zahlungsmittel) kann im Formular mit optisch durch einen Einleitungsbereich aufbereitet werden und mit einer Überschrift versehen werden.

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| UnterberichtTitel | 3 TextVariable<br>6410 | Überschrift zur Einleitung eines Unterberichts |
| | | | | |

###### Druckbereich 860 / Kassenfuß

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| Kassierername | 3 TextVariable<br>6208 | Name des Bedieners lt. Bedienerstamm |
| | | | | |
