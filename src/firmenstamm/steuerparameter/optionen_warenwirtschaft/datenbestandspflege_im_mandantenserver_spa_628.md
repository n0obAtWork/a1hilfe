# Datenbestandspflege im Mandantenserver(SPA 628)

<!-- source: https://amic.de/hilfe/_SPA_628.htm -->

Bei „Ja“, „\*“ oder „?“ wird das Modul Datenbestandspflege (DBP) periodisch vom Mandantenserver aufgerufen. Das Modul prüft dabei selbstständig alle notwendigen Startbedingungen wie etwa Einzelplatzmodus oder zeitabhängige Laufeinschränkungen.

Parameter:

0 = „--“: Start nicht zulässig

1 = „ja“: Start zulässig mit Prüfbedingungen

2 = „\*“: Start auch bei noch anderen angemeldeten A.eins-Benutzern möglich

3 = „?“: wie \* und zusätzlich Start auch bei noch vorhandenen unbearbeiteten Mandantenserver Aufträgen möglich
