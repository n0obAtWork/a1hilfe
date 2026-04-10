# Hauptmaske

Stammdatenpflege > Kunden-/Lieferantenstamm > Kundenstamm

Der Aufbau der Kunden und Lieferanten Stammdaten soll exemplarisch am Beispiel des Kundenstamms erläutert werden. Auf Unterschiede wird separat eingegangen. Folgende Informationen werden in Abhängigkeit von der Anwendung in der Hauptmaske erfasst:

<p class="just-emphasize bottom-closer-together">Nummer</p>

> [!WARNING]
> Eine Kontonummer kann nur einmal vergeben werden!<br>
> Auch wenn ein Kunde gelöscht wird, kann diese Kontonummer nicht erneut vergeben werden.

Dies ist der sichtbare (logische) Kundenschlüssel für Sortierungen und Selektionen.

Die Kunden-/Lieferantennummer ist eindeutiges Suchkriterium. Die Länge der Nummer kann in den Steuerparametern **[SPA]** eingestellt werden; die Voreinstellung beträgt 8 Stellen Eine sichere Verwaltung des Kundennummernbereiches wird über einen im Mandantenstamm hinterlegten Nummernkreis gewährleistet. Darüber hinaus kann in den **[SPA]** bestimmt werden, dass bei einer Neuanlage immer die nächste freie Nummer vorgeschlagen wird.

<p class="just-emphasize bottom-closer-together">Kurzbezeichnung</p>

Kurzbezeichnung des Kunden für Listen etc.; vorgeschlagen wird der Nachname.

<p class="just-emphasize bottom-closer-together">Matchcode</p>

Suchbegriff für diesen Kunden; vorgeschlagen wird der Nachname.

<p class="just-emphasize bottom-closer-together">Gegen-Nummer</p>

Die Kontonummer für Kunden/Lieferanten wird von A.eins je nach Einstellung vergeben. Bei diesen Kunden existiert unsere Firma jedoch auch als Lieferant/Kunde mit einer eigenen Kontonummer. Diese Nummer wird hier eingetragen. Sie wird z.B. beim DTA verwendet oder kann beim Vorgangsdruck mit ausgegeben werden.

<p class="just-emphasize bottom-closer-together">Kundentyp</p>

Debitor: Über das Konto werden ausschließlich Verkäufe abgewickelt

Kontokorrent: Kunde und Lieferant in einer Person, über das Konto werden Verkäufe und Einkäufe abgewickelt

Interessent: über das Konto werden keine Verkäufe abgewickelt; die Daten stehen aber z.B. für Marketingmaßnahmen zur Verfügung.

Muster – Debitor: dient als Standardvorbelegung für die Debitorenerfassung

Muster – Interessent: dient als Standardvorbelegung für die Interessenerfassung

<p class="just-emphasize bottom-closer-together">Anschrift</p>

Erfassung der Adressinformation des Kunden, bestehend aus **Anrede, Vorname, Name, Straße, Postleitzahl, Ort, Ortsteil, Telefon- und Fax - Nr**. Die maximale Eingabelänge der Felder ist größer als auf der Maske dargestellt. Über den **"Rand"** kann also weitergeschrieben werden. Für Kunden, die zur Anschrift auch eine Postfachanschrift haben, kann diese mit der Funktion ***Anschriftenpflege*** **F10** erfasst werden. Bei der Erfassung der Postleitzahl wird automatisch geprüft, ob sie bereits vorhanden ist. Wenn ja, wird der Ort vorgeschlagen. Darüber hinaus besteht die Möglichkeit, mittels F3 nach Postleitzahl oder Ort zu suchen. Im Kundenstamm ist es jetzt auch möglich, für einen Kunden mehrere Adressen zu hinterlegen. Hierbei gibt es nur eine Hauptadresse, die auch nicht gelöscht werden kann, da sonst der Kunde nicht mehr in der Auswahlliste zu sehen war, bzw. die Informationsfelder in der Finanzbuchhaltung leer bleiben würden. Wenn jetzt bei solchen Kunden die Anschrift gepflegt werden soll, wird zusätzlich angezeigt, ob es sich bei der in Bearbeitung befindlichen Anschrift um die Hauptadresse handelt. Wenn dieses der Fall ist, steht in der ersten Zeile der Maske "Adresstyp (Hauptadresse)". Falls es sich bei der in Bearbeitung befindlichen Adresse nicht um die Hauptadresse handelt, steht in der ersten Zeile der Maske "Adresstyp". Mit der Funktion CTRL+F9 lässt sich eine andere als die bisherige Adresse als Hauptadresse festlegen.

Auf der Hauptanschrift kann für die Nachhaltigkeitsvorbelegung ein Anbauland gespeichert werden. Wenn bei der Belegerfassung keine Kundenversandanschrift angegeben wurde, dann wird bei richtiger Nachhaltigkeitseinrichtung das Anbauland aus der Hauptanschrift gezogen. Im Kunden muss also ein Nachhaltigkeitszertifikat existieren und der gewünschte Artikel muss in Verbindung mit dem Anbauland aus der Hauptanschrift übereinstimmen. Ansonsten wird das Anbauland aus der Hauptanschrift ignoriert.

<p class="just-emphasize bottom-closer-together">E-Mail</p>

Im Kundenstamm ist es möglich, die Standard e-Mail Adresse für den Kunden einzutragen. Im Anschriftenpfleger erscheint dieses unter „Standard e-Mail 1“.

Die Eintragung erfolgt in der allgemein bekannten Form (**z.B. Mustermann@Beispiel.de**). Das **@** Zeichen ist über die Tastenkombination rechte Alt Taste und Q Taste zu erreichen

<p class="just-emphasize bottom-closer-together">Mobiltelefonnummer</p>

Eingabe der Telefonnummer

<p class="just-emphasize bottom-closer-together">EDI-Partner</p>

Wenn man EDI benutzen möchte, muss hier für den jeweiligen Kunden der Partner hinterlegt werden. In diesem Partner sind nämlich Profile hinterlegt, mit der die EDI-Nachrichten erstellt werden.

Die Profile werden im Direktsprung **[EDI]** gepflegt. Dort kann man dann mit der Funktion **Shift+F9** ***(B und N Setup)*** die Formatierung der Nachricht anpassen.

<p class="just-emphasize bottom-closer-together">Partner 1</p>

Eingabe der/des Ansprechpartner/s in der Firma.

<p class="just-emphasize bottom-closer-together">Bankverbindung</p>

Kontonummer des Kunden. Bei der Neuanlage muss die Bank im Bankenstamm angelegt sein, die mit der Kontonummer erfasst werden soll. Die Bankdaten werden hier nur angezeigt und können in einer separaten Maske, die von hier per Mausklick im Menü oder über **Shift+F9** erreicht werden kann, erfasst werden.

<p class="just-emphasize bottom-closer-together">Kreditlimit</p>

Dieses Feld gibt das aktuell für den Kunden/Lieferanten zur Verfügung gestellte Kreditlimit an. Durch die Einstellung **„nein“** des Steuerparameters 503 *„Alle Kredite als Summe übernehmen“*, lässt sich dieses Feld bearbeiten. Angegebene Werte werden beim Speichern einer Änderung auch an den Kreditlimitpfleger, Funktion **„Kreditvergabe“** in der Funktionsbox, übertragen.

<p class="just-emphasize bottom-closer-together">Fakturiersperre</p>

Für jeden Kunden kann eine weiche oder eine harte Fakturiersperre eingestellt werden. Bei einer weichen Sperre wird beim Versuch einer Fakturierung eine Warnung angezeigt. Die Fakturierung ist aber trotzdem möglich. Bei einer harten Sperre dagegen ist der Kunde komplett für Fakturierung gesperrt.

<p class="just-emphasize bottom-closer-together">Fakturiersperrengrund</p>

Wenn der Kunde für Fakturierung gesperrt ist, kann hier ein Grund für die Sperre angegeben werden, der zusätzlich zur normalen Fehlermeldung angezeigt wird. Siehe auch Sperrbemerkungen.

<p class="just-emphasize bottom-closer-together">Preisklasse EK/VK</p>

Gibt an welcher Preisklasse der Kunde zugeordnet ist. In Zusammenhang mit der **"Preisfindung auf der Grundlage von Preislisten"** wird dann die dieser Preisklasse zugeordnete Preisliste gezogen.

<p class="just-emphasize bottom-closer-together">Steuergruppe</p>

Die dem Kunden zugeordnete Steuergruppe. Auslandskunden wird so z.B. die Steuergruppe **"Auslandskunde"** zugeordnet, bei der die Mehrwertsteuerermittlung entfällt und eine Auswertung nach Auslandsumsätzen möglich wird. In der Regel kann hier die Eintragung „Standard“ übernommen werden. Die Steuer ergibt sich dann aus der Eintragung beim Artikel (voller oder reduzierter Steuersatz).

<p class="just-emphasize bottom-closer-together">Fakturiergruppe</p>

Steuerungskennzeichen bei der Fakturierung, z.B. um die automatische Erstellung von Monatsrechnungen für bestimmte Kundengruppen zu ermöglichen

<p class="just-emphasize bottom-closer-together">Vertretergruppe</p>

Verweis auf die zugeordneten Vertreter. Siehe hierzu auch Vertreterdaten.

<p class="just-emphasize bottom-closer-together">Zahlungsbedingung EK/VK</p>

Die dem Kunden zugeordnete Zahlungsbedingung

<p class="just-emphasize bottom-closer-together">USt.-Ident</p>

Hier wird die Umsatzsteueridentifikationsnummer des Kunden eingetragen

<p class="just-emphasize bottom-closer-together">Handelsname</p>

Vom Firmennamen abweichender Handelsname.

<p class="just-emphasize bottom-closer-together">Handelsregister</p>

Nummer des Eintrags im Handelsregister.

<p class="just-emphasize bottom-closer-together">Aktiv</p>

> [!WARNING]
> *Die ehemalige Variante unter **[ZMDO]** die dieses Kennzeichen auswertet (Zusammenfassende Meldung) ist veraltet. Das Kennzeichen wird nicht weiter unterstützt.*
> 
> *Die aktuell gültige Variante in **[ZMDO]** ist „Zusammenfassende Meldung n. AWPosition“. Diese benutzt das Aktiv-Kennzeichen nicht.*

Dieses Feld ist nur betretbar, wenn im Feld USt.-Ident eine Umsatzsteueridentifikationsnummer eingetragen wurde. Ansonsten ist das Feld deaktiviert und belegt mit ‚Nein‘.

Für den Fall, dass eine Umsatzsteueridentifikationsnummer angegeben wurde, hängt die Vorbelegung des Feldes aktiv vom Einrichterparameter „Umsatzsteuer-ID nach Eingabe automatisch aktivieren?“ ab. Der Einrichterparameter ist vorbelegt mit ‚Nein‘.

Dieses Feld bestimmt, ob die Umsatzsteueridentifikationsnummer in der zusammenfassenden Meldung **[ZMDO]** in der Variante mit angezeigt wird.

<p class="just-emphasize bottom-closer-together">BBN / BBS / ILN – Nummer</p>

Die Betriebsstättennummer kann hier ebenso wie die ILN Nummer (International Location Number) für Auswertungen / Ausdrucke hinterlegt werden

<p class="just-emphasize bottom-closer-together">W-IdNr.</p>

Die W-IdNr. (Wirtschafts-Identifikationsnummer) soll in Deutschland als eindeutiges und dauerhaftes Identifikationsmerkmal für Steuerzwecke eingeführt werden. Sie wird voraussichtlich erst 2015 kommen, wurde jedoch im Zuge der Einführung der e-Bilanz bereits in die Stammdaten aufgenommen.

<p class="just-emphasize bottom-closer-together">EKS-Nr.</p>

Die aktuelle Einkommensteuernummer des Kunden. So lange nur eine Einkommensteuernummer angegeben ist, kann sie hier bearbeitet werden. Wenn aber mehrere Nummern mit ihre jeweiligen Gültigkeitszeiträume gepflegt werden sollen, geschieht das mit der Funktion ***Einkommensteuernummer bearbeiten***.

<p class="just-emphasize bottom-closer-together">Bemerkung</p>

Hier kann ein freier, 10 Zeilen langer, Text für Bemerkungen zum Kunden eingetragen werden. Mittels SF10 kann der Texteditor für komfortablere Bearbeitung aufgerufen werden.

<p class="siehe-auch">Siehe auch:</p>

- [Funktion Löschen / Wiederherstellen](fkt_loeschen_wiederherstellen.md)
- [Sharepoint](sharepoint.md)
- [Kreditlimit](kreditlimit.md)
- [Zolldaten (Kundenstamm)](zolldaten_kundenstamm.md)
