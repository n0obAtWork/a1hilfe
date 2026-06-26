# Mögliche Zahlungsvorgänge

<!-- source: https://amic.de/hilfe/mglichezahlungsvorgnge.htm -->

Es existieren folgende Finanzvorgänge (hier zieht der Steuerparameter Aut. Buchungen von Finanzvorgängen):

a) **Einzahlung,**  
d.h. es wird eingegeben, wie viel von welcher Zahlungsart in die Kasse gegeben wird; außerdem wird ein Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt. In der vorgeschalteten Maske wird die Art der Einzahlung abgefragt: entweder sie kommt von einer Bank (Haben: Verrechnungskonto der zugehörigen Hausbank, Soll: Kassenkonto) oder von einem Kunden (Haben: Kundenkonto, Soll: Kassenkonto).

b) **Auszahlung an Kunden,**  
d.h. es wird eingegeben, wie viel von welcher Zahlungsart aus der Kasse genommen wird. Dabei muss sich das bargeldlose Zahlungsmittel auch in der Kasse befinden (Identifikation z.B. über Gutscheinnummer o.ä.). Ebenso muss auch genug Bargeld vorhanden sein, außerdem wird ein Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt, wobei vorher der Kunde ausgewählt werden muss, an den ausgezahlt werden soll (Haben: Kassenkonto, Soll: Kundenkonto).

c) **Entnahme mit Zuordnung Kostenkonto**  
gemäß FIBU-Eintrag in AcashStmdKsse. Dabei ist das mit Werten aus AcashStmdKsse vorbelegte Verrechnungskonto auf diesem Fenster änderbar. (Haben: Kassenkonto, Soll: gewähltes Verrechnungskonto).  
Hier existieren auf der Zahlungsmaske zwei Einrichterparameter, die dafür sorgen, ob es eine Vorbelegung des Steuersatzes gemäß Sachkontenstamm geben soll.  
(siehe auch EPA)

d) **Zahlungsmeldung für Kreditrechnungen**  
Diese funktioniert ähnlich wie eine Einzahlung jedoch mit passender Bedienerführung (Eingabe eines Verweises auf die zu begleichende Rechnung und Eingabe des Rechnungsbetrages); hier muss die Kundennummer auf der Finanzvorgangsauswahlmaske angegeben werden. Angezeigt werden dann Beleg aus der Warenwirtschaft, die noch nicht teilgezahlt wurden. Hinterher sollte dann eine Bearbeitung durch eine autorisierte Person in der Offenen-Posten-Verwaltung stattfinden. (Haben: Kundenkonto, Soll: Kassenkonto)

Ausgewählte Kunden werden in anderen Kassen für Zahlungsmeldung gesperrt. Diese werden nach Abschluss der Zahlungsmeldungen wieder freigegeben. Sollte es während der Erfassung zu einem unerwarteten Programmabbruch kommen, wird der Kunde bei der nächsten Anmeldung an eine Kassenanwendung mit derselben Kasse wieder freigegeben.

e) **Abschöpfung an Bank / Hauptkasse**  
Dabei wird bei Nichthauptkassen von der zugehörigen Hauptkasse abgeschöpft (hier werden die an die Hauptkasse eingereichten Zahlungssätze und Zahlungsmittelsätze automatisch in AcashBelgKsiz – der Verwaltung der Kassenbestände – umgebucht; hierfür existiert eine Automatik beim Kassenabschluss, wenn ein zugehöriger SPA eingeschaltet ist); bei Hauptkassen wird an die Bank eingereicht, die in der Finanzvorgangsauswahlmaske angegeben worden ist. Dabei sollte man das entsprechende FIBU-Konto der entsprechenden Hausbank auswählen. Wenn die Kasse Hauptkasse ist, wird an die Bank eingereicht (Haben: Kassenkonto, Soll: Verrechnungskonto der Bank). Ist diese Kasse Unterkasse, wird an die zugehörige Hauptkasse eingereicht (Haben: Kassenkonto der Unterkasse, Soll: Kassenkonto der Hauptkasse). Zur Durchführung der Einreichung wurde ein eigener Dialog erstellt. Die [Eingabemaske Einreichung](../abschoepfung_einreichung_an_bank_zugehoerige_hauptkasse.md).

f) **Wechseln von Bargeld**  
Dabei muss der Bediener zuerst die Einzahlungsart (F11 für Kassenwährung Bargeldwechsel, F2 für Barauszahlung eines Scheck, F6 für Barauszahlung eines Gutscheines, F12 für KW-Ausgabe eines Fremdwährungsbetrages) eingeben. Dann wird automatisch verrechnet und der in KW zurückzuerstattende Bargeldbetrag steht unter Rückgeld. (Soll: Kassenkonto, Haben: Kassenkonto)

g) **Geldübergabe an andere Kasse**  
auf der Finanzvorgangsauswahlmaske wird die Zielkasse angegeben; die Bestände werden gleichzeitig gegengerechnet. Diese Funktionalität ist wichtig, wenn eine Kasse nicht genug Bargeld besitzt. Um den Bestand zu erhöhen, ohne dass etwaige erfasste Artikel storniert werden müssen, führt eine Kasse eine Geldübergabe an diejenige Kasse durch, die zu wenig Bargeld besitzt. Es wird ein Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt und wie folgt gebucht. (Haben: Kassenkonto, Soll: Kassenkonto der Gegenkasse)

h) **Geldübernahme von Kasse**  
auf der Finanzvorgangsauswahlmaske wird die Entnahmekasse angegeben, die Bestände werden gleichzeitig gegengerechnet. Es wird ein Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt und wie folgt gebucht. (Haben: Kassenkonto der Gegenkasse, Soll: Kassenkonto)

i) **Kassensturz (ohne Abschluss)**  
Man kann eine Geldzählung durchführen, ohne dass die Kasse abgeschlossen werden muss

j) **Kassensturz (mit Abschluss), ist direkt in den Kassenabschluss integriert**  
Dabei wird auch dieses Ergebnis – eine mögliche Differenz – als Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt und wie folgt verbucht (bei Manko gilt Haben: Kassenkonto, Soll: Verrechnungskonto der Kasse; bei Überschuss gilt Haben: Verrechnungskonto der Kasse, Soll: Kassenkonto).  
ACHTUNG:

Wenn eine Zählung nur unvollständig durchgeführt wird, man allerdings die Vollständigkeit in der Abfrage bestätigt, wird diese unvollständige Zählung als vollständig angesehen und auch so verbucht. Dabei werden auch die Kassenbestände gemäß Zählung angepasst! Nach jeder Zählung wird die Differenzsumme in KsiDiff geschrieben und auf KsiSoll. gebucht. Dabei gilt folgendes: positive Werte in KsiDiff... beschreiben einen Überschuss, negative Werte in KsiDiff... beschreiben ein Manko Um nicht alte KsiDiff... erneut zu buchen, wird KsiDiff...bei jeder neuen vollständigen Zählung auf 0 gesetzt. Um einen Überblick über die einzelnen Differenzen bei jeder Zählung zu bekommen, muss man nach jeder Zählung einen Kassenabschluss durchführen. Zu einer Zählung gehören auch etwaige Schecks, Gutscheine, ...  
Beim Abschluss gibt es bei eingestelltem SPA außerdem, die Möglichkeit, eingenommene Zahlungsmittel automatisch auf eingerichtete Kostenkonten zu buchen.

k) **Einzahlung von Kostenkonto**  
d.h. es wird eingegeben, wie viel von welchem Zahlungsmittel in die Kasse gegeben wird; außerdem wird ein Eintrag in den Datenstrom (und über MS dann in die FiBu) erzeugt. (Haben: Kostenkontos, Soll: Kassenkonto). Dieses kann genutzt werden, um z.B. einen Vortrag in die Kasse zu bringen.

Es besteht die Möglichkeit, gewisse obige Vorgänge zu stornieren, indem man den entsprechenden Vorgang im Belegüberblick auswählt, die Maske aufruft und dann den Knopf stornieren anwählt.
