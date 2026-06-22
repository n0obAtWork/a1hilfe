# Parameter des Artikelstamms

<!-- source: https://amic.de/hilfe/_parameterdesartikels.htm -->

Hauptmenü > Stammdatenpflege > Artikelstamm > Artikelstamm

oder Direktsprung [ARS]

In den Artikelstammdaten werden alle Informationen über einen Artikel zusammen­ge­fasst, auf die für eine weitgehend automatisierte Verarbeitung zugegriffen werden muss. Dies sind z.B. **Mengeneinheiten, Preise, Gebindegröße etc.** Da zur Verein­fachung der Erfassung bei der Anlage eines Artikels auf vorerfasste Informationen zugegriffen wird, müssen diese natürlich vorhanden sein. So wird sicherlich häufig die Mengeneinheit **"Stück"** benötigt. Diese muss also zuvor in der Tabelle **"Men­geneinheiten"** erfasst werden.

Vor der Erfassung der Artikel- und Kundenstammdaten genauso wie der Finanz­buch­haltungskonten, müssen also verschiedene Konstanten eingegeben, da auf sie bei der Stammdatenerfassung zugegriffen wird. Darüber hinaus können weitere Kon­stanten in Abhängigkeit von der Anwendung hinzu­kommen. So sind die Gefahr­gut­kennzeichen zu erfassen, wenn die Gefahr­gutabwicklung aktiviert werden muss.

Im Artikelstamm werden nur diejenigen Daten eines Artikels vermerkt, die ent­weder völlig lagerunabhängig sind, das sind globale Dinge wie Gefahrgut, Ge­binde­größen, Mengeneinheiten usw., oder häufig globalen Charakter haben.

Dies sind z.B. die Gruppierungs- oder Statistikkennzeichen. Im letzten Fall werden im Artikelstamm also Vorbelegungen vorgenommen, die ggf. jedoch in der Ausprägung überschrieben werden können.

Alles, was in verschiedenen Lagern unterschiedlich sein **KÖNNTE**, muss im Artikel hinterlegt werden!

Folgende Felder stehen zur Verfügung:

| Feld | Bedeutung |
| --- | --- |
| Artikelstammnummer | Dies ist die logische, also für den Anwender sichtbare und durch ihn zu pflegende, Artikelstamm-Identifikation.  
Je nach Auslegung des Systems handelt es sich hierbei um einen alphanumerischen oder numerischen Schlüssel.  
Auf Organisationsprinzipien zur Vergabe von Artikelnummern wird an anderer Stelle eingegangen.  
Wichtig ist hier jedoch, dass bei der Vergabe der Nummer keine über die Identifikation hinausgehenden Bedeutungen in den Schlüssel eingebaut werden müssen, da zahlreiche Felder im Artikelstamm hierfür zur Verfügung stehen. |
| Artikeltext | Bezeichnung des Artikels. Der Aufruf erfolgt über die Funktion Texterfassung in der Funktions-Box.  
Der Text kann mehrere Zeilen umfassen; er wird in dem Artikeltexterfassungsbildschirm eingetragen.  
Mit Beendigung der Erfassung gelangt man mit ESC wieder in die Artikelmaske zurück.  
Über diese Erfassung hinaus besteht die Möglichkeit, Artikeltexte in verschiedenen Sprachen zu erfassen.  
Hierzu wird die Funktion andere Sprache SF5 angewählt, danach die gewünschte Sprache ausgewählt und der Text eingegeben.  
Dieser Text wird z.B. bei der Fakturierung automatisch in Verbindung mit dem Sprachkennzeichen des Kunden gezogen. Ähnlich verhält es sich mit der Artikeltextvariante. Innerhalb der Sprachen kann der Artikeltext in andere Variante F5 mit z.B. unterschiedlichem Umfang angelegt werden.  
Auf diese wird dann wiederum in der Fakturierung etc. zugegriffen. |
| Kurzbezeichnung | Die Kurzbezeichnung wird z.B. in Listen benötigt, da häufig der Artikeltext zu viel Platz benötigt. |
| Matchcode | Kurzbegriff für die Artikelsuche. Hier wird der Hauptbegriff erfasst, bis zu 90 weitere können unter  
"Sekundärschlüssel" eingegeben werden. |
| EAN-Nummer | Hier kann die EAN-Nummer für die Artikelsuche eingegeben werden. |

Registerkarte „Allgemein“

| Feld | Bedeutung |
| --- | --- |
| Warengruppe | Zuordnung einer Warengruppennummer für z.B. Auswertungen. |
| Erlöskennziffer | Zuordnung der Erlöskennziffer für die automatische Verbuchung in der Finanzbuchhaltung.  
Hiervon abweichend kann im Artikel eine individuelle EKZ vergeben werden. Die Zuordnung hier dient dann als Vorschlag. |
| Steuerschlüssel | Der Steuerschlüssel zur Ermittlung der Umsatzsteuer von der Artikelseite her. Zusammen mit dem Steuerschlüssel des Kunden/Lieferanten ergibt sich bei der Vorgangs­er­fassung der Steuersatz. |
| Mengeneinheitsgruppe | Angabe der Mengeneinheitsgruppe, die diesem Artikel zugeordnet ist.  
In Abhängigkeit von der Funktionalität der Mengen­ein­hei­ten­gruppe, z.B. Gebindefakturierung oder nicht, werden unter „Gebindefaktoren" weitere Eingaben erwartet. |
| Preisauszeichnung Grundeinheit | |
| Gewicht / Grundmengeneinheit | Gewicht des Artikels je Mengengrundeinheit. Die Eingabe kann hier mit 4 Nachkommastellen erfolgen. Muss gepflegt werden für nachhaltige Artikel. |
| Bruttogewicht | Hier wird das Gewicht inkl. Verpackung je Mengen­grund­einheit eingetragen. Die Eingabe kann hier mit 4 Nach­komma­stellen erfolgen. |
| Verpackungsgewicht | Hier wird das Gewicht der Verpackung und anschließend die Mengeneinheit festgehalten  
Die Mengeneinheit für das Verpackungsgewicht wird entsprechend dem SPA "Standard-Mengeneinheit-Gewicht" vorbelegt.  
Hier sollte also ein passender Mengeneinheitsschlüssel hinterlegt sein (z.B. kg oder g).  
Ist der SPA "Verpackungsgewicht aut. = Brutto - Netto" auf " ja " gesetzt, so wird das Verpackungsgewicht nicht manuell eingegeben, sondern aus Brutto- und Nettogewicht ermittelt. |
| Mengeneinheit Verpackungsgewicht | |
| Etikettentyp | |
| Archiv-Referenz | |
| | |

<p class="siehe-auch">Siehe auch:</p>

- [Registerkarte Konstanten](./registerkarte_konstanten.md)
- [Registerkarte Markt](./registerkarte_markt.md)
- [Registerkarte Preise](./registerkarte_preise/index.md)
- [Registerkarte Gruppen](./registerkarte_gruppen/index.md)
- [Registerkarte Zusatz](./registerkarte_zusatz.md)
- [Registerkarte Gebinde](./registerkarte_gebinde.md)
- [Registerkarte Zertifiakte](./registerkarte_zertifiakte.md)
- [Registerkarte Steuern](./registerkarte_steuern.md)
- [Textzeilen](./textzeilen.md)
- [Gefahrgut](./gefahrgut/index.md)
- [Sekundärschlüssel](./sekundaerschluessel.md)
- [Zusammensetzung](./zusammensetzung.md)
- [Lieferanten / Hersteller](./lieferanten_hersteller.md)
- [Kundenindividuelle Artikelnummern](./kundenindividuelle_artikelnummern.md)
- [Ausweichliste](./ausweichliste.md)
- [Folgeartikel verwalten](./folgeartikel_verwalten/index.md)
- [Reporte Artikelstamm](./reporte_artikelstamm/index.md)
