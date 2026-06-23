# Testmandant

<!-- source: https://amic.de/hilfe/_Testmandant.htm -->

Menü: Administration > Werkzeuge > Testmandant (Direktsprung: [TMD])

Dieses Werkzeug bietet die Möglichkeit, einen Testmandanten zu erstellen. Dies ist eine Kopie des aktuellen Mandanten mit sämtlichen Einrichtungen, die nötig sind, um A.eins auf einem Testmandanten zu starten.

<p class="just-emphasize">Voraussetzungen</p>

Es müssen ausreichend Rechte und Speicherplatz auf dem Server und dem Zielordner vorhanden sein.

Die „AmicConf.ini“ wird anhand des Profilnamens angelegt, wenn der Eintrag noch nicht vorhanden ist. Ansonsten wird der Eintrag nicht aktualisiert. Es wird kein automatischer ODBC-Eintrag erzeugt.

Eine weitere Voraussetzung ist, dass die Einstellung der „Serverlog.txt“ auf den Pfad „bin“ oder „bin64“ im A.eins Pfad eingestellt ist.

Beim Erzeugen des Backups wird vorher versucht die Testdatenbank runterzufahren, auch wenn noch Bediener angemeldet sind. Dabei wird angenommen, dass sich die Testdatenbank auf dem gleichen Server befindet und den Namen des Profils hat.

<p class="just-emphasize">Erstellung</p>

Für die Erstellung des Testmandanten stehen folgende Felder zur Verfügung. Nachdem diese ausgefüllt wurden, kann über die Funktion „Erstelle Testmandant“ die Erstellung begonnen werden.

| Feldname | Beschreibung |
| --- | --- |
| Profil | Hier kann ein Profilname eingetragen oder ausgewählt werden. Mit diesem lassen sich gespeicherte Einstellungen schnell wieder laden.<br>Des Weiteren wird der Profilname auch für die „AmicConf.ini“ als Section und Datenbankname verwendet.<br> |
| Aktueller Datenbankpfad | Hier wird der aktuelle Datenbankpfad angezeigt.<br> |
| Zielverzeichnis | Hier kann das Zielverzeichnis des Testmandanten eingetragen werde.<br> |
| Datenbankpräfix | Hier kann ein Präfix für die Datenbank eingetragen werden.<br> |
| Verzeichnis leeren | Hiermit kann eingestellt werden, ob das Zielverzeichnis gelöscht werden soll.<br> |
| Nachlaufprozedur | Hier kann eine Nachlaufprozedur eingetragen werden, mit der bestimmte Einstellungen geändert werden könnten. Standardmäßig wird die Prozedur „*createTestmandantVorlage*“ aufgerufen, in der die unter „Wichtiger Hinweis“ eingetragenen Einstellungen durchgeführt werden.<br>**WICHTIG:** *Wenn man eine eigene Nachlaufprozedur angibt, sollte die Prozedur „createTestmandantVorlage“ als Vorlage verwendet werden, da sonst die unter „Wichtiger Hinweis“ aufgelisteten Arbeitsschritte nicht gemacht werden*<br> |
| Lokales A.eins | Hier kann eingestellt werden, dass die Nachlaufprozedur mit einem lokalen A.eins gestartet wird. Dafür muss die Section richtig eingerichtet sein.<br>Verwenden sollte man diese Einstellung, wenn der Aufruf der Nachlaufprozedur auf dem Server nicht gestartet werden kann.<br> |

<p class="just-emphasize">Profil / Profil löschen</p>

Es existiert die Möglichkeit Profile zu speichern, zu laden oder zu löschen. Beim Erstellen des Testmandanten wird gefragt, ob das Profil gespeichert werden soll.

Mit der Funktion „Profil löschen“ lässt sich das aktuell ausgewählte Profil entfernen

<p class="just-emphasize">Wichtiger Hinweis</p>

Damit es nicht zu Datenverlust kommt, ist vor Erstellung des Testmandanten sicher zu stellen, dass keine privaten Makros, Trigger, Funktionen, Events oder Statements mit absoluten Dateipfaden existieren.

Des Weiteren muss beachtete werden, dass mit der Einstellung „Verzeichnis leeren“ der komplette Ordner einmal gelöscht wird, inklusive der Unterordner.

Nach der Erstellung des Testmandanten werden noch einige Einstellungen geändert oder gelöscht, damit dies nicht zu Problemen mit dem Hauptmandanten führt, solange keine eigene Nachlaufprozedur angegeben wurde. Folgend eine Liste der geänderten Einstellungen:

- Im Mandantenstamm wird das Kennzeichen „MandIstTestmandant“ gesetzt, welches nicht zurückgesetzt werden kann.
- Die Bezeichnung des Mandanten wird so erweitert: „MMTT*(Monat, Tag)*TEST“. Bsp.: am 15.07.2020 ist die Bezeichnungserweiterung „0715TEST“.
- Bei allen Formularstämmen wird das Formulararchiv deaktiviert.
- Die Hintergrundfarbe des Menüs wird auf AMIC-Farbe gesetzt.
- Der letzte Mandantenserverstatus wird zurückgesetzt.
- Dem aktuellen Publikationseigentümer wird der Status entzogen.
- Allen Remotebenutzern werden die REMOTE und CONNECT Rechte entzogen.
- Alle Events werden deaktiviert.
- Alle externen Tabellen werden entfernt.
- Alle entfernten Server werden entfernt.
- Alle Publikationen werden gelöscht.
- Entfernen gelockter Einträge
- Deaktivieren der Mandantenserverprozesse
- openTRANS-Einstellungen im Kunden
- BelegMailVersandeinstellungen im Kunden
- Versandprofile – Servernamen werden verfälscht.
- SPA 933 – TAMMO Optionen werden mit einem leeren Wert auf dem Tagesdatum überschrieben
- SPA 994 – Wird mit einem leeren Wert auf dem Tagesdatum überschrieben
- In der [Auswahlliste 2.0](../auswahlliste_2_0/datentabelle.md#Testmandant) wird als Hintergrund der Text „Testmandant“ eingeblendet.

<p class="siehe-auch">Siehe auch:</p>

- [Erstellung eines Testmandanten auf einen entfernten Server](./erstellung_eines_testmandanten_auf_einen_entfernten_server.md)
- [Parameter für den Testmandanten im Mandantprofil](./parameter_fuer_den_testmandanten_im_mandantprofil.md)
- [Pfleger für das Mandantenprofil](./pfleger_fuer_das_mandantenprofil.md)
