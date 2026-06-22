# Export Mandantenübernahme

<!-- source: https://amic.de/hilfe/_fibuexpmandant.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Export > Variante Mandantenübernahme

Direktsprung **[FIEX]**

In dieser Variante ist es möglich, Belege aus der Warenwirtschaft (Einkaufsrechnungen, Einkaufsgutschriften, Ausgangsrechnungen, Ausgangsgutschriften,…), die bereits in die Fibu übertragen und gebucht worden sind von einem A.eins System(Quell-Mandant) in einen oder mehrere A.eins Mandanten(Ziel-Mandanten) zu übertragen. Die Daten werden durch den Quell-Mandant direkt in die Tabelle FiBuImport der Ziel-Mandanten geschrieben. Das Übertragen funktioniert nur dann, wenn die Relation FibuImport des Ziel-Mandanten als Proxy-Tabelle im Quell-Mandanten eingerichtet ist. Wie sie die Einrichtung vornehmen, ist an dieser [Stelle](./export_mandantenuebernahme.md#EinrichtungProxy) beschrieben.

Bei diesem Verfahren findet kein Dateiaustausch statt.

Voraussetzungen

Dieses Verfahren setzt voraus, dass die FiBu auf dem Quell-Mandanten und den Ziel-Mandanten gleich eingerichtet sind. Dies bedeutet auch, dass neue Konten die neu auf dem Quell-Mandanten hinzugefügt wurden, auch auf den Zielmandaten neu eingerichtet werden müssen.

Einrichtung des Exportes

Über die Funktion ***Einrichten*** **F5** kann eine andere Prozedur ausgewählt werden. Die Prozedur muss die gleichen Eingangs-, sowie Ausgangsparameter wie die Standardprozedur haben. Des Weiteren müssen hier die Proxy-Tabellen eingetragen werden. Für jede Proxy-Tabelle wird der Export durchgeführt.

Start

Mit der Funktion ***„Start“*** wird der Export der Ausgewählten Daten gestartet. Die Export Daten werden nur an die Mandanten verteilt, bei denen die Verbindung vorhanden ist und das Kennzeichen Übertragen auf „Ja“ gestellt ist.

Export Wiederholen

Mit der Funktion Export Wiederholen, können übertragenen Daten erneut in die Ziel Mandanten verteilt werden. Dabei ist zu beachten, dass in den Zielmandanten die Buchung des zu wiederholenden Exports zurückgesetzt worden sind. Beim Ausführen der Funktion öffnet sich eine Auswahl aus dieser Auswahl kann dann der erneut zu tätigende Buchungssatz ausgewählt werden. Das erneute Exportieren wird dann direkt ausgeführt. Dabei ist zu beachten, das im Vorwege auf den Ziel-Mandanten die Buchungen die zu dem zu wiederholenden Exportlauf gehören wieder aus der FiBu ausgebucht werden.

Verhalten beim Export

Es werden nur komplette FiBu Vorgänge übertragen. In der Auswahlliste werden alle Positionen zu einem Beleg angezeigt. Wird nur eine Position markiert wird trotzdem der ganze Fibu Vorgang exportiert.

Stellt das System während eines Übertragungslaufes fest, dass ein Ziel nicht erreichbar ist, obwohl das Ziel beim Starten des Exportes erreichbar gewesen ist so wird der Lauf abgebrochen. Teilt die Dateneinspiel-Prozedur bei einem Durchlauf dem Aufrufenden System mit, dass die Daten nicht eingespielt werden können, so wird auch in diesem Fall der Export abgebrochen.

In beiden Fällen ist manuell zu prüfen welche Daten in den Ziel-Mandanten angekommen sind.

Einrichtung des Ferndatenzugriffs

Einrichtung eines Fremdservers(SQL Central)

Starten Sie Sybase Central

Verbinden Sie sich mit der Quell-Datenbank.

Auf der linken Seite den Punkt Fremdserver auswählen.

Klicken Sie Auf Datei > Neu > Fremdserver oder mit der rechten Maustaste auf Fremdserver > Neu > Fremdserver.

Geben Sie in das Feld einen Namen für den Fremdserver ein und klicken Sie dann auf Weiter.

Wählen Sie hier als Fremdservertyp SQL Anywhere aus und klicken Sie auf Weiter.

Tragen Sie im Feld Verbindungsinformationen den Name der zu verwendenden ODBC Verbindung ein klicken Sie dann auf Weiter. Dabei ist zu beachten, dass die zu verwendende ODBC Verbindung richtig eingerichtet worden ist.

Der Fremdserver darf nicht schreibgeschützt sein klicken Sie dann auf Weiter.

Klicken Sie auf Externes Login für den aktuellen Benutzer erstellen und füllen Sie die erforderlichen Felder aus. Im A.eins System kann dann später im Bedienerstamm für die jeweiligen Bediener die Berechtigungen gesetzt werden.

Die Verbindung zu Fremdserver lässt sich mit einem Klick auf den Button „Verbindung testen“ testen.

Mit dem Klick auf „Fertig stellen“ wird der Assistent beendet.

Nach dem der Fremdserver eingerichtet worden ist, muss jetzt die Proxy-Tabelle eingerichtet werden.

Einrichtung einer Proxy-Tabelle(SQL Central)

Starten von Sybase Central wenn das Programm noch nicht gestartet wurde.

Verbinden Sie sich mit der Quell-Datenbank, wenn Sie noch nicht mit der Datenbank verbunden sind.

Klicken Sie im Bereich Fremdserver auf den Server für den eine Proxy-Tabelle eingerichtet werden soll.

Klicken Sie Auf Datei > Neu > Proxy-Tabelle oder mit der rechten Maustaste auf den Fremdserver > Neu > Proxy-Tabelle.

Im Folgenden Fenstere wählen Sie den Fremdserver erneut aus und Klicken dann auf Weiter

Als nächstes wählen Sie im oberen Abschnitt die Tabelle „FiBuImport“ aus und geben dieser einen eindeutigen neuen Name im den Textfeld wie z.B. „FiBuImportZiel1“. Die Tabelle gehört dem Bediener „Admin“. Die Tabelle darf keinem anderen Bediener zugeordnet werden. Klicken Sie dann auf Weiter.

Es müssen alle Spalten der Tabelle zur Verfügung stehen. Also klicken Sie in diesem Fenster auf Weiter.

Im nächsten Fenster besteht noch die Möglichkeit ein Kommentar zu hinterlegen. Wenn kein Kommentar mehr hinterlegt werden soll, wird mit klicken auf „Fertig stellen“ der Assistent beendet.

Nach dem der Assistent beendet worden ist, wird die Proxy-Tabelle angezeigt. Um zu testen, ob die Verbindung zu dieser Tabelle besteht, wird diese ausgewählt. Bei einem rechten Mausklick auf diese Tabelle wird ein Menu geöffnet. Mit einem klicken auf den Punkt „Zeige Daten in Interactive SQL an“ öffnet sich dann Interactive SQL mit einer Abfrage auf die Tabelle. Wenn die Abfrage kein Fehler liefert ist die Tabelle richtig eingerichtet worden.
