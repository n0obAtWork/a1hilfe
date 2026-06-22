# Zapfsäulen-Schlüsselung

<!-- source: https://amic.de/hilfe/zapfsulenschlsselung.htm -->

Zapfsäulen bzw. verschiedene Treibstoffe können wie Sorten verarbeitet werden, wobei eine Umschlüsselung von Sorte zu Artikel stattfinden muss. Übertragung der Script-Parameter zum Kunden.

Um die Parameter der Waagenschnittstelle auf eine Kundendatenbank zu übertragen, geht man wie folgt vor:

Start von Aeins mit der Referenzdatenbank, auf der sich das für den Kunden vorbereitete Waagen-Modul befindet.

Direktsprung [MAKRO], Auswahl des Makros Waagenschnittstelle_an_Kunde. Taste F5, Taste F9 (ausführen).

Das Script erzeugt eine Datei mit dem Namen WAAGE_AN_KUNDE.SQL.

Der Datenträger wird auf der Kundendatenbank wie folgt eingespielt:

Direktsprung [OSQL], Taste F3, Button Filebox, Auswählen der Datei mit dem Namen WAAGE_AN_KUNDE.SQL. Button Start. Fertig.

Das Import-Script der Waagen-Schnittstelle kann individuell parametrisiert werden. Eine Reihe von Parametern steht dazu zur Verfügung, die im folgenden erläutert werden. Die Pflege der Parameter erfolgt über den Direktsprung [SCPA] in der Anwendung Scriptparameter. Die Parametergruppe mit der **ScriptPId „WaagenImport“** ist für die Steuerung der Waagenschnittstelle zuständig. Im Detailbereich sind die einzelnen der unten beschriebenen Parameter aufrufbar. Anwender mit der entsprechenden Zugangsberechtigung können die Werte der Parameter individuell an die eigene Waagendatei anpassen.

Zur besseren Übersicht kann eine Liste der Parameter mit einer Crystal-Reports-Liste ausgedruckt werden. Diese Funktion steht innerhalb der Anwendung unter **Script-Parameter (DRUCK)** zur Verfügung.
