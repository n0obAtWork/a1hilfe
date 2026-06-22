# Szenario

<!-- source: https://amic.de/hilfe/_szenario.htm -->

**Definition der Packelemente**

Hier müssen die möglichen Abpackungselemente erfasst werden.

**Name Buchungskennzeichen**

Falls Ja, Artikelstammnummer für die Leergutbuchung

**Zusammenstellung der Abpackungen**

Hier werden je Kombination Artikel / Kunde folgende Werte hinterlegt: Abpackungselemente 1 bis 3 (siehe 1.) Multiplikatoren dazu EAN Ware EAN Umverpackung Etikett drucken (J/N) Preis Kennzeichen Preis hat Priorität, Vorbelegung mit Ja

**Ablauf Lieferschein**

In der Artikelerfassung wird nach Abpackvorschriften in der gegebenen Kombination Kunde/Artikel gesucht. Falls Eintrag vorhanden, wird der Gebindedialog mit den Faktoren geöffnet; hier wird die Anzahl der obersten Ebene (Paletten) abgefragt. Bei Verlassen des Positionsteils wird je nach Einstellung die kumulierte Menge der Verpackungsartikel (siehe 1.) dem Beleg zugefügt.

**Druck NVE-Etiketten**

Bei Verlassen des Beleges wird der Druck der NVE - Etiketten gemäß den Einstellungen unter 2. ausgelöst. Hierfür ist unter OPT eine Vorschrift Drucker / Etikettname zu hinterlegen.

**Korrektur des Beleges**

Es wird hinterlegt, für welche Palette bereits ein Etikett gedruckt wurde. Wird die Menge in der Korrektur verkleinert, werden diejenigen nicht verwendeten NVE zum Löschen markiert. Wird die Menge in der Korrektur vergrößert, werden die zugefügten NVE-Etiketten nachgedruckt.
