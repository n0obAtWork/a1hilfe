# WLAN Einstellung mit SCU

<!-- source: https://amic.de/hilfe/_cescannerwlanscu.htm -->

Um das WLAN mit der integrierten Installations-Software SCU einzurichten gehen Sie wie folgt vor:

Windowssymbol unten links auf der Taskleiste > Settings > Control Panel > SCU (Wi-Fi)

#### Automatische Anlage eines WLAN Profils

1. Als erstes wird die Registerkarte Profile ausgewählt. Mit dem Button Scan werden alle verfügbaren WLAN Verbindungen angezeigt. Jetzt sollte das WLAN Netz angezeigt werden mit dem sich der Scanner verbinden soll. Wird das WLAN nicht angezeigt, ist das WLAN Netz nicht verfügbar oder die SSID ist versteckt worden.

2. Um sich mit dem gewünschten Netz zu verbinden, wird ein Doppelklick auf die SSID gemacht. Danach erscheint eine Abfrage, ob für diese SSID ein Profil angelegt werden soll. Dies wird mit Yes bestätigt. Danach fordert der Scanner den WLAN Key an. Der Key kann per [Tastatur](./index.md#DatalogicTastatur) eingegeben werden.

3. Nach dem Key eingegeben worden ist, wird mit dem Button „commit“ die Daten übernommen.

4. Danach wird auf der Registerkarte Main, das neu angelegte Profil ausgewählt. Mit dem OK Button oben rechts, wird diese Einstellung gespeichert und die Maske wird verlassen. Mit einem erneuten öffnen der des Programmes, kann auf der Registerkarte Status überprüft werden, ob sich der Scanner in das WLAN Netz verbinden konnte. Neben dem Text IP sollte nun eine IP Adresse angezeigt werden.

#### Manuelle Anlage eines WLAN Profils

1. Auswählen der Registerkarte Profil

2. Drücken des Buttons „NEW“.

3. Jetzt muss der Profilname eingegeben werden und mit OK Bestätigen. Diesen Namen finden Sie dann auf dem Startbildschirm.

4. Wählen Sie jetzt bitte auf der linken Seite den Punkt SSID aus und tragen Sie bitte im rechten Eingabefeld den eindeutigen Namen des WLAN ein, mit dem sich der Scanner verbinden soll. Hierbei achten Sie bitte auf Groß- und Kleinschreibung.

5. Als Encryption wählen Sie bitte die von Ihnen gewählte Verschlüsselungsmethode der Access Points aus. Durch das Drücken des Buttons WEP Keys/PSKs werden Sie aufgefordert, den WLAN Schlüssel einzugeben. Hier empfehlen wir Ihnen aus Datenschutzrechtlichen Gründen mindestens eine WPA Verschlüsselung zu wählen. EAP Type lassen Sie bitte auf der Einstellung „None“.

6. Mit Commit werden die Änderungen übernommen.

7. Danach wird auf der Registerkarte Main, das neu angelegte Profil ausgewählt. Mit dem OK Button oben rechts, wird diese Einstellung gespeichert und die Maske wird verlassen. Mit einem erneuten öffnen der des Programmes, kann auf der Registerkarte Status überprüft werden, ob sich der Scanner in das WLAN Netz verbinden konnte. Neben dem Text IP sollte nun eine IP Adresse angezeigt werden.

#### Editieren eines WLAN Profils

1. Auf der Registerkarte Profile wird das zu bearbeitende Profil ausgewählt

2. Jetzt kann die SSID oder der Schlüssel des Profils bearbeitet werden.

3. Mit Commit werden die Änderungen übernommen.

#### Testen der WLAN Verbindung

Auf der Registerkarte DIAGS kann in das Eingabefeld hinter Start Ping eine IP-Adresse z.B. die des Datenbankservers angegeben werden. Durch Drücken der Taste „Start Ping“ wird der Ping durchgeführt. Ist der Ping erfolgreich, so ist der Datenbankserver vom Scanner aus erreichbar.
