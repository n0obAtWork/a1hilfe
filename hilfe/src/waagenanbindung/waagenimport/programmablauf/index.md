# Programmablauf

<!-- source: https://amic.de/hilfe/programmablauf.htm -->

Zuerst werden die wesentlichen Scriptparameter aus der Relation ScriptparamPar mit der ScriptPId = „WaagenImport“ eingelesen. Bei nicht vorhandenen oder inaktiv geschalteten Datensätzen wird eine Standardeinstellung für die betreffenden Programmvariablen vorgenommen. Bei schwerwiegenden Fehlern beim Einlesen der Scriptparameter stoppt das Programm mit der Fehlermeldung "SKRIPT FALSCH PARAMETRISIERT!". Es erfolgt ein Eintrag ins Fehlerprotokoll wie „Skript gestoppt: ART_AUS_SORTx=1 und SORT_AUS_ARTx=1“ oder „Skript gestoppt: ART_AUS_SORTx&lt;>1 und ART_SAx=0 bzw. ARTLEN_SAx=0“.

Nach erfolgreichem Einlesen der Parameter, was einige Sekunden benötigt, wird die Datei zum Lesen geöffnet und auf die Platte kopiert.

Bei Einstellung des Parameters MASKE_QUELLPFAD=1 wird zuvor eine Maske angezeigt, die zur Eingabe des vollständigen Pfades der Importdatei auffordert. Wird nichts eingegeben oder ist MASKE_QUELLPFAD=0, so wird der Name der Datei und das Laufwerk durch die Scriptparameter DISK und DATEINAME eingestellt.

Der Parameter WAAGEDAT enthält den vollständigen Dateipfad des Kopier-Zieles auf der Platte. Steht MULT_FILES=1, so werden ALLE Dateien von dem Datenträger gelesen und zusammenkopiert, egal was in DATEINAME steht.

Wird beim Aufruf des Scriptes als 1. Parameter ein Dateiname angegeben, so wird versucht, ausschließlich die angegebene Datei einzulesen, unabhängig von der Einstellung von MULTI_FILES.

Bei dem Vorgang des Einlesens kann es besonders unter Windows 95 zu Problemen kommen (Fehlermeldung „Kann Datei nicht finden / öffnen ...“, die zum Abbruch des Scriptes führen (Behebung s. unten: Fehlermeldungen).

Nun beginnt der eigentliche Einlesevorgang der Daten. Da die Originaldaten nicht markiert sind, kann leider nicht festgestellt werden, ob die Daten doppelt eingelesen werden.

Zunächst wird eine eindeutige UebernahmeId erzeugt, die für alle Datensätze eines Datenimportes gilt. (Außerdem wird an späterer Stelle im Programm für jeden Datensatz aus demselben Nummernkreis eine SatzId bereitgestellt. Bei Fehlern wird immer auf diese beiden Ids verwiesen.)

Hier beginnt die Hauptschleife des Programmes.

Die Datei wird Zeilenweise eingelesen. Ist eine Zeile höchstens 3 Zeichen lang, so wird dies als Dateiende interpretiert.

Nun wird die Satzart ermittelt. Hierzu wird der Parameter SA ausgewertet. Insgesamt sind 4 Satzarten möglich, wobei nicht festgelegt ist, welche Zielansprache einer Satzart zugeordnet wird.

**Tip:** So ist es durchaus möglich, mehreren Satzarten dieselbe Zielansprache zuzuweisen. Je Satzart kann dadurch ein eigener Aufbau der Importdaten in der ASCII-Datei definiert werden.

Es folgt die Ermittlung der einzelnen Daten in der angegebenen Reihenfolge.

Filialnummer

Kann die Filialnummer nicht gelesen werden, so wird bei eingeschaltetem Filialwesen die aktuelle Filiale aus Aeins bezogen. Ansonsten zieht die im Parameter FILIALE_DEFAULT abgelegte Filialnummer. Sie ist 0, wenn auch dieser Parameter nicht lesbar ist.

Eine Validierung findet nicht statt.

(Zugehörige Positionsparameter: FIL_SAx)

Lieferscheinnummer, Wiegenummer

Lässt sich die Lieferscheinnummer nicht lesen, wird sie als 0 angenommen. Dies hat zur Folge, dass in der Weiterverarbeitung keine Zusammenfassung mehrerer Datensätze zu einem Vorgang erfolgen wird. Nur Belege mit gleicher Lieferscheinnummer ungleich 0 können zusammengefasst werden.

(Zugehörige Positionsparameter: LS_SAx, WGN_SAx)

Durch den Parameter WIEGENR_AUS_LS wird anschließend gesteuert, ob die Lieferscheinnummer auch als Wiegenummer verwendet werden soll (Wert1=1). Ist dies der Fall, so wird bei Wert2 gesteuert, ob die Lieferscheinnummer auf 0 gesetzt werden soll (Wert2 = 0) oder nicht (Wert2 = 1). Standardeinstellung für den Parameter WIEGENR_AUS_LS ist Wert1=0, Wert2=1.

Wird die Wiegenummer nicht aus der Lieferscheinnummer erzeugt, so wird die Wiegenummer eingelesen. Ist auch dies nicht möglich, wird als Wiegenummer in allen Fällen 0 eingetragen.

Mengeneinheit der Menge

Die Mengeneinheit wird nach dem Einlesen konvertiert. In den Scriptparametern MEM_1 bis MEM_5 können 5 Umsetzungen von einer Mengeneinheit aus der Importdatei (Wert1) in eine Mengeneinheit (ME_Nummer) des Aeins-Systems (Wert2) definiert werden.

Falls keine Mengeneinheit eingelesen werden kann oder eine Konvertierung durch die Parameter MEM_1 bis MEM_5 nicht möglich ist, z. B. wegen Fehlwert oder Inaktivschaltung des betreffenden Parameters, wird diejenige Aeins-Mengeneinheit vorgegeben, die im Parameter MEM_DEFAULT abgelegt ist.

Eine Validierung findet nicht statt.

(Zugehörige Positionsparameter: MEM_SAx)

Kontraktnummer, Partienummer

Kann die betreffende Nummer nicht gelesen werden, so wird jeweils 0 vorbelegt.

Eine Validierung findet nicht statt.

(Zugehörige Positionsparameter: KTR_SAx, PAR_SAx)

<p class="siehe-auch">Siehe auch:</p>

- [Preis, Preiseinheit](./preis_preiseinheit.md)
- [Bruttokennzeichen, Gesamtpreiskennzeichen](./bruttokennzeichen_gesamtpreiskennzeichen.md)
- [LKW-Kennzeichen](./lkw_kennzeichen.md)
- [Einkaufs-/Verkaufskennzeichen](./einkaufs_verkaufskennzeichen.md)
- [Zielansprache](./zielansprache.md)
- [Vorgangsklasse](./vorgangsklasse.md)
- [Kundennummer (Waagendatenimport-/-export)](./kundennummer_waagendatenimport_export.md)
- [Vertretergruppe](./vertretergruppe.md)
- [Artikel / Sorte / Lager](./artikel_sorte_lager.md)
- [Steuerschlüssel](./steuerschluessel.md)
- [Lagerplatz (Waagendatenimport-/-export)](./lagerplatz_waagendatenimport_export.md)
- [Zugangslager bei Umbuchungen](./zugangslager_bei_umbuchungen.md)
- [Zugangslagerplatz bei Umbuchung](./zugangslagerplatz_bei_umbuchung.md)
- [Weiterverarbeitung der eingelesenen Daten](./weiterverarbeitung_der_eingelesenen_daten.md)
