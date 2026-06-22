# ImportVorgPosition

<!-- source: https://amic.de/hilfe/importvorgposition.htm -->

In dieser Relation werden Daten der Vorgangswarenposition gespeichert.

| Feld | Pflicht | Bedeutung |
| --- | --- | --- |
| UebernahmeId | Ja | Übernahemid des Stammsatzes |
| SatzId | Ja | SatzId wie im Stammsatz( 1) |
| PositionId | Ja | Positionszähler dieser muss manuell erhöht werden. |
| Status | Ja | Der Status der Position muss auf zwei gesetzt werden, ansonsten wird der Beleg nicht verarbeitet. |
| ArtikelNummer, ArtikelId, ArtiStammId  
 | Ja | Artikel des Umzubuchende Artikels, wenn die ArtikelId übergegeben wird, dann muss in der Abgangszeile der Artikel des Abganglagers stehen und in der Zugangszeile die ArtikelId des Zugangslagers.  
Bei Lagerumbuchung gilt:  
• In der Abgangszeile wird hier der Artikel des Abgangs eingetragen  
• In der Zugangszeile wird hier der Artikel des Zugangs eingetragen |
| LagerNummer | Wenn nicht Artikelid | Lagernummer der Position  
Bei Lagerumbuchung gilt:  
• In der Abgangszeile wird hier die Lagernummer des Abgangslagers eingetragen  
• In der Zugangszeile wird hier die Lagernummer des ZugangsLagers eingetragen |
| LagerplatzNummer | Nein | Lagerplatz der Position |
| Menge | Ja | Hier wird die Menge eingetragen, bei einem Gebinde muss hier die Gebindeanzahl eingetragen werden |
| ME | Ja | Mengeneinheit der Position |
| ME_Preis | Nein | Mengeneinheit des Preises |
| Preiseinheit | Nein | Einheit des Preise |
| Preis | Nein | Preis der Position |
| Preisgesamt | Nein | • 0 Der Preis wird als Preis genommen  
• 1 Der Preis der Warenposition als Gesamtpreis gesetzt werden. |
| KontraktNummer | Nein | Nummer des Kontraktes |
| ZusatzInfo | | Zusatzinfo begrenzt auf 40 Zeichen |
| ZusatzInfo2 | | ZusatzInfo2 begrenzt auf 40 Zeichen |
| PartieId, PartieNummer, PartieBezeich | | Legt die Partie zu einer Zeile an. Die suche passiert zurzeit per Partienummer und oder Partiebezeichnung. Sollen mehrere Partiepositionen an eine Positionszeile übergeben werden, so muss die Tabelle ImportVorgPositionPartie verwendet werden.  
Die Partie muss dann in diesem Fall in dieser Tabelle Leer sein.  
Existiert eine Partie mit der Angegebenen Partiebezeichnung nicht, so wird eine Partie neu Angelegt. |
| IVP_GUID | Auto | Wird automatisch pro Satz erzeugt wird als Primary Key für abhängige Relation vom Positionssatz benötigt wie z.B. bei der Relation ImportVorgStammUFLD. Dies bedeutet, dass beim Einspielen der Daten sich das Feld ausgelesen werden muss. |
| IVS_GUID | Auto | Guid des zugehörigen Stamm-Eintrags |
| Wabewerfassid | Nur bei Teilumwandlung | Wird hier die wabewerfassid einer existierenden Warenposition gegeben, so kann von dieser teildisponiert werden. Alternativ kann zur Teildisposition die GUID der Warenposition als InterneReferenz gegeben werden.  
Im Fall der Teildispo kann ein abweichendes Ziel-Lager bzw. eine ArtikelId aus einem abweichenden Lager angegeben werden, wenn es sich bei dem Quellbeleg um ein Angebot mit Sortimentslager handelt. |
| InterneReferenz | Nur bei Teilumwandlung | Wird hier die GUID einer existierenden Warenposition gegeben, so kann von dieser teildisponiert werden. Alternativ kann zur Teildisposition die wabewerfassid gegeben werden.  
Im Fall der Teildispo kann ein abweichendes Ziel-Lager bzw. eine ArtikelId aus einem abweichenden Lager angegeben werden, wenn es sich bei dem Quellbeleg um ein Angebot mit Sortimentslager handelt. |
| RestAusbuchKennz | Nur bei Teilumwandlung | Wird hier eine 1 gegeben, so wird der nicht teildisponierte Teil der Teilumwandlung ausgebucht und die Position als erledigt markiert. |
| ArtiAusprBezeich?? | | Hier wird die Artikelausprägung beschrieben |
| Zusatzfeld1 | Ausgabe | Hier wird die Inventurbelegnummer eingetragen, wenn ein Inventurbeleg erstellt werden sollte. |
| Zusatzfeld2 | Ausgabe | Hier wird die Inventurbeleg-Positionsnummer eingetragen, wenn ein Inventurbeleg erstellt werden sollte. |
| TypZuAbgang | Nur bei Umbuchung | 1 = Zeile ist Abgang  
2 = Zeile ist Zugang |
| Positionsklammer | Nur bei Umbuchung | Dieser Wert muss für die Zu- und Abgangszeile einer Position identisch sein. Er klammert diese beiden Zeilen zusammen.  
Es empfiehlt sich die PositionId der Abgangszeile zu nehmen. |
| GebFaktor1 | Bei Gebinde | Gebindefaktor 1 |
| GebFaktor2 | Bei Gebinde | Gebindefaktor 2 |
| GebFaktor3 | Bei Gebinde | Gebindefaktor 3 |
| GebFaktor4 | Bei Gebinde | Gebindefaktor 4 |
| GebindeAnzahl | Bei Gebinde | Anzahl Gebinde |
| ZielLagerPlatz | Bei Lagerplatzumbuchung | Eine Besonderheit ist die Lagerplatzumbuchung. Diese kann in einer Zeile abgehandelt werden - Dabei werden der Lagerplatz und der Ziellagerplatz in einer Zeile gegeben. |
| LagerNummerZug | Nein | Verwendung unklar – evtl. veraltet |
| LagerPlatzNrZug | Nein | Verwendung unklar – evtl. veraltet |
| Rabatt | Nein | Verwendung unklar – evtl. veraltet |
| RabattTyp | Nein | Verwendung unklar – evtl. veraltet |
| ZuAbschlag | Nein | Verwendung unklar – evtl. veraltet |
| SteuerSchluessel | Nein | Verwendung unklar – evtl. veraltet |
| Gebinde | Nein | Verwendung unklar – evtl. veraltet |
| GebMass1 | Nein | Verwendung unklar – evtl. veraltet |
| GebMass2 | Nein | Verwendung unklar – evtl. veraltet |
| GebMass3 | Nein | Verwendung unklar – evtl. veraltet |
| GebMass4 | Nein | Verwendung unklar – evtl. veraltet |
| InfoText | Nein | Verwendung unklar – evtl. veraltet |
| ArtikelVariante | Nein | Verwendung unklar – evtl. veraltet |
| MHD | Nein | Verwendung unklar – evtl. veraltet |
| NVE | Nein | Verwendung unklar – evtl. veraltet |
| MarkierIdent | Nein | Verwendung unklar – evtl. veraltet |
| ZielMarkierIdent | Nein | Verwendung unklar – evtl. veraltet |
| LagerPlatzBezeich | Nein | Verwendung unklar – evtl. veraltet |
| ZielLagerPlatzBezeich | Nein | Verwendung unklar – evtl. veraltet |
| ME_NummerGebinde | Nein | Verwendung unklar – evtl. veraltet |
| ExterneReferenz | Nein | Verwendung unklar – evtl. veraltet |
| KostStelNummer | Nein | Verwendung unklar – evtl. veraltet |
| KostStelBezeich | Nein | Verwendung unklar – evtl. veraltet |
| ProdVerwTyp | Nein | Verwendung unklar – evtl. veraltet |
| ProdStlVaria | Nein | Verwendung unklar – evtl. veraltet |
| LadeeinheitsNr | Nein | Verwendung unklar – evtl. veraltet |
| PartieGrNummer | Nein | Verwendung unklar – evtl. veraltet |
| PositionsText | Nein | Verwendung unklar – evtl. veraltet |
| GebindeTeildispo | Nein | Verwendung unklar – evtl. veraltet |
| Kostenobjekt | Nein | Kostenobjekt |
