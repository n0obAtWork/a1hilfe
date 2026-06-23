# Marktscanner einrichten

<!-- source: https://amic.de/hilfe/_scanner_markteinrichten.htm -->

In dieser Anwendung kann die Standard Einrichtung für den Marktbereich für den jeweiligen ausgewählten Scanner eingerichtet werden. Des Weiteren ist es möglich aus dieser Anwendung heraus den [Windowsscanner](../../aeinswindowsscanner/index.md) zu starten. Dies kann durch zwei Aktionen ausgelöst werden.

1. Doppelklick auf die Zeile mit den Verbindungsparametern

2. Markieren einer Zeile und mit **F9** oder der Menüfunktion ***Scannerprogramm Starten*** das Programm aufrufen.

Wird diese Anwendung zum ersten Mal gestartet und im A.eins System befindet sich nicht die Datei dbconfig.xml welche die Verbindungsparameter enthält, damit sich der Windows Scanner mit dem A.eins System verbindet, werden die Felder mit den Verbindungsparameter der aktuellen Verbindung vorbelegt.

Menüfunktionen

| Menüfunktion | Bedeutung |
| --- | --- |
| Scannerprogramm Starten | Mit dieser Funktion wird der Windows Scanner gestartet. Es werden die Verbindungsparameter aus markierten Zeile genommen. |
| Marktscanner Standard Einspielen | Ist diese Menüfunktion aktiv, so kann für den ausgewählten Scanner den Marktstandard eingespielt werden. Der Standard kann nur für die aktuelle Datenbank eingespielt werden. Ist das Feld Permanente Inventur betretbar, so kann hier noch ausgewählt werden, ob beim Einspielen die Permanente Inventur berücksichtigt werden soll. |

Maskenfelder

| Feld | Bedeutung |
| --- | --- |
| Datei mit Verbindungsparameter | In diesem Feld kann per F3 Auswahl eine xml Datei ausgewählt werden, welche Verbindungsparameter für das Verbinden von der Scanner-Software zu einer A.eins System enthält. Wird per eine Datei ausgewählt, so werden die darin enthaltenen Verbindungsdaten in die Datentabelle geladen. |
| Standard mit Permanenten Inventur einspielen | Wird dieses Feld vorm Einspielen des Standards auf „Ja“ gestellt, so wird die Permanente Inventur für den Scanner mit eingespielt. Kann dieses Feld nicht betreten werden, dann wurde die Permanente Inventur schon eingerichtet. |

<p class="just-emphasize">Datentabelle mit den Verbindungsparametern</p>

| Feld | Bedeutung |
| --- | --- |
| Profilname | In diesem Feld wird der Profilname eingetragen. Jeder Profilname darf nur einmal in einer xml Datei vorkommen |
| Bediener | A.eins Bediener |
| Passwort | Passwort des A.eins Bedieners |
| DB-Server | Datenbankserver |
| Datenbank | Name der Datenbank |
| Commlinks | Hier muss **tcpip** eingetragen werden |
| Extras | Verbindungsspezifische Parameter wie z.B. idle=60;lto=30;<br>Werden immer mit einem „;“ abgetrennt |
| Logische IP-Adresse | In diesem Feld kann eine X Beliebige Bezeichnung oder IP-Adresse eingetragen werden. Diese wird bei der Kommunikation mit der Datenbank benutzt um die erfassten Daten zu einem Scanner zuzuordnen zu können und zum Auslesen der Steuerparameter die für diesen Scanner gelten soll, da diese pro Scanner zum eingerichtet werden. Wird in diesem Feld keine Adresse oder Bezeichnung eingetragen, so wird die Hardware IP-Adresse des Scanners benutzt. Wird der Scanner an verschiedenen Standorten benutzt, so lohnt es sich hier ein Wert einzutragen, da dann die Steuerparameter für diesen Scanner nur einmal eingerichtet werden müssen. |
| Connectionstring | Wird in diesem Feld ein Connectionstring eingetragen, so wird dieser benutzt um die Verbindung zum A.eins System herzustellen. |
