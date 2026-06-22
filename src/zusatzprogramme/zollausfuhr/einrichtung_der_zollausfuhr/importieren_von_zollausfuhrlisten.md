# Importieren von Zollausfuhrlisten

<!-- source: https://amic.de/hilfe/_importzollausfuhr.htm -->

Mit diesem Einspieler werden die Zollausfuhrlisten in das A.eins System eingespielt. Dazu werden die Zollausfuhr unter dem Link „Atlas Ausfuhr Codeliste“ von der [Atlas](https://www.ausfuhrplus.internetzollanmeldung.de/iaap/logon.do) Webseite heruntergeladen. Die Dateien müssen auf dem Computer gespeichert werden, um diese in das A.eins System ein zuspielen. Nach dem die Dateien heruntergeladen worden sind, können diese per Funktion „Einspielen“ in das System importiert werden. Sind alle Dateien erfolgreich eingespielt, so bekomme diese die Endung „_imported“. Hat das Einspielen der Dateien nicht funktioniert so, bekommen diese die Endung „_error“.

| Maskenfeld | Bedeutung |
| --- | --- |
| Import Verzeichnis | In diesem Feld wird das Verzeichnis hinterlegt, in dem sich die Zollausfuhrlisten befinden. Der Pfad muss relative zum Datenbank Server liegen. |
| Import Funktion | In diesem Feld wird die Datenbankprozedur hinterlegt, welche die Zollausfuhrlisten in die Datenbank einspielt. Diese Prozedur kann privatisiert werden. |
| Statustext | Zeigt den Status nach dem Export an. Tritt die Meldung  
„Fehler beim Einspielen der Daten“ auf so steht im Fehlerprotokoll nähere Informationen zu der Meldung |

In der Tabelle werden alle eingespielten Zollausfuhrlisten angezeigt.

| Maskenfeld | Bedeutung |
| --- | --- |
| ID | Ident der Zollausfuhrliste |
| Name | Name der Zollausfuhrliste |
| Gültig ab | Ab welchem Zeitpunkt ist die Datei gültig |
| System | EX |
| Version | Versionsnummer der Datei. |

Folgende Dateien können zurzeit eingespielt werden:

1. A0027

2. A0108

3. A0122

4. A0127

5. A1150

6. A1270

7. A1840

8. C0017

9. C0018

10. Coo31

11. C0092

12. C0093

13. I0100
