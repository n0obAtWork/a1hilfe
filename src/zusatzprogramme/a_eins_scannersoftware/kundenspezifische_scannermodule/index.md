# Kundenspezifische Scannermodule

<!-- source: https://amic.de/hilfe/_scanner_module.htm -->

Die mittels Scanner erfassten Daten werden in den Vorgangsimport Tabellen gespeichert. In der Anwendung [Vorgangimport](../../../vorgangsabwicklung/vorgangsimport/index.md) **[VIMP]**

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

können dann die Daten nachträglich bearbeitet werden. Dort besteht dann auch die Möglich aus den erfassten Daten ein A.eins Beleg zu erzeugen.

<p class="just-emphasize">Einrichtung eines Kundenspezifisches Scanner Moduls</p>

Um ein spezifisches Scannermodul aufzurufen muss der [Steuerparameter 801](../../../firmenstamm/steuerparameter/scanner/private_scannerprozedur_spa_801.md) auf Private Prozedur eingestellt werden. In dem Feld „IP-Adresse“ wird die IP-Adresse oder die [Alibi](../a_eins_ce_scannersoftware/index.md#AlibiIPAdresse) IP-Adresse des Scanners hinterlegt. In dem Feld „private Prozedur“ muss dann Prozedur „CallScannerModul“ eingetragen werden. Es kann aber auch eine private Prozedur hinterlegt werden, die ein privates Modul aufruft.

Mit dem setzten des [Steuerparameters 885](../../../firmenstamm/steuerparameter/scanner/private_prozedur_zum_uebersteuern_der_standard_melodie_spa_8.md) kann eine private Prozedur hinterlegt werden, die die Standard Sounds für die Fehler und Erfolgmelodie überscheibt

Mit dem [Steuerparameter 880](../../../firmenstamm/steuerparameter/scanner/html_style_sheet_spa_880.md) kann ein eigens Style-Sheet für die HTML Anzeige auf dem Scanner hinterlegt werden.

<p class="just-emphasize">Lagernummer setzen</p>

Bei der Ersteinrichtung eines Scannerbedieners muss die Lagernummer in den **Vorgangskonstanten** Direktsprung <strong>[VKONS]</strong> gesetzt werden. Dies hat den Grund, weil die Lagernummer aus den **Vorgangskonstanten** des [Bedieners](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) bestimmt, auf welchem Lager der Scanner operiert. Anhand der Lagernummer und dem EAN Code oder der Artikelnummer werden dann die erfassten Artikel gesucht. Dies gilt auch, wenn im A.eins System nur das Lager 0 vorhanden ist.

Nach dem das Lager gesetzt worden ist, kann dieses mittels Scancode auf dem Scanner gewechselt werden.

Aufbau des Scancodes im EAN 128 verschlüsselt

1. LG 1 wobei zwischen dem LG und der 1 ein Leerzeichen ist.

2. VKONS 1 wobei zwischen dem LG und der 1 ein Leerzeichen ist.

Die beiden Befehle können auch manuell per Tastatur eingegeben werden. Beide Befehle ändern die aktuelle Lagernummer auf die Lagernummer 1 ab.

<p class="siehe-auch">Siehe auch:</p>

- [Scanner im Marktbereich](./scanner_im_marktbereich.md)
- [Permanente Inventur](./permanente_inventur.md)
- [Kommissionierung im Ernährungsmittelbereich](./kommissionierung_im_ernaehrungsmittelbereich/index.md)
- [Weitere Module](./weitere_module.md)
- [Terres-Markt Bestellung](./terres_markt_bestellung/index.md)
- [Produktion mit Seriennummern](./produktion_mit_seriennummern.md)
- [Auftrag zu Ladeschein zu Lieferschein](./auftrag_zu_ladeschein_zu_lieferschein.md)
- [Lagerplatzumbuchung](./lagerplatzumbuchung/index.md)
- [Eingangslieferschein](./eingangslieferschein/index.md)
- [Inventur mit der Vorgangsimport Schnittstelle](./inventur_mit_der_vorgangsimport_schnittstelle/index.md)
