# Stapelverarbeitung einrichten

<!-- source: https://amic.de/hilfe/stapelverarbeitungeinrichten.htm -->

Hauptmenü > Administration > Firmenkonstanten > Bediener

oder Direktsprung **[BD]**

Im Bedienerstamm kann pro Bediener hinterlegt werden, ob und wie mit der Stapelfunktion gearbeitet werden darf. Die Einstellung wird im Feld „Ausw. Stapel“ eingetragen. Es stehen drei Möglichkeiten zu Verfügung, die mit F3 ausgewählt werden können:

1. Globaler Stapel:

   Der Stapel wird gespeichert und bleibt so lange erhalten, bis er manuell gelöscht wird oder der Stapel älter ist als der in den Einrichterparametern hinterlegte Zeitraum. Es stehen alle im Folgenden beschriebenen Funktionen zur Verfügung.  
    

2. Temporärer Stapel:

   Der Stapel bleibt nur so lange bestehen, solange man in die Anwendung nicht verlässt. Es stehen nur die Funktionen „Zu Stapel hinzufügen“, „aus Stapel entfernen“ und „Umschalten Stapelverarbeitung“ zur Verfügung. Temporäre Stapel sind immer privat und man kann auch nur die eigenen Stapel bearbeiten und nicht auf globale Stapel zugreifen.  
    

3. Keine Stapelfunktionalität:

   Es kann kein Stapel gebildet werden und der entsprechende Bediener sieht das Register „Stapelverarbeitung“ nicht.

Es kann vorkommen, dass für bestimmte Anwendungen bzw. Varianten keine Stapelverarbeitung angeboten wird. Dies kann neben der Lizenz und Benutzereinstellung noch folgende Ursachen haben:

- Die Anwendung/Variante enthält keine IDENT-Felder
- In der Anwendung /Variante ist das Markieren von Zeilen nicht erlaubt
- In der Anwendung /Variante wurde mit der Option „NOSTAPEL“ die Stapelverarbeitung deaktiviert.

```text
OPTIONS NOSTAPEL
```
