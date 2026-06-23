# Test Stammdaten

<!-- source: https://amic.de/hilfe/teststammdaten.htm -->

Hauptmenü > Abschlussarbeiten > Reorganisation > Fibureorganisation > Funktion ***Test Stammdaten***

Direktsprung **[FIREO]**

Im Test Stammdaten werden Ihre Stammdaten - hierzu gehören der Kundenstamm, der Sachkontenstamm sowie die Forderungsgruppen u.v.m. - auf korrekte Einrichtung getestet. Um einen möglichst reibungslosen Ablauf zu gewährleisten, sollten Ihre Daten so eingerichtet werden, dass bei diesem Test keine Fehler auftauchen.

**Steuerklasse 0  
**Es muss im System einen Steuersatz mit der Steuerklasse 0 geben. Dieser wird als Fehlersteuersatz herangezogen. Wird hier ein Fehler angezeigt, so gehen Sie in den Stammdatenpfleger "Steuersätze einrichten" (Direktsprung <strong>[STS]</strong>) und tragen dort den fehlenden Satz ein.

**Fehlende Konten im Steuersatz  
**Es kann vorkommen (z.B. durch Datenimport), dass im Steuersatz Sachkonten fehlen, oder Sachkonten eingetragen sind, die nicht existieren. Dies darf auf keinen Fall vorkommen, da Daten, die diese Steuersätze verwenden können, nicht verbucht werden. Löschen Sie entweder den Steuersatz, oder tragen Sie die Konten im Stammdatenpfleger für [Sachkonten](../stammdaten_der_fibu/sachkonten.md) (Direktsprung **[SKS]**) nach.

**Steuerkonto bei innergemeinschaftlichem Erwerb**

Wenn Steuersätze mit der Steuerformel „Innergemeinschaftlicher Erwerb“ eingerichtet sind, muss der Steuersatz bei „Satz innergem.Erw.“ eingetragen werden und nicht bei Steuersatz. Im Normalfall wird diese Fehleingabe jedoch bereits vom Programm abgefangen.

**Steuersätze ohne Auswertungspositionen**

Wenn man die Umsatzsteuervoranmeldung in A.eins bzw. die Übertragung via ELSTER verwenden will, so müssen den Steuersätzen gültige Kennzahlen zugewiesen ein. Es werden hier alle Steuersätze aufgelistet, bei denen ein Prozentsatz ungleich 0 eingetragen ist, jedoch keine Kennzeichen für die Umsatzsteuervoranmeldung.

**Doppelte Kontonummern  
**Kontonummern müssen eindeutig sein. Dies wird für den Normalfall der manuellen Erfassung von Sach- bzw. Personenkonten auch abgefangen und kann über die im Mandanten hinterlegten Mandantenzählkreise gesteuert werden. Wenn hier ein Fehler auftritt, wurde ein Konto sowohl als Sach- und Personenkonten angelegt. Sie müssen dann das falsch angelegte/eingespielte Konto wieder löschen.

**Fehlender Kontostamm bei Sachkonten  
**Alle Kontonummern werden im System in einer Relation Kontostamm gehalten, um die Eindeutigkeit gewährleisten zu können. Fehlt hier ein Eintrag, kann es vorkommen, dass ein Konto nicht gefunden wird. Dieser Fehler muss bei AMIC oder Ihrem zuständigen Supporter gemeldet werden, damit die Fehlerursache geklärt und der Fehler behoben werden kann.

Bei diesem Test erfolgt ein automatischer Nachtrag des Kontostammeintrages und der Anwender wird durch eine entsprechende Nachricht informiert.

**Fehlender Kontostamm bei Kunden / Lieferanten  
**Alle Kontonummern werden im System in einer Relation Kontostamm gehalten, um die Eindeutigkeit gewährleisten zu können. Fehlt hier ein Eintrag, kann es vorkommen, dass ein Konto nicht gefunden wird. Dieser Fehler muss bei AMIC oder Ihrem zuständigen Supporter gemeldet werden, damit die Fehlerursache geklärt und der Fehler behoben werden kann.

Bei diesem Test erfolgt ein automatischer Nachtrag des Kontostammeintrages und der Anwender wird durch eine entsprechende Nachricht informiert.

**Personenkonto mit Steuergruppe 0  
**Die Steuergruppe 0 ist für Sachkontenbuchungen der Fibu exklusiv vorgesehen. Kunden, Lieferanten, Kontokorrent und Interessenten mit Steuergruppe 0 werden hier aufgelistet.

**Hinweis**: *Im Kundenstammpfleger wird die Prüfung der Steuergruppe dann durchgeführt, wenn der Einrichterparameter „Steuergruppe darf nicht 0 sein“ auf „Warnung“ oder „Fehler“ steht.*

**Fehlender Kundenstamm  
**Dies ist genau der umgekehrte Fall zu Punkt 4.Hier ist im Kontostamm eingetragen, dass ein Kunden/Lieferant existieren müsste, aber kein Datensatz vorhanden ist. Wenden Sie sich bitte auch hier an AMIC oder Ihren zuständigen Betreuer.

**Fehlender Kontostamm bei Sachkonten  
**Für Sachkonten gilt dieselbe interne Logik wie für Personenkonten. Dieser Fehler muss an AMIC gemeldet werden, damit die Fehlerursache geklärt und der Fehler behoben werden kann.

**Fehlender Kontostamm bei Oberkonten  
**Auch Oberkonten unterliegen der gleichen Mechanik wie in Punkt 4 beschrieben. Dieser Fehler muss an AMIC gemeldet werden, damit die Fehlerursache geklärt und der Fehler behoben werden kann.

**Gelöschte Oberkonten  
**Hier wurde ein Oberkonto gelöscht, das im Sachkontenstamm bereits verwendet wurde. Löschen Sie entweder den Eintrag im Sachkontenstamm, oder tragen Sie ein neues Oberkonto mit dieser Nummer ein.

**Oberkonten mit fehlenden Verteilkonten  
**Bei einem Oberkonto steht Verteilung auf Ja, und es sind keine Verteilungen erfasst worden. Dies führt beim Buchen zu der Fehlermeldung "Oberkonto 0 nicht gefunden". Alle Oberkonten mit diesem Problem werden hier aufgelistet und man kann dann im Stammdatenpfleger für Oberkonten (Direktsprung OKS) die Daten vervollständigen.

**Kontonummer doppelt als Sach- bzw. Oberkonto  
**Kontonummern müssen eindeutig sein. Dies wird für den Normalfall der manuellen Erfassung von Sach- bzw. Oberkonten auch abgefangen und kann über die im Mandanten hinterlegten Mandantenzählkreise gesteuert werden. Wenn hier ein Fehler auftritt, wurde ein Konto sowohl als Sach- und Personenkonto angelegt. Sie müssen dann das falsch angelegte/eingespielte Konto wieder löschen.

**Druckposition SuSa  
**Die Summen und Saldenliste kann mit Hilfe von Druckpositionen in logische Gruppen unterteilt werden. Wenn Sie beim Druck unter Sortierung "Mit Druckpositionen" wählen, kann nur eine vollständige Auswertung erfolgen, wenn zu allen Sachkonten auch Druckpositionen für die Summen und Saldenliste hinterlegt sind. Hier werden Sie auf Lücken hingewiesen, die Sie nachtragen sollten ( [Sachkontenstamm](../stammdaten_der_fibu/sachkonten.md) Direktsprung **[SKS]** ).

**Druckposition Bilanz/GuV  
**Die Sachkonten werden für die Bilanz/GuV in A.eins über Druckpositionen den einzelnen Gruppen zugeordnet. Da es Konten gibt, die je nachdem, ob sie im Soll oder im Haben stehen, mal auf der Aktiv- und mal auf der Passivseite der Bilanz erscheinen können, gibt es zwei Druckpositionen. Diese müssen auch beide eingetragen sein, wenn diese Konten auf der Bilanz erscheinen sollen. Auf Lücken werden Sie hier hingewiesen. Bitte tragen Sie die fehlenden Druckpositionen nach.

**Oberdruckpositionen  
**Die Untergruppen in der Bilanz / GuV / SuSa werden durch Oberdruckpositionen kenntlich gemacht. Auch fehlende Oberdruckpositionen führen dazu, dass diese Auswertungen nicht korrekt sind. 

**Währung ohne Kursgewinn-/Kursverlustkonto  
**Wenn Sie in A.eins mit Fremdwährung arbeiten, können infolge von Kursschwankungen Kursdifferenzbuchungen nötig werden. Diese übernimmt A.eins für Sie. Dazu müssen im Währungsstamm die Kursgewinn bzw. die Kursverlustkonten eingerichtet sein. Für die hier angezeigten Währungen müssen Sie im Stammdatenpfleger Währung (Direktsprung **[WAE]**) die Konten nachtragen.

**Forderungsgruppen  
**Hier wird geprüft, ob in den angelegten Forderungsgruppen auch die Konten vorhanden sind. Für die hier angezeigten Forderungsgruppen tragen Sie bitte die korrekten Konten im Stammdatenpfleger Forderungsgruppen (Direktsprung **[FORG]**) ein oder legen Sie die fehlenden Sachkonten (Direktsprung **[SKS]**) an.

**Kunden mit fehlerhafter Forderungsgruppe  
**Die Forderungsgruppe im Kundenstamm sorgt dafür, auf welche Konten die Forderungen bzw. die Verbindlichkeiten gebucht werden. Ist im Kunden-/Lieferantenstamm keine Forderungsgruppe hinterlegt, können Belege mit diesen Kunden / Lieferanten zwar erfasst werden, jedoch werden sie nicht verbucht. Bitte tragen Sie für diese Personenkonten die Forderungsgruppen nach. 

**Kontozuordnung Erlöskennziffer  
**Hier wird geprüft, ob in der Erlöskennziffer Kontozuordnung (Direktsprung **[EKZZ]**) auch die Steuersätze (Direktsprung STS) bzw. die Erlösklassen (Direktsprung **[ERLK]**) existieren. Diese Daten werden für den Fibuübertrag aus dem Warenwirtschaftsteil benötigt.

**Kunden mit fehlerhafter Erlösklasse  
**Diesen Personenkonten ist eine fehlerhafte bzw. nicht in der Kontozuordnung existierende Erlösklasse zugeordnet. Um einen ordnungsgemäßen Fibuübertrag zu ermöglichen, sollte diese überprüft werden.

**Kostenstellen bei Bilanzkonten  
**Dies ist nur ein Hinweis, dass bei Bilanzkonten Kostenstellen hinterlegt sind. Sie sollten im Sachkonten überprüfen, ob dies so von Ihnen gewollt ist. Ansonsten können Sie diese Meldung ignorieren.

**Verteilkostenstellen ohne Verteilschlüssel  
**Es ist eine Verteilkostenstelle angelegt worden, für die der Verteilschlüssel fehlt. Unter dem Stammdatenpfleger für Verteilkostenstellen (Direktsprung **[VKST]**) können Sie dies nachtragen.

**Kostenstelle mit fehlerhafter Verteilkostenstelle  
**Hier ist im Kostenstellenstamm (Direktsprung **[KST]**) eine nicht existierende Verteilkostenstelle eingetragen. 

**Falsche Kundentypen  
**In A.eins werden Prüfungen anhand des Kundentyps durchgeführt bzw. Listen über Kundentypen abgegrenzt. Sollten Personenkonten existieren, deren Kundentyp nicht dem Schema entspricht, kann es somit zu falschen Werten kommen. Diese Konten müssen daraufhin überprüft werden. Wenden Sie sich zur Unterstützung an AMIC bzw. an Ihren zuständigen Supporter.

**Kunden ohne Anschriftstamm  
**Jedem Personenkonto muss auch eine Anschrift zugeordnet sein. Wenn Sie die Kunden/Lieferanten über die manuelle Erfassung eingegeben haben, ist zumindest eine leere Anschrift vorhanden. Ist jetzt durch fehlerhafte Einspielung von Daten oder durch Systemausfälle keine Anschrift vorhanden, kann es bei diversen Auswertungen dazu führen, dass dieser Kunde/Lieferant nicht erscheint. Bitte wenden Sie sich zur Unterstützung bei der Behebung dieses Problems an AMIC bzw. an Ihren zuständigen Supporter.

**Bankenstamm ohne Bankregion/Bankgruppe  
**Im Automatischen Zahlungsverkehr werden die Banken eines Kunden/Lieferanten nach Bankregion bzw. Bankgruppe für den DTA bestimmt, falls mehr als eine gültige Bank hinterlegt und keine als Standard gekennzeichnet ist. Durch einen fehlerhaften Stammdatenimport kann es sein, dass hier die Daten fehlen. Sie finden unter SQL eine Datei BNKREGIO.SQL, die Sie ausführen können, um diese Werte nachzutragen.

**Bankenstamm ohne Swiftcode/BIC**

Steht der Steuerparameter DTA-Ausgabeformat auf „SEPA“, dann werden noch vier weitere Tests durchgeführt

- Test der im Kundenstamm verwendeten Banken, ob der BIC eingetragen ist.  
- Test der Kundenbanken, ob die IBAN eingetragen ist. Für Kundenbanken, deren Bank sich in Deutschland oder Österreich befindet wird getestet, ob die Eingetragenen IBAN gültig ist. Dazu wird ein Prüfziffernverfahren angewandt, das auch bei der Stammdatenerfassung der IBAN eingesetzt wird.  
- Test der Hausbanken, ob die IBAN eingetragen worden ist. Die IBAN wird - wie auch bei den Kundenbanken –bei eingetragenen IBANs auf Korrektheit nach dem Prüfziffernverfahren geprüft.  
- Test, ob die Gläubiger-ID, die für das [SEPA-Lastschriftverfahren](../zahlungsverkehr/sepa/sepa_kennzeichen_im_mandantenstamm.md) benötigt wird, im Mandantenstamm eingetragen ist.

**SEPA-Lastschrift Mandatszuordnung**

Die Mandate werden über die Kundid und den Bankkontozaehler an die Kundenbanken gehängt, gleichzeitig wird in den Kundenbanken die zugehörige Ident des Mandats hinterlegt. Wenn diese Zuweisung nicht paarig ist, erscheint eine Liste der Kundenbanken mit defekten Zuweisungen.

**Mandantenstamm  
**Für den Umsatzsteuervoranmeldung Vordruck müssen im Mandantenstamm die Felder Bundesland, Anschrift Finanzamt, Steuernummer sowie Voranmeldezeitraum eingetragen sein. 

**Auswertungspositionen  
**Für den Umsatzsteuervoranmeldung Vordruck ist es notwendig, dass in den Auswertungspositionen die Kennzahlen eingetragen sind. Hier wird geprüft, ob alle Werte vorhanden sind, egal ob für ihren Betreib notwendig oder nicht.

**Gültigkeit ohne Nummern oder Zählkreis  
**In der Gültigkeitszuordnung für Nummernkreise sind entweder Zählkreise eingetragen, die nicht existieren, oder Nummernkreise, die nicht existieren.

**Nummernkreiszuordnung FIBU  
**In der Fibu Nummernkreiszuordnung (Direktsprung NKF) sind nicht existierende Nummernkreise eingetragen.

**Allgemeine Nummernkreiszuordnung**

Für Sachkonten, Oberkonten, Personenkonten sowie Kostenstellen bzw. Kostenträger kann man im Mandantenstamm Nummernkreise hinterlegen, die den gültigen Nummernbereich eingrenzen. Sind hier keine Gültigen Nummernkreise hinterlegt, so kann vom Programm kein Bereichstest durchgeführt werden. Je nach Einrichrichterparameter im Sachkontenstamm bzw. im Oberkontostamm werden Eingaben ohne gültige Einrichtung nicht mehr erlaubt.

**Nummernkreisüberschneidungen**

Es dürfen sich die Bereiche von Sach-, Personen- und Oberkonten nicht überschneiden. Ist dies – bei eingerichteten „**Allgemeinen Nummernkreiszuordnungen**“ – der Fall, nützt der Bereichstest nichts. Der allgemeine Programmablauf ist davon zwar nicht betroffen und es kann auch nie ein Oberkonto bzw. Sachkonten dieselbe Nummer haben wie ein Personenkonto, jedoch kann es bei der Einrichtung zu Ärgernissen kommen, wenn eine Kontonummer bereits vergeben ist. Aus diesem Grund werden Bereichsüberschneidungen als Fehler ausgegeben.
