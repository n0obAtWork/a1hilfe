# Signature Pad am Arbeitsplatz und Software auf Remoteserver einrichten

<!-- source: https://amic.de/hilfe/_SigPad_WS_RS_einrichten.htm -->

Diese Kapitel beschreibt die Einrichtung des Signature Pads an einer Workstation und dem Einrichten der Applikationssoftware auf dem Remoteserver.

Voraussetzungen und benötigte Software siehe Kapitel [Signature Pad einrichten](./index.md).

<p class="just-emphasize">Einrichtung der Treiber auf der Workstation:</p>

1. Signature Pad an einen freien USB-Steckplatz anschließen.

2. Treiber (Zip-Datei) entpacken.

3. Die im vorigen Schritt entpackte Exe-Datei ausführen.

4. Nach Abschluss der Installation erscheint das Logo von Signotec auf dem Display des Signatur Pads.

<p class="just-emphasize">Einrichten der Remotedesktopverbindung:</p>

1. Dialog für „Remotedesktopverbindung“ öffnen.

2. Schaltfläche „Optionen einblenden“ drücken.

3. Feld „Computer“ ausfüllen.

4. Feld „Benutzername“ ausfüllen.

5. Reiter „Lokale Ressourcen“ auswählen.

6. Schaltfläche „Weitere …“ im Bereich „Lokale Geräte und Ressourcen“ drücken.

7. Auswahlfeld „Ports“ aktivieren.

8. Schaltfläche „OK“ drücken.

9. Schaltfläche „Verbinden“ drücken.

10\. Am Remoterechner anmelden.

<p class="just-emphasize">Einrichtung der Werkzeug-Software auf dem Remoteserver:</p>

1. Setup-Datei der Werkzeug-Software ausführen.

2. Sprache auswählen.

3. Lizenzvereinbarung annehmen.

4. Pfad übernehmen (hier „C:\\Program Files\\signotec\\“).

5. Installation „Vollständig“ auswählen.

6. Funktionsweise: 1. Option auswählen.

7. Treiber „Signotec_WinUSB_64Bit“ + „Signotec_HID_64Bit“ nicht installieren.

<p class="just-emphasize">Einrichtung der Software „SignoSign/2“ auf dem Remoteserver:</p>

1. Setup-Datei der Software „SignoSign/2“ ausführen.

2. Sprache auswählen.

3. Lizenzvereinbarung annehmen.

4. Pfad übernehmen (hier C:\\Programme\\signotec).

5. Setup-Option „Angepasst“ und alle Programmteile auswählen.

6. Treiber „Signotec_WinUSB_64Bit“ + „Signotec_HID_64Bit“ nicht installieren.

7. Das Programm „SignoSign/2“ starten.

8. Im Fenster „Wichtige Informationen“ das Auswahlfeld „Nicht mehr anzeigen“ deaktivieren und die Schaltfläche „OK“ drücken.

9. Das Fenster mit dem Hinweis Evaluierungsversion mit Schaltfläche „OK“ bestätigen.

10. Den Menü-Eintrag „Extras“ -> „Konfiguration“ wählen. Das Fenster „Konfiguration“ wird geöffnet.

11. Den Reiter „Signaturgeräteeinstellungen“ (5ter Reiter von oben) wählen.

12. Im Abschnitt „Signaturgerät auswählen“ die Schaltfläche „Suche konfigurieren“ drücken.

13. Im Fenster „Konfiguration der Pad-Suche“ das Auswahlfeld „Nach seriellen Geräten suchen“ aktiveren.

14. Die Schaltfläche „OK“ drücken. Das Fenster wird geschlossen.

15. Die Schaltfläche „Geräte suchen“ drücken. Die Tabelle mit den Signaturgeräten wird aktualisiert.

16. Ein Signaturgerät selektieren und die Schaltfläche „Erstes Signaturgerät zuweisen“ drücken.

17. Die Schaltfläche „OK“ drücken. Das Fenster „Konfiguration“ wird geschlossen.

18. Die Lizenzdatei „licence.lic“ ins Verzeichnis „Aeins\\User\\Lizenz\\SignoSign2“ kopieren.

19. Den Menüpunkt „Hilfe“ -> „Lizensierung…“ auswählen. Das Fenster „Lizenzinformationen“ wird geöffnet.

20. Im Abschnitt „Lizenz aktualisieren“ das Auswahlfeld „Lizenzdatei verwenden“ auswählen.

21. Im Abschnitt „Lizenz aktualisieren“ die Schaltfläche „…“ drücken. Ein Dateiauswahl-Dialog wird geöffnet.

22. Lizenzdatei „Aeins\\User\\Lizenz\\SignoSign2\\licence.lic“ auswählen und Schaltfläche „Öffnen“ drücken. Der Auswahl-Dialog wird geschlossen. Im Abschnitt „Lizenzinformation“ erscheinen die Informationen zur Lizenz.

23. Schaltfläche „Schließen“ drücken. Das Fenster „Lizenzinformation“ wird geschlossen.

24. Das Programm „SignoSign/2“ beenden.

<p class="just-emphasize">Erstellung der allgemeinen Dokumententypvorlage auf dem Remoteserver:</p>

1. Das Programm „SignoSign/2“ starten.

2. Den Menü-Eintrag „Extras“ -> „Dokumenttypen“ auswählen. Das Fenster „Dokumenttypen­administration“ wird geöffnet.

3. Die Schaltfläche „Neuer Dokumenttyp“ (Bild: Textseite und Plus-Zeichen) drücken.

4. Die PDF-Datei „allgemein.pdf“ als Vorlage auswählen und die Schaltfläche „Öffnen“ drücken.  
Achtung: Die ausgewählte PDF-Datei wird gelöscht!

5. Im Abschnitt „Dokumententyp“ in das Feld „Name“ die Text „Allgemeine Vorlage“ eingeben.

6. Im Abschnitt „Dokumententyp“ in das Feld „Beschreibung“ den Text „Allgemeine Vorlage“ eingeben.

7. Im Abschnitt „Identifizierungsbegriffe“ die Schaltfläche „Identifizierungsbegriff hinzufügen“ (Bild: Plus-Zeichen) drücken.

8. Im neu geöffneten Fenster „Identifizierungsbegriff konfigurieren“ im Feld „Suchbegriff“ ein Leerzeichen eingeben. Das Feld ist nicht mehr rot umrandet.

9. Die Schaltfläche „OK“ drücken. Das Fenster wird geschlossen. In der Tabelle „Identifizierungs­begriffe“ erscheint ein Eintrag.

10. Im Abschnitt „Dokumentenverarbeitung“ das Auswahlfeld „Erstellung von Unterschriftenfeldern mit der Maus erlauben“ deaktivieren.

11. Im Abschnitt „Dokumentenverarbeitung“ in der Auswahlliste „Nach dem Öffnen des Dokumentes:“ den Eintrag „Signaturvorgang starten“ auswählen.

12. Die Schaltfläche „Übernehmen“ drücken.

13. Den Reiter „Signaturfeldeinstellungen“ (3ter Reiter von oben) auswählen.

14. Im Abschnitt „Globale Signaturfeldeinstellungen“ das Auswahlfeld „Vor dem Signaturvorgang ist die Bestätigung folgenden Textes erforderlich“ deaktivieren.

15. Im Abschnitt „Signaturfeld definieren“ die Schaltfläche „Neues Signaturfeld definieren“ (Bild: Plus-Zeichen) drücken.

16. Im neu geöffneten Fenster „Position festlegen“ den Abschnitt „Absolute Positionierung“ auswählen.

17. Es erscheint ein blaues Feld (Signierfeld) in der Vorschau. Dieses kann mit der Maus positioniert und in der Größe verändert werden.  
Alternativ können auch die Werte im Abschnitt „Absolute Positionierung“ geändert werden.

18. Die Schaltfläche „OK“ drücken. Das Fenster wird geschlossen.

19. Im neu geöffneten Fenster „Signaturfeld-Einstellungen bearbeiten“ im Abschnitt „Allgemeine Einstellungen“ in das Feld „Feldname“ den Text „Signaturfeld“ eintragen.

20. Die Schaltfläche „OK“ drücken. Das Fenster wird geschlossen. In der Tabelle „Signaturfelder definieren“ erscheint der Eintrag „Signaturfeld“.

21. Die Schaltfläche „Übernehmen“ drücken.

22. Die Schaltfläche „OK“ drücken. Das Fenster „Dokumenttypenadministration“ wird geschlossen.

23. Das Programm „SignoSign/2“ beenden.

<p class="just-emphasize">Signiertes Dokument automatisch drucken (Optional)</p>

1. Das Programm „SignoSign/2“ starten.

2. Den Menü-Eintrag „Extras“ -> „Dokumententypen“ auswählen. Das Fenster „Dokumententypen­administration“ wird geöffnet.

3. Den Dokumententyp „Alle“ auswählen.

4. Im Abschnitt „Dokumentverarbeitung“ das Auswahlfeld „Archivierung nach dem Signiervorgang automatisch starten“ aktivieren.

5. Den Reiter „Druckeinstellungen“ (2ter Reiter von oben) auswählen.

6. Im Abschnitt „Allgemeine Einstellungen“ in Auswahlliste „Standarddrucker“ den zu benutzenden Drucker auswählen. Der Server muss Zugriff auf diesen Drucker haben.

7. Im Abschnitt „Allgemeine Einstellungen“ im Feld „Anzahl Kopien“ die Anzahl der zu erstellenden Kopien einstellen.

8. Wenn kein benutzerdefinierter Text ab der 2ten Kopie erwünscht ist, mit Schritt 11 fortfahren.

9. Im Abschnitt „Benutzerdefinierten Text ab der 2ten Kopie drucken“ im Feld „Text“ den zu druckenden Text eingeben.

10. Im Abschnitt „Benutzerdefinierten Text ab der 2ten Kopie drucken“ in den Feldern „Abstand oben“ und „Abstand links“ die Position des zu druckenden Textes festlegen.

11. Den Reiter „Archivierungseinstellungen“ (6ter Reiter von oben) auswählen.

12. Im Abschnitt „Archivdienst konfigurieren“ im Feld „Archivordner“ den Pfad zum Speichern der Archivdateien festlegen.

13. Im Abschnitt „Zusätzliche Aktionen“ das Auswahlfeld „Beim Archivieren automatisch drucken“ aktivieren.

14. Im Abschnitt „Zusätzliche Aktionen“ das Auswahlfeld „Die Quelldatei nach dem Archivieren des Dokumentes löschen“ deaktivieren.

15. Die Schaltfläche „Übernehmen“ drücken.

16. Die Schaltfläche „OK“ drücken. Das Fenster „Dokumenttypenadministration“ wird geschlossen.

17. Das Programm „SignoSign/2“ beenden.

<p class="just-emphasize">Allgemeine Einrichtung auf dem Remoteserver:</p>

1. Aeins starten.

2. Direktsprung [FAM] ausführen und danach Reiter „Sonstiges“ auswählen.

3. Prüfen, ob das in Feld „Signatur-Importpfad“ angegebene Verzeichnis vorhanden ist. Wenn nicht, dann ist dieses Verzeichnis anzulegen.

4. In ein Verzeichnis mit einer/mehrere PDF-Datei(en) wechseln.

5. Eine PDF-Datei selektieren und im Popup-Menü den Eintrag „Öffnen mit“ -> „Andere App auswählen“ bzw. „Standardprogramm“ auswählen.

6. Das Programm „signotec signoSign/2“ auswählen und das Auswahlfeld „Immer diese App zum Öffnen von .pdf-Dateien verwenden“ bzw. „Datentyp immer mit dem ausgewählten Programm“ aktivieren.

7. Die Schaltfläche „OK“ drücken.
