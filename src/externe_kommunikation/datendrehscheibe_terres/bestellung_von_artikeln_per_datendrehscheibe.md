# Bestellung von Artikeln per Datendrehscheibe

<!-- source: https://amic.de/hilfe/_terres_bestellungen.htm -->

Hier gibt es die Möglichkeit der Übermittlung von Bestellungen an die Terres Zentrale. Eine entsprechende Umschlüsselung gemäß Umschlüsselwerk (Querverweis Bereich Importumsetzer) wird vorgenommen

<p class="just-emphasize">Vorgehensweise</p>

Um eine Bestellung für Terres auszulösen, wird wie folgt vorgegangen:

1. Eine Bestellung wird wie im A.eins Standard erfasst.  
Manuell ist darauf zuachten, dass ein Lieferrant ausgewählt wird, der für Terres gelistet ist, außerdem müssen die gewählten Artikel ebenfalls bei Terrres gelistet sein.

2. Bestellung über [OpenTrans](../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/opentrans_thebe/index.md) drucken.  
In diesem Vorgang wird mittels einem dazugehörigem Konverter, der auf das Umschlüsselwerk zugreift, die Bestellung in eine von Terres lesbare XML Struktur gewandelt. Abschließend wird die Bestellung inklusive dem XML als Anhang in das Archiv geschrieben und in den Ausgabepfad der [OpenTrans](../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/opentrans_thebe/index.md) Schnittstelle gespeichert.

3. Bestellung an Terres senden  
Die erstellte Bestellung aus der [OpenTrans](../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/opentrans_thebe/index.md) Schnittstelle kann per [Belegversand](../../zusatzprogramme/mailversand_allgemein/index.md) an Terres übermittelt werden.

4. Die erfassten Bestellungen können unter dem Terres Bestellexport angesehen werden.  
Hauptmenü > Externe Kommunikation > Datendrehscheibe > Bestellexport [**TERRX**]

<p class="just-emphasize">Besonderheiten</p>

Wird eine Bestellung erfasst, und diese ist noch nicht an Terres übermittelt worden, und in der zwischenzeit erhält ein Artikel das Kennzeichen „Bestellung-zulassen“ 0 so wird in den Standard Varianten der Anwendung Bestellexport [**TERRX**] die Bestellung rot markiert.

Vorher müssen einige Einstellungen im A.eins System vorgenommen sein.

<p class="just-emphasize">Einrichtungen</p>

1. Die [Dokumentenverwaltung](../../dokumentenverwaltung/archiv_manager/index.md) muss aktiviert sein. Eventuell muss eine Lizenz für die Dokumentenverwaltung erworben werden.  
    

2. [OpenTrans](../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/opentrans_thebe/index.md) muss Eingerichtet werden. (Lizenz erforderlich)  
    

3. Für das [Bestellformular](../../zusatzprogramme/formulareinrichtung_und_zuordnung/index.md) muss das [Archivierungskennzeichen](../../zusatzprogramme/formulareinrichtung_und_zuordnung/der_formular_pfleger/archivierung_aktivieren_fuer_das_formular.md) gesetzt worden sein.

4. In der [Formularzuordnung / Vorgangsklasse](../../vorgangsabwicklung/formularzuordnung/ot_opentrans.md) muss für die Bestellung und der dazugehörigen Unterklasse das OpenTrans aktiviert werden. [OpenTrans](../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/opentrans_thebe/index.md) wird auf der Registerkarte OT aktiviert.

5. Auf der Registerkarte OT gibt es die Option eine [Prozedur für Dateinamen](../../vorgangsabwicklung/formularzuordnung/ot_opentrans.md#OpenTRANS_Prozedur_Dateinamen) anzugeben. Dies sollte nun über den [SPA 911](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/terres_belegexport_belegnummer_spa_911.md) erledigt werden.  
    

6. Im Lieferanten müssen die Einstellungen zu [OpenTrans](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/kennzeichen/opentrans.md) auf der Registerkarte Kennzeichen eingetragen werden.

7. In den [Vorgangsdruckklassen](../../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md) muss geprüft werden, ob nicht für die Bestellung ein anderes Formular eingerichtet worden ist, welches nicht Archiviert wird. Wenn dies der Fall sein sollte muss das Archivkennzeichen für das Formular gesetzt werden.

8. Im [Mandantenstamm](../../firmenstamm/firmenkonstanten/mandantenstamm.md) muss die [Terres Bestellnummer](../../firmenstamm/firmenkonstanten/mandantenstamm.md#Terres_bestellnummer) auf der Registerkarte Allgemein hinterlegt werden. Diese Nummer bekommen Sie von Terres.

9. In den [Mengeneinheiten](../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/internationale_mengeneinheit_un.md) müssen die Internationalen Mengeneinheit(UN) deklariert sein. Wenn diese nicht vorhanden sind kann die Basiseinrichtung aufgerufen werden. Diese trägt die gängigsten Umsetzungen zwischen den UN Mengeinheiten und den A.eins Mengeneinheit ein.

10. Der [Belegversand](../../zusatzprogramme/mailversand_allgemein/index.md) muss eingerichtet werden.(Lizenz erforderlich)

<p class="just-emphasize">Konverterprozedur</p>

In der Konverterprozedur, welche aus dem OpenTrans Dokument ein Terres Dokument macht, müssen einige Stellen umkonvertiert werden.

<p class="just-emphasize">Felder für die Bestellung</p>

Erklärung der Einzelnen Felder in der Bestellung

<p class="just-emphasize">Header OpenTrans Pfad '/ORDER/ORDER_HEADER/ORDER_INFO/PARTIES/PARTY/.'</p>

| ![\*](../../ImagesExt/image8_1558.jpg "*") XML Element | ![\*](../../ImagesExt/image8_1558.jpg "*") Konstante | ![\*](../../ImagesExt/image8_1558.jpg "*") Wert /Herkunft | ![\*](../../ImagesExt/image8_1558.jpg "*") XML Pfad OpenTrans |
| --- | --- | --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Sendercode | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") GEX | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Empfaengercode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") **ILN** aus dem Mandantenstamm | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Sendername | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") **Name** aus dem OpenTrans Dokument | ![\*](../../ImagesExt/image8_1556.jpg "*") Party_Role supplier<br>![\*](../../ImagesExt/image8_1556.jpg "*") ADRESS/bemact:NAME |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Senderort | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") **City** aus dem OpenTrans Dokument | ![\*](../../ImagesExt/image8_1556.jpg "*") Party_Role supplier<br>![\*](../../ImagesExt/image8_1556.jpg "*") ADRESS/bemact:CITY |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Erstellungsdatum | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Tagesdatum | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Erstellungszeit | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Uhrzeit bei der Erstellung des Dokumentes | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Externe-Referenz | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja/Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Kann mit einem eigenen Wert gefüllt werden | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Testuebertragung | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1557.jpg "*") 0 Echte Bestellung<br>![\*](../../ImagesExt/image8_1558.jpg "*") 1 Test Bestellung | ![\*](../../ImagesExt/image8_1556.jpg "*") |

<p class="just-emphasize">Kopfdaten OpenTrans Pfad '/ORDER/ORDER_HEADER/ORDER_INFO/PARTIES/PARTY/.'</p>

| ![\*](../../ImagesExt/image8_1558.jpg "*") XML Element | ![\*](../../ImagesExt/image8_1558.jpg "*") Konstante | ![\*](../../ImagesExt/image8_1558.jpg "*") Wert /Herkunft | ![\*](../../ImagesExt/image8_1558.jpg "*") XML Pfad OpenTrans |
| --- | --- | --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Belegart | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Bestellung | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Nr | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Belegnummer | ![\*](../../ImagesExt/image8_1556.jpg "*") ../../ORDER_ID |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Verk-an-Deb-Nr | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Terresnummer aus dem Mandantenstamm | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") GWSNr | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Feld bleibt leer | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-Name | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Liefername | ![\*](../../ImagesExt/image8_1556.jpg "*") ./ADDRESS/bmecat:NAME |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-Name-2 | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Weiterer Name | ![\*](../../ImagesExt/image8_1556.jpg "*") ./ADDRESS/bmecat:NAME2 |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-Adresse | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferadresse | ![\*](../../ImagesExt/image8_1556.jpg "*") ./ADDRESS/bmecat:STREET |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-PLZ-Code | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferpostleitzahl | ![\*](../../ImagesExt/image8_1556.jpg "*") ./ADDRESS/bmecat:ZIPBOX |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-Ort | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferort | ![\*](../../ImagesExt/image8_1556.jpg "*") ./ADDRESS/bmecat:CITY |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferdatum | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferdatum ist aber Belegdatum | ![\*](../../ImagesExt/image8_1556.jpg "*") ../../ORDER_DATE |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lagerortcode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Lager aus dem Vorgang | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Waehrungscode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Währung des Beleges | ![\*](../../ImagesExt/image8_1556.jpg "*") ../../bmecat:CURRENCY |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Belegdatum | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Wann ist Bestellung erstellt worden | ![\*](../../ImagesExt/image8_1556.jpg "*") ../../ORDER_DATE |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferart | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Lager | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Eink-von-Kred-Nr | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Kreditorennummer des Lieferanten. Entweder ist die Lieferantennummer in A.eins die Kreditorennummer des Leiferanter, oder die Nummer muss sich aus dem Stammsatz des Artikels gezogen werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./bmecat:PARTY_ID |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Palettenanzahl | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") 0 Wird bei der Bestellung nicht benötigt. | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Barverkauf | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") 0 Wird bei der Bestellung nicht benötigt. | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Storno-Beleg | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Vorgangsklasse | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Eink-von-ILN | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") ILN Nummer des Lieferanten. Ist für die Bestellung über Agravis nicht kritisch. Wird aber für Bestellungen von Streckenliefernaten benötigt. Deswegen sollte dies Feld gefüllt werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") './ADDRESS/\*:Eink-von-ILN'<br>![\*](../../ImagesExt/image8_1556.jpg "*") Muss als char 13 übergeben werden |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Verk-an-ILN | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") ILN Nummer aus dem Mandenstamm | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lief-an-ILN | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") ILN Nummer des Kunden der Lagerzuordnung | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") EDI-Nachrichtentyp | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Orders | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") EDI-Anwendungscode | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") A.eins | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") EDI-Referenznummer | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja/Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Kann mit einem eigenen Wert gefüllt werden | ![\*](../../ImagesExt/image8_1556.jpg "*") |

<p class="just-emphasize">Positionsdaten OpenTrans Pfad '/ORDER/ORDER_ITEM_LIST/ORDER_ITEM/.'</p>

Die Positionsdaten können n mal vorkommen.

| ![\*](../../ImagesExt/image8_1558.jpg "*") XML Element | ![\*](../../ImagesExt/image8_1558.jpg "*") Konstante | ![\*](../../ImagesExt/image8_1558.jpg "*") Wert /Herkunft | ![\*](../../ImagesExt/image8_1558.jpg "*") XML Pfad OpenTrans |
| --- | --- | --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Zeilennr | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Positionszähler für die Artikel in der Bestellung | ![\*](../../ImagesExt/image8_1556.jpg "*") ./LINE_ITEM_ID<br>![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Art | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Beschreibung | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Bezeichnung des Artikels | ![\*](../../ImagesExt/image8_1556.jpg "*") ./PRODUCT_ID/bmecat:DESCRIPTION_LONG<br>![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Menge | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Menge der Bestellung | ![\*](../../ImagesExt/image8_1556.jpg "*") ./QUANTITY |
| ![\*](../../ImagesExt/image8_1556.jpg "*") VK-Preis | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Verkaufspreis des Artikels, hier kann aber auch eine 0 übergeben werden | ![\*](../../ImagesExt/image8_1556.jpg "*") ./VK-Preis |
| ![\*](../../ImagesExt/image8_1556.jpg "*") MWSt-p | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Auch hier kann eine 0 übergeben werden | ![\*](../../ImagesExt/image8_1556.jpg "*") ./MWSt-p |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Betrag | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Netto Betrag für den Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Betrag |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Betrag-inkl-MWSt | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Brutto Betrag für den Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Betrag-inkl-MWSt |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Bruttogewicht | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Bruttogewicht des Artikels<br>![\*](../../ImagesExt/image8_1556.jpg "*") Auch hier kann eine 0 übergeben werden | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Bruttogewicht |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Nettogewicht | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Nettogewicht des Artikels<br>![\*](../../ImagesExt/image8_1556.jpg "*") Auch hier kann eine 0 übergeben werden | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Nettogewich |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Anzahl-pro-Paket | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Strecke | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Strecke |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Gehoert-zu-Zeilennr | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Gehoert-zu-Zeilennr |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Menge-pro-Einheit | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Wird aus dem Original Datensatz zu dem Artikel aus der Datendrehscheibe genommen. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Menge-pro-Einheit' |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Einheitencode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Wird aus dem Original Datensatz zu dem Artikel aus der Datendrehscheibe genommen. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./bmecat:ORDER_UNIT |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preisabschlag | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. (Preisgestalltung liegt bei Agravis) | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preisaufschlag | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. (Preisgestalltung liegt bei Agravis) | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preiseinheitencode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Einheit des Preises | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Preiseinheitencode |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Menge-pro-Preiseinheit | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Wird aus dem Original Datensatz zu dem Artikel aus der Datendrehscheibe genommen. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./PRODUCT_PRICE_FIX/bmecat:PRICE_QUANTITY<br>![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Gebindefaktor | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird keine Info erwartet | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Gebindefaktor |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Basiseinheitencode | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Mengeinheit des Terresartikel, hier muss noch eine Umschlüsselung von UN Norm auf Terres passieren | ![\*](../../ImagesExt/image8_1556.jpg "*") ./bmecat:ORDER_UNIT |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Basiseinheit | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Mengeinheit wird aus dem Orginal Datensatz von Terres genommen. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./bmecat:ORDER_UNIT |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preis | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Preis des Artikels | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Gebindepreis | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird keine Info erwartet | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Gebindepreis |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preisart | ![\*](../../ImagesExt/image8_1556.jpg "*") | ![\*](../../ImagesExt/image8_1556.jpg "*") Ist immer Netto, aber die Preise werdden nicht vearbeitet. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./Preisart |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Palettenanzahl | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird eine 0 übergeben | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Menge-pro-Paletteneinheit | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier kann ein 0 übergeben werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preis-inkl-MWSt | ![\*](../../ImagesExt/image8_1556.jpg "*") Nein | ![\*](../../ImagesExt/image8_1556.jpg "*") | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikel-EDI | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelnummer diese muss noch in die Terresartikelnummer Umgeschlüsselt werden. | ![\*](../../ImagesExt/image8_1556.jpg "*") ./PRODUCT_ID/bmecat:SUPPLIER_PID |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelverfahren-EDI | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") SA | ![\*](../../ImagesExt/image8_1556.jpg "*") |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Einheitencode-EDI | ![\*](../../ImagesExt/image8_1556.jpg "*") Ja | ![\*](../../ImagesExt/image8_1556.jpg "*") PCE | ![\*](../../ImagesExt/image8_1556.jpg "*") |
