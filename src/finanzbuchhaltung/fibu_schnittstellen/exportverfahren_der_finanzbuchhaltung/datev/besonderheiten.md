# Besonderheiten

<!-- source: https://amic.de/hilfe/besonderheiten.htm -->

Besonderheiten bei den Personenkonten

Unter den Fibumerkmalen kann man eine abweichende Kontonummer für den DATEV-Export angeben. Diese wird dann an Stelle der Kontonummer des Personenkontos für die Übergabe der Stamm- und der Bewegungsdaten verwendet. Dies ist immer dann Sinnvoll bzw. sogar unumgänglich, wenn man die Personenkonten so eingerichtet hat, dass sie nicht den Anforderungen der DATEV entsprechen.

Besonderheiten im Sachkontenstamm

Im Sachkontenstamm existieren zwei für die DATEV relevante Felder:

Abweichende DATEV-Kontonummer:  
Wenn es vorkommt, dass man bereits mit einem Kontenrahmen gearbeitet hat und sich irgendwann entschließt die DATEV-Schnittstelle zu verwenden, hat man die Schwierigkeit, dass nur die DATEV-Kontenrahmen erlaubt sind. Dann hatte man bisher nur die Möglichkeit die Fibu neu aufzusetzen oder zu versuchen, während des laufenden Betriebs die Sachkonten für bestehende Belege zu ändern.  
 Um hier ein einfacheres Verfahren anzubieten, gibt es im Sachkontenstamm jetzt ein weiteres Feld "Abweichendes DATEV-Konto". Steht hier eine 0, wird nach wie vor die Kontonummer des Sachkontos verwendet. Steht hier eine abweichende Nummer, wird diese Nummer bei der Übertragung in die Datei geschrieben.

DATEV Automatik:  
Bei der DATEV existieren sogenannte Automatikkonten. Für diese Konten wird von der DATEV automatisch die Steuer errechnet. Es darf also keine Steuer von der Schnittstelle übertragen werden. Um dies der Schnittstelle mitzuteilen gibt es das Feld DATEV Automatik, das für diese Konten auf **Ja** gesetzt werden muss. Eine Liste der Automatikkonten kann Ihnen der Steuerberater zur Verfügung stellen. In den ausgelieferten Kontenplänen SKR03 und SKR04 sind die entsprechenden Konten bereits korrekt gekennzeichnet.  
**HINWEIS:** *Treten zwischen den Steuerkonten beim Steuerberater und denen in A.eins Differenzen auf, so liegt dies mit großer Wahrscheinlichkeit an falsch hinterlegten Automatikkennzeichen!*

Besonderheiten Steuer

Es existieren drei Möglichkeiten, die Steuerinformationen an die DATEV zu übermitteln, die alle von A.eins unterstützt werden. Im [Firmenstamm](./datev_firmenstamm.md) wird festgelegt, ob die Steuer über den Umsatzsteuerschlüssel oder über das Steuerkonto übermittelt werden soll.  
    

[Automatikkonten](./besonderheiten.md#Datevautomatik): Automatische Konten haben eine Programmfunktion, die auf der DATEV-Seite bewirkt, dass aus dem Bruttobetrag der Buchung die Umsatzsteuer gerechnet wird. Auf das automatische Konto bucht das DATEV-Programm den um die Steuer verminderten Betrag. Die Umsatzsteuer selbst bucht das System dann automatisch auf das für den betreffenden Steuersatz vorgesehene Umsatzsteuersammelkonto.  
In A.eins müssen die Konten in Absprache mit dem Steuerberater im Sachkontenstamm entsprechend gekennzeichnet werden (s.o.).

**Umsatzsteuerschlüssel**: Die Anwendung des Umsatzsteuerschlüssels im Buchungssatz ist eine weitere Möglichkeit der Umsatzsteuerrechnung. Dieser DATEV-Steuerschlüssel muss in allen verwendeten Steuersätzen hinterlegt sein. Der Einsatz des Steuerschlüssels bewirkt, dass vom DATEV-Programm automatisch der UST-Betrag errechnet und auf das entsprechende Sammelkonto gebucht wird. Die Verwendung des Umsatzsteuerschlüssels bei Automatikkonten ist nicht möglich.

**Hinweis:** Ab Format 7.0 können die DATEV-Steuerschlüssel bis zu vier Stellen haben. Die neuen Steuerschlüssel können im Anwenderformat AF_DATEVSCHL selbstständig in Absprache mit dem Steuerberater erfasst werden.

**Übertragen des Steuerkontos**: Der Beleg wird so wie er in A.eins erfasst wurde an die DATEV übertragen. Dabei werden eine Zeile mit dem Erlöskonto und dem Nettobetrag und eine Zeile mit dem Steuerkonto und dem Steuerbetrag übermittelt. Damit ist gewährleitstet, dass alle Beträge so beim Steuerberater ankommen, wie sie auch in A.eins existieren. Dieses Verhalten wird nur bei Automatikkonten unterdrückt, damit die Steuer nicht doppelt errechnet wird.  
Um ggf. die den Steuerschlüssel an den Steuerberater zu übermitteln, existiert die Möglichkeit, den [DATEV-Steuerschlüssel](./datev_export_bearbeiten.md#DatevSteuerSchluessel) im Textfeld zu übermitteln.

Buchungen zum Innergemeinschaftlichen Erwerb lassen sich nicht mit Methode 3 übermitteln.

Steuerparameter

Für das Festschreibungskennzeichen existiert der Steuerparameter „DATEV Festschreibungskennzeichen übertragen“ (SPA 1061). Dieser wird nur ab dem DATEV-Format 7.0 ausgewertet. Im Standard steht der SPA auf „**Mit Festschreibungskennzeichen“**. Das bedeutet, dass die Daten beim Datenempfänger nicht geändert werden können. Setzt man diesen Steuerparameter auf **“Ohne Festschreibungskennzeichen“**, so wird das Kennzeichen beim Export so gesetzt, dass die Daten beim Exportempfänger änderbar sind. Dies ändert nichts daran, dass von A.eins nur gebuchte Belege, die bekanntermaßen nicht mehr änderbar sind, übertragen werden.

Dieser SPA wird beim Erstellen der Datei in der Anwendung „DATEV-Export bearbeiten“ (Direktsprung **[DATEV])** ausgewertet, d.h. man kann bereits exportierte Daten erneut mit geändertem Kennzeichen übertragen, ohne dass die Daten erneut zusammengestellt werden müssen.
