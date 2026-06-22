# Druckerstamm: Pfleger

<!-- source: https://amic.de/hilfe/_druckerstammdrst_pfleger.htm -->

<details>
<summary>Einrichtung</summary>

| Druckerstamm – Register Einrichtung | Beschreibung |
| --- | --- |
| Druckernummer | Druckernummer, Ident des Druckerstammes |
| Kurzname | Kurzbezeichnung |
| Bezeichnung | Druckerbezeichnung |
| Queue/Datei | Kann entweder die verfügbare LPTx Schnittstelle sein, welche im Capture zugeordnet wurde oder die Direktansprache für eine Queue in der Syntax \\\\{Druckserver}\\{Druckername}\\<br>Bei Windows-Druck die Bezeichnung der Druckerwarteschlange.<br>Im Windows-System zu finden unter *Systemsteuerung\\Hardware und Sound\\Geräte und Drucker **bzw. Funktion „Druckerdialog“***<br>Mit [F3] können Sie einen Drucker auswählen. |
| Druckertyp | Auswahl mit **F3** aus vorher eingerichteten Druckern (Nadel, Laser usw.), Direktsprung **[DRT]** |
| Einzelblatteinz | 0: Fortlaufender Druck<br>1: anhalten bei Blattwechsel<br>2: anhalten und Meldung |
| Drucker gesperrt | Kennzeichen um Drucker zu sperren<br>Bei JA kann auf diesem Drucker nicht gedruckt werden. |
| Zusatz-Funktionen | Kennzeichen für Zusatz-Funktionen |
| Datei Append | Falls unter QUEUE/Datei eine Datei spezifiziert wurde, bewirkt ein ‚Ja‘, dass die Ausgabe stets an das Ende der Datei angefügt wird, ‚nein‘ löscht die Datei vor der Ausgabe.<br>*Achtung: Bei der Ausgabe auf Druckerwarteschlangen von Novell-Servern diesen Parameter immer auf ‚Nein‘ stellen!* |
| Schließfunktion | Angabe einer Schließfunktion<br>Ruft eine Programmfunktion auf, z.B. „notepad“. In Kombination mit der Angabe eines Dateinamens (z.B. Print.txt) in „Queue“ wird dann hier hineingedruckt und der Notepad geöffnet. |
| Seitenlänge | Vorgabe einer Seitenlänge |
| Nummernkreis (Datei) | Bei „Spooldruck“ kann hier ein Nummernkreis zugeordnet werden, der die „Ausdrucke“ nummeriert |
| Windows Druck | Kennzeichen, ob Drucker ein Windows-Drucker ist |
| Default Font Normal | Optionale Angabe eines Default-Fonts |
| Default Font Compress | Optionale Angabe eines Default-Compress-Fonts |
| Ohne ASCII-Konvert. | Kennzeichen für ASCII-Konvertierung |
| Drucker init. Für COM | Kennzeichen für COM-Schnittstellen-Behandlung |
| Schachtbenutzung | Gibt an, ob ein Schacht verwendet werden soll und wenn ja, welcher der beiden von A.eins unterstützten Schächte der Standardschacht ist, der verwendet werden soll, wenn keine andere Definition gesetzt wurde. |
| Schacht 1 | Der Schacht des Druckers, der als Schacht 1 verwendet werden soll. Die Auswahl kann aus der Liste der für den angeschlossenen Drucker verfügbaren Schächte gewählt werden. |
| Schacht 2 | Der Schacht des Druckers, der als Schacht 2 verwendet werden soll. Die Auswahl kann aus der Liste der für den angeschlossenen Drucker verfügbaren Schächte gewählt werden. |
| Senden An | Kennzeichen ob „Senden-An“-Funktionalität des Archivs unterstützt wird<br>Wird mit diesem Drucker eine Archivierung durchgeführt, so wird im Druckprozess der „Senden an“-Dialog aufgerufen. Dieser ermöglicht dann je nach Einrichtung Versand/Fax der erzeugten Belege. |
| Senden An-Funktion | Angabe einer privaten Datenbank-Funktion.<br>Standard-Name ist p_accept_sendto.<br>Diese Datenbank-Funktion unterstützt Laufzeit-Entscheidungen hinsichtlich der „Senden An“-Funktionalität und des „Nullprinter“-Verhaltens.<br>Innerhalb eines Senden-An-Szenarios fragt das Drucksystem mit Hilfe dieser Funktion folgendes ab:<br>1) Ob der physikalische Druck planmäßig abgeschlossen werden soll. (siehe auch Nulldrucker)<br>2) Ob der planmäßige „Senden-An“-Dialog aufgerufen werden soll (siehe auch Senden An)<br>Die Signatur der Funktion ist wie folgt:<br> in in_cmd char(32),<br> in in_v_id integer,<br> in in_fa_id integer default null,<br> in in_fa_mndnr integer default null<br>in_cmd kann sein:<br>1) ask_print<br>Dann ist nur die Vorgangs-Id in_v_id relevant, die Parameter in_fa_id und in_fa_mndnr sind null.<br>2) ask_sendto<br>Neben der Vorgangsid wird der Primary Key des archivierten Eintrags in in_fa_id/in_fa_mndnr übergeben.<br>Die Funktion muss 0 oder 1 zurückgeben. Wird eine 0 zurückgegeben, wird die planmäßige Funktionalität nicht veranlasst.<br> <br>Die interne Feststellung beruht auf den Inhalt der System-JVARS<br>3567 / DruckerSendenAn und 3567 / DruckerSendenAnProc.<br>Da diese auch makro-gesteuert beeinflusst werden können, ist ein direkter Zusammenhang mit den ebenfalls hier einstellbaren Schaltern „Nulldrucker“ bzw. „Senden An“ nur bedingt herstellbar. |
| Archiv unterdrücken | Kennzeichen ob Drucker zu archivierende Dokumente ins Archiv einstellt. |
| Nulldrucker | Kennzeichen ob Drucker tatsächlich druckt. |
| Bemerkung | Ergänzende Informationen. |

</details>

<details>
<summary>Druckertest</summary>

| Druckerstamm – Register Druckertest | Beschreibung |
| --- | --- |
| Protokoll eines Druckvorganges | Eben dies. |
| Test mit Formular | Druckvorgang wird mit diesem Formular durchgeführt. |

</details>

<details>
<summary>Funktionen des Druckstammpfleger:</summary>

| Druckerstamm Pfleger – Funktionen | Beschreibung |
| --- | --- |
| Speichern und Test | Speichert und führt einen Druckertest durch. |
| Druckerdialog | Öffnet den allgemeinen Windows-Dialog für die Druckeinstellungen. |
| Senden-An Funktion bearbeiten | Bei aktiviertem „Senden an“ wird hiermit die<br>bekannte Möglichkeit die private Funktion zu bearbeiten freigeschaltet.<br>Ist keine Funktion angegeben wird eine Default-Funktion erzeugt und zugewiesen. |

</details>

<details>
<summary>Device-Informationen</summary>

| Druckerstamm – Register Device-Informationen | Beschreibung |
| --- | --- |
| Beim Öffnen des Registers wird eine Analyse auf die Queue durchgeführt.<br><br>Das kann u.U. länger dauern, je nach Verfügbarkeit des Systems. | Angezeigt werden technische Informationen, die A.eins im Falle eines Druckes vom Druckertreiber und Windows zur Verfügung gestellt werden.<br><br><br> |

</details>
