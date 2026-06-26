# Vorgehensweise

<!-- source: https://amic.de/hilfe/vorgehensweise.htm -->

Vorgehen, wenn man bei verschiedenen Kassen auf unterschiedliche Kasseneinstellungen zurückgreifen will:

Vorgehen, wenn man bei verschiedenen Kassen auf unterschiedliche Kasseneinstellungen zurückgreifen will:

1. Es ist für Systemadministratoren möglich, in den Kasseneinstellungen über F8 Neuer Vorlagesatz einen neuen Satz von Kasseneinstellungen zu erzeugen, der eine fortlaufende Vorlagenummer bekommt. Diesen kann man dann z.B. bei entsprechender Profilierung bearbeiten. Dabei werden die Einstellungen der gewählten Vorlage übernommen. In dieser Anwendung besteht auch die Möglichkeit über F7 Lösche Vorlagesatz einen kompletten Satz von Kasseneinstellungen zu löschen, allerdings nur dann, wenn auf diesen Satz in der Kassenverwaltung nicht verwiesen wird. Die Auswahl, welcher Satz gelöscht werden soll, erfolgt über eine vorgeschaltete Item-Box.

2. Dieser neu erzeugte hat eine fortlaufende Nummer und man kann diesen Satz in den Kasseneinstellungen bearbeiten, indem man in den Profilen die Vorlage auf die Nummer der neuen Vorlage einstellt.

3. In der Kassenverwaltung sucht man sich jetzt die Kasse aus, die nach den neuen Einstellungen arbeiten soll. Dort gibt es ein zusätzliches Feld namens Vorlage. In dieses trägt man die Vorlagenummer gemäß Kasseneinstellungen ein. Auf diesem Feld ist die F3-Auswahl implementiert über alle bisher erzeugten Sätze von Kasseneinstellungen, von denen genau eine zuzuordnen ist. Standardmäßig ist 0 eingestellt.

Die oben beschriebene Funktionalität kann man für unterschiedliche Zwecke nutzen

(die aufgeführten Beispiele erheben keinen Anspruch auf Vollständigkeit):

a) Dadurch, dass pro Kasse unterschiedliche Standardkunden hinterlegt sind ist es möglich, über die Vorgangsdruckklassen unterschiedliche Drucker anzusteuern (OptiGruppe Kunden).

b) Man kann je nach Kasse/Region unterschiedliche Displaytexte hinterlegen (OptiGruppe Displaytext).

c) Es ist möglich, verschiedenen Kassen eine unterschiedliche Kassenwährung zu hinterlegen (interessant, wenn ein Betrieb in unterschiedlichen Ländern operiert), dort sind dann auch die Währungseinheiten beim Kassensturz je nach Land anpassbar (OptiGruppe Kasse, Geld und Waehrung).

d) Man kann Kassen unterteilen in Kassen, an denen ein Lesegerät angeschlossen ist bzw. nicht (OptiGruppe Allgemein).

#### ACHTUNG!!!

Die Vorlagenummer sollte nur bei abgeschlossenen Kassen geändert werden.

Bem.: Alle diejenigen, die diese Sonderfunktionalität nicht nutzen wollen, können problemlos weiterarbeiten, ohne etwas nachzupflegen. Das gilt auch für die Kassen, die auf der Defaultvorlage 0 der Kasseneinstellungen zurückgreifen möchten.

PS: Dieses Feature kann auch bei der Umstellung auf EURO benutzt werden, indem man vorab einen Satz von Kasseneinstellungen erzeugt, der nach der EURO-Umstellung gelten soll (z.B. Kassenwährung ist jetzt EURO, bei der Zählung muss die Stückelung angepasst werden).

```sql
Update AcashStmdKsse set VorlageNummer=???
```
