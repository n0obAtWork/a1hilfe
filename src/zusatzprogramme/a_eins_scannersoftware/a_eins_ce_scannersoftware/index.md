# A.eins.CE Scannersoftware

<!-- source: https://amic.de/hilfe/a_eins_ce.htm -->

Die Scannersoftware A.eins vor der Version 7.8.6..xxx ist nicht an die Hardware gebunden und benötigt daher andere Einstellungsmöglichkeiten

Die A.eins.CE Scannersoftware kann ab der **Version 7.8.6.xxx** nur auf CE Geräten von bestimmten Herstellern installiert werden.

Folgende Scanner werden mit folgende Plattformen unterstützt.

1. Datalogic mit Windows CE

2. Motorola Symbol mit Windows CE

3. Intermec mit Windows Mobile 6.5

<p class="just-emphasize">Installation der Software</p>

Die Installation der Software ist an dieser Stelle [beschrieben](../einrichtung_des_scanners/index.md).

<p class="just-emphasize">Starten der Software</p>

Nach dem die Scannersoftware [Installiert](../einrichtung_des_scanners/index.md) worden ist kann diese gestartet werden. Beim ersten Start der Software müssen die Verbindungsdaten eingetragen werden. Ab der Version 8.1.2.xxx werden die Verbindungsdaten Kundenspezifisch mit der Software ausgeliefert. Dies bedeutet, dass nach der ersten Installation die Verbindungsdaten nicht eingetragen werden müssen.

<p class="just-emphasize">Ab Version 8.1.2.xxx</p>

Erscheint beim Starten der Software ein Fenster mit dem Fehlercode 78 so kann keine Verbindung zur Datenbank aufgebaut werden. Dies kann mehrere Ursachen haben.

1. Die WLAN Verbindung ist nicht eingerichtet

2. Die Datenverbindungsdaten sind nicht korrekt eingegeben. Um die Datenbankverbindungsdaten einzugeben, muss auf dem Fenster unten rechts auf den Punkt Configuration geklickt werden.

<p class="just-emphasize">Version 8.0.1.xxx</p>

Wird die Software gestartet und die Datenbank Verbindung kann nicht aufgebaut werden, startet die Scannersoftware mit einem großem rotem X. Dass die Verbindung zur Datenbank nicht aufgebaut werden kann, kann mehrere Ursachen haben.

1. Die WLAN Verbindung ist nicht eingerichtet

2. Die Datenbankverbindungsdaten wurden nicht richtig eingegeben. Um die Verbindungsdaten einzugeben wird oben rechts auf das Symbol neben dem Eingabefeld geklickt

<p class="just-emphasize">Einrichtung der Datenbankverbindung</p>

Die Datenverbindungs Daten können in Software mit einem Klick auf den „Grünen Haken“ oder den roten Kreis geändert werden. Dann erscheint der Punkt DB-Daten ändern.

Die Erfassungsmaske zur Eingabe der Verbindungsdaten hat sich von Version zu Version verändert. Folgende Felder sind in allen Version vorhanden.

1. Engine

2. DB-Name

3. User-ID

4. Password

5. Commlinks

6. Extras

Ab der Version vor der 7.8.5.xxx sind folgende Felder hinzugekommen

1. Section

2. Profilname

Ab der Version 8.0.1.xxx sind folgende Felder hinzugekommen

1. Remote

2. IP-Adresse

Ab der Version 8.1.2.xxx sind folgende Felder hinzugekommen

1. Profil IP-Adresse

Folgende [Kommandozeilenparameter](../../amic_etikettendruck/kommandozeile.md) stehen zur Verfügung.

<p class="just-emphasize">Maskenfelder</p>

| Felder | Bedeutung |
| --- | --- |
| Section | Hier kann ein Profil ausgewählt werden |
| Profilname | Name des Profils |
| Engine | Hier wird der Name der Datenbankmaschine eingetragen |
| DB-Name | Hier wird der Name der Datenbankdatei eingetragen |
| User-ID | Benutzer zur Anmeldung |
| Password | Passwort für den Benutzer |
| CommLinks | Verbindungsart (tcpip) |
| Extra | Setzen z.B. des IDLE oder der Liveness Timeout(lto=30)<br>1. Setzen von lto=30<br>2. Setzen von idle=60<br>3. Setzen von pooling=false<br>Beispiel lto=30;idle=60;pooling=false; |
| Remote | Pfad zum Ordner in dem sich die Datenbank + dbremote.exe befinden |
| IP-Adresse | Hier wird die IP-Adresse die der Scanner im Offlinebetrieb verwenden soll angegeben, oder eine Alibi IP Adresse. Eine in diesem Feld hinterlegte IP-Adresse überschreibt, die eigentliche IP Adresse des Scanners. Dies ist vom Vorteil, wenn der Scanner in unterschiedlichen Netzen eingesetzt werden soll. Damit müssen die Steuerparameter für den Scanner nur einmal eingerichtet werden. |
| Standard | Mit setzten des Haken, wird dies das aktive Profil |
| Profil IP Adresse | In diesem Feld wird hinterlegt welches Profil Standardmäßig gezogen wird unabhängig vom Standard Profil. Die IP Adresse des Scanners wird mit dem Profil verknüpft, so dass bei einer Kundenbezogenen CAB Datei jeder Scanner mit einem Unterschiedlichen Bediener automatisch startet. Dies hat den Vorteil, dass nach einem Cold Boot die A.eins Software auf dem Scanner wieder Einsatzbereit ist, ohne dass die Konfigurationsdatei eingerichtet werden muss. Dieses Feld wird von AMIC gepflegt. |

<p class="just-emphasize">Liveness Timeout lto</p>

Der Zeitraum für die Verfügbarkeitszeitüberschreitung der Verbindung (in Sekunden). Der Mindestwert für den Verbindungsparameter LivenessTimeout ist 30 Sekunden, und der Höchstwert ist 32767 Sekunden. Wenn Sie "0" angeben, ist die Überprüfung der Verfügbarkeitszeitüberschreitung deaktiviert. Jeder Wert ungleich NULL, der niedriger als der Mindestwert ist, wird auf den Mindestwert zurückgesetzt. Beispiel: Eine Verbindungszeichenfolge, die "LivenessTimeout=5" enthält, verwendet "LivenessTimeout=30". Falls kein LivenessTimeout-Wert gesetzt ist, wird die Verfügbarkeitszeitüberschreitung durch die Einstellung auf dem Server gesteuert, die standardmäßig 120 Sekunden beträgt.

<p class="just-emphasize">Neuanlage eines Profils.</p>

Um eine Profil neu anzulegen müssen alle Felder ausgefüllt werden. Soll dies Profil das aktive Profil werden, so muss der Haken bei Standard gesetzt werden. Mit OK wird das Profil gespeichert, ist das Feld Standard aktiviert, so wird das Profil in die Datei dbconinf.xml übernommen. 

<p class="just-emphasize">Ändern eines Profils</p>

Um ein Profil zu ändern wird das gewünschte Profil über Section ausgewählt. Jetzt können die Änderung vorgenommen werden. Mit OK wird die Änderung übernommen.

<p class="just-emphasize">Löschen eines Profils</p>

Um ein Profil zu löschen wird das gewünschte Profil über Section ausgewählt. Mit Löschen wird das Profil gelöscht.

<p class="just-emphasize">Verhaltenswiese des Scanners wenn die WLAN Verbindung abbricht.</p>

Bricht während der Datenübertragung zum Server die WLAN Verbindung ab, so wird ein Stoppschild angezeigt. Solange das Stoppschild angezeigt wird können keine Datenerfasst werden. Das Stoppschild kann nur durch drücken auf das selbige Ausgeblendet werden. Wird auf das Stoppschild gedrückt probiert das System eine Verbindung zum Datenbankserver aufzubauen. Ist dies nicht möglich, so wird das Stoppschild wieder angezeigt.

Daten die während des Abbruch der Verbindung zum Datenbankserver und dem Anzeigen des Stoppschildes noch erfasst worden sind, werden bei der nächsten Verbindung an den Datenbankserver übertragen. Während dieser Phase wird ein Wartenbild angezeigt.

<p class="just-emphasize">Funktionsweise der Software</p>

<p class="just-emphasize">Vor der 7.8.6.xxx Version</p>

1. Eingabe der Menge muss mit Enter Bestätigt werden

2. Manuell eingegebene EAN Nummern oder Scancodes müssen mit F2 oder TAB bestätigt werden.

<p class="just-emphasize">Ab der Version 7.8.6.xxx</p>

1. Manuell eingegebene Werte werden mit Enter bestätigt

2. Das Programm aktiviert oder deaktiviert je nach Fehlersituation den Scanner.

3. Wird eine Fehlermeldung auf dem Scanner ausgegeben so kann erst dann weitergearbeitet werden, wenn aktiv auf das Fehlerbild geklickt wird.

4. Wird die WLAN Verbindung verlassen kann erst dann weiter gearbeitet werden, wenn wieder eine Verbindung möglich zum Datenbankserver möglich ist.

5. Wenn keine Verbindung beim Starten zum Datenbankserver hergestellt werden kann und die Steuerparameter nicht geladen wurden, so kann mit dem Programm nicht gearbeitet werden.

<p class="just-emphasize">Kommandozeilen Parameter</p>

Sind an dieser [Stelle](../../amic_etikettendruck/kommandozeile.md) beschrieben.

<p class="just-emphasize">Itembox als HTML darstellen</p>

Mit dem [Steuerparameter 842](../../../firmenstamm/steuerparameter/scanner/html_anzeige_beim_scanner_spa_842.md) kann eingestellt werden, ob der Scanner das Ergebnis einer Itembox in eine HTML Seite umwandeln soll.

Das Erscheinungsbild des HTML kann in der Itembox angepasst werden.

<p class="just-emphasize">Zelleninhalt</p>

Hinter jedem FIELD in der Itembox kann mit dem Kommandowort XML auf das Schriftbild Einfluss genommen werden. Dabei ist zu beachten, dass die &lt;td> Anweisung in der gleichen Zeile Steht wie das FIELD. Der Text darf nicht umgebrochen werden. Das Abschließende HTML Tag &lt;/td> wird Automatisch gesetzt. Bei der Farbensteuerung ist darauf zu achten, dass Farbe nur per Name oder per RGB zu setzten ist. Das Setzen der Farbe mit Hexadezimal werten führt zur falschen Darstellung.

z.B.

FIELD Pos,PositionsNummer,I4,10,XML=&lt;td align="left"> &lt;font face="Arial" size="3" color="black">

<p class="just-emphasize">Tabellenattribute</p>

Des Weiteren ist es möglich mittels Parameter auf die Tabellen Eigenschaften zuzugreifen. Dazu wird in der Itembox hinter dem Schlagwort PARAMS der Parameter Table angegeben. Damit kann z.B. die „border“ gesetzt werden

Beispiel:

PARAMS TABLE=border="1" align="right"

Es können alle Attribute gesetzt werden, die im HTML Tag &lt;table> zugelassen sind. Hierbei ist auch zu beachten, dass die Zeile nicht umgebrochen wird.

<p class="just-emphasize">Style Sheet</p>

Im Standard wird ein Style Sheet für jede Itembox verwendet. Das Style Sheet wird durch die Datenbankfunktion [ScannerHTMLStyleSheet](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_function_ScannerHTMLStyleSheet_1_bb3a7a4f.htm) bereitgestellt.

An dem [Steuerparamter 880](../../../firmenstamm/steuerparameter/scanner/html_style_sheet_spa_880.md) kann eine private Datenbankfunktion hinterlegt werden. In dieser Funktion kann dann für jede Itembox ein eigenes Style Sheet definiert werden kann. Als Eingangsparameter hat die Funktion den Namen der Itembox.

<p class="just-emphasize">Spezielle Tasten</p>

| Taste | Bedeutung |
| --- | --- |
| ESC | Beendet die Scanner Software |
| Enter | Manuelle Eingabe von Werten |
| KeyUP | Blättern in der Ansicht der erfassten Daten nach oben |
| KeyDown | Blättern in der Ansicht der erfassten Daten nach unten |
| KeyLeft | Alternative Itembox falls eine eingerichtet worden ist |
| KeyRight | Rücksprung zur eigentlicher Itembox wenn eine Alternative Itembox eingerichtet worden ist |
| F1 | Sondertaste kann in einer privaten Funktion abgefangen werden.<br>Diese wird mit dem AI-Code -1 und den Wert KEYF1 übermittelt |
| F2 | Sondertaste kann in einer privaten Funktion abgefangen werden.<br>Diese wird mit dem AI-Code -1 und den Wert KEYF2 übermittelt |
| TAB | Mit der Taste kann ein manuell eingegebener Scancode an das System übergeben werden.<br>Dies wird nicht mehr benötigt, da die Entertaste das Verhalten jetzt übernimmt. |

<p class="siehe-auch">Siehe auch:</p>

- [Anbindung eines Fremd SDK von einem dritt Hersteller](./anbindung_eines_fremd_sdk_von_einem_dritt_hersteller.md)
