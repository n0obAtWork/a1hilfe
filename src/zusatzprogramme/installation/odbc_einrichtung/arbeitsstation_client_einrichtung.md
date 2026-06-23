# Arbeitsstation/Client - Einrichtung

<!-- source: https://amic.de/hilfe/arbeitsstationclienteinrichtun.htm -->

Das Setup-Programm stellt die technische Verfügbarkeit auf den Clienten her.

Beim Start von A.eins wird die ODBC-Einrichtung des Mandanten in HKEY_CURRENT_USER\\Software\\ODBC\\ODBC.INI eingepflegt.

Damit wird der Einsatz von

1) Aeins-Crystal-Report,

2) Crystal-Report-Direktanwendungen

vorbereitet und unterstützt.

Der Name des Schlüssels für den ODBC-Eintrag ergibt sich durch den Wert des A.eins-Parameters „crw_login_info“. Ist keine explizite Angabe dieses Parameters erfolgt, dann ist es in aller Regel der Name des Mandanten aus den zugehörigen Inis (Stichwort: [Mandantenname]).

Folgende Setzungen erfolgen:

| Registry-Schlüssel | Vorbelegung erfolgt durch Aeins-Parameter | Standard-Wert |
| --- | --- | --- |
| Uid | odbc_admin_name | (\*\*) |
| Pwd | odbc_admin_pass | (\*\*) |
| Autostop | odbc_autostop | No |
| CommLinks | odbc_commlinks | Wert des A.eins-Parameters „Links“ (\*) |
| Driver | odbc_driver | Wert des A.eins-Parameters „sybase_odbc“ |
| DatabaseName | Dbn | (\*) |
| DatabaseFile | Dbf | (\*) |
| EngineName | Eng | (\*) |
| Description(\*\*\*) | Programm_name | |

<strong>(\*)</strong> Diese „Standard-Werte“ werden intern – falls nicht extra vorgegeben – aus dem „Database_Connect“-Parameter ermittelt.

**(\*\*)**

a) Das Einsatzgebiet 1) pflegt die Daten zur Laufzeit und ist nicht funktional abhängig von diesen Einträgen.

b) Das Einsatzgebiet 2) wird standardmäßig durch die Pflege der Registry-Schlüssel mit Standardwerten unterstützt. Ist das aus sicherheitstechnischen Gründen unerwünscht, kann das durch individuelle Steuerung der A.eins-Parameter odbc_admin_name/odbc_admin_pass erreicht werden.

Durch geeignete odbc_admin_pass - Vorgabe lässt sich administrativ erreichen, dass die von A.eins erzeugte ODBC-Einrichtung nicht unmittelbar verwendet werden kann! 

Der A.eins-Parameter „odbchelper\=FALSE“ schaltet die gesamte Pflege der ODBC-Einrichtung des Mandanten grundsätzlich ab und sollte nur in Spezialfällen von Nöten sein.

**(\*\*\*)**

Die Pflege der ODBC-Einrichtung erfolgt nur, wenn sich die „Description“ im ODBC-Eintrag von der durch A.eins ermittelten „Description“ unterscheidet. Dieses Verfahren bewirkt, dass im Standardfall die ODBC-Einrichtungen höchstens einmal nach einem Programm-Update (andere Versionsnummer von A.eins ) erfolgen.

Fängt die „Description“ mit „Setup“ an, findet keine Pflege der ODBC-Einrichtung statt!

Über [SYSIN], Version Crystal Report ist ein Dialog startbar der Auskunft über bestimmte ODBC-Einstellungen gibt.

Folgende Service-Funktionen sind dort verfügbar:

| Funktion | |
| --- | --- |
| ODBC testen | Es wird eine Verbindung via ODBC aufgebaut.<br>Mögliche Ausnahmen werden in einer Notepad-Übersicht dargestellt. |
| ODBC erstellen | Erstellt gemäß den obigen Spezifikationen den ODBC-Client-Anschluss. Dabei werden die unter (\*\*\*) angegebenen Restriktionen nicht beachtet! |
| ODBC Administrator | Öffnet den ODBC-Datenquellen-Administrator |
