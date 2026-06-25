# Archiv Mail Versand (EPA FA_MAIL)

<!-- source: https://amic.de/hilfe/_EPA_FA_MAIL.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Absender aus Bedienerstamm | Nein | |
| Versandfunktion | Archiveintraege_Versenden | SQL-Prozedur die zum Versenden verwendet werden soll. |
| Eigene Prozedur zur Adressaufbereitung | FA_mail_senden_an_Adressen | Die Adressen werden in der oberen Tabelle des Senden-An Pflegers zur Verfügung gestellt und können individuell angepasst werden.<br>Die Standard Vorbelegung, welche auch privatisiet werden kann ist die Prozedur:<br>FA_mail_senden_an_Adressen<br> <br>Diese sammelt ausgehend vom Formulararchiv die E-Mail-Adressen der Kunden, welche im Kundenstamm des zugehörigen Kunden sowie unter dem Vorgangskunden und dem Rechnungskunden im zugehörigen Vorgang eingetragen sind. |
| Versand-XML anzeigen | Nein | Mit dieser Einstellung kann das Versand-XML zu Debug-Zwecken angezeigt werden |
| Alternative zusätzliche Adresse 1 ( 0 = keine, sonst ADRESSID ) | 0 | Im Standard nicht implementiert. |
| Alternative zusätzliche Adresse 2 ( 0 = keine, sonst ADRESSID ) | 0 | Im Standard nicht implementiert. |
| Alternative zusätzliche Adresse 3 ( 0 = keine, sonst ADRESSID ) | 0 | Im Standard nicht implementiert. |
| Alternative zusätzliche Adresse 4 ( 0 = keine, sonst ADRESSID ) | 0 | Im Standard nicht implementiert. |
| Alternative zusätzliche Adresse 5 ( 0 = keine, sonst ADRESSID ) | 0 | Im Standard nicht implementiert. |
| Lager-Adressen aus Vorgang ermitteln | Ja | Hiermit kann festgelegt werden, ob bei Vorgängen die Adresse des Lagers ermittelt wird. |
| IP-Adresse des SMTP-Relays für Mails | | IP-Adresse des SMTP-Servers der für den Versand der E-Mails verwendet werden soll. |
| Maske nach korrekter Versendung verlassen | Nein | Soll die Maske nach dem Versenden verlassen werden, wenn keine Fehlerprotokolleinträge vorhanden sind. |
| Adressmaske bei Doppelklick auf Mail-Adresse/Fax-Nummer aufrufen | Nein | Soll beim Doppelklick auf die Mail-Adresse/Faxnummer die Adressmaske aufgerufen werden. |
| Prozedur für die Vorbelegung der Felder Betreff und Kurztext | | Prozedur, mit der die Felder Betreff und Kurztext vorbelegt werden sollen. Die Rückgabefelder der Prozedur müssen wie folgt heißen:<br><ul><li>&nbsp;&nbsp;&nbsp; Betreff<br>Kurztext</li></ul> |
| Protokoll ins Fehlerprotokoll schreiben? | Nein | Steht dieser Parameter auf „Ja“ werden bei versenden über die Standardprozedur Versandinformationen ins Fehlerprotokoll geschrieben.<br>Die Maske wird dann auch bei korrektem Versand nicht verlassen.<br>Im Standard nicht implementiert. |
| VBA-Script für den Outlookversand | | <ul><li>&nbsp;&nbsp;&nbsp; Hier kann ein vom Skript „AMIC_FAVersandOutlook“ abweichendes VBS-Skript zum Versand von E-Mails in der Funktion „Versenden(Outlook) angegeben werden.</li></ul> |
| Alle Kunden-Adressen ermitteln | Ja | Im Standard nicht implementiert |
| Dokumentenanlagen standardmäßig mitschicken | Ja | Standardmäßig werden die im Anlagenmanager als Anlage eingerichtete Dokumente automatisch bei „Senden An“ mitgeschickt. Dieser Parameter erlaubt eine Unterdrückung dieser Funktion. |
| Versandprofil | | Nummer des Versandprofildatensatzes[VPST], der für den Mailversand genutzt werden soll |
| Auch an mich senden (Bediener) | Nein | Im Standard nicht implementiert |
