# SQL Anywhere-Dienstprogramm für Dienste (dbsvc)

<!-- source: https://amic.de/hilfe/sqlanywheredienstprogrammfrdie.htm -->

Syntax:

dbsvc [Optionen] -d &lt;Dienst> Dienst löschen

 dbsvc [Optionen] -g &lt;Dienst> Details abrufen

 dbsvc [Optionen] -l Alle SQL Anywhere-Dienste auflisten

 dbsvc [Optionen] -u &lt;Dienst> Dienst starten

 dbsvc [Optionen] -x &lt;Dienst> Dienst stoppen

 dbsvc [Erst.-Optionen] -w &lt;Dienst> &lt;Details> Dienst erstellen

 @&lt;data> erweitert &lt;data> aus Umgebungsvariable &lt;data> oder Datei &lt;data>

**Optionen (Groß- und Kleinschreibung wie angezeigt verwenden):**

 -cm Diensterstellungsbefehl anzeigen (mit -g oder -l)

 -o &lt;Datei> Ausgabenachrichten in Datei protokollieren

 -q Meldungen nicht anzeigen

 -y Dienst ohne Bestätigung löschen oder überschreiben

**Erstellungsoptionen (Groß- und Kleinschreibung wie angezeigt verwenden):**

\-a &lt;Konto> Zu benutzender Kontoname

\-as Konto "LocalSystem" verwenden

\-i Interaktion mit dem Desktop zulassen

\-p &lt;Kennwort> Kennwort für Konto (mit -a)

\-rg &lt;Abh>,... Dienstgruppenabhängigkeiten

\-rs &lt;Abh>,... Dienstabhängigkeiten

\-s &lt;Start> Startoption (Standard = Manual)

 Automatic, Manual, Disabled

\-sd &lt;Beschr> Anzuzeigende Beschreibung

\-sn &lt;Name> Anzuzeigender Name

\-t &lt;Typ> Diensttyp (Standard = Personal)

 Network, Personal, DBRemote, MobiLink, DBMLSync,

 dbns, dblsn, dbvss, rshost, rsoe, mlagent

Details: &lt;voller Programmpfad> [Optionen]

***Entweder -a oder -as erforderlich, wenn -w benutzt wird***
