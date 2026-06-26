# Vorbelegung Kassennummer

<!-- source: https://amic.de/hilfe/vorbelegungkassennummer.htm -->

Die Nummer der Kasse an der der Bediener sitzt, wird in der Regel vor der ersten Erfassung abgefragt. Um diese Abfrage zu umgehen, können zwei Wege beschritten werden:

1. In der AHOI.INI im Windows-Verzeichnis kann in der Abteilung [ACASH2] der Wert Kassensystem mit der Nummer des vorzubelegenden Kassensystems belegt werden. Achtung! Es handelt sich hier um die Nummer des Kassensystems, also der Hardware, nicht der Kasse (obwohl diese i.d.R. identisch sein könnte).

   Diese Vorgehensweise empfiehlt sich selbstredend nicht für eine A.eins-Installation auf dem Terminalserver.

```text
[ACASH2]
Kassensystem=34
```

2. In der Registry kann im Pfad [HKEY_CURRENT_USER\\Software\\AMIC\\A.eins\\ACASH2] der Wert Kassensystem eingetragen werden.

   Diese Vorgehensweise kann bei der A.eins-Installation auf einem Terminalserver verwendet werden.

```text
Windows Registry Editor Version 5.00
[HKEY_CURRENT_USER\Software\AMIC\A.eins\ACASH2]
"Kassensystem"="3"
```
