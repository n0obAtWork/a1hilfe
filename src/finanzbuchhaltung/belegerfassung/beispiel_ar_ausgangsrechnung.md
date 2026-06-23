# Beispiel AR Ausgangsrechnung

<!-- source: https://amic.de/hilfe/beispielarausgangsrechnung.htm -->

Hauptmenü > Finanzbuchhaltung > Erfassung > Belegerfassung

Direktsprung **[FIBE]**

**Erfassung  
**Der Belegkreis dient der Erfassung von Ausgangsrechnungen, die lediglich finanzbuchhalterisch gebucht werden sollen. Diese Buchungen haben keinen Einfluss auf die Warenwirtschaft. Bei angeschlossener Warenwirtschaft handelt es sich dann in der Regel um Kostenrechnungen, in die die Daten einfließen sollen.

Es können folgende Daten erfasst werden.

**Belegdatum  
**Hierbei handelt es sich um das Belegdatum der Ausgangsrechnung. Das Feld ist vorbelegt mit dem Tagesdatum, kann jedoch überschrieben werden. Für das Belegdatum spielen drei Einrichterparameter eine Rolle:

1. Belegdatum und Text löschen  
Das Belegdatum wird beim ersten Einstieg mit dem Tagesdatum bzw. – falls das Tagesdatum nicht in der aktiven Periode liegt - dem Enddatum der aktiven Periode vorbelegt. Wenn dieser Einrichterparameter auf JA steht, wird das Belegdatum bei jedem Datensatz so vorbelegt.

2. Vorbel. Belegdat 0=Tagesdatum; 1-…=Tage zurück; -1=leer  
Wenn das Belegdatum vorbelegt werden soll (s.o.) dann kann man hier noch genauer definieren, nach welcher Regel dies geschehen soll.

3. Belegdatum mit Periode prüfen?  
Man kann hier hinterlegen, wie nach der Eingabe des Belegdatums die Prüfung mit der zugeordneten Periode zu erfolgen hat:  
0 = Kein Test.  
1 (Test und Warnung)= Testet, ob das Belegdatum in der Periode liegt und gibt ggf. eine entsprechende Meldung aus. Man kann weiterarbeiten.  
2 (Test und Fehler) = Testet, ob das Belegdatum in der Periode liegt und erlaubt das Weiterarbeiten nur, wenn das Datum in der aktiver Periode liegt.  
3 (Test Jahr und Warnung)= Testet, ob das Belegdatum in dem Jahr liegt und gibt ggf. eine Meldung aus. Man kann trotz Meldung weiterarbeiten.  
4 (Test Jahr und Fehler)= Testet, ob das Belegdatum in dem Jahr liegt. Weiterarbeiten ist nur dann möglich, wenn das Datum im Jahr liegt

**Belegkreisnummer  
**Die Ausgangsrechnungen werden über einen Belegkreis verwaltet. Für den Belegkreis **AR** sollte ein eigener Nummernkreis angelegt werden, um eine Trennung zu den anderen Belegkreisen und zum Nummernkreis der Ausgangsrechnungen in der Warenwirtschaft zu erreichen. Mit Eingabe der Belegkreisnummer wird dann automatisch die nächste zulässige Belegnummer vorgeschlagen. Mit dieser Belegnummer wird der Beleg auch in der OP-Verwaltung geführt.

**Konto  
**Eingabe der gewünschten Kontonummer des Hauptkontos. Die Eingabe erfolgt entweder durch manuelle Erfassung der Kontonummer oder durch Aufruf der Auswahlliste mittels **F3**. Zulässig ist hier nur ein Debitorenkonto.  
Rechts neben dem Eingabebereich werden die Kontobezeichnung (der Name) sowie der aktuelle Saldo angezeigt. Diese Anzeige ändert sich mit jeder erfassten Position des Beleges.

**OP-Text  
**Hierbei handelt es sich um einen frei eingebbaren Text, der zur Erläuterung der Buchung auf dem Hauptkonto mit abgespeichert wird. Er wird später auf Kontoblättern, Journalen, etc. mit ausgedruckt. Neben der manuellen Texteingabe kann auch über **F3** auf einen Textbaustein zugegriffen werden. Darüber hinaus wird der offene Posten mit diesem Text versehen.

**Zahlungsbedingung/Skontodatum**

Hier werden die vereinbarten Zahlungsbedingungen hinterlegt. Sie wirken sich später beim Zahlungsverkehr, Mahnwesen oder bei der Zinsrechnung aus und müssen entsprechend sorgfältig erfasst werden.

**Folgende Möglichkeiten bestehen:**

**Zugriff auf gespeicherte Zahlungsbedingungen  
**Mit **F3** im ersten Feld wird eine Liste der eingerichteten Zahlungsbedingungen angezeigt.  
Die zugrundeliegenden Daten (Skontodatum und betrag, Zieldatum) werden übernommen und es wird in das Feld "Gegenkonto" verzweigt, wo die Erfassung fortgesetzt werden kann, wenn die Daten so gelten sollen.  
Ist dies nicht der Fall, kann mit ­ in die Felder Skontosatz, Valutadatum und Zahlungsbedingung positioniert und Änderungen vorgenommen werden.

**Manuelle Eingabe  
**Die Zahlungsbedingung 0 erlaubt eine manuelle Vergabe der Zahlungs-/ Skontobedingungen. Skonto- und Valutadatum werden mit dem Tagesdatum vorbelegt, das überschrieben werden kann. Der Skontosatz kann eingetragen werden.

**Fällig am  
**Datum der Fälligkeit der Rechnung. Dieses Datum ist wichtig für die Ermittlung von Skontoziehungsfristen, Mahnfristen etc. und muss korrekt eingegeben werden.

**Skontosatz  
**Eingabe des Skontosatzes

**Achtung:  
**Nach Eingabe der Zahlungsbedingung verzweigt das Programm sofort zum "Gegenkonto". Wenn gegenüber der Standardvorbelegung in den Feldern "Skontodatum", "Fällig am", "Skontosatz" Änderungen vorgenommen werden sollen, so erfolgt dies mittels der ­- Taste aus dem Feld "Gegenkonto" heraus durch Positionierung in das gewünschte Feld.

**Gegenkonto  
**Die Kontonummer wird manuell eingegeben oder über die Auswahlliste abgerufen. In der Belegerfassung wird zu einem Sachkonto der Steuerschlüssel bzw. die Klasse vorgeschlagen bzw. gesperrt, wenn dies so im Sachkontenstamm für dieses Konto hinterlegt ist. 

**Kostenstelle  
**Die Kostenstelle wird abgefragt. Entsprechend der Voreinstellung im Sachkontenstamm wird eine Kostenstelle vorgeschlagen, kann oder muss eine Eingabe erfolgen. Mit **F3** ist auch hier eine Anzeige und Auswahl möglich.

**Betrag**

Eingabe des Rechnungsbetrages.

**Sonderfall:**

Eingabe eines negativen Betrages, um eine Buchung seitengerecht zu stornieren. Solche Buchungssätze können nicht gemischt mit normalen Buchungen erfasst werden, sondern müssen unter einer eigenen Belegnummer gebucht werden. Für diese Fälle empfiehlt es sich, einen eigenen Belegnummernkreis einzurichten.

**S / H - Kennzeichen  
**Das S / H - Kennzeichen ist durch den Typ "Ausgangsrechnung" eindeutig bestimmt und kann hier nicht geändert werden. Stornierungen können über den Belegkreis "Ausgangsgutschrift" oder mit einer "negativen Ausgangsrechnung" vorgenommen werden. Das hier erfasste Sollhabenkennzeichen bezieht sich auf das Hauptkonto. Mit der Option „**FIBU_SOLLHABEN**“(Direktsprung [OPT]) lässt sich die Belegerfassung so einstellen, dass sich das Sollhabenkennzeichen auf das Gegenkonto bezieht.

**Klasse / Schl.**  
Dieses Feld wird entsprechend der Voreinstellung im Sachkontenplan zur Ermittlung von Umsatz- und Vorsteuer vorbelegt.  
Unabhängig von der Vorbelegung können diese Werte natürlich überschrieben werden.

**Prinzipiell sind folgende Fälle denkbar:**

Die Erfassung auf dem Sachkonto erfolgt netto und die Umsatzsteuer wird zum Abschluss direkt auf dem entsprechenden Umsatzsteuerkonto verbucht. In diesem Fall ist Klasse 0 anzugeben, solange Erlöskonten angesprochen werden. Zum Schluss wird dann der Umsatzsteuerbetrag eingegeben und das zugehörige MwSt.-Konto angesprochen.

Es wird der Nettobetrag eingegeben, die Umsatzsteuer wird automatisch auf der Grundlage von Steuerklasse (1) und -schlüssel berechnet, intern aufsummiert und bei Belegabschluss insgesamt verbucht. Es wird der Bruttobetrag eingegeben, die Umsatzsteuer wird automatisch auf der Grundlage von Steuerklasse (2) und -schlüssel herausgerechnet, intern aufsummiert und bei Belegabschluss insgesamt verbucht.

**Belegtext  
**Hierbei handelt es sich um einen frei eingebbaren Text, der zur Erläuterung der Buchung auf dem Gegenkonto mit abgespeichert wird. Er wird später auf Kontoblättern, Journalen, etc. mit ausgedruckt. Neben der manuellen Texteingabe kann auch auf einen Textbaustein mit F3 zugegriffen werden. Den Pfleger für die Textbausteine erreicht man über

Hauptmenü > Finanzbuchhaltung > Stammdaten > Textvorbelegungen

oder mit dem Direktsprung **[FITXT]**.

Sind die Nummern zu den Texten bekannt, so ist es möglich, die Zahle einzugeben und nach **F2** wird der Text übernommen.

**Stapelerfassung**  
Mit Erfassung des Belegtextes wird die Belegzeile in den Anzeigebereich des Grundbildschirms übergeben. Dabei verändert sich der Betrag in der Zeile, in der das Hauptkonto dargestellt ist, mit jeder Buchung.  
Es wird wieder das Gegenkonto abgefragt, um jetzt Zeile für Zeile die Ausgangsrechnung eingeben zu können. Der Kundensaldo und der Kontrollsaldo unterhalb des Kundensaldos erhöhen sich mit jeder Zeile, wobei der Kontrollsaldo die Summe der erfassten Rechnungspositionen umfasst.  
Die Erfassung wird beendet im Feld Gegenkonto:

Mit **TAB** wird der Belegkreis beibehalten, der Kontrollsaldo jedoch auf "0" gesetzt. Dies erfolgt, wenn z.B. der Beleg eines anderen Kunden erfasst werden soll.  
Mit **ESC** wird der Belegkreis verlassen.

In beiden Fällen wird zum Abschluss der ermittelte Steuerbetrag angezeigt, der hier ggf. korrigiert werden kann. Mit Bestätigung der Steueranzeige wird die Steuerbuchung in den Grundbildschirm übernommen. Näheres zur Steueranzeige können Sie dem nachfolgenden Punkt entnehmen.

**Anzeige der Steuern  
**Während der Belegerfassung besteht nach Eingabe von Steuerklasse und -schlüssel die Möglichkeit, sich mittels **F8** die aufgelaufene Steuer anzeigen zu lassen:  
Dies erfolgt getrennt für Vorsteuer und Umsatzsteuer sowie -sätze. Darüber hinaus kann hier der automatisch berechnete Betrag geändert werden. Diese Änderungen sind erforderlich, wenn der auf dem Originalbeleg ausgewiesene Betrag sich von dem hier ermittelten unterscheidet, was z.B. durch ein anderes Rechenverfahren zur Steuerermittlung hervorgerufen werden kann.  
Da natürlich nur der Betrag des Originalbeleges verbucht werden darf, muss dann hier eine Anpassung erfolgen.  
Eine vollständig erfasste Ausgangsrechnung mit zwei Positionen und unterschiedlichen Steuersätzen führt zu folgender Darstellung:

- Buchung des eingegebenen Betrages auf dem Debitorenkonto (Hauptkonto).
- Buchung des Betrages auf dem Forderungskonto.
- Buchung des um den Steueranteil reduzierten Betrages (Nettobetrag)
- Buchung auf dem Erlöskonto (Gegenkonto).
- Buchung des Steueranteils auf dem Steuerkonto.

**Parametereinstellungen Ausgangsrechnungserfassung**  
In Abhängigkeit von den Anforderungen der Anwender können die Arbeitsabläufe und -inhalte angepasst werden. Dies geschieht mittels "Erfassungsparameter".

Folgende Einstellmöglichkeiten bestehen:

Mit Währung?

Bei "Ja" wird die Fremdwährungsabwicklung aktiviert.

Ankauf, Verkauf oder Mittelkurs?

Angabe, mit welchem Kurs standardmäßig die Währung bewertet werden soll.

Belegdatum mit Periode überprüfen?

Es kann eingestellt werden, ob ein Warnhinweis kommen soll, wenn das Belegdatum nicht in der Periode liegt oder eine Fehlermeldung und damit eine Eingabeverweigerung. Gleiches ist für das Buchungsjahr möglich.

Default Nummernkreis Vorbelegung

Der Nummernkreis, der standardmäßig, wenn nichts anderes bekannt ist, vorgeschlagen werden soll Fehler bei Bereichstest Nummernkreis
