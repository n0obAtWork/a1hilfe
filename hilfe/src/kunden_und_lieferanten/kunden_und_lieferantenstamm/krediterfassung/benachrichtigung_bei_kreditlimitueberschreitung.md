# Benachrichtigung bei Kreditlimitüberschreitung

<!-- source: https://amic.de/hilfe/benachrichtigungbeikreditlimit.htm -->

Es gibt die Möglichkeit sich benachrichtigen zu lassen, sobald mit dem Speichern eines Vorganges (z.B. Auftrag) das Kreditlimit eines Kunden überschritten wird.

Dazu sind folgende Einstellungen nötig:

1. Das Kreditlimit beim jeweiligen Kunden muss gepflegt sein. (siehe auch [Kreditvergabe](./eingabe_kundenkredit.md))

2. Der [Steuerparameter 233](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_spa_233.md) sollte dazu so eingestellt sein, dass er ein Speichern des Beleges nicht verhindert. Mehr dazu finden Sie auch weiter oben unter [Überwachung des Kreditlimits](./ueberwachung_des_kreditlimits.md).

3. Das Feld der Vorgangsunterklasse ‚ [Fehlerprotokolleintrag bei Kreditlimitüberschreitung](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md#Allgemein) ‘ muss auf Ja stehen.

4. Im Mandantenstamm **[MND]** muss dafür im Register Allgemein Abschnitt Fehlerprotokoll-Meldewesen die Empfänger-Prozedur entsprechend hinterlegt werden.  
Es gibt eine Prozedur, die von AMIC angelegt wurde, die einfach in eine private Prozedur kopiert und angepasst werden kann: FehlerprotokollAbweichendeEmpfaenger (zu finden unter [SQLP]). Dazu muss die Abfrage auf ‚%Kreditlimit%‘ geändert und die E-Mail-Adresse entsprechend angepasst werden.  
[http://www.amic.de/ihilfe/index.html?turl=XMLDocuments%2FiAeins%2Fhtml%2FM_SQL_FehlerprotokollMeldewesen_FehlerprotokollAbweichendeEmpfaenger.htm](http://www.amic.de/ihilfe/index.html?turl=XMLDocuments%2FiAeins%2Fhtml%2FM_SQL_FehlerprotokollMeldewesen_FehlerprotokollAbweichendeEmpfaenger.htm)
