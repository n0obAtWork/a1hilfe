# Einrichtung des Publishers und der Remote-User mit Sybase Central

<!-- source: https://amic.de/hilfe/einrichtungdespublishersundder.htm -->

Die Einrichtung des Publishers und der Remote-User lässt sich auch in Sybase Central unkompliziert erledigen.

1. Starten Sie Sybase Central

2. Verbinden Sie sich mit der gewünschten Datenbank

3. Klicken Sie in der Ordneransicht mit der RECHTEN Maustaste auf Benutzer à Neu

4. Im Assistenten zum Erstellen von Benutzern geben Sie zunächst den Namen des anzulegenden Benutzers ein. Der erste Name ist BST1. Bei weiteren anzulegenden Benutzern wird die Zahl jeweils um Eins erhöht (BST2). Klicken Sie anschließend auf „Weiter“

5. Vergeben Sie das Passwort für den Benutzer

6. Lassen Sie die Login-Richtlinie auf „root“ und klicken Sie anschließend auf „Weiter“

7. Vergeben Sie die benötigten Rechte des Benutzers und klicken Sie anschließend auf „Weiter“. Mindestens:

a. DBA

b. Ressource

c. Entfernter DBA

d. Alle anderen schaden jedenfalls nicht…

8. Geben Sie einen Kommentar ein (optional) und klicken Sie anschließend auf „Fertig stellen“

9. Suchen Sie nun den Benutzer den Sie gerade angelegt haben in der Ordneransicht aus der Liste der Benutzer heraus und klicken Sie diesen mit der RECHTEN Maustaste an und klicken anschließend auf Neu à Mitgliedschaft

10. Markieren Sie die Gruppe ADMIN und bestätigen Sie mit OK

11. Klicken Sie den Benutzer erneut mit der RECHTEN Maustaste an und wählen Sie anschließend „Auf Publikationseigentümer ändern“ oder für SQL Remote Benutzer „auf entfernten Benutzer ändern…“ aus

12. Klicken Sie in der Ordneransicht nun auf SQL Remote Benutzer und wechseln Sie anschließend auf die Registerkarte „Nachrichtentypen“

13. Klicken Sie dort mit der RECHTEN Maustaste auf den Eintrag „File“ und wählen dort Eigenschaften

14. Geben Sie hier die Adresse des Publikationseigentümers ein

15. Bestätigen Sie die Änderung mit OK

Zum Anlegen der SQL Remote Benutzer können Sie auch direkt durch Anklicken des Menüpunktes SQL Remote Benutzer in der Ordneransicht mit der RECHTEN Maustaste, dann auf Neu àSQL Remote Benutzer, den Assistenten zum Erstellen von SQL Remote Benutzern öffnen. Hier haben Sie den Vorteil die für die Replikation nötigen Benutzerinfos direkt anzugeben.

<p class="just-emphasize">Publisher anlegen via ISQL</p>

Werte in den Klammern ersetzen!

```sql
CREATE USER
"<BST(Zähler)>" IDENTIFIED BY '***';
GRANT DBA, RESOURCE, REMOTE DBA TO
"<BST(Zähler)>";
grant
membership in group "admin" to "<BST(Zähler)>";
GRANT PUBLISH TO "<BST(Zähler)>";
ALTER REMOTE MESSAGE TYPE
file ADDRESS '<Pfad zum Nachrichtenverzeichnis>';
```

<p class="just-emphasize">SQL Remote Benutzer anlegen via ISQL</p>

Werte in den Klammern ersetzen!

```sql
CREATE USER
"<BST(Zähler)>" IDENTIFIED BY '***';
GRANT DBA, RESOURCE, REMOTE DBA TO
"<BST(Zähler)>";
grant
membership in group "admin" to "<BST(Zähler)>";
GRANT REMOTE TO
"<BST(Zähler)>" TYPE "FILE" ADDRESS '<Pfad zum
Nachrichtenverzeichnis>' SEND EVERY '01:00';
```

Jede entfernte Datenbank muss die konsolidierte Datenbank angeben, von der sie Nachrichten empfängt. Um eine konsolidierte Datenbank auf der entfernten Datenbank anzugeben, erteilen Sie dem Publikationseigentümer der konsolidierten Datenbank die CONSOLIDATE-Berechtigung. Eine entfernte Datenbank kann nur von einer einzigen konsolidierten Datenbank Nachrichten erhalten. Die CONSOLIDATE-Berechtigung identifiziert die Datenbank, die Nachrichten an diese entfernte Datenbank sendet.

*Beispiel:*

*BST1 ist in der ZENTRALE der Publikationseigentümer. Somit MUSS der USER BST1 in der Filiale die CONSOLIDATE-Berechtigung erhalten.*

Werte in den Klammern ersetzen!

```sql
CREATE USER
"<BST(Zähler)>" IDENTIFIED BY '***';
GRANT DBA, RESOURCE, REMOTE DBA TO
"<BST(Zähler)>";
grant
membership in group "admin" to "<BST(Zähler)>";
GRANT CONSOLIDATE TO
"<BST(Zähler)>" TYPE "FILE" ADDRESS '<Pfad zum
Nachrichtenverzeichnis>' SEND EVERY '01:00';
```
