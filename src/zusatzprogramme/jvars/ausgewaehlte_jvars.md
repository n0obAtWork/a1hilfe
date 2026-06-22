# Ausgewählte JVARS

<!-- source: https://amic.de/hilfe/_jvars_ausgew.htm -->

| JVars | Die JVARS des Owners 6000 geben zur Laufzeit Auskunft über den aktuellen Kontext im Programm. |
| --- | --- |
| JVAR_KOMPETENZ_HELPER_BAG | Die JVAR der „obersten“ Anwendung. |
| JVAR_KOMPETENZ_IST_ANWENDUNG | 1, wenn die aktuelle Position eine Anwendung ist, sonst 0. |
| JVAR_MASKE | Die JVAR der Maske |
| K_ANWENDUNG | Name der obersten Anwendung |
| K_ANZAHL_MARKIERT | die Anzahl der markierten Zeilen der obersten Anwendung. |
| K_FUNKTION | Der „letzte“ ausgeführte Controlstring über SHIFT-F4 |
| K_FUNKTION_AKTUELL | Der „letzte“ ausgeführte Controlstring |
| K_MASKE | Der Name der Maske |
| K_MASKE_DIALOG | 1 wenn die Maske ein Dialog ist, 0 sonst |
| K_MENU_LINKS | Wert ermittelt durch das ActiveX-Menü |
| K_MENU_RECHTS | Wert ermittelt durch das ActiveX-Menü |
| K_OPTIONBOX | Optboxid der „letzten“ Optionbox |
| K_VARIANTE | Variante der obersten Anwendung |
| K_VARIANTE_BESITZER | 0 AMIC, 1 sonst |

| **JVars** | Die JVARS des Owners 3561 geben zur Laufzeit Auskunft über den aktuellen Kontext im Programm. |
| --- | --- |
| JVAR_SYSTEM_STATUS_AMICAUTHORITAET | 1 = Der Aeins-Anwender ist als Aeins-Autorität identifiziert, 0 sonst. |
| JVAR_SYSTEM_STATUS_AMICENTWICKLER | 1 = Der Aeins-Anwender ist als AMIC-Entwickler identifiziert, 0 sonst. |
| JVAR_SYSTEM_STATUS_BEDIENERKLASSE | Die Bedienerklasse des Aeins-Anwenders. |
| JVAR_SYSTEM_STATUS_CLIENT | Der Name des Clients, also der Inhalt der Umgebungsvariablen CLIENTNAME |
| JVAR_SYSTEM_STATUS_DBSYSADMIN | 1 = Der Aeins-Anwender ist ein Datenbank-Administrator, 0 sonst.  
Veraltet, ist immer 1. |
| JVAR_SYSTEM_STATUS_DEBUGGERACTIVE | 1 = ein Debugger ist aktiv (Laufzeitentscheidung!), 0 sonst |
| JVAR_SYSTEM_STATUS_ENTWICKLERDATENBANK | 1 = Aeins ist mit der Entwickler-Datenbank verbunden, 0 sonst |
| JVAR_SYSTEM_STATUS_ENTWICKLERON | Parameter ENTWICKLER |
| JVAR_SYSTEM_STATUS_FDSMODE | Interne technische Verwendung, immer 1 |
| JVAR_SYSTEM_STATUS_HOST | Der Name des Hostes, entspricht der Rückgabe des Kommandozeilen-Befehles hostname |
| JVAR_SYSTEM_STATUS_PASSTHROUGH | 1 = Passthrough-Modus ist aktiviert, 0 sonst |
| JVAR_SYSTEM_STATUS_PID | Prozess-ID der Aeins-Instanz  
(Taskmanager, Details, Spalte PID) |
| JVAR_SYSTEM_STATUS_RELEASE | 1 = die Aeins-Anwendung läuft im RELEASE-Modus, 0 sonst |
| JVAR_SYSTEM_STATUS_TECHADMIN | 1 = der Aeins-Anwender ist technischer Administrator, 0 sonst |
| JVAR_SYSTEM_STATUS_TERMINALSERVER | 1 = die Aeins-Anwendung läuft nach Bekunden des Laufzeitsystems auf einem Terminalserver, 0 = sonst |
| JVAR_SYSTEM_STATUS_TESTDATENBANK | Die JVar ist 1 wenn,  
die Parameter „section“, „cmdline“ oder „database_connect“ das Wort “test” enthalten.  
Wenn die Option „Test_Indikator“ eingerichtet ist, hat diese Vorrang.  
Mit Hilfe des Parameters „TESTDBOFF“ kann die Erkennung umgangen werden. |
| JVAR_SYSTEM_STATUS_WINDOWSUSER | Windows-Login-User  
Entspricht der Windows-System-Umgebungsvariable USERNAME. |
| JVars | Die JVARS des Owners 1004 geben zur Laufzeit Auskunft über einige ausgewählte Programmpfade |
| JVAR_PATH_Root | Root-Verzeichnis, also das Verzeichnis welches in aller Regel das Bin-Verzeichnis, Masken-verzeichnis etc.pp. |
| JVAR_PATH_BIN | Bin-Verzeichnis |
| JVAR_PATH_MASKEN | Masken-Verzeichnis |
| JVAR_PATH_JPL | JPL-Verzeichnis |
| JVAR_PATH_A1SQL_PROC | Verzeichnis für Datenbank-Prozedur-Sourcen |
| JVAR_PATH_A1SQL_FUNC | Verzeichnis für Datenbank-Funktions-Sourcen |
| JVAR_PATH_IMPORT | allg. Verzeichnis für Import |
| JVAR_PATH_EXPORT | allg. Verzeichnis für Export |
| JVAR_PATH_USER | User-Verzeichnis |

JVars im Zusammenhang mit dem Archiv sind unter [JVars im Archiv](../../dokumentenverwaltung/technisches_zum_formulararchiv/archiv_ansehen_jvars.md#ueb_Archiv_5001) zu finden.
