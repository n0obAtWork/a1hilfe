# Usage

<!-- source: https://amic.de/hilfe/_email_connector_usage.htm -->

Der Import wird über den Aufruf der Anwendung EmailConnector.exe aus dem bin-Verzeichnis gestartet. Übergabeparameter sind ein Connection-String und die Id des Profils.

```text
Bsp.:
A.eins.EmailConnector.exe
connectionstring="eng=test;dbn=test;links=tcpip;uid=test;pwd=test"
id=2
```

Der Aufruf kann über die Windows-Aufgabenplanung in regelmäßigen Abständen erfolgen.

Formulararchivgruppe

Die Mail und alle Anhänge werden im Formulararchiv in einer Gruppe zusammengefasst. Anhand dieser kann man die Dokumente zusammengehörenden Dokumente identifizieren.

Der Name der Gruppe ist ein vorangestelltes „EmailConnector“ und eine GUID. Sie könnte wie folgt aussehen:

```text
EmailConnector-{98cb6768-7fbd-477d-a4fa-1564fd46dc90}
```
