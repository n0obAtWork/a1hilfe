# Interne Fehlercodes vom AMIC Etikettendruck

<!-- source: https://amic.de/hilfe/internefehlercodesvomamicetike.htm -->

| \-1 | Es wurde eine Funktion mit einem Jobhandle als Parameter aufgerufen, das nicht mit *LlJob-Open*() erzeugt wurde. |
| --- | --- |
| \-2 | Pro Applikation darf nur ein Designerfenster geöffnet sein, Sie haben versucht, ein zweites zu öffnen |
| \-3 | Einer Funktion, die den Objekttyp als Parameter benötigt, wurde ein ungültiger Typ übergeben. |
| \-4 | Es wurde eine Druckfunktion aufgerufen, obwohl noch kein Druckjob gestartet wurde. |
| \-5 | *LlPrintSetBoxText()* wurde aufgerufen, obwohl der Druckjob nicht mit <em>LlPrintWithBoxStart12()</em> geöffnet wurde. |
| \-7 | *LlPrint[G&#124;S]etOption[String](), LlPrintResetProjectState()*. Der Druckjob ist noch nicht gestartet |
| \-10 | *LlPrint[WithBox]Start()*: Es existiert kein Objekt mit dem angegebenen Dateinamen. |
| \-11 | *LlPrint[WithBox]Start()*: Druckerjob konnte nicht gestartet werden, da kein Drucker-Device geöffnet werden konnte. |
| \-12 | Während des Druckens trat ein Fehler auf. Häufigste Ursache:<br>Druckspooler voll, bzw. der vom Druckspooler benötigte Platz ist auf dem Laufwerk auf das TEMP zeigt nicht mehr vorhanden (Pro Seite kann je nach Druckauflösung und verwendeter Grafik ein Platzbedarf von einigen MB entstehen. Abhilfe schafft meist auch die Einstellung des Direktdrucks ohne Spooler). Mögliche Ursache bei Direktdruck: allg. Druckerfehler, Papierstau, etc. |
| \-13 | Beim Exportieren ist ein Fehler aufgetreten (z.B. keine Zugriffsrechte auf Zielpfad, zu exportierende Datei schon vorhanden und schreibgeschützt,...) |
| \-14 | Diese DLL-Version benötigt Visual Basic. |
| \-15 | Bei Druckoptionen: kein Drucker verfügbar. |
| \-16 | Preview-Funktionen: bei <em>LlPrint[WithBox]Start()</em> wurde kein Preview-Mode eingestellt. |
| \-17 | *LlPreviewDisplay()*: Keine Preview-Dateien gefunden. |
| \-18 | NULL Zeiger als Parameter ist hier nicht gestattet, möglicherweise auch andere Parameter-Fehler. Bitte benutzen<br>Sie den Debug-Modus zur Bestimmung des Fehlers. |
| \-19 | Neuer Expression-Modus: Ein Ausdruck in <em>LlExprEvaluate()</em> konnte nicht interpretiert werden. |
| \-20 | Unbekannter Ausdrucks-Modus in *LlSetOption()*. |
| \-22 | *LlPrint[WithBox]Start()*: Projektdatei wurde nicht gefunden. |
| \-23 | *LlPrint[WithBox]Start()*: Einer der verwendeten Ausdrücke hat einen Fehler. Verwenden Sie *LlExprError()*, um den<br>Fehler zu finden. |
| \-24 | *LlPrint[WithBox]Start()*: Projektdatei hat falsches Format oder ist defekt. |
| \-25 | *LlPrintEnableObject()*: Der Objektname ist nicht korrekt. |
| \-27 | *LlPrintEnableObject()*: Es existiert kein Objekt mit diesem Objektnamen. |
| \-28 | wird von <em>LlPrintStart()</em> und <em>LlPrintWithBoxStart()</em> zurückgegeben, wenn ein Listen-Projekt gestartet werden soll, das<br>kein Listenobjekt enthält. Wird nur im neuen Expression-Modus zurückgegeben. |
| \-29 | *LlPrint[WithBox]Start()*: Das Projekt besitzt keine Objekte,und leere Seiten kann man auch anders drucken! |
| \-30 | *LlPrintGetTextCharsPrinted()*: Kein Textobjekt in diesemProjekt. |
| \-31 | *LlPrintIsVariableUsed(), LlPrintIsFieldUsed()*: Die angegebene Variable gibt es nicht. *LlGetUsedIdentifiers()*: Das<br>Projekt wurde noch nicht mit AMIC Etikettendruck 11 oder neuer gespeichert und enthält daher keine Informationen über<br>verwendete Variablen uind Felder. |
| \-32 | Feld-Funktionen wurden benutzt, obgleich das Projekt kein Tabellenprojekt ist. |
| \-33 | *LlPrint[WithBox]Start(), LlDefineLayout()*: Der Ausdruck-Modus der Projektdatei ist der neue Modus, eingestellt ist<br>jedoch der alte Modus (siehe *LlSetOption()*). |
| \-34 | Funktion ist nur anwendbar, wenn der OneTable-Modus gewählt wurde (*LL_OPTION_ ONLYONETABLE*) (siehe<br>*LlSetOption()*). |
| \-35 | Die bei <em>LlGetVariableType()</em> oder <em>LlGetVariableContents()</em> angegebene Variable wurde nicht definiert. |
| \-36 | Das bei <em>LlGetFieldType()</em> oder <em>LlGetFieldContents()</em> angegebene Feld wurde nicht definiert. |
| \-37 | Die über die ID bei den Gruppierungs-Funktionen angegebene Sortierreihenfolge wurde nicht definiert. |
| \-38 | *LlPrintCopyPrinterConfiguration()*: Datei wurde nicht gefunden oder hat falsches Format |
| \-39 | *LlPrintCopyPrinterConfiguration()*: Datei konnte nicht geschrieben werden: Problem mit Festplattenplatz oder Zugriffsrechten |
| \-40 | Reserviert |
| \-41 | Die Storage-Datei enthält keine gültigen Seiten *NOTINHOSTPRINTERMODE* |
| \-42 | Dieser Befehl kann nicht im *HOSTPRINTER*\-Modus aufgerufen werden (z.B. *LlSetPrinterInPrinterFile()*) |
| \-43 | Ein oder mehrere Objekte sind noch nicht fertig gedruckt |
| \-44 | *Ll[G&#124;S]etOptionString(), LlPrint[G&#124;S]etOptionString()*, ...: Ein übergebener Puffer ist nicht groß genug für die darin zu speichernden Daten. |
| \-45 | *LL_OPTION_CODEPAGE*: Die Codepage ist nicht gültig (NLS nicht auf dem System installiert). |
| \-46 | Eine Temporärdatei konnte nicht erzeugt werden (falscher Temp-Pfad!) |
| \-47 | AMIC Etikettendruck hat kein gültiges Ausgabemedium beim Start des Drucks (siehe *LL_OPTIONSTRING_EXPORTS_ALLOWED*) |
| \-48 | *LlPrintDeclareChartRow()*: Kein Chartobjekt im Projekt vorhanden. |
| \-49 | Kann nur in Server/Webserverapplikationen auftreten. Die Anzahl der Benutzer übersteigt die lizensierte Benutzeranzahl. |
| \-50 | Kann nur in Server/Webserverapplikationen auftreten. Die Webserver-Lizenzdatei (\*.wsl) ist ungültig oder beschädigt. |
| \-51 | Kann nur in Server/Webserverapplikationen auftreten. Die Webserver-Lizenzdatei (\*.wsl) wurde nicht gefunden. |
| \-53 | Eine benötigte Grafikdatei wurde nicht gefunden. S. LL_OPTION_ERR_ON_FILENOTFOUND |
| \-99 | Der Benutzer unterbrach den Ausdruck. |
| \-100 | Die von AMIC Etikettendruck benötigten DLLs sind nicht auf dem benötigten Stand. |
| \-101 | Die benötigte Sprach-DLL wurde nicht gefunden, und auch CMLL11@@.LNG ist nicht vorhanden. |
| \-102 | Zu wenig freier Speicher |
| \-104 | Ein GPF ist innerhalb der Funktion aufgetreten. AMIC Etikettendruck könnte bei der weiteren Ausführungen instabil sein |
| \-105 | Es wurde versucht, eine Funktion aufzurufen, die durch den Lizenzumfang nicht gedeckt ist (z.B. Designeraufruf<br>auf Endkundenrechner mit Standardversion) |
| \-996 | In einem hierarchischen Layout ändert sich der Tabellenname. s. Kap. "4. Unterberichte und mehrere Tabellen" |
