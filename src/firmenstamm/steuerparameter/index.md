# Steuerparameter

<!-- source: https://amic.de/hilfe/_steuerparameter.htm -->

Steuerparameter steuern bestimmte Vorgänge im Ablauf von Anwendungen und anderen Programmen in A.eins. Durch die in den Steuerparametern hinterlegten Werte können diese Anwendungen und Programme zu einem bestimmten Verhalten veranlasst werden.

Steuerparameter lassen sich über die Auswahlliste in zwei Darstellungen anzeigen.

1. Einfach (ohne Schlüssel)

2. Komplex (mit Schlüssel)

Die Ansicht wird bereits bei der Erstellung des Steuerparameters gewählt.

Ist der Steuerparameter „einfach“, so zeigt die Darstellung nur die Gültigkeitszeiträume und die Werte des verwendeten Abfrageformats an.

Ist der Steuerparameter „komplex“, so ist auch die Darstellung entsprechend:

| Feld | Bezeichnung |
| --- | --- |
| Gültig ab | Das Datum ab dem der folgende Wert gültig ist |
| Einstellung / individuell | Zeigt die Werte des Abfrageformats zum gewählten Gültigkeitsdatums |
| Schlüssel / individuell | Der Schlüssel der Gültigkeit zum gewählten Gültigkeitsdatum |
| Option / individuell | Der Wert, der zum ggf. verwendeten Schlüssel und dem gewählten Gültigkeitsdatum gültig ist |

<p class="just-emphasize">Neu</p>

Um einen neuen Steuerparameter einzurichten, verwendet man in der Auswahlliste entweder die Funktion „SPA Ändern“ oder das Tastenkürzel „Shift+F5“.

Über „Neu“ oder „F8“ lässt sich nun ein neuer Steuerparameter erstellen.

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer des Steuerparameters wird automatisch gesetzt |
| Bezeichnung | Bezeichnung des Steuerparameters |
| Gruppe | F3 Auswahl zur Wahl der Gruppe für den Steuerparameter |
| Sortierung in der Gruppe | ES wird eine Nummer vorgeschlagen, die der letzte stelle in der Gruppe entspricht. Diese kann angepasst werden.  
 Es ist nur eine Nummer zwischen 0 und 32767 gültig. |
| Ausprägung | F3 Auswahl  
einfach (ohne Schlüssel): Gibt an, das der Steuerparameter nur über das Gültigkeitsdatum und ein Abfrageformat verfügt  
komplex(mit Schlüssel und Option): Gibt an, das auch ein Schlüssel und ein weiterer Wert angegeben werden kann  
zweidimensional(mit Schlüssel): Hier können zu einem Datum mehrere Werte erfasst werden. |
| **Überschrift Schlüsselfeld** | Nur bei komplexer und zweidimensionaler Ausprägung.  
Zur individuellen Überschriftengestaltung des Feldes Schlüssel. Bleibt die Zeile leer, wird davon ausgegangen, dass das Feld nicht verwendet wird. |
| Schlüsselfeld Vorlage | F3 Auswahl  
Hier kann eine Itembox oder ein FS Format angegeben werden. |
| Anfangswert Schlüssel ab | Startwert |
| **Überschrift Option Feld** | Nur bei komplexer Ausprägung.  
Zur individuellen Überschriftengestaltung des Feldes Option. Bleibt die Zeile leer, wird davon ausgegangen, dass das Feld nicht verwendet wird. |
| Optionfeld Vorlage | F3 Auswahl  
Hier kann eine Itembox oder ein FS Format angegeben werden. |
| Anfangswert Option ab | Startwert |
| **Überschrift Bezeichnungsfeld** | Nur bei zweidimensionaler Ausprägung.  
Wird unter Schlüsselfeld „Vorlage“ eine Itembox ausgewählt, dann kann in der hier definierten Spalte ein Bezeichnungsfeld mit angezeigt werden. Hier steht die individuelle Überschrift. Wenn leer, steht über der Spalte „Bezeichnung“ |
| Name Bezeichnungsfeld in IB | Wird unter Schlüsselfeld „Vorlage“ eine Itembox ausgewählt, dann kann hier der Name eines Feldes aus der Itembox eingetragen werden, dessen Inhalt in dieser Spalte als Information angezeigt werden soll. |
| **Überschrift Wertfeld** | Nur bei einfacher oder komplexer Ausprägung.  
Zur individuellen Überschriftengestaltung des Feldes Wert. Bleibt die Zeile leer, wird davon ausgegangen, dass das Feld nicht verwendet wird. |
| Abfrageformat für Wert | F3 Auswahl zur Wahl eines Abfrageformates (z.B. JANEIN) |
| Anfangswert ab | Zunächst ist das Standard Gültigkeitsdatum 01.01.1901 angegeben. Anschließend kann der Startwert angegeben werden |

<p class="just-emphasize">Gültigkeiten</p>

Zeigt den Steuerparameter. Wurde der Steuerparameter einfach (ohne Schlüssel) eingerichtet so erscheint ein einfaches Dialogfenster mit folgender Beschreibung:

| Feld | Bezeichnung |
| --- | --- |
| Gültig ab | Das Datum ab dem der folgende Wert gültig ist |
| Einstellung | Wert des Abfrageformats |

Wurde komplex (mit Schlüssel) verwendet, so öffnet sich ein Dialogfenster mit folgender Beschreibung:

| Feld | Bezeichnung |
| --- | --- |
| Steuerparameternummer | Steuerparameternummer |
| Steuerparameterbezeichnung | Steuerparameterbezeichnung |
| Gültigkeit ab | Datum der Gültigkeit. S.h., ab wann die Einstellungen gültig sind. |
| Erste Spalte | Überschrift individuell.  
Enthält den Wert des eingerichteten Steuerparameters und ist optional eingerichtet. Wird die Spalte nicht benötigt, so ist diese geschützt. Es können mehrere Schlüssel eingerichtet werden, zu denen je nachdem andere Werte und/oder Optionen angegeben werden können. |
| Zweite Spalte | Überschrift individuell.  
Enthält den Schlüssel des eingerichteten Steuerparameters und ist optional eingerichtet. Wird die Spalte nicht benötigt, so ist diese geschützt. |
| Dritte Spalte | Überschrift individuell.  
Enthält die Option des eingerichteten Steuerparameters und ist optional eingerichtet. Wird die Spalte nicht benötigt, so ist diese geschützt. |

<p class="just-emphasize">Gültigkeiten ansehen</p>

Wie unter Gültigkeiten beschrieben, allerdings im „Ansehen-Modus“.

<p class="siehe-auch">Siehe auch:</p>

- [allgemeine Programmsteuerung](./allgemeine_programmsteuerung/index.md)
- [Uhrzeitorientierte Zeiterfassung (SPA 1049)](./uhrzeitorientierte_zeiterfassung_spa_1049.md)
- [MDE Prozeduren Einzelhandel (SPA 1059)](./mde_prozeduren_einzelhandel_spa_1059/index.md)
- [Artikel und Artikelstammdaten](./artikel_und_artikelstammdaten/index.md)
- [Artikelvorbelegungen](./artikelvorbelegungen/index.md)
- [Objektverwaltungswesen](./objektverwaltungswesen/index.md)
- [Bestände und Bewertung](./bestaende_und_bewertung/index.md)
- [Fibu-Übertrag Warenwirtschaft](./fibu_uebertrag_warenwirtschaft/index.md)
- [Folgeartikel](./folgeartikel/index.md)
- [Frachten und Frachtwesen](./frachten_und_frachtwesen/index.md)
- [Hilfsprozeduren](./hilfsprozeduren/index.md)
- [Kasse / Barverkauf](./kasse_barverkauf/index.md)
- [Kasse / Daten aus Strichcode](./kasse_daten_aus_strichcode/index.md)
- [Kontenblatt](./kontenblatt/index.md)
- [Kontraktwesen](./kontraktwesen/index.md)
- [Kundenstammdaten](./kundenstammdaten/index.md)
- [Lizenzen](./lizenzen/index.md)
- [Optionen Finanzwesen](./optionen_finanzwesen/index.md)
- [Optionen global](./optionen_global/index.md)
- [Optionen Warenwirtschaft](./optionen_warenwirtschaft/index.md)
- [Partiewesen](./partiewesen/index.md)
- [Preisfindung](./preisfindung/index.md)
- [Preiskalkulation (generelle Einrichtung)](./preiskalkulation_generelle_einrichtung/index.md)
- [Rezeptur/Stückliste/Produktion](./rezeptur_stueckliste_produktion/index.md)
- [Streckenwesen](./streckenwesen/index.md)
- [Tourverwaltung](./tourverwaltung/index.md)
- [Trennkriterien Umwandlung](./trennkriterien_umwandlung/index.md)
- [Vertreter / Provision (Einrichtung)](./vertreter_provision_einrichtung/index.md)
- [Vorbelegungen ME / Gebinde](./vorbelegungen_me_gebinde/index.md)
- [Vorgangsbearbeitung Spezialitäten](./vorgangsbearbeitung_spezialitaeten/index.md)
- [Vorgangsbearbeitung allg.](./vorgangsbearbeitung_allg/index.md)
- [Vorgangsbearbeitung Positionen](./vorgangsbearbeitung_positionen/index.md)
- [Vorgangsbearbeitung Umwandlung](./vorgangsbearbeitung_umwandlung/index.md)
- [Vorgangsbearbeitung Warenposition](./vorgangsbearbeitung_warenposition/index.md)
- [Scanner](./scanner/index.md)
- [WebPortal](./webportal/index.md)
- [EDI-Datenaustausch](./edi_datenaustausch/index.md)
- [Hauptmenü](./hauptmenue/index.md)
- [Waagensteuerung](./waagensteuerung/index.md)
- [SEPA Testlauf (SPA 969)](./sepa_testlauf_spa_969.md)
- [Bitzer Fehlerprotokoll (SPA 970)](./bitzer_fehlerprotokoll_spa_970.md)
- [Projektverwaltung (SPA 989)](./projektverwaltung_spa_989.md)
- [SMTP - Mailversand (SPA 999)](./smtp_mailversand_spa_999.md)
- [Sybase Umstellung (SPA 1008)](./sybase_umstellung_spa_1008.md)
- [Sybase Umstellung (SPA 1013)](./sybase_umstellung_spa_1013.md)
- [Teilproduktion (SPA 1046)](./teilproduktion_spa_1046.md)
- [FutterApp-Optionen (SPA 1047)](./futterapp_optionen_spa_1047.md)
- [Signierung eines Beleges (SPA 1048)](./signierung_eines_beleges_spa_1048.md)
- [Kassensicherungsverordnung (SPA 1056)](./kassensicherungsverordnung_spa_1056/index.md)
