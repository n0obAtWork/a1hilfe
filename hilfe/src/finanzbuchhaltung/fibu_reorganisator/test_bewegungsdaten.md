# Test Bewegungsdaten

<!-- source: https://amic.de/hilfe/testbewegungsdaten.htm -->

Hauptmenü > Abschlussarbeiten > Reorganisation > Fibureorganisation > Funktion ***Test Bewegungsdaten***

Direktsprung **[FIREO]**

Im **Test Bewegungsdaten** werden Belege, Summen und Fehlerprotokolle geprüft und Sie werden gegebenenfalls aufgefordert, bestimmte Aktionen durchzuführen, um das Problem zu beseitigen. Diese Tests lassen sich über Optionen steuern (dazu schalten weiterer Tests bzw. Ausblenden von Test). Die Optionen sind so eingestellt, dass alle für den normalen Geschäftsablauf nötigen Daten getestet werden.

**Fehlbuchungen**  
Wenn beim Verbuchen von Belegen Fehler aufgetreten sind, die noch nicht bereinigt wurden, werden Sie hier noch einmal darauf hingewiesen, dass Handlungsbedarf besteht. Eine genauere Beschreibung dieser Fehler finden Sie im Bereich Buchungen Fibu unter Journal/Ereignisprotokoll (Direktsprung **[JOUR]**)

**Interne Ident**  
Hier wir die interne Verkettung von Belegen geprüft. Sollte wider Erwarten die interne Identifikation bei einem Beleg nicht stimmen, lässt sich dies durch eine Reorganisation wieder berichtigen.

**Buchwährung**

Es muss in jedem Fall ein Eintrag im Währungsstamm für die Buchwährung vorhanden sein. 

**Nachkommastellen**  
Man kann im Währungsstamm die Anzahl der Nachkommastellen verändern. Tut man dies unbedacht und ändert die Währung so, dass sich die Nachkommastellen verringern, so kann es zu Rundungsproblemen kommen. Hier werden alle Belege angezeigt, die mehr Nachkommastellen haben, als hinterlegt.

**Mahnvorschläge ohne OP**  
Eine Liste der Mahnvorschläge, zu denen kein OP mehr existiert. Sollte im laufenden Betrieb nicht vorkommen. Wird durch eine Reorganisation zurückgesetzt, wenn unter Optionen Mahnwesen separat angeschaltet wird.

**OP ohne Mahnvorschläge**  
Eine Liste der OPs, die eine Mahnvorschlagsnummer eingetragen haben, für die jedoch keine Vorschlagsliste mehr existiert. Wird durch eine Reorganisation zurückgesetzt, wenn unter Optionen Mahnwesen separat angeschaltet wird.

**Mahnvorschläge ohne Liste**  
Hier existiert für ein Konto ein Mahnvorschlag im System, die keiner Liste mehr zugeordnet sind. Wird durch eine Reorganisation zurückgesetzt, wenn unter Optionen Mahnwesen separat angeschaltet wird.

**Mahnvorschläge ohne Stamm**  
Hier existieren Mahnvorschlagspositionen im System, die keinem Mahnvorschlag bzw. keiner Liste mehr zugeordnet sind. Wird durch eine Reorganisation zurückgesetzt, wenn unter Optionen Mahnwesen separat angeschaltet wird.

**Skontobeträge**  
Alle Skontobeträge, die Einzelpositionen zugeordnet sind, müssen in der Summenzeile wieder erscheinen (Kontrollsumme). Ist dies nicht der Fall, werden die Belege, die nicht in Ordnung sind aufgelistet. Sind diese Belege noch nicht ausgeziffert, lassen sich die Skontobeträge in der [OP-Verwaltung](../op_verwaltung/index.md) mit **F5** korrigieren.

**Skonto ohne Konto**  
Ist in einem Beleg ein Steuersatz hinterlegt, in dem kein Skontokonto steht, diesem Beleg aber ein Skontobetrag zugewiesen ist, so ist ein Ausziffern, bei dem automatisch Skonto gezogen werden soll, nicht möglich. Korrigieren Sie zuerst den Steuersatz, bevor Sie die entsprechenden Belege weiterverarbeiten.

**Steuerzuordnung**  
Es wird geprüft, ob innerhalb eines Beleges zu den Steuerpositionen auch Erlöspositionen mit entsprechender Schlüsselkombination existieren. Ist diese nicht der Fall, werden die betroffenen Belege aufgelistet. Eine Reorganisation ist nicht möglich.

**Steuerwerte**  
Sollte ein Steuerwert im Beleg eingetragen sein obwohl der Steuersatz 0,00 % beträgt, werden diese Belege hier aufgelistet. Eine Reorganisation ist nicht möglich.

**Steuerabdatum**  
steht in den Belegpositionen ein Steuersatz, deren Gültigkeitsbereich nicht mit dem Belegdatum übereinstimmt, also Belegdatum kleiner Steuerabdatum oder zwischen Belegdatum und Steuerabdatum existiert noch ein Steuersatz, so werden diese hier aufgelistet. Eine Reorganisation ist nicht möglich.

**Datenkonsistenz**  
Dies ist ein Test, bei dem kein Fehler auftreten darf. Belege, die hier erscheinen, sind defekt, weil die Kontrollsumme nicht stimmt. Sie lassen sich auch nicht reorganisieren. Diese Belege können nur gelöscht und neu erfasst werden. Handelt es sich um einen Skontobeleg oder einer Kursdifferenzbuchung, die zu dieser Fehlermeldung führte, können Sie nur durch zurücksetzen der Auszifferung diesen Beleg löschen. Diese Fehler bitte immer bei AMIC melden.

**Auszifferung**  
Eine Auszifferung muss in sich immer auf null aufgehen. Sollte - durch was für Umstände auch immer - eine Auszifferung nicht aufgehen, müssen Sie manuell eingreifen und in der OP-Verwaltung diese Auszifferung mit **F7** zurücksetzen und danach wieder ausziffern. Wenn sich der Fehler dadurch nicht beseitigen lässt, wenden Sie sich bitte mit einer genauen Fehlerbeschreibung an AMIC.  
In der OP-Verwaltung existiert ein Einrichterparameter „Auszifferung bei Kontoeingabe testen?“ mit dem man einschalten kann, dass dieser Test immer sofort nach Eingabe eines Kontos in der OP-Verwaltung durchgeführt wird.

**Auszifferungsdatum**  
Das Datum der Auszifferung dient unter anderem dazu, die OP’s zeitlich zuordnen zu können, z.B. in der historischen OP-Liste. Wenn nun das Auszifferungsdatum vor dem Belegdatum liegt, so ist eine Zuordnung nicht mehr möglich. Es werden hier die Belege, bei denen das Auszifferungsdatum in einem Wirtschaftsjahr vor dem des Belegdatums liegen angezeigt. Es muss dann die Auszifferung zurückgesetzt und mit dem korrekten Auszifferungsdatum neu gebildet werden.

**Fehlende Ops**  
Dies bedeutet, dass ein Beleg kein OP mehr ist, jedoch auch nicht ausgeziffert. Dieses kann nicht sein. Belege, die diesen Fehler aufweisen, lassen sich mit einer Reorganisation wieder in Ordnung bringen. Dazu müssen Sie jedoch vorher unter Optionen "**Offene Posten**" aktivieren.

**Überzählige Ops**  
Dieses ist das Gegenstück zum vorherigen Punkt. Ein ausgezifferter Beleg, der trotzdem OP ist, ist genauso unmöglich. Hier genauso vorgehen wie oben beschrieben.

Dieser Test wird bei den Auszifferungslisten für Sach- (Direktsprung **[AKZS]**) bzw. für Personenkonten (Direktsprung **[AKZ]**) automatisch mit durchgeführt. Es erscheint dann in der Belegzeile die Kennzeichnung **?OP?**, so dass man Fehlerhafte Belege in dieser Liste sofort erkennen kann.

**Interne Umbuchungen**  
Interne Umbuchungen sind nie Offene Posten. Durch einen Fehler in einer älteren Version kam es beim zurücksetzen von Auszifferungen dazu, dass diese Internen Umbuchungen nicht automatisch entfernt/storniert wurden. Dieser Test listet alle Internen Umbuchungen auf, die nicht ausgeziffert sind. Dieses Problem lässt sich nur durch eine Reorganisation beheben.

**Fehlende Sachkonten-Ops**  
Für Sachkonten können auch OPs geführt werden. Hier kann bei nachträglichem Setzen eines Kontos auf OP-Konto für bereits erfasste Belege ein Problem auftreten, wenn man vergessen hat anzugeben, dass der OP-Status angepasst werden soll. Um die OPs nachträglich zu erzeugen, wählen Sie unter Optionen Sachkonten OPs und starten Sie die Reorganisation.

**Forderungen & Verbindlichkeiten**  
Prüft, ob innerhalb eines Beleges die Forderungen und die Verbindlichkeiten mit der Kontrollsumme übereinstimmen. 

**F & V je Auszifferung**  
Wie für Beträge gilt genauso für die Forderungen und Verbindlichkeiten, dass die Summe innerhalb einer Auszifferung auf Null aufgehen muss. Diese Auszifferung muss dann zurückgesetzt werden wie bereits unter Punkt 12 beschrieben.

**Kontosummen erfasst**  
Schon bei der Erfassung eines Beleges werden für alle Konten Beträge periodenabhängig aufsummiert. Diese Summe lässt sich jederzeit neu aus den Belegdaten ermitteln (Funktion ***Reorganisation***).

**Kontosummen gebucht**  
Beim Buchen werden alle Beträge noch einmal aufsummiert, ähnlich wie bei der Erfassung. Diese Summe lässt sich jederzeit neu aus den Belegdaten ermitteln (Funktion ***Reorganisation***).

**Fehlende Buchungen**  
Sollten für Perioden und Konten Einträge in der Summenrelation existieren, in denen es keine Belege gibt, werden diese hier aufgeführt. 

**Fehlende Konten**  
Belege mit Kontonummer, die nicht im Kontostamm vorhanden sind, werden hier angeführt. Hier müssen Sie im Sach- bzw. Personenkontenstamm prüfen, was mit den Konten geschehen ist. Sollten sie versehentlich gelöscht worden sein, legen Sie diese wieder an. Ist der Eintrag im Beleg falsch, können Sie diesen in der Belegerfassung ändern, falls der Beleg noch nicht verbucht worden ist.

**Periodensaldo**  
Hier kann nur ein Fehler auftreten, wenn einer der vorangegangenen Tests einen Fehler aufwies oder eine Reorganisation abgebrochen worden ist. Nachdem Sie alle manuell zu beseitigenden Probleme (z.B.: Beleg löschen Punkt 11, neu ausziffern Punkt 12) behoben haben, starten Sie die Reorganisation.
