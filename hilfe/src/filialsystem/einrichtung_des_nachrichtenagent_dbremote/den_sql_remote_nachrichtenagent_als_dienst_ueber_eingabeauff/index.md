# Den SQL Remote-Nachrichtenagent als Dienst über Eingabeaufforderung anlegen

<!-- source: https://amic.de/hilfe/densqlremotenachrichtenagental.htm -->

1. Öffnen Sie eine Windows-Eingabeaufforderung im Verzeichnis „..\\Aeins\\bin\\“

2. Geben Sie nun folgende Befehlszeilen ein:

```dos
dbsvc -as -s auto -t network -w <Anzeigename> "<Pfad zur dbsrv12.exe>" @<Pfad zur KonfigDateiDBsrv12>
```

```dos
dbsvc -as -s auto -t DBRemote -rs <AnzeigenameAbhängigkeit> -w <Anzeigename> "<Pfad zur dbremote.exe>" @<Pfad zur KonfigDateiDBRemote>
```

Werte in den spitzen Klammern bitte entsprechend ersetzen!

Legen Sie die Dienste in der angegebenen Reihenfolge an. In der Zweiten Befehlskette wird mit der **Option –rs** auf den Anzeigenamen des ersten Dienstes verwiesen und bedeutet, dass dieser Dienst erst ausgeführt wird, wenn der abhängige Dienst gestartet ist und läuft.

**Die verwendeten Optionen (dbsvc):**

\-as Konto "LocalSystem" verwenden

\-s &lt;Start> Startoption Automatic, Manual, Disabled

\-t &lt;Typ> Diensttyp Network, Personal, DBRemote, MobiLink, DBMLSync,

 dbns, dblsn, dbvss, rshost, rsoe, mlagent

\-rs &lt;Abh>,... Dienstabhängigkeiten

\-w &lt;Dienst> &lt;Details> Dienst erstellen

**Die verwendeten Optionen (dbsrv):**

 @&lt;Pfad zur KonfigDateiDBsrv12> Pfad zur Konfigurationsdatei

 [Zur Übersicht](./datenbank_serveroptionen_dbsrv17.md)

**Die verwendeten Optionen (dbremote):**

 @&lt;Pfad zur KonfigDateiDBRemote> Pfad zur Konfigurationsdatei

[Zur Übersicht](../die_sql_remote_nachrichtenagent_konfigurationsdatei_anlegen/index.md)

<p class="siehe-auch">Siehe auch:</p>

- [SQL Anywhere-Dienstprogramm für Dienste (dbsvc)](./sql_anywhere_dienstprogramm_fuer_dienste_dbsvc.md)
- [Datenbank-Serveroptionen (dbsrv17)](./datenbank_serveroptionen_dbsrv17.md)
