# Endkontrolle/Einspielung Kunden

<!-- source: https://amic.de/hilfe/_endkontrolle_einspielung_kunden.htm -->

Neuanlage eines Imports

Mit F8 kann ein neuer Kundenimport angelegt werden.

Mit F5 können Datensätze, die über den Stammdatenimport ins System gekommen sind, bearbeitet und korrigiert werden.

Tabreiter übergreifende Felder

| Eingabefelder | Bedeutung |
| --- | --- |
| Satznummer | Automatisch generierte Zahl  
 |
| Kundennummer | Gewünschte Kundennummer. Rechts davon ein Feld für die gewünschte Kundenbezeichnung  
 |
| Kundentyp | Mittels F3 kann der Kundentyp ausgewählt werden.  
 |
| Musterkunde | Mittels F3 kann ein Musterkunde ausgewählt werden.  
 |

Die Kundennummer ist ein Pflichtfeld. Wird eine schon vorhandenen Kundennummer angegeben, so wird durch den Import der Kunde mit den Daten aus dem Import aktualisiert.

Gehört die Kundennummer einem Kunden, der gelöscht wurde, so wird der Import abgebrochen.

Der Kundentyp ist ein Pflichtangabe.

In Abhängig vom Kundentyp werden im Musterkunden die Musterkunden des Kundentyp angezeigt.

Musterkunden werden in [KU] der Variante Musterkunde angelegt.

Der Musterkunde ist ein Pflichtfeld. Aus dem Musterkunden werden sehr viel Daten für den Import genommen.

Tabreiter Adresse

| Eingabefelder | Bedeutung |
| --- | --- |
| Anrede | Anrede des Kunden  
 |
| Vorname | Vorname des Kunden  
 |
| Name | Nachname des Kunden  
 |
| Staat | Staat des Kunden  
 |
| Postleitzahl | Postleitzahl des Kunden  
 |
| Ort | Ort des Kunden  
 |
| Ortsteil | Ortsteil des Kunden  
 |
| Straße | Straße des Kunden  
 |
| Telefon | Telefonnummer des Kunden  
 |
| Telefax | Telefaxnummer des Kunden  
 |

Tabreiter Zusatzinfo/Bank

| Eingabefelder | Bedeutung |
| --- | --- |
| Kurzname | Kurzname des Kunden  
 |
| Matchcode | Matchcode des Kunden  
 |
| Zusatz | Zusatzinformation zum Kunden  
 |
| Partner1 | Partner1 des Kunden  
 |
| Partner2 | Partner2 des Kunden  
 |
| Bankleitzahl | Bankleitzahl des Kunden  
 |
| Bankname | Mittels F3 kann der Bankname des Kunden ausgewählt werden.  
 |
| Bankkonto | Bankkonto des Kunden  
 |

Tabreiter FiBu-Daten

| Eingabefelder | Bedeutung |
| --- | --- |
| Mahnsperre | Mittels F3 kann eine Mahnsperre eingerichtet werden.  
 |
| Zinssperre | Mittels F3 kann eine Zinssperre eingerichtet werden.  
 |
| Kokore-Kennz | Mittels F3 kann eingerichtet werden, ob der Kunde Kontokorrent ist.  
 |
| Erlösklasse | Mittels F3 kann die Erlösklasse des Kunden ausgewählt werden.  
 |
| Ford.grp. | Mittels F3 kann die Forderungsgruppe des Kunden ausgewählt werden.  
 |
| Mahngruppe | Mittels F3 kann die Mahngruppe des Kunde ausgewählt werden.  
 |
| ZahlartDebitor | Mittels F3 kann der ZahlartDebitor des Kunden ausgewählt werden.  
 |
| ZahlartKredit | Mittels F3 kann der ZahlartKredit des Kunden ausgewählt werden.  
 |
| Zinsgruppe | Mittels F3 kann die Zinsgruppe des Kunden ausgewählt werden  
 |
| OP Art | Mittels F3 kann die OP Art des Kunden ausgewählt werden.  
 |
| Kredit | Kreditlimit des Kunden  
 |
| SollZins Bagatell | Sollzins des Kunden  
 |
| HabenZins Bagate | Habenzins des Kunden  
 |

Die Forderungsgruppe ist ein Pflichtfeld beim Kunden und muss ausgefüllt sein, damit der Import funktioniert. Ansonsten gibt es beim Import den Fehler, dass die Daten nicht plausibel sind.

Tabreiter Vorgangs-Daten

| Eingabefelder | Bedeutung |
| --- | --- |
| Steuergruppe | Mittels F3 kann die Steuergruppe des Kunden ausgewählt werden.  
 |
| Fakturiergruppe | Mittels F3 kann die Fakturiergruppe des Kunden ausgewählt werden.  
 |
| Vertretergruppe | Mittels F3 kann die Vertretergruppe des Kunden ausgewählt werden.  
 |
| Zahlbed. EK | Mittels F3 kann die Zahlungsbedingung im Einkauf für den Kunden ausgewählt werden.  
 |
| Zahlbed. VK | Mittels F3 kann die Zahlungsbedingung im Verkauf für den Kunden ausgewählt werden.  
 |
| UST-Kennz | Umsatzsteuerkennzeichen des Kunden  
 |
| Waehrung | Mittels F3 kann die Währung des Kunden ausgewählt werden.  
 |

Die Steuergruppe ist ein Pflichtfeld beim Kunden und muss ausgefüllt sein, damit der Import funktioniert. Ansonsten gibt es beim Import den Fehler, dass die Daten nicht plausibel sind.

Wenn man mit den Daten aller zu importierenden Kunden einverstanden ist, dann wählt man im Rechtemaustastenmenü die Funktion *\-> A.eins Kunden* aus und startet den Import.

Importprozeduren

Für den Import sind die Prozeduren *amic_kunden_insupd* und *amic_muster_kunden_abgleich* verantwortlich.

Durch die Prozeduren werden so gut es geht die fehlenden Informationen bei einem Import vervollständigt.

Es gibt 3 Fälle beim Kundenimport.

1. Im 1. Fall ist die Bemerkung des zu importierenden Kunden nicht leer. Dann wird der Kunde importiert und die Bemerkung des Kunden bleibt erhalten.

2. Im 2. Fall ist die Bemerkung des zu importieren Kunden leer und die Kundbemerk in der Tabelle amic_kunden_import ist 0 oder Null. Dann wird die Bemerkung des angegeben Musterkunden benutzt.

3. Im 3. Fall ist die Bemerkung des zu importierenden Kunden leer und die Kundbemerk in der Tablle amic_kunden_import ist nicht 0 oder Null. Dann wird die existierende Bemerkung aus einem anderen Kunden auch für diesen Kunden benutzt. Beide Bemerkungen sind aber unabhängig voneinander.
