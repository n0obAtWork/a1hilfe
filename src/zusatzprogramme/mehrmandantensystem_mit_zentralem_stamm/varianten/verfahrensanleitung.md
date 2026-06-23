# Verfahrensanleitung

<!-- source: https://amic.de/hilfe/_mms_verfahrensanleitung.htm -->

<p class="just-emphasize">Synchronisierung des Datenbestandes</p>

<p class="just-emphasize">eines Mehrmandantensystems im Bereich Saatgut</p>

<p class="just-emphasize">Voraussetzung:</p>

Ein bestehendes lauffähiges Mehrmandantensystem.

Informationen zur Einrichtung eines Mehrmandantensystems finden Sie gegebenenfalls in unserer A.eins Online-Hilfe unter dem Punkt „[Mehrmandantensystem mit zentralem Stamm](http://www.amic.de/hilfe/#!mehrmandantensystemmitzentralemstamm.htm)“.

Wir weisen Sie hier nochmal darauf hin, dass nur im Hauptmandant eine Stammdatenpflege durchgeführt werden darf! Sämtliche Datenbestände der Untermandanten werden vom Hauptmandanten überschrieben und sind nicht wiederherstellbar! Um hier Probleme zu vermeiden sollten Sie in den Stammdatenpflegern der Untermandaten, die entsprechenden Funktionalitäten („Löschen“, „Ändern“ und „Neu“) deaktivieren.

<p class="just-emphasize">Vorgehensweise:</p>

1. Erstellung von Sicherungen („Backup“) der bestehenden Datenbanken sowohl vom Hauptmandant als auch von den Untermandanten.

2. Installieren des A.eins Updates und somit Umstellung der Teilnehmer im Mehrmandantensystem (also Hauptmandant und Untermandanten) auf das neue Datenmodell.

3. Prüfen der bestehenden Konfiguration des Mehrmandantensystems auf den beteiligten Systemen (Trigger usw.) nach dem Einspielen des Updates.

4. **In den Untermandanten** sämtliche Datenbestände aus den Tabellen entfernen:

- SaatFruchtArt
- SaatFruchtSorte
- SaatFruchtSorteAddon
- SorteMaskeDaten
- SaatFrSortPosit
- SaatFrSortPositAddon

Dies erfolgt vorzugsweise durch den Aufruf der SQL-Prozeduren

```text
mms_untermandant_loesche_saatgut.sql
```

oder „per Hand“ durch den SQL-Befehl

```sql
DELETE FROM
SAATFRUCHTART
```

In den der Tabelle SaatFruchtArt untergeordneten Tabellen (siehe Auflistung oben) werden, durch die in diesem A.eins Update zum Datenmodell hinzugefügten Fremdschlüsselbeziehungen („Foreign Keys“), automatisch alle Datenbestände aus den oben aufgelisteten Tabellen gelöscht.

5. **Im Hauptmandant** ausführen der SQL-Prozedur

```text
mms_matching_saatgut.sql
```

Hierdurch wird das Verteilen via Mehrmandantensystems, des aktuellen Datenbestandes des Hauptmandanten in den unter Punkt 4. aufgelisteten Tabellen, vom Hauptmandant auf die ihm bekannten Untermandanten angestoßen. Die Datenbestände werden also nun synchronisiert.

Die Zeitdauer dieses Vorganges hängt von Ihrem Datenbestand in den beteiligten Tabellen, sowie der Leistungsfähigkeit ihres Netzwerkes und der Computersysteme ab.

6. Überprüfung, ob die Datenbestände vollständig synchronisiert wurden.

Beispielsweise durch das Vergleichen, der Anzahl der Einträge in allen beteiligten Tabellen auf dem Hauptmandant und Untermandanten via SQL-Befehl. Hier am Beispiel der Tabelle SaatFruchtSorte beschrieben.

```sql
SELECT
COUNT(*) FROM SaatFruchtSorte
```
