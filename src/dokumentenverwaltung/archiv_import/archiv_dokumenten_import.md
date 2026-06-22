# Archiv-Dokumenten-Import

<!-- source: https://amic.de/hilfe/_archivbasis.htm -->

Hauptmenü > Administration > Archiv > Importverwaltung

Direktsprung **[FAI]**

**Hier werden die Dokumenten-Importe verwaltet. Ein Dokumenten-Import wird durch ein Import-Profil beschrieben.**

| Felder |
| --- |
| Name | Eindeutiger Name des Dokumenten-Import-Profils<br>(mögliche Farbgebungen siehe „Ziel-Datenbank-Name“) |
| Automatik | Bei Einstellung **Ja** übernimmt der Mandantenserver den Import |
| Ident | Ident des Imports<br>Zusammen mit der Bedienerklasse ist der technische Schlüssel gegeben. |
| Bedienerklasse | \-1 ist die sogenannte „Defaultklasse Kunden“, das bedeutet das keine spezielle Bedienerklasseneinschränkung vorhanden ist.<br>\-1 ist der Standard.<br>A.eins verwendet für interne Zwecke (z.B. Archiv-Ansichten) auch andere Bedienerklassen |
| Bedienerklassenbezeichnung | Bezeichnung der Bedienerklasse |
| Weitere Elemente … | |
| Import-Datenbank-Name | Der Datenbank-Datei-Name gegen den der Import prüfen soll, ob der „richtige“ Mandant vorliegt.<br>Durch Duplikate der Datenbank kann es leicht passieren, dass der Import Dateien aus Verzeichnissen importiert, die ausschließlich dem Original-Mandanten vorbehalten sind. Deshalb wird nun geprüft, ob der Datenbank-Datei-Name des Mandanten der den Import ausführt mit „Ziel-Datenbank-Namen“ übereinstimmt.<br>Ist eine Differenz festzustellen wird der „Name“ gelb eingefärbt, ist zusätzlich die Automatik aktiviert, dann wird der „Name“ rot eingefärbt. Diese Farbgebungen dienen lediglich der Information. Importe führt das System nicht aus. |
| Wartezeit in Minuten | Es existieren Scanner-Systeme die ihr Erzeugnis in mehreren Schritten erzeugen. Um diese „Reifezeit“ von A.eins zu unterstützen gibt es hier die Möglichkeit eine Wartezeit in Minuten anzugeben, bevor das A.eins-Archiv-Import-System die Datei verarbeitet. |
| Max. Anzahl pro Durchlauf | Da je nach Dateiaufkommen und -größe der allgemeine Mandantenserver-Betrieb in Stoßzeiten durch den Import behindert werden könnte gibt es hier die Möglichkeit die die Anzahl der zu importierenden Dateien zu konfigurieren. |

| Filter/Bereichsauswahl |
| --- |
| Name | Suche in Name |

| Funktionen |
| --- |
| Filter / Bereichsauswahl **F2** | Öffnet die Bereichsauswahl. |
| Neu **F8** | [Stammdatenpfleger](./archiv_import_stammdatenpfleger_formulararchiv_importe/index.md) „Neu“<br>Diese Funktionalität ist den SPA226 gebunden. |
| Ändern **F5, F6, F7** | [Stammdatenpfleger](./archiv_import_stammdatenpfleger_formulararchiv_importe/index.md) „Ändern, Ansehen, Löschen“ |
| Ausführen **F9** | Führt das selektiere Import-Profil aus |
| Funktion anlegen **F10** | Legt eine private Funktion in der Optionbox OB_AHFORMULARARCHIV an. Die Beschriftung der Funktion ist der Name des Import-Profils mit einem vorgestellten „Import“.<br>Nach einem Neustart des A.eins findet sich diese Funktion dann in der Anwendung „Formulararchiv“, Variante „Formulararchiv“.<br>Ist zum Beispiel die Ident „*23*“, dann ist es die Funktion „PF_FAI_*23*“ mit dem Controlstring „^jpl fa_exec do *23*“<br> <br>(Siehe auch [Funktion anlegen](./archiv_import_stammdatenpfleger_formulararchiv_importe/funktion_anlegen/index.md)) |
| Duplizieren **F11** | Vervielfältigt das ausgewählte Import-Profil.<br>Der Name des Duplikats hat ein vorgestelltes „Kopie von“.<br> <br>Alternativ steht in **Ändern** **F5** die Funktion „***Speichern unter***“ zur Verfügung. |
