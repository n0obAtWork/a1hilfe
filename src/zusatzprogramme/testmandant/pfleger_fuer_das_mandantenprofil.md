# Pfleger für das Mandantenprofil

<!-- source: https://amic.de/hilfe/_mandantprofilpfleger.htm -->

Administration > Firmenkonstanten > Mandantenprofil **[MPR]**

Im Mandantenprofilpfleger lassen sich Einstellungen pflegen, die z.B. in Makros verwendet werden und im Test- und Livesystem unterschiedlich sind. So könnten zum Beispiel Ablageorte für Reporte, Meldungen, Ergebnisse oder Exporte für Live- und Testsystem unterschiedlich sein. Damit im Makro nach der Erstellung eines Testmandanten keine Änderung gemacht werden muss, muss dieses nur die Daten in der im Bereich [Testmandantenparameter](./parameter_fuer_den_testmandanten_im_mandantprofil.md) beschriebenen Weise gelesen werden.

| Feld | Beschreibung |
| --- | --- |
| Profil | Hier wählen Sie, ob der Wert im Livesystem oder im Testsystem gültig sein soll. Die Einstellung AMIC-Testmandant ist dem Support vorbehalten. Die Einstellung wird verwendet, um ein lokales Testsystem zu betreiben, das aus einem Testsystem kopiert wurde. |
| Name | Name des Parameters |
| Wert | Wert, der dem gewählten Parameter in der angegebenen Umgebung zugewiesen werden soll. |
