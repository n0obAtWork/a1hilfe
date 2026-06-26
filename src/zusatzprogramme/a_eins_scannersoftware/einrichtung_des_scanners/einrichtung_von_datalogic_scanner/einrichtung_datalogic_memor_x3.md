# Einrichtung Datalogic Memor X3

<!-- source: https://amic.de/hilfe/_datalogic_memor_x3.htm -->

#### Einrichten des WLAN Moduls

Das WLAN Modul wird mit SCU eingerichtet, dies ist an dieser [Stelle](./wlan_einstellung_mit_scu.md) beschreiben worden.

#### Setzen der Hintergrundbeleuchtung

Windowssymbol > Settings > Control Panel > Display

Auf der Registerkarte Backlight muss der Schalter „Turn of backlight when using“

1. Battery Power auf “5 minutes” gestellt werden.

2. External Power auf „10 minutes“ gestellt werden.

#### Power Einstellungen

Windowssymbol > Settings > Control Panel > Power

Mit dieser Funktion kann eingestellt werden, wann der Scanner sich nach einer bestimmten Zeit bei Nichtgebrauch abschaltet. Die Empfehlung ist das automatische Abschalten komplett zu Verhindern.

Dazu wird auf der Registerkarte Auto-Off unter dem Punkt

1. „When battery powerd“ die Einstellung „never“ gewählt.

2. „When externally power“ kann die Einstellung 5 minutes gewählt werden, da der Scanner meistens sich zum Aufladen an der Steckdose sich befindet.

#### Datum/Zeit Einstellung

Windowssymbol > Settings > Control Panel > Date/Time

Hier kann das aktuelle Datum eingestellt werden. Dies ist wichtig bei der Offline Variante, da auch von dem Datum abhängige Einstellungen gelesen werden z.B. Steuerparameter

#### Ausstellen des Scannermodules, da die A.eins Software das Scannermodul selbst startet ab Version 8.0.1.xxx

Klicken auf das Grau oder Gün hinterlegte Barcode Symbol auf der Taskleiste. Das Häkchen vor dem Punkt „Wedge“ entfernen durch das klicken auf den Text „Wedge“. Das Symbol sollte jetzt Grau hinterlegt worden sein.

#### Einstellung des Scannermoduls

Windowssymbol > Settings > Control Panel > Decoding > Configure

Oder

Klicken auf das Grau oder Gün hinterlegte Barcode Symbol auf der Taskleiste. > Configure

Es erscheint jetzt folgende Anzeige.

| Parameter | Value |
| --- | --- |
| [Reader Parameter](./einrichtung_datalogic_memor_x3.md#Readerparamters) | .. |
| [Scan Parameter](./einrichtung_datalogic_memor_x3.md#Scanparameters) | .. |

Mit einem Doppelklick auf den Parameter oder auf die Value wird das jeweilige Untermenü geöffnet.

Sind alle Einstellungen vorgenommen worden wird über File > Save die Einstellungen gespeichert

Die Einstellungsmaske wird per File > Exit verlassen

#### Readerparameters

1. Als erstes muss geprüft werden, ob ein Steuerzeichen vorm oder hinter dem gelesen Barcode angefügt wird. Wenn ja müssen diese Zeichen ab der A.eins Scannerversion 7.8.6.xxx entfernt werden. Bei einer Version vor der 7.8.6.xxx muss als Postamble ein TAB oder ein F“ hinzugefügt werden.

   1.1 Doppelklick auf den Text „Text Formatting“. Die Einstellungen sollten wie folgt aussehen.

| Parameter | Value |
| --- | --- |
| .. | .. |
| Identifier | None |
| Preamble | [] |
| Postamble | [] |
| Seperator | [] |

**7.8.6.xxx**

Ist dies nicht der Fall, so wird mit einem Doppelklick auf den Parameter ein weiteres Fenster geöffnet, mit dem Minuszeichen werden die Einträge entfernt. Dies muss solange wiederholt werden, bis nur noch die beiden eckigen Klammern [] zu sehen sind. Mit OK wird die Eingabe übernommen. Mit einem Doppelklick auf die beiden Punkte wird vorherige Übersicht aufgerufen.

**Versionen vor der 7.8.6.xxx**

Mit einem Doppelklick auf den Postamble öffnet sich ein Fenster. In dem Drop Down Menü wird jetzt F2 oder TAB ausgewählt mit dem + Zeichen wird das Zeichen übernommen. Mit OK wird die Eingabe übernommen. Mit einem Doppelklick auf die beiden Punkte wird vorherige Übersicht aufgerufen.

| Parameter | Value |
| --- | --- |
| .. | .. |
| Identifier | None |
| Preamble | [] |
| Postamble | [TAB] oder [F2] |
| Seperator | [] |

2. Einstellungen für das Lesen von EAN und UPC Codes

   2.1. Doppelklick auf den Parameter UPC/EAN family. Folgende Codes müssen angepasst werden. Mit einem Doppelklick auf die beiden Punkte wird eine Ebene zurück navigiert. Mit einem Doppelklick auf die Value wird diese umgeschaltet. Mit einem Doppelklick auf den Parameter kann die Value Ausgewählt werden.

   2.1.1. Einstellungen UPC-A

| Parameter | Value |
| --- | --- |
| .. | .. |
| UPC-A | Enabled |
| Chk Digit Tx | Enabled |
| To EAN-13 | Disabled |

2.1.2. Einstellungen EAN-8

| Parameter | Value |
| --- | --- |
| .. | .. |
| EAN-8 | Enabled |
| Chk Digit Tx | Enabled |
| To EAN-13 | Disabled |

2.1.3. Einstellungen EAN-13

| Parameter | Value |
| --- | --- |
| .. | .. |
| EAN 13 | Enabled |
| Chk Digit Tx | Enabled |

Sind die Einstellungen vorgenommen worden, so wird mit Doppelklick auf die beiden Punkte solange zurück navigiert, bis der Anfangsbildschirm mit Reader Parameters und Scan Parameters zu sehen ist.

#### Scanparameters

Folgende Parameter müssen umgestellt werden

**Ab Version 7.8.6.xxx**

| Parameter | Standard | Value |
| --- | --- | --- |
| .. | | .. |
| Keyboard Emulation | Yes(Keys) | No |
| Non Printable Char | Enabled | Remove |

**Vor der Version 7.8.6.xxx**

| Parameter | Standard | Value |
| --- | --- | --- |
| .. | | .. |
| Keyboard Emulation | Yes(Keys) | Yes(Keys) |
| Non Printable Char | Leave | Leave |

Durch den Klick auf die Value kann die Einstellung für den jeweiligen Parameter eingestellt werden. Durch ein Doppelklick auf den Parameter öffnet sich eine weitere Maske in dieser kann dann die Value ausgewählt werden.

#### Installieren der Scanner Software

#### Anschließen des Scanners an einen Rechner

Um die Scannersoftware zu installieren wird der Scanner per USB Kabel an den Rechner angeschlossen. Als Voraussetzungen um den Scanner anzuschließen müssen wahrscheinlich folgende Programme Installiert werden. 

1. Bei Windows XP Active Sync

2. Ab Windows Vista Windows Mobile Center

Falls der Scanner nicht automatisch erkennt wird, so muss der USB Treiber für den Scanner nachinstalliert werden. Die USB Treiber können auf der Webseite [http://www.datalogic.com](http://www.datalogic.com) unter Support & Service > Software & Software heruntergeladen werden.

#### Aufspielen der Software

Nach dem Scanner an den Rechner angeschlossen worden ist, muss nun die Software auf den Scanner kopiert und installiert werden.

Als erstes wird die Software auf die Storage Card kopiert, wenn das Gerät mit einer SD Karte ausgeliefert worden. In das Backup Verzeichnis muss die Scanner Software auf jeden Fall kopiert werden. Damit die Software nach einem Cold Boot oder leer gelaufenen Akku neu installiert wird.

My Device > Storage Card

My Device > Backup

Folgende Software Version können auf den Scanner Installiert werden

1. Version 8.0.1.xxx

2. Version 8.1.2.xxx

#### Version 8.0.1.xxx

Diese Software gibt es einmal als Installationsdatei(CAB) oder als Ordner

1. Installieren der Software als Ordner

   1.1. Als erstes wird im Ordner My Device > Backup > Windows(wenn nicht vorhanden neu anlegen) der Ordner „Desktop“ neu angelegt.

   Der Ordner mit der Scanner Software wird in den Windows Ordner kopiert. Dieses muss gemacht werden, denn nach einem Cold Boot wird die Software von dem Backup Ordner in den Windows Ordner kopiert.

   1.2. Der Ordner mit Aeins Software wird in den Windows Ordner My Device > Windows kopiert. Nach dem Kopiervorgang sollte sich jetzt ein Ordner Aeins im Windows Verzeichnis befinden.

   1.3. Nach dem kopieren des Ordners Aeins diesen öffnen und die A.eins.Ce.exe suchen. Dann lange mit dem Stift auf die A.eins.CE.exe klicken, bis sich ein neues Fenster öffnet. Jetzt auf den Punkt „Copy“ klicken. Danach den Explorer mit dem X oben rechts verlassen. Jetzt ist der Desktop wieder zu sehen. Hier mit dem Stift wieder lange auf den Hintergrund drücken bis sich ein MenüFenster öffnet. Dann den Punkt „Paste Shortcut“ auswählen. Danach sollte sich auf dem Symbol befinden „A.eins.Ce.exe“. Danach muss der Shortcut in den Ordner Desktop im Backup Verzeichnis unterhalb des Windows Ordner kopiert werden. My Device > Backup > Windows > Desktop

2. Installieren der Software als CAB Datei

   2.1 Die CAB Datei wird in das Verzeichnis Backup kopiert Device > Backup

   2.2 Danach wird das Programm Files Admin ausgeführt.

   Windowssymbol > Settings > Control Panel > Files Admin.

   In dem Programm „Files Admin“ wird dann der Punkt „Safe Setup“ ausgewählt. Jetzt öffnet sich der „Explorer“. In diesem wird in das Backup Verzeichnis navigiert. In dem Ordner wird die CAB Datei mit der Aeins Software ausgewählt. Danach öffnet sich der Explorer ein zweites Mal. Jetzt muss in das Windows Verzeichnis navigiert werden. Die Software muss in das Windows Verzeichnis installiert werden, weil dieses Verzeichnis nach einem Coldboot auf jeden Fall vorhanden ist. Die anderen Ordner können noch nicht in der Initialisierungsphase des Scanners angesprochen werden.

   Mit einem Klick auf den Button OK oben rechts wird die Installation ausgeführt. Mit OK wird dann das Programm Files Admin geschlossen mit. Mit dem X wird dann die System Steuerung verlassen. Die Desktop Verknüpfung zu der A.eins.Ce.exe Software wird automatisch angelegt.

Nach dem die [Software](../../private_prozeduren/index.md) Installiert worden ist, kann diese dann über das Desktop Symbol gestartet werden.

#### Version 8.1.2.xxx

Diese Version wird nur als CAB Datei ausgeliefert. Es müssen zwei CAB Dateien auf den Scanner kopiert werden.

1. A.eins

2. NETCFv35.Messages.EN

Nach dem Kopieren der Dateien in das Backup Verzeichnis Device > Backup. Muss jetzt das Programm Files Admin ausgeführt werden.

Windowssymbol > Settings > Control Panel > Files Admin

Dieser Schritt muss für alle zwei CAB Dateien wiederholt werden.

In dem Programm „Files Admin“ wird der Punkt „Safe Setup“ ausgewählt. Jetzt öffnet sich der „Explorer“. In diesem wird in das Backup Verzeichnis Navigiert. In dem Ordner wird die CAB Datei mit der zu installierenden Software ausgewählt. Danach öffnet sich der Explorer ein zweites Mal. Jetzt muss in das Windows Verzeichnis navigiert werden. Die Software muss in das Windows Verzeichnis installiert werden, weil dieses Verzeichnis nach einem Coldboot auf jeden Fall vorhanden ist. Die anderen Ordner können noch nicht in der Initialisierungsphase des Scanners angesprochen werden. Mit einem Klick auf den Button OK oben rechts wird die Installation ausgeführt.

Mit OK wird dann das Programm Files Admin geschlossen mit. Die Desktop Verknüpfung zu der A.eins Software wird automatisch angelegt.

Nach der erfolgreichen Installation der [Software](../../private_prozeduren/index.md) kann diese dann gestartet werden.
