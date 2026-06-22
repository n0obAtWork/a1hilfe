# Server starten

<!-- source: https://amic.de/hilfe/_cescannerserverstarten.htm -->

Hier finden Sie die Erklärung der einzelnen Maskenfelder

<p class="just-emphasize">Registerkarte Server</p>

| Eingabefelder | Bedeutung |
| --- | --- |
| Grid (TCP/IP Adressen) | In dem Grid werden alle aktiven und jemals aktiven Scanner-IP’s angezeigt |
| Serverfehler | In dem Feld Serverfehler wird der Datenbank Fehler angezeigt falls es zu einem Fehler mit dem Scanner kommt.  
 |
| Port | In dem Feld Port wird der Port der Datenbank eingegeben.  
 |
| Updatezyklus | In dem Feld Updatezyklus wird die Updatezeit eingegeben.  
 |
| Vorgangsprotokoll | In das Feld Vorgangsprotokoll kann eingetragen werden, ob während der Verarbeitung eines Scanvorganges in das Vorgangsprotokoll geschrieben werden soll. Dies ist nützlich bei einer eventuellen Fehlersuche. |
| Aufräumen | Hier kann entschieden werden, ob die gescannten Daten nach erfolgreicher Vorgangserzeugung aus den Scannertabellen geleert werden.  
 |
| Testumgebung | Hier kann entschieden werden, ob die Testumgebung genutzt werden soll.  
 |
| Testvorgang | |
| Belegdruckernummer | Verwendet den Drucker der standardmäßig unter Windows für den Scannerbenutzer eingetragen worden ist. Wird in dem Feld Ja ausgewählt so wird nach der Erstellung des Vorgangs der Beleg automatisch ausgedruckt. Steht ein Nein in diesem Feld so wird der Beleg manuell gedruckt.  
 |

<p class="just-emphasize">Registerkarte Allgemein</p>

Auf der Registerkarte Allgemein finden Sie noch weitere Einstellungen, die für den Betrieb der Scanner von Bedeutung sind.

| Eingabefelder | Bedeutung |
| --- | --- |
| Prozedur Dateieingang | |
| Script Vorgangsabschluss | |
| Kundengruppe für Etikett druck | |
| Rückantwort EAN | |
| Protokoll IP | |
| Scanner addiert Menge | Es gibt die Möglichkeit die Menge per Hand einzugeben. Stellt man aber diesen Punkt auf “JA“, so addiert der Scanner die Menge dies bedeutet, dass jede Position eines Artikels einzeln eingescannt werden muss. |
| Kundensuchkriterium | Im AeinsCE gibt es die Möglichkeit für Eingangslieferscheine den Kunden anzugeben, hier kann das Auswahlkriterium für den Kunden bestimmt werden. |
| MHD Maximale Obergrenze | Bitte hier die Maximale Jahresanzahl eintragen, wie weit das MHD in der Zukunft liegen soll. |
| MHD Karenztage | Bitte hier die Maximale Tagesanzahl eintragen, wie weit das default MHD abhängig vom Tagesdatum in der Zukunft liegen soll. |
| Kommissioniert | Bitte tragen Sie einen Wert für die Arbeitsregel ein, der für die kommissionierten Aufträge gilt. So kann dann unter dem Direktsprung “AUB“ ein Profil erstellt werden, welches die Aufträge anzeigt die kommissioniert wurden. Benutzen Sie keine Arbeitsregel so tragen Sie als Standard eine 0 ein. |
| Nicht Kommissioniert | Bitte tragen Sie einen Wert für die Arbeitsregel ein, der für die nicht kommissionierten Aufträge gilt. So kann dann unter dem Direktsprung “AUB“ ein Profil erstellt werden, welches die Aufträge anzeigt welche noch nicht kommissioniert wurden. Benutzen Sie keine Arbeitsregel so tragen Sie als Standard eine 0 ein. |
| Scanner Scrollbar | Mit dieser Einstellung kann entschieden werden, ob in die Scanneranzeige alle gescannten Positionen oder nur ein Teil geladen werden. Wird die Einstellung auf „Ja“ gestellt so werden in der AeinsCE Maske nur neun Positionen angezeigt. Mit den Pfeiltasten kann dann hoch und runter geblättert werden. Dies bedeutet es werden immer die nächsten neun Positionen nachgeladen. Wird „Nein“ ausgewählt so werden alle Daten angezeigt und man kann mit dem Scrollbalken nach unten Scrollen. |
| Unterdrückung von AI Start | Mit dieser Einstellung kann entschieden werden, wenn der Scanner sich noch in einem Vorgangserfassungmodus befindet, der nicht beendet worden ist, ob ein Wechsel auf ein anderen Vorgang möglich ist (z.B. von Auftrag zur Inventur) |

<p class="just-emphasize">Registerkarte Vorgangseinstellungen</p>

| Eingabefelder | Bedeutung |
| --- | --- |
| Mehrpartie | Ist Mehrpartie nicht gesetzt, wird nur Wabewinfo überschrieben  
 |
| Fixpartie änderbar | Hiermit kann eingestellt werden, ob eine Partie die Fix ist doch noch abgeändert werden darf. |
| Warenbewegungsaddonfeld | Als Sonderfunktion steht die Möglichkeit bereit, die Originalmenge des Beleges vor Korrektur zu sichern, hierbei kann ein beliebiges Feld in dem Warenbewegungsaddon definiert werden. |
| Prozentabweichung der Menge | Durch die Angabe eines Prozentwertes kann weiterhin gesteuert werden, ob die Menge, die durch den Scanner in den Vorgang eingefügt wird, überhaupt so akzeptiert wird. Liegt der Wert prozentual oberhalb des festgelegten Prozentwertes, so wird die eingescannte Menge nicht verarbeitet, es wird ein entsprechender Eintrag im Protokoll erzeugt. |
| Unbearbeitete Position auf 0 setzen | Mit diesem Parameter kann eingestellt werden, ob alle nicht bearbeiteten Positionen auf 0 gesetzt werden sollen. Dies ist nur für den Fall Interessant wenn keine Teildisposition gemacht wird. |
| Teildisposition Auftrag oder Bestellung | Mit dieser Einstellung kann eingestellt werden, ob der Beleg der mit dem Scanner bearbeitet wird korrigiert werden soll, oder ob eine Teildisposition vorgenommen werden soll. Dies bedeutet, dass ein neuer Vorgang mit der nicht gelieferten Ware erstellt wird. |
| Teildisposition Lagerumbuchung | Mit dieser Einstellung kann eingestellt werden, ob der Beleg der mit dem Scanner bearbeitet wird korrigiert werden soll, oder ob eine Teildisposition vorgenommen werden soll. Dies bedeutet, dass ein neuer Vorgang mit der nicht gelieferten Ware erstellt wird. |
| Buchungstyp Lagerumbuchung | Hier kann hinterlegt werden, welcher Buchungstyp der Lagerumbuchung zugeordnet werden soll, nach dem diese erfolgreich Bearbeitet worden ist. |
| Lagerumbuchung Addon Speichern | Sollen die im Addon Daten gespeichert werden. |
| Leervorgang Löschen | Hier kann eingestellt werden, ob ein Auftrag mit leeren Positionen gelöscht werden soll. |
| Lösche 0 Positionen | Mit dieser Funktion kann eingestellt werden, ob Positionen mit einer 0 Menge aus dem Vorgang gelöscht werden sollen. |
| Bestellung/Auftrag zum Lieferschein | Hier kann eingestellt werden, ob ein Auftrag, Bestellung gleich in ein Lieferschein umgewandelt werden soll |
| Gebindefaktor aus Vorgang | Hier kann eingestellt werden, ob der Gebinde Faktor aus dem Vorgang gelesen werden soll. |

<p class="just-emphasize">Registerkarte LVS</p>

| Eingabefelder | Bedeutung |
| --- | --- |
| Pfad zum Aeins Programmstart | Pfad zur Datei Aeins_Programmstart.vbs. Diese liegt im bin Verzeichnis des A.eins System. Bei der Eingabe ist darauf zu achten, wenn der Pfad ein UNC Pfad ist, dass dieser am Anfang mit drei „\\“ beginnt. |
| Section in der Aeins.ini | Section für das A.eins System |
| Automatische Boxen Anlage | Sollen Automatische Boxen angelegt werden, wenn diese nicht vorhanden sind. |
| Ladeträgertyp Auto Anlage | Der Ladeträgertyp bei der Automatischen Boxen Anlage. |
| Lokalitätsnummer | Ist keine Lokalität eingescannt worden oder der Scanner hat keine gültige Lokalität so kann hier ein Default Wert gesetzt werden. Diese Funktion wird für die Lagerverwaltung benötigt.  
 |
| Menge automatisch bei EL | Bei Ja wird die Menge die im Füllen oder Wiegen Kommando eingeben worden ist automatisch an die Warenposition geschrieben. Bei muss die Menge nach dem Erfassen der Warenposition gespeichert werden. |
| Private Mengenbehandlung Umpacken | Hier kann eine Private Prozedur hinterlegt werden, die beim Kommando Umpacken anstelle der Standard Mengenbehandlung aufgerufen wird. |
| Abweichung in Prozent beim Umpacken | Hier kann hinterlegt werden, wie viel % Verlust beim Umpacken auftreten darf. Diese Einstellung gilt nur für den Standard Fall. |

<p class="just-emphasize">Registrierkarte Warenbewegung Addon</p>

Auf dieser Registrierkarte kann eingestellt werden welcher AI Code 91,92,93,94,95,96,98 in einem Warenbewegung Addon Feld gespeichert werden kann.
