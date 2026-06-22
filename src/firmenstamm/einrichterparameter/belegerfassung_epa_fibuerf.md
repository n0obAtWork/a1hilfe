# Belegerfassung (EPA FIBUERF)

<!-- source: https://amic.de/hilfe/_EPA_FIBUERF.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Belegmappe abfragen | Nicht aktiv | Es existieren drei Ausprägungen:  
• Nicht aktiv. Belegmappe wird nicht abgefragt bzw. angezeigt  
• Belegmappe einmal zentral abfragen. Die Belegmappe wird nur einmal im Periodenabfragefenster abgefragt.  
• Belegmappe in der Belegerfassung abfragen. Man hat in der Belegerfassungsmaske zusätzlich die Möglichkeit die Belegmappe zu ändern.  
• |
| Saldo nur aus den letzten zwei Geschäftsjahren ermitteln | Ja | Zur Performancesteigerung existiert die Möglichkeit, den Saldo eines Kontos nur aus den letzten zwei Jahren bestimmen. Dies setzt jedoch voraus, dass der Jahreswechsel ordentlich durchgeführt worden ist.  
 |
| Bei Eingangsrechnungen Anlagenstamm automatisch aufrufen | Ja | Bei Erfassung von Eingangsrechnungen, die als Gegenkonto ein als Anlagekonto gekennzeichnetes Konto verwenden, kann direkt in die Anlagenbuchhaltung verzweigt werden. Dazu muss hier ein „Ja“ eingetragen werden  
 |
| Bei Zahlungen automatisch F6 ? | Nein | Es ist möglich bei der Erfassung von Zahlungsbelegen direkt in die OP-Verwaltung zu springen um die Zahlung sofort zu verrechnen/auszuziffern. Dazu muss hier ein „Ja“ eingetragen werden.  
 |
| Bei Zahlungsverkehr Bank (ZB) Verrechnungskonto aus Hausbank vorbelegen? | Ja | Folgende Einstellmöglichkeiten existieren:  
• Ja: Es wird das Verrechnungskonto verwendet  
• Nein: Es wird das Finanzbuchhaltungskonto verwendet  
 |
| Belegdatum mit Periode prüfen? | Test und Warnung | Folgende Einstellmöglichkeiten existieren:  
• Kein Test  
• Test und Warnung  
• Test und Fehler  
• Teste Jahr mit Warnung  
• Teste Jahr und Fehler  
Beim Test Jahr muss das Belegdatum nur im aktuellen Jahr liegen. Bei Warnung wird nur ein Hinweis auf das inkorrekte Datum gegeben und man kann weiter erfassen.  
 |
| Vorbelegung Belegdatum: 0=Tagesdatum, 1-…=Tage zurück, -1=leer | 0 | Bei Erfassung eines neuen Belegs wird das Belegdatum laut Einstellung vorbelegt. |
| Druckabfrage bei Belegabschluss | Nein | Belege können über ein Formular des Typs 600 „Belegdruck Finanzbuchhaltung“ direkt nach der Erfassung gedruckt werden. Dazu muss hier ein „Ja“ eingetragen werden.  
 |
| Fehler beim Bereichstest Nummernkreis? | Nein | Liegt die eingegebene Nummer nicht im Bereich der im Nummernkreis hinterlegten Werte, so kann man hier einstellen, dass eine Fehlermeldung ausgegeben wird.  
 |
| Erlaubter Kundentyp Ausgangsgutschrift | Alle | Der Kundentyp für diese Belegart kann eingeschränkt werde.  
 |
| Erlaubter Kundentyp Ausgangsrechnungen | Alle | s.o.  
 |
| Erlaubter Kundentyp Eröffnungsbuchung | Alle | s.o.  
 |
| Erlaubter Kundentyp Eingangsgutschrift | Alle | s.o.  
 |
| Erlaubter Kundentyp Eingangsrechnung | Alle | s.o.  
 |
| Erlaubter Kundentyp Sonstige Belege | Alle | s.o.  
 |
| Erlaubter Kundentyp Zahlungen | Alle | s.o.  
 |
| Steuergruppe abfragbar? | Ja | Es gibt drei Ausprägungen:  
• **Nein**: Die vorbelegte Nummer ist nicht änderbar.  
• **Ja:** Die Vorbelegte Nummer ist immer änderbar  
• **Freischaltbar:** Das Feld mit der Steuergruppe kann per Funktion Steuergruppe freischalten CF5 freigeschaltet werden.  
 |
| Gegenkonto nach jeder Zeile auf 0 setzen? | Ja | Stellt man diesen Parameter auf nein, bleibt das Gegenkonto mit der letzten eingegebenen Nummer vorbelegt.  
 |
| Verteilung nur entweder Kostenträger oder Kostenstelle zulassen. | Nein | Mit diesem Parameter lässt sich das gleichzeitige Verteilen eines Betrags auf Kostenstellen und auf Kostenträger Verbieten. Damit kann dann auch nicht gleichzeitig für Verteilkostenträger und Verteilkostenstellen erfasst werden.  
 |
| Belegdatum und Text löschen | Ja | Stellt man diesen Parameter auf nein, so werden nach F9 das Belegdatum und der Belegtext nicht gelöscht.  
 |
| Bei Aufruf der OP-Verwaltung den Obligokunden abfragen? | Nein | Wenn man von der Erfassung einer Zahlung heraus direkt in die OP-Verwaltung springt, kann man hier die Abfrage des Obligokunden erzwingen.  
 |
| Paginiernummer bei ER+EG+EB mit abfragen? | Nein | Die Paginiernummer kann für diese Belegarten manuell geändert werden.  
 |
| Paginiernummer bei allen Belegarten mit abfragen? | Nein | Die Paginiernummer kann für alle Belegarten geändert werden.  
 |
| Paginiernummer muss Daten enthalten? | Nein | Steht hier ein Nein, so ist die Angabe einer Nummer nicht Pflicht.  
 |
| Konto zur Paginiernummer im Archiv mit anpassen. | Nein | Wenn man einem Finanzbuchhaltungs-Beleg eine Paginiernummer zuordnet, zu der bereits ein Beleg mit dieser Belegreferenz (Paginiernummer = FA_BELEGREFERENZ) im Archiv existiert, hat man oft das Problem, dass dort noch kein Konto zugeordnet wurde. Oder man muss die Kontonummer ändern und im Archiv steht dann eine andere Kontonummer als im Beleg.  
Setzt man diesen Einrichterparameter auf **Ja**, so wird geprüft, ob der Archiveintrag ohne Konto oder mit dem alten Konto vor Änderung existiert. Nur in diesem Fall wird das Personenkonto nachgetragen.  
 |
| Periodenwechsel erlaubt? | Nein | Beim Einstieg in die Belegerfassung wird die Periode abgefragt und dann bei jedem Beleg neu Vorbelegt. Trägt man hier ein „Ja“ ein, so kann man die Periode bei der Erfassung ändern.  
 |
| Referenznummer bei AR+AG mit abfragen? | Nein | Referenznummern werden gewöhnlich nur bei Eingangsrechnungen und Gutschriften abgefragt. Hier kann man auch für Ausgangsrechnungen die Abfrage aktivieren.  
 |
| Referenznummer muss Daten enthalten! | Nein | Stellt man diesen Parameter auf „Ja“, so wird geprüft, ob die Referenznummer eingegeben wurde.  
 |
| Referenznummer muss eindeutig sein! | Warnung | Hier können folgende Werte vorkommen:  
• Ignorieren  
Es wird keine Prüfung vorgenommen  
• „Warnung innerhalb der Periode“ und „Fehler innerhalb der Periode“.  
Es wird geprüft, ob diese Referenznummer bereits für diesen Kunden in der aktuellen Periode existiert. Es erscheint dann ein entsprechender Hinweis.  
• „Warnung innerhalb des Geschäftsjahres“ und „Fehler innerhalb des Geschäftsjahres“  
Der Zeitraum, in dem die Referenznummer eindeutig sein muss bezieht sich auf das aktuelle Geschäftsjahr.  
• „Warnung innerhalb zurückliegender Monate“ und „Fehler innerhalb zurückliegender Monate“  
Hier wird geprüft, ob zwischen Tagesdatum und soundso vielen Vormonaten bereits diese Nummer vergeben wurde. Die Anzahl der Vormonate wird mit dem Einrichterparameter „Anzahl zurückliegender Monate für Eindeutigkeitsprüfung“ eingestellt. Bei dieser Prüfung wird anstelle der Perioden das Erfassungsdatum als Bezugszeitpunkt herangezogen.  
    
Hat man „Fehler“ eingestellt, so springt der Cursor ins Feld der Referenznummer und man kann das Feld erst verlassen, wenn eine eindeutige Nummer erfasst wurde.  
Bei dem Test werden auch die Einträge in der Eingangsmappe berücksichtigt.  
 |
| Anzahl zurückliegender Monate für Eindeutigkeitsprüfung | 12 | Für den Einrichterparameter „Referenznummer muss eindeutig sein!“ kann man für die Einstellung „Warnung innerhalb zurückliegender Monate“ und „Fehler innerhalb zurückliegender Monate“ hier die Anzahl der zu prüfenden Monate einstellen.  
 |
| Saldo bei Zahlungsverkehr Kasse/Bank für das Hauptkonto drehen | Nein | \-- |
| Bei Neuerfassung ändern der Kopfinformationen nicht zulassen? | Nein | Bei Erfassung eines neuen Beleges kann man alle Informationen ändern. |
| Steuerfenster automatisch? | Nein | Bei Erfassung von Eingangsrechnungen kann es auf Grund unterschiedlicher Rundungsmechanismen zu Abweichungen der Steuerbeträge kommen. Man kann Belege daher „mit Steueränderung“ abschließen. Hier kann eingestellt werden, dass das Steueränderungsfenster automatisch erscheint.  
 |
| Steuergruppe 0 auch bei Personenkonten erlaubt? | Nein | Steuergruppe 0 ist normalerweise für Sachkonten vorgesehen. Dieser Schalter ist daher eingebaut worden, da auch Personenkonten mit Steuergruppe 0 eingerichtet wurden.  
 |
| Test der Steuerklasse mit der Vorbelegung im Sachkontenstamm | Warnung | Hier kann eingestellt werden, dass die Steuerklasse mit der im Sachkontenstamm getestet wird: Ausprägungen sind „Ignorieren“, „Warnung“ und „Fehler“.  
 |
| Belegdatum + nnn Tage = Valutadatum | 0 | Für Belegarten ohne Zahlungsbedingung (ZA, SO, ZB, SE, KB) kann die Vorbelegung des Valutadatums hier eingerichtet werden.  
 |
| Valutadatum gegenüber Belegdatum prüfen | Ignorieren | Stellt man hier etwas anderes als Ignorieren ein, so wird geprüft, ob auch das Valutadatum innerhalb der Periode liegt.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Ausgangsgutschriften | Verkauf | In den Währungskursen kann man Kurse für Ankauf, Verkauf und Mittel Pflegen. Welcher Kurs bei Fremdwährungsbelegen herangezogen wird, wird hier hinterlegt.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Ausgangsrechnungen | Verkauf | s.o.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Eröffnungsbuchungen | Mittel | s.o.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Eingangsgutschriften | Ankauf | s.o.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Eingangsrechnungen | Ankauf | s.o.  
 |
| Ankauf, Verkauf o. Mittelkurs bei sonstigen Belegen | Mittel | s.o.  
 |
| Ankauf, Verkauf o. Mittelkurs bei Zahlungen | Mittel | s.o.  
 |
| Zahlsperre berücksichtigen? | Warnung | Hier kann hinterlegt werden, dass die Zahlsperre auch bei der Manuellen Erfassung geprüft wird. Für den automatischen Zahlungsverkehr hat dieser Schalter keine Auswirkungen.  
 |
| Bei Zahlungen ins Wertstellungsdatum springen? | Nein | Bei der Belegart ZA wird nach Abschluss einer Position in das Feld Gegenkonto gesprungen. Kommt es im täglichen Betrieb häufiger vor, dass das Wertstellungsdatum pro Position abweicht, ist es sinnvoll hier „Ja“ einzutragen.  
 |
| Bei Zahlungen kein GuV-Konto als Hauptkonto zulassen? | Ja | Hier kann die Sicherheitsprüfung (Bank- und Kasse-Konten müssen Bilanzkonten sein) abgeschaltet werden.  
 |
| Zahlungsbedingung nur mit Bezug Rechnungsdatum zulassen? | Warnung | Zahlungsbedingungen in der Finanzbuchhaltung können nur aufs Rechnungsdatum bezogen sein. Wenn man hier Ignorieren oder Warnung einträgt, so rechnet das System auch bei anderer Einstellung in den Zahlungsbedingungs-Stammdaten mit dem Belegdatum. Bei Warnung wird jedoch noch darauf hingewiesen, dass diese Zahlungsbedingung eine andere Einstellung hat.  
 |
| Steuersatz mit den Kennziffern 37 oder 50 nur für Minderungen erlauben? | Fehler | Hier kann eingestellt werden, ob ein Steuersatz mit den Kennziffern 37 oder 50 nur für Minderungen ausgewählt werden kann.  
 |
