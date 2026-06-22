# Archivierungsmerkmale der Dokumente

<!-- source: https://amic.de/hilfe/_archivierungsmerkmal.htm -->

| Felder | |
| --- | --- |
| Quick-Reporte | Ja/Nein  
Gibt an, ob Quickreporte archiviert werden sollen. |
| Nur letzte Korrektur speichern | Beim erneuten Druck eines schon archivierten Vorgangs, der erneut zur Archivierung führen würde, wird anhand dieses Kennzeichens entschieden, wie die Archivierung durchgeführt wird:  
• **Nein**  
Der erneute Druck erzeugt in jedem Falle einen neuen Archiv-Eintrag  
• **Ja**  
Wenn die Belegnummer, Klassennummer, Unterklassennummer, Unternummer und die Jahrnummer des zu druckenden Beleges mit Belegen aus dem Formulararchiv übereinstimmen, dann wird derjenige Beleg von diesen Belegen mit dem jüngsten Änderungsdatum als Vergleichsbeleg herangezogen.  
In diesem Falle wird kein neuer Formulararchiv-Eintrag erzeugt, sondern das Dokument in der Tabelle „Archiv“ aktualisiert.  
Findet eine Aktualisierung des Dokumentes statt, wird die „Inkarnation“ aus dem Formulararchiv jeweils um 1 erhöht.  
   
Andernfalls wird ein neuer „Archiv-Eintrag“ erzeugt.  
• **Ja, Formular berücksichtigen**  
Wenn die Belegnummer, Klassennummer, Unterklassennummer, Unternummer und die Jahrnummer und die Formularid des zu druckenden Beleges mit Belegen aus dem Formulararchiv übereinstimmen, dann wird derjenige Beleg von diesen Belegen mit dem jüngsten Änderungsdatum als Vergleichsbeleg herangezogen.  
In diesem Falle wird kein neuer Formulararchiv-Eintrag erzeugt, sondern das Dokument in der Tabelle „Archiv“ aktualisiert.  
Findet eine Aktualisierung des Dokumentes statt wird die „Inkarnation“ aus dem Formulararchiv jeweils um 1 erhöht.  
   
Andernfalls wird ein neuer „Archiv-Eintrag“ erzeugt.  
• **Ja, keine erneute Archivierung durchführen**  
Wenn die Vorgangsid des zu druckenden Beleges mit Belegen aus dem Formulararchivs übereinstimmt, dann wird derjenige Beleg von diesen Belegen mit dem jüngsten Änderungsdatum als Vergleichsbeleg herangezogen.  
In diesem Falle wird kein neuer Formulararchiv-Eintrag erzeugt, und auch nicht das Dokument archiviert.  
Es hat somit den Effekt als hat keine Archivierung stattgefunden!  
   
Das Archivierungs-Basissystem gibt in diesem Falle „Status OK“ mit den Rahmendaten des Vergleich-Beleges zurück (also fa_id, fa_mndnr, fa_mandant)  
Andernfalls wird ein neuer „Archiv-Eintrag“ erzeugt. |
| Mandantherkunft | Entscheidet darüber was bei der Archivierung im Feld „Mnd“ des Formulararchives eingetragen wird.  
• Sektion  
Der Name der Sektion aus der ahoi.ini des Aeins-Systems  
• Kurztext  
Der Kurztext der Firma (siehe **[MND]**)  
• Nummer  
Die Nummer der Firma (siehe **[MND]**) |
| Default-Mandant | Im Standardfall wird der Mandant (Feld „Mnd“ des Formulararchivs) gemäß obiger „Mandantherkunft“ ermittelt.  
Anwendungen verwenden für die Archivierung die Standard Datenbank-Funktion amic_fa_set. Wird im dort bei der Mandanten-Ermittlung ein „leerer“ Mandant festgestellt, dann wird dieser hier eingetragene „Default-Mandant“ verwendet. |
