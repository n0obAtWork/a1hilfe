# Hauptmaske

<!-- source: https://amic.de/hilfe/_hauptmaske.htm -->

Stammdatenpflege > Kunden-/Lieferantenstamm > Kundenstamm

Der Aufbau der Kunden und Lieferanten Stammdaten soll exemplarisch am Beispiel des Kundenstamms erläutert werden. Auf Unterschiede wird separat eingegangen. Folgende Informationen werden in Abhängigkeit von der Anwendung in der Hauptmaske erfasst:

**Nummer**

<p class="just-emphasize">Achtung:</p>

Eine Kontonummer kann nur einmal vergeben werden!

Auch wenn ein Kunde gelöscht wird, kann diese Kontonummer nicht erneut vergeben werden.

Dies ist der sichtbare (logische) Kundenschlüssel für Sortierungen und Selektionen.

Die Kunden-/Lieferantennummer ist eindeutiges Suchkriterium. Die Länge der Nummer kann in den Steuerparametern **[SPA]** eingestellt werden; die Voreinstellung beträgt **8** Stellen Eine sichere Verwaltung des Kundennummernbereiches wird über einen im Mandantenstamm hinterlegten Nummernkreis gewährleistet. Darüber hinaus kann in den **[SPA]** bestimmt werden, dass bei einer Neuanlage immer die nächste freie Nummer vorgeschlagen wird.

**Kurzbezeichnung**

Kurzbezeichnung des Kunden für Listen etc.; vorgeschlagen wird der Nachname.

**Matchcode**

Suchbegriff für diesen Kunden; vorgeschlagen wird der Nachname.

**Gegen-Nummer**

Die Kontonummer für Kunden/Lieferanten wird von A.eins je nach Einstellung vergeben. Bei diesen Kunden existiert unsere Firma jedoch auch als Lieferant/Kunde mit einer eigenen Kontonummer. Diese Nummer wird hier eingetragen. Sie wird z.B. beim DTA verwendet oder kann beim Vorgangsdruck mit ausgegeben werden.

**Kundentyp**

Debitor: Über das Konto werden ausschließlich Verkäufe abgewickelt

Kontokorrent: Kunde und Lieferant in einer Person, über das Konto werden Verkäufe und Einkäufe abgewickelt

Interessent: über das Konto werden keine Verkäufe abgewickelt; die Daten stehen aber z.B. für Marketingmaßnahmen zur Verfügung.

Muster – Debitor: dient als Standardvorbelegung für die Debitorenerfassung

Muster – Interessent: dient als Standardvorbelegung für die Interessenerfassung

Der Kundentyp kann unter bestimmten Voraussetzungen nachträglich geändert werden.

1. Es darf kein Datensatz in der Finanzbuchhaltung zu der gewählten Kundennummer existieren.

2. Die Bedingungen der folgenden Tabelle sind erfüllt.

| **Von** | **Nach** | **Bedingung** | **Nummernvergabe** |
| --- | --- | --- | --- |
| Interessent | Kunde | zulassen | Nummernkreis neu ziehen, existierende Vorgänge werden passend umgeändert |
| Interessent | Lieferant | zulassen | Nummernkreis neu ziehen, existierende Vorgänge werden passend umgeändert |
| Interessent | Kontokorrent | zulassen | Nummernkreis neu ziehen, existierende Vorgänge werden passend umgeändert |
| Kontokorrent | Kunde | Die Kundennummer ändert sich nicht.<br>Es existieren keine Einkaufsbelege zu dieser Kundennummer. | keine Nummernänderung möglich |
| Kontokorrent | Lieferant | Die Kundennummer ändert sich nicht.<br>Es existieren keine Verkaufsbelege zu dieser Kundennummer. | keine Nummernänderung möglich |
| Kontokorrent | Interessent | Es sind keine Belge zu dieser Kundennummer vorhanden | Nummernkreis neu ziehen |
| Lieferant | Kunde | Es sind keine Belge zu dieser Kundennummer vorhanden | Nummernkreis neu ziehen |
| Lieferant | Kontokorrent | Die Kundennummer ändert sich nicht | Nummernkreis neu ziehen |
| Lieferant | Interessent | Es sind keine Belge zu dieser Kundennummer vorhanden | Nummernkreis neu ziehen |
| Kunde | Lieferant | Es sind keine Belge zu dieser Kundennummer vorhanden | Nummernkreis neu ziehen |
| Kunde | Kontokorrent | Die Kundennummer ändert sich nicht | Nummernkreis neu ziehen |
| Kunde | Interessent | Es sind keine Belge zu dieser Kundennummer vorhanden | Nummernkreis neu ziehen |
| | | | | |

**Anschrift**

Erfassung der Adressinformation des Kunden, bestehend aus **Anrede, Vorname, Name, Straße, Postleitzahl, Ort, Ortsteil, Telefon- und Fax - Nr.** Die maximale Eingabelänge der Felder ist größer als auf der Maske dargestellt. Über den **"Rand"** kann also weitergeschrieben werden. Für Kunden, die zur Anschrift auch eine Postfachanschrift haben, kann diese mit der Funktion ***Anschriftenpflege*** **F10** erfasst werden. Bei der Erfassung der Postleitzahl wird automatisch geprüft, ob sie bereits vorhanden ist. Wenn ja, wird der Ort vorgeschlagen. Darüber hinaus besteht die Möglichkeit, mittels **F3** nach Postleitzahl oder Ort zu suchen. Im Kundenstamm ist es jetzt auch möglich, für einen Kunden mehrere Adressen zu hinterlegen. Hierbei gibt es nur eine Hauptadresse, die auch nicht gelöscht werden kann, da sonst der Kunde nicht mehr in der Auswahlliste zu sehen war, bzw. die Informationsfelder in der Finanzbuchhaltung leer bleiben würden. Wenn jetzt bei solchen Kunden die Anschrift gepflegt werden soll, wird zusätzlich angezeigt, ob es sich bei der in Bearbeitung befindlichen Anschrift um die Hauptadresse handelt. Wenn dieses der Fall ist, steht in der ersten Zeile der Maske "Adresstyp (Hauptadresse)". Falls es sich bei der in Bearbeitung befindlichen Adresse nicht um die Hauptadresse handelt, steht in der ersten Zeile der Maske "Adresstyp". Mit der Funktion **CTRL**+**F9** lässt sich eine andere als die bisherige Adresse als Hauptadresse festlegen.

Auf der Hauptanschrift kann für die Nachhaltigkeitsvorbelegung ein Anbauland gespeichert werden. Wenn bei der Belegerfassung keine Kundenversandanschrift angegeben wurde, dann wird bei richtiger Nachhaltigkeitseinrichtung das Anbauland aus der Hauptanschrift gezogen. Im Kunden muss also ein Nachhaltigkeitszertifikat existieren und der gewünschte Artikel muss in Verbindung mit dem Anbauland aus der Hauptanschrift übereinstimmen. Ansonsten wird das Anbauland aus der Hauptanschrift ignoriert.

**E-Mail**

Im Kundenstamm ist es möglich, die Standard e-Mail Adresse für den Kunden einzutragen. Im [Anschriftenpfleger](../anschriften/registerkarten_in_anschriften/allgemein.md#Anschriften_Mail) erscheint dieses unter „Standard e-Mail 1“.  
Die Eintragung erfolgt in der allgemein bekannten Form **(z.B. Mustermann@Beispiel.de**). Das **@** Zeichen ist über die Tastenkombination rechte Alt Taste und Q Taste zu erreichen

**Mobiltelefonnummer**

Eingabe der Telefonnummer

**EDI-Partner**

Wenn man EDI benutzen möchte, muss hier für den jeweiligen Kunden der Partner hinterlegt werden. In diesem Partner sind nämlich Profile hinterlegt, mit der die EDI-Nachrichten erstellt werden.  
Die Profile werden im Direktsprung **[EDI]** gepflegt. Dort kann man dann mit der Funktion **Shift+F9** ***(B und N Setup)*** die Formatierung der Nachricht anpassen.

**Partner 1**

Eingabe der/des Ansprechpartner/s in der Firma.

**Bankverbindung**

Kontonummer des Kunden. Bei der Neuanlage muss die Bank im Bankenstamm angelegt sein, die mit der Kontonummer erfasst werden soll. Die Bankdaten werden hier nur angezeigt und können in einer separaten Maske, die von hier per Mausklick im Menü oder über **Shift**+**F9** erreicht werden kann, erfasst werden.

**Kreditlimit**

Dieses Feld gibt das aktuell für den Kunden/Lieferanten zur Verfügung gestellte Kreditlimit an. Durch die Einstellung „**nein**“ des Steuerparameters 503 [„*Alle Kredite als Summe übernehmen*“](../../../firmenstamm/steuerparameter/kundenstammdaten/alle_kredite_als_summe_uebernehmen_spa_503.md), lässt sich dieses Feld bearbeiten. Angegebene Werte werden beim Speichern einer Änderung auch an den Kreditlimitpfleger, Funktion „**Kreditvergabe**“ in der Funktionsbox, übertragen.

**Fakturiersperre**

Für jeden Kunden kann eine weiche oder eine harte Fakturiersperre eingestellt werden. Bei einer weichen Sperre wird beim Versuch einer Fakturierung eine Warnung angezeigt. Die Fakturierung ist aber trotzdem möglich. Bei einer harten Sperre dagegen ist der Kunde komplett für Fakturierung gesperrt.

**Fakturiersperrengrund**

Wenn der Kunde für Fakturierung gesperrt ist, kann hier ein Grund für die Sperre angegeben werden, der zusätzlich zur normalen Fehlermeldung angezeigt wird. Siehe auch Sperrbemerkungen.

**Preisklasse EK/VK**

Gibt an welcher Preisklasse der Kunde zugeordnet ist. In Zusammenhang mit der **"Preisfindung auf der Grundlage von Preislisten"** wird dann die dieser Preisklasse zugeordnete Preisliste gezogen.

**Steuergruppe**

Die dem Kunden zugeordnete Steuergruppe. Auslandskunden wird so z.B. die Steuergruppe **"Auslandskunde"** zugeordnet, bei der die Mehrwertsteuerermittlung entfällt und eine Auswertung nach Auslandsumsätzen möglich wird. In der Regel kann hier die Eintragung „Standard“ übernommen werden. Die Steuer ergibt sich dann aus der Eintragung beim Artikel (voller oder reduzierter Steuersatz).

**Fakturiergruppe**

Steuerungskennzeichen bei der Fakturierung, z.B. um die automatische Erstellung von Monatsrechnungen für bestimmte Kundengruppen zu ermöglichen

**Vertretergruppe**

Verweis auf die zugeordneten Vertreter. Siehe hierzu auch [Vertreterdaten](../../../firmenstamm/vertreterdaten/index.md)**.**

**Zahlungsbedingung EK/VK**

Die dem Kunden zugeordnete Zahlungsbedingung

**USt.-Ident**

Hier wird die Umsatzsteueridentifikationsnummer des Kunden eingetragen

**Handelsname**

Vom Firmennamen abweichender Handelsname.

**Handelsregister**

Nummer des Eintrags im Handelsregister.

**Aktiv**

<p class="just-emphasize">Achtung!!!</p>

*Die ehemalige Variante unter **[ZMDO]*** *die dieses Kennzeichen auswertet (Zusammenfassende Meldung) ist veraltet. Das Kennzeichen wird nicht weiter unterstützt.  
Die aktuell gültige Variante in **[ZMDO]*** *ist „Zusammenfassende Meldung n. AWPosition“. Diese benutzt das Aktiv-Kennzeichen nicht.*

Dieses Feld ist nur betretbar, wenn im Feld USt.-Ident eine Umsatzsteueridentifikationsnummer eingetragen wurde. Ansonsten ist das Feld deaktiviert und belegt mit ‚Nein‘.  
Für den Fall, dass eine Umsatzsteueridentifikationsnummer angegeben wurde, hängt die Vorbelegung des Feldes aktiv vom Einrichterparameter „Umsatzsteuer-ID nach Eingabe automatisch aktivieren?“ ab. Der Einrichterparameter ist vorbelegt mit ‚Nein‘.  
Dieses Feld bestimmt, ob die Umsatzsteueridentifikationsnummer in der zusammenfassenden Meldung **[ZMDO]** in der Variante mit angezeigt wird.

**BBN / BBS / ILN – Nummer**

Die Betriebsstättennummer kann hier ebenso wie die ILN Nummer (International Location Number) für Auswertungen / Ausdrucke hinterlegt werden

**W-IdNr.**

Die W-IdNr. (Wirtschafts-Identifikationsnummer) soll in Deutschland als eindeutiges und dauerhaftes Identifikationsmerkmal für Steuerzwecke eingeführt werden. Sie wird voraussichtlich erst 2015 kommen, wurde jedoch im Zuge der Einführung der e-Bilanz bereits in die Stammdaten aufgenommen.

**EKS-Nr.**

Die aktuelle Einkommensteuernummer des Kunden. So lange nur eine Einkommensteuernummer angegeben ist, kann sie hier bearbeitet werden. Wenn aber mehrere Nummern mit ihre jeweiligen Gültigkeitszeiträume gepflegt werden sollen, geschieht das mit der Funktion ***Einkommensteuernummer bearbeiten***.

**Bemerkung**

Hier kann ein freier, 10 Zeilen langer, Text für Bemerkungen zum Kunden eingetragen werden. Mittels **SF10** kann der Texteditor für komfortablere Bearbeitung aufgerufen werden.

<p class="siehe-auch">Siehe auch:</p>

- [Funktion Löschen/Wiederherstellen](./funktion_loeschen_wiederherstellen.md)
- [Sharepoint](./sharepoint.md)
- [Kreditlimit](./kreditlimit.md)
- [Zolldaten (Kundenstamm)](./zolldaten_kundenstamm.md)
