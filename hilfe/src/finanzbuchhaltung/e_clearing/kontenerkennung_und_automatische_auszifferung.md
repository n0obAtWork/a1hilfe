# Kontenerkennung und automatische Auszifferung

<!-- source: https://amic.de/hilfe/kontenerkennungundautomatische.htm -->

Hauptmenü > Mahn-/Zahl-/Zinswesen > Zahlungsverkehr > e-Clearing > Funktion ***Kontierung /Auszifferung***

Direktsprung **[ECL]**

Dieser Prüfungslauf kann jederzeit separat gestartet werden. Zusätzlich kann man per ***Option*** **F10** einstellen, dass nach erfolgreicher Dateneinspielung der Prüfungslauf automatisch startet. Folgende Tests werden in der angegebenen Reihenfolge durchgeführt:

Anhand der Bankleitzahl und der Kontonummer wird die Hausbank bestimmt. Ist dies nicht möglich, wird ein Hinweis ausgegeben und dieser Belege als nicht zu verarbeiten markiert. Dieses Problem kann auf mehrere Arten behoben werden:

1. Man trägt die Hausbank manuell über die Funktion „Hausbank zuordnen“ ein. Hier öffnet sich eine F3-Auswahl mit zwei Varianten. Die aktive Variante zeigt alle Hausbanken mit der gleichen IBAN aus dem Kontoauszug an, die zweite Variante zeigt alle Hausbanken an.

2. Die nicht gefundene Bank wird im Hausbankenstamm hinterlegt.

3. Es kann vorkommen, dass in der Übergabedatei die Kontonummern in einem Format übergeben werden, dass nicht dem gängigen Format mit 10-stelligen Kontonummern entspricht. Dafür gibt es im Hausbankenstamm das Feld „abw. Bankkonto e-Clearing“. Ist hier eine Nummer hinterlegt, so wird mit dieser Nummer versucht die Hausbank zu bestimmen. Die Optionen „*Bei 11-stelligem Hausbankkonto "Deutsche Bank" Format annehmen*“ und „*Hausbankkonto bei Commerzbank anpassen*“ werden dann nicht angewendet.  
    

Anschließend werden die einzelnen Positionen verarbeitet. Die Suchstrategie erfolgt in der hier angegebenen Reihenfolge.

Als erstes Suchverfahren kann die [VWZ-Zuordnung](./vwz_zuordnung.md) **F11** herangezogen werden. Die VWZ-Zuordnung wird für die Datensätze gleich zu Beginn ausgeführt, wenn im Feld „Ausführungszeitpunkt“ der Wert „vor Kontenerkennung steht. Wird für den Bereich Gutschrift bzw. Lastschrift im Verwendungszweck der entsprechende Text gefunden, wird das dort hinterlegte Konto entsprechend vorbelegt.

Sind in den übermittelten Datensätzen Bankleitzahl und Kontonummer bzw. BIC und IBAN hinterlegt, wird versucht über diese und die eingetragenen Kundenbanken das Personenkonto zu ermitteln. Wird eine Kontonummer gefunden, wird diese in dem Satz zur späteren Verarbeitung vermerkt. Diese Suchaktion lässt sich in Optionen abstellen.  
    

Ist keine Bankleitzahl und Kontonummer bzw. BIC und IBAN keine Kontonummer in den entsprechenden Feldern hinterlegt bzw. wurde keine dazugehörige Kontonummer gefunden, so wird der Name des Zahlungspflichtigen bzw. des Überweisungsempfängers herangezogen. Ist dieser eindeutig über den Matchcode zuzuordnen, wird diese Kontonummer vorgeschlagen.  
    

Entspricht der Name des Zahlungspflichtigen bzw. des Überweisungsempfängers eindeutig der Kundenbezeichnung, wird die so gefunden Kontonummer vorgeschlagen.  
    

Wird keine entsprechende Bankverbindung im Kundenstamm gefunden, wird im Verwendungszweck nach einer Kundennummer gesucht. Als Kriterien zum Erkennen dieser Nummer werden vorangestellte Kürzel herangezogen. Diese sind unter anderem:  
KD. / KDNR. / KD.NR. / KD.-NR. / KD-NR / KU. / KUNR. / KU.NR. / KU.-NR. / KU-NR / KNR. / K.NR. / KUNDENNUMMER / KUNDENNR. / KUND.NR. / K. / RE-KTO / KONTO / CUST.NO / CUSTOMER  
Wird diese Nummer eindeutig im Kundenstamm gefunden, wird sie so vorgeschlagen. Jetzt kann es jedoch häufig dazu kommen, dass die Nummer nur 5 oder 6-stellig im Kontoauszug geführt wird. Um trotzdem eine Erkennung durchführen zu können, wird Anhand der angegebenen Stellen und dem ersten Teil des Namens eine Übereinstimmung zum Kundenmatchcode, zur Kundenbezeichnung bzw. zum Namen und Vornamen in der Anschrift des Kunden versucht herzustellen. Kann hier eine eindeutige Bestimmung durchgeführt werden, wird diese Kontonummer vorgeschlagen.  
    

Falls mehrere Konten bei der Hausbank geführt werden, und diese als Unterkonten im Hausbankenstamm eingetragen sind, ist eine Bestimmung des dazugehörigen Verrechnungskontos über die Bankleitzahl und die Bankkontonummer möglich.  
    

Als ein weiteres Suchverfahren wird die [VWZ-Zuordnung](./vwz_zuordnung.md) **F11** herangezogen, wenn im Feld „Ausführungszeitpunkt“ der Wert „nach Kontenerkennung steht. Wird für den Bereich Gutschrift bzw. Lastschrift im Verwendungszweck der entsprechende Text gefunden, wird das dort hinterlegte Konto entsprechend vorbelegt.

Das letzte Suchverfahren ist die Bestimmung der Kontonummer anhand der Belegnummer. Hierbei wird ähnlich wie bei der Textsuche nach Kontonummer nach bestimmten vorangestellten Kürzeln gesucht (z.B.: LF.NR / R.-NR / RE.NR / L-NR / RECH.NR / BELEG. NR / BEL. NR / RG.NR / INVOICE /...). Wird zu dieser Nummer ein eindeutiger OP gefunden, so wird hieraus die Kontonummer gezogen. Diese Suchoption muss erst unter [Optionen](./optionen.md) aktiviert werden.  
Zusätzlich wird versucht, alle numerischen Teile als Belegnummer zu interpretieren. Es muss dabei der Betrag des eventuell gefundenen Beleges eindeutig zur Zahlung passen.

Kann keine Kontonummer zugeordnet werden, so wird die Position als nicht zu verarbeiten gekennzeichnet und eine entsprechende Fehlermeldung ausgegeben. Diese Meldung lässt sich dann auf mehrere Arten beheben:  
    

1\. Nachtrag der Bank im Kunden/Lieferantenstamm.  
2\. Eintrag des Matchcodes  
3\. Eintrag der Kundenbezeichnung  
4\. Eintrag des Namens in der Kundenanschrift  
5\. Zuordnung des Verwendungszwecks über [VWZ-Zuordnung](./vwz_zuordnung.md)  
6\. [Manueller Eintrag der Kontonummer](./dta_und_e_clearing.md) in die Position  
7\. [Manuelle Kontenaufteilung](./dta_und_e_clearing.md)

    
Diese Fehlermeldung führt nicht zum Abbruch des Testlaufes.  
    

Es wird Anhand eines Kennzeichens innerhalb des Kunden/Lieferantenstammes geprüft, ob eine automatische Auszifferung erlaubt ist oder nicht. Ist keine Auszifferung erlaubt, wird dies als Hinweis ausgegeben. Dies ist kein Fehler!  
    

Es wird versucht, für den Kunden eine automatische Auszifferung vorzunehmen. Ist die Optionen „Zahlungsreferenz bei Ausziffern verwenden“ ausgewählt, wird diese Suche zuerst ausgeführt(siehe [PayPal](./paypal_freier_datenimport.md)).

Als nächstes wird geprüft, ob in einer der Verwendungszweckzeilen ein numerischer Anteil enthalten ist (entweder linksbündig oder mit Leerzeichen abgetrennt). Es wird versucht diese gefundene Nummer als Rechnungsnummer zu interpretieren und einen fälligen, nicht gesperrten Beleg zu finden. Gelingt dies, muss die Summe sämtlicher Beleg abzüglich dem zu gewährenden Skonto (bezogen auf das Erstelldatum/Valutadatums) der gesamten Summe entsprechen. Entspricht die Summe nicht der Gesamtsumme, wird noch versucht die Summe ohne Abzug von Skonto zu bestimmen. Bei Bestimmung des Skontos ist es möglich per [Option](./optionen.md) **F10** anzugeben, um wie viele Tage das Datum überschritten werden darf.  
    

Ist die Option „*„Diverse“ Kunden nur mit eindeutiger Referenz ausziffern*“ gesetzt und handelt es sich um ein diverses Konto(einstellbar in den Stammdaten[KU] oder [LF] im Register ***Kennzeichen***), werden die folgenden Suchen nicht mehr ausgeführt.

Ansonsten wird noch gesucht, ob ein fälliger OP existiert, der nicht gesperrt ist, der abzüglich Skonto dem Betrag der Position entspricht. Bei Bestimmung des Skontos ist es möglich per [Option](./optionen.md) **F10** anzugeben, um wie viele Tage das Datum überschritten werden darf. Existiert kein passender Beleg inklusive Skonto, wird geprüft, ob ein fälliger OP ohne Skonto diesem Betrag entspricht.  
    

Sollte bisher immer noch keine Auszifferung möglich sein, wird noch versucht alle fälligen OPs heranzuziehen. Die Summe wird einmal mit und einmal ohne Gewährung von Skonto geprüft.  
    

Als letzter Test werden noch alle OPs - egal ob fällig oder nicht - herangezogen.  
    

Werden keine entsprechenden OPs gefunden, wird eine entsprechende Meldung ausgegeben. Vorgeschlagene Auszifferungen können nachträglich vor dem Buchungslauf (Erstellen der echten Belege) gelöscht werden(s.u.).
