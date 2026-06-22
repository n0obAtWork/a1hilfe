# Einrichtungen transportieren

<!-- source: https://amic.de/hilfe/einrichtungentransportieren.htm -->

Formulare inklusive ihrer Einrichtungen können zwischen Systemen hin- und hertransportiert werden. Dazu ist der Name einer Datei anzugeben, in der beliebig viele Formulareinrichtungen abgelegt werden können.

**Achtung:** bei mehreren Exporten hintereinander werden die jeweils neuesten Exporte hinten angehängt. Vor dem ersten Export kann ein Löschen dieser Datei nötig sein. (Datei editieren = Löschen)

Das Entladen **(Export Formular)** erzeugt ein SQL-Script.

Das Beladen **(Import aus Datei)** führt dieses Script aus.

Stellen Sie dazu sicher, dass die zu importierenden Formulare auf der Empfängerseite nicht existieren. Sonst wird die Ausführung einiger Statements mit den entsprechenden Fehlermeldungen verweigert.
