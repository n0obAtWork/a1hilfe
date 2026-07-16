# DSO

<!-- source: https://amic.de/hilfe/_dso.htm -->

Das Formulararchiv bedient sich einer Microsoft-Technologie, um im Falle einer Dateieingliederung ins Formulararchiv auf NTFS-Systemen die erweiterten Datei-Attribute zu ermitteln. Die Abfolge wird über das VBA-Script AMIC_FA_INFO abgewickelt.

Technisch muss die Datei *dsofile.dll* als COM-Objekt auf dem System registriert sein.

Das erledigt man „händisch“ auf dem System über **regsvr32 dsofile.dll**
