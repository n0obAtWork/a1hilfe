# Importvorgstamm

<!-- source: https://amic.de/hilfe/importvorgstamm.htm -->

In dieser Relation werden Kopfdaten des Vorgangs hinterlegt.

| Feld | Pflicht | Bedeutung |
| --- | --- | --- |
| UebernahmeId | Ja | Ident des Stammsatzes dieser muss mit in die Tabelle ImportVorgPosition für die dazugehörigen Positionen geschrieben werden.<br>Der Ident wird mit der Prozedur<br>Amic_dbx_ident(‘ImportVorgStamm‘,1) gezogen |
| SatzId | Ja | In diesem Feld ist eine 1 einzutragen. |
| Status | Ja | Der Status des Stammsatzes muss auf 2 gesetzt werden, ansonsten wird der Beleg nicht verarbeitet.<br>Ausnahme bildet hier die Umwandlung eines Ladescheins zu einem Lieferschein. Hier muss der Status 5 sein ! |
| V_KlassNummer | Ja | Klassennummer des Typs<br>Siehe [Vorgangsklassen](../importierbare_vorgaenge/index.md#Vorgangsliste) |
| V_UKlassNummer | Ja | Unterklasse des Vorgangs |
| V_Unternummer | Ja – sonst 0 | 0 |
| Jahrnummer | Ja | Jahr des Beleges |
| ImportTyp | Ja | Dies wird nur bislang beim Ladeschein und Produktion ausgewertet.<br><ul><li>&nbsp;&nbsp;&nbsp; <a href="../importierbare_vorgaenge/ladeschein_aus_auftrag_zu_lieferschein_rechnung.md">0 Ist Auftrag -&gt; Ladeschein -&gt; Lieferschein / Rechnung</a></li><li>&nbsp;&nbsp;&nbsp; 1 Normaler Ladeschein</li><li>&nbsp;&nbsp;&nbsp; <a href="../importierbare_vorgaenge/produktion.md">10 Ändern einer Produktion</a></li><li><a href="../importierbare_vorgaenge/produktion.md">•&nbsp;&nbsp;&nbsp; 11 Explizite Änderung einer Produktion</a></li></ul> |
| Belegdatum | Ja – sonst aktuelles Datum | Wird das Datum des Beleges |
| Bedieneridneu | Ja | Erfasser des Beleges |
| IVS_GUID | Auto | Wird automatisch pro Satz erzeugt wird als Primary Key für abhängige Relation vom Stammsatz benötigt wie z.B. bei der Relation ImportVorgStammUFLD. Dies bedeutet, dass beim Einspielen der Daten das Feld ausgelesen werden muss. |
| KundNummer | Bei EK/VK | Kundennummer – Kunde/Lieferanten-Nummer des Vorgangs alternativ zur KundId |
| KundId | Bei EK/VK | KundID – Kunde/Lieferanten-ID des Vorgangs alternativ zur Kundennummer |
| ExterneReferenz | Nein | Wird als EDI_KU_Auftragsnummer im Beleg geführt – So kann die externe Belegreferenz im Beleg angezeigt werden. |
| V_NumNummer | Nein | Belegnummer – ist die gegebene Belegnummer bereits vorhanden und handelt es sich nicht um den Importtyp 1 – (Umwandlung Ladeschein zu Lieferschein/Rechnung) so wird eine neue Belegnummer aus dem Nummernkreis verwendet. |
| BuchTyp | Nur bei Umbuchung | 0 = Vorgemerkt (AG)<br>1 = dispositiv (AU)<br>2 = gebucht (RE) |
| Druckernummer | Nein | Wenn der Beleg nach dem Import gedruckt werden soll, kann hier ein Drucker dafür angegeben werden. |
| TCPIP_Adresse | Nein | Hinweis auf die IP-Adresse des erfassenden MDE-Geräts – wird nicht in den Vorgang übernommen. |
| V_id | Ausgabe | Dieses Feld wird nach dem Import mit der V_ID des Vorgangs befüllt und ggf. bei Weiterverarbeitungen mitgepflegt, so dass stets eine Verbindung dieses Quellsatzes zum Zielbeleg bestehen bleibt. |
| UseCs | Nein | Unterscheidung zwischen den technischen Umsetzungen. Nicht alle Vorgangsimporte funktionieren mit dem Defaultwert 0 – viele benötigen inzwischen den Wert 1 |
| Lfd_Nummer | Nein | Verwendung unklar |
| CreateTime | Nein | Zeitstempel |
| Datum | Nein | Verwendung unklar |
| Filialnummer | Nein | Verwendung unklar |
| PeriNummer | Nein | Verwendung unklar |
| VertGrNummer | Nein | Verwendung unklar |
| Zeilenrabatt | Nein | Verwendung unklar |
| Vorgident | Nein | Verwendung unklar |
| Referenz | Nein | Verwendung unklar |
| FileName | Nein | Verwendung unklar |
| FremdNummer | Nein | Verwendung unklar |
| RabattTyp | Nein | Verwendung unklar |
| InfoText | Nein | Verwendung unklar |
| PreisBrutto | Nein | Verwendung unklar |
| VorgKlasse | Nein | Verwendung unklar |
| BelegErzId | Nein | Verwendung unklar |
| Erfasst | Nein | Verwendung unklar |
| DruckAuspraegung | Nein | Verwendung unklar |
| Vxml | Nein | Verwendung unklar |
| KundenAnschriftManuell | Nein | Verwendung unklar |
| AlternativeVersandanschrift | Nein | Verwendung unklar |
| BaustId | Nein | Verwendung unklar |
| AdressId | Nein | Verwendung unklar |
| ZusatzText | Nein | Verwendung unklar |
| BestellerKundId | Nein | Verwendung unklar |
| GesamtPreis | Nein | Verwendung unklar |
| GesamtSteuer | Nein | Verwendung unklar |
| ValutaDatum | Nein | Verwendung unklar |
| SkontoDatum | Nein | Verwendung unklar |
| SkontoBetrag | Nein | Verwendung unklar |
| GesamtWertBrutto | Nein | Verwendung unklar |
| GesamtWertSteuer | Nein | Verwendung unklar |
| GesamtWertRabatte | Nein | Verwendung unklar |
| GesamtWertZuschlaege | Nein | Verwendung unklar |
| SperrUmwand | Nein | Verwendung unklar |
| SperrFibu | Nein | Verwendung unklar |
| SperrBearbeit | Nein | Verwendung unklar |
| SperrFilia | Nein | Verwendung unklar |
| SperrreBuch | Nein | Verwendung unklar |
| Fa_belegreferenz | Nein | Verwendung unklar |
| MENummerAusLadeschein | Nein | Verwendung unklar |
| Quelle | Nein | Quelle als Referenz auf den Importprozess |
| Eingangsdatum | Nein | Verwendung unklar |
| Zusatzfeld1 | Nein | Verwendung unklar |
| Zusatzfeld2 | Nein | Verwendung unklar |
| Zusatzfeld3 | Nein | Verwendung unklar |
| Zusatzfeld4 | Nein | Verwendung unklar |
| Zusatzfeld5 | Nein | Verwendung unklar |
