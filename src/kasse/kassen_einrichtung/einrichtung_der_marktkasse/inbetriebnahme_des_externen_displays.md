# Inbetriebnahme des externen Displays

<!-- source: https://amic.de/hilfe/inbetriebnahmedesexternendispl.htm -->

Starten Sie die Software „A.eins.KassenDisplay.exe“ mit einem Startparameter. Dieser soll den Pfad incl. des Namens der Datei enthalten, die die Konfigurations- und Anzeigedaten enthält. Den Pfad entnehmen Sie den Einstellungen „Client“ im Rahmen „Hardware externes Display“ auf der Registerkarte „Geräte“ in der Kassenverwaltung.

Der Name der Datei setzt sich zusammen aus „Kasse00“+ der Kassennummer + „.kas“. Also für die Kasse 7 „kasse007.kas“.

<p class="just-emphasize">Beispiel für Kasse 7:</p>

```text
A.eins.KassenDisplay.exe \\rechnername\freigabe\kasse007.kas
```

Nach dem Start des Programms wird zunächst die Konfiguration gelesen. Diese wird beim Start der Kasse in das gleiche Verzeichnis wie die im Aufruf angegebene Datei mit der Endung „.kas“ geschrieben. Die Konfigurationsdatei endet auf „.cfg“.

Ist diese Datei noch nicht vorhanden, wird zunächst die Meldung ausgegeben, dass diese noch nicht vorhanden ist und beim Start der Kasse erstellt werden wird. Starten Sie einen Kassenvorgang oder die Funktion „ext. Display testen“ in der Kassenverwaltung, um die Konfigurationsdatei zu erzeugen.

Ist aus vorherigen Kassenvorgängen eine Konfigurationsdatei vorhanden, so wird das externe Kassendisplay geöffnet und zeigt die konfigurierten Felder an.

<p class="just-emphasize">Hinweis zur Fenster- und Bildschirmgröße:</p>

Ist das Fenster zu groß für den Bildschirm eingerichtet oder ragt aufgrund der Einrichtung über die Grenzen des sichtbaren Bildschirms hinaus, so wird dies automatisch korrigiert und eine Meldung ausgegeben. Sie können mit Hilfe der Funktion „Positionen anzeigen“ aus dem Icon-Tray der Anwendung die Position und Größe des Fensters abrufen, nachdem Sie das Fenster auf den korrekten Bildschirm verschoben und in der Größe angepasst haben. Diese Werte können Sie dann auch in die Kassenkonfiguration übertragen, um die Meldung zukünftig zu umgehen und das Fenster an der gewünschten Stelle darzustellen.
