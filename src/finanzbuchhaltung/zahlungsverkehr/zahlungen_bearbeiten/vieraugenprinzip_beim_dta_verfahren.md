# Vieraugenprinzip beim DTA Verfahren

<!-- source: https://amic.de/hilfe/vieraugenprinzipbeimdtaverfahr.htm -->

Hauptmenü > Mahn-,Zahl-, Zinswesen > Zahlungsverkehr > Zahlungen bearbeiten > Variante „Zahlungen nach DTA-Laufnr. Vieraugen“

Direktsprung **[ZHB]**

Gerade im Zahlungsverkehr kann es wünschenswert sein, dass es eine Trennung zwischen den Bedienern gibt, die einen Zahlungsauftrag erstellen und denen die den Auftrag prüfen und dann an die Bank geben. Um das zu gewährleisten sind folgende Einrichtungsschritte notwendig:

1) Die Bediener, welche die Zahlungen erstellen, müssen in einer anderen Bedienerklasse sein als die Bediener, die die Zahlungen kontrollieren und freigeben.

2) Der [Steuerparameter 716](../../../firmenstamm/steuerparameter/optionen_finanzwesen/vieraugenprinzip_zahlungen_spa_716.md) „Vieraugenprinzip beim DTA Verfahren“ muss angeschaltet werden. Dieser Steuerparameter sorgt dafür, dass die Variante „Zahlungen nach DTA-Laufnr. Vieraugen“ aktiviert wird, und beim Erstellen des Datenträgeraustausch die Ausgabedatei nicht geschrieben wird und auch der Explorer – unabhängig von den Einstellungen in den Einrichterparametern – nicht geöffnet wird.

Die Funktion „Übernahme in die Primanota“ steht bei gesetztem Steuerparameter nur in der Variante „Zahlungen nach DTA-Laufnr. Vieraugen“ zur Verfügung. Dort werden nur Zahlungen an die Primanota übertragen, deren Status „Übertragen“ ist.

3) Der Druck der Banksammelliste, des Diskettenbegleitzettels und der Avisen werden weiterhin beim Erstellen abgefragt.

4) Folgende Nachlauffunktionen, die in den Einrichterparametern der Erstellungsmaske eingestellt werden können, werden beim Zusammenstellen der Daten weiterhin ausgeführt und können z.B. dazu genutzt werden, um die Anwender aus der Kontroll-Bedienerklasse per Mail automatisch zu informieren. Zu beachten ist, dass die Datei, die mit /FILE= an das Skript übergeben wird, nicht existiert:

a. VBS-Skript ausführen.

b. VBA-Skript ausführen.

c. SQL-Prozedur ausführen.

d. Crystal Report ausführen.

5) Mithilfe des Schutzsystems „Zugriffsrechte Varianten“ (Direktsprung [ZUGV]) muss die Variante „Zahlungen nach DTA-Laufnr. Vieraugen“ für die Kontroll-Bedienerklasse erlaubt sein, alle anderen Varianten müssen für diese Bedienerklasse gesperrt sein.

Ist alles so weit eingerichtet, können die Zahlungen wie gewohnt erstellt werden. Ein Anwender aus der Kontroll-Bedienerklasse kann anschließend die Zahlung prüfen, die DTA-Datei erstellen und transportieren. Dafür muss die Funktion Transfer starten F9 in der Variante „Zahlungen nach DTA-Laufnr. Vieraugen“ ausgeführt werden.

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung](../../../ImagesExt/image8_660.png "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung")

Diese Maske enthält genau wie die Maske für das Erstellen des DTA’s [Einrichterparameter](../../../firmenstamm/einrichterparameter/vieraugenprinzip_zahlungsverkehr_epa_zahlungen_vieraugenprin.md), in denen man Skripte angeben kann, die nach erfolgter Übertragung (Funktion „Zahlung übertragen“ F9) ausgeführt werden.

Auf der sich öffnenden Maske stehen folgende Funktionen zur Verfügung:

| **Funktion** | **Beschreibung** |
| --- | --- |
| Verzeichnis ändern | Der Speicherort der DTA-Datei kann geändert werden. Bei Betätigung wird dieses Feld freigegeben. Mit F3 kann man den Dateiauswahldialog aufrufen.  
Die Änderung wird zwischengespeichert und jedes Mal wieder vorgeschlagen.  
    
    
**Achtung**:  
1) *Die Dateiausgabe ist für Daten, die per Datenbankprozedur erstellt werden, relativ zum Datenbankserver*.  
2) *Für die Verfahren ohne private Datenbankprozedur arbeiten (SEPA,DTA,DTINT) wird weiterhin die unter* “Prozedur zur Anpassung des Dateinamens“ *hinterlegte Prozedur aufgerufen(*[Siehe DTA](./dta.md)*).*  
 |
| Zahlung übertragen | Diese Funktion steht nur für noch nicht übertragene Dateien zur Verfügung. Die DTA-Datei wird auf das ausgewählte Verzeichnis neu geschrieben und je nach Einstellung in den Einrichterparametern im Explorer angezeigt oder per VBA / VBS Script übertragen.  
 |
| Übertragung zurücksetzten | Diese Funktion steht dann zur Verfügung, wenn die Daten bereits als übertragen gekennzeichnet wurden. Es wird der Eintrag aus der Tabelle Vieraugenprinzip_zahlungen gelöscht und die Übertragung kann noch einmal gestartet werden. Aus sicherheitstechnischen Gründen sollte diese Funktion nur für eine bestimmte Bedienerklasse zugänglich sein.  
 |
| Rücksetzung beantragen | Hier wird eine Mail an die Bediener der in den Einrichterparametern hinterlegten Bedienerklasse gesendet. Für den Mail-Versand wird die Prozedur aus dem [Einrichterparameter](../../../firmenstamm/einrichterparameter/vieraugenprinzip_zahlungsverkehr_epa_zahlungen_vieraugenprin.md) „Prozedur zum Beantragen des Rücksetzens der Zahlungsnummer“ verwendet. Vorbelegt ist hier die Standard Prozedur „ZahlungRueckBeantragen“. Diese kann durch eine private Prozedur ersetzt werden.  
   
Als Eingangs Parameter werden die Zahlungsnummer, Bedienerklasse und der SMTP Server übergeben.  
Als Ergebnis liefert die Prozedur den Mailstatus zurück.  
• 0 für Mail-Versand hat funktioniert.  
• 1 für Mail-Versand hat nicht funktioniert  
   
 |
