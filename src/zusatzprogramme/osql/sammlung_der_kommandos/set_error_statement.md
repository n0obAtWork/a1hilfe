# SET ERROR Statement

<!-- source: https://amic.de/hilfe/seterrorstatement.htm -->

Syntax

SET ERROR | CONTINUE |

 | NOCONTINUE|

 | DISPLAY |

 | NODISPLAY |

Purpose

Beeinflussung des Verhaltens bei Fehlern.

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[CONTINUE](./continue_statement.md), [SET OUTERR](./set_outerr_statement.md)

Beschreibung

Mit SET ERROR kann das allgemeine Verhalten bei SQL – Fehlern eingestellt werden.

SET ERROR CONTINUE ==> Datei hält nicht bei Fehlern an.

SET ERROR NOCONTINUE ==> Beendung der Kommandodatei bei  
 Fehlern. Dies ist die Standarteinstellung

SET ERROR DISPLAY ==> Fehlerbildschirm wird angezeigt. Dies ist   
 die Standardeinstellung.

SET ERROR NODISPLAY ==> Keine Anzeige von Fehlermeldungen.

Beispiel

SET ERROR CONTINUE;

SET ERROR NODISPLAY

Select \* from DIESERELATIONGIBTSNICH;

MSG Hier kam keine Fehlermeldung;

SET ERROR DISPLAY

Select \* from DIESERELATIONGIBTSNICH;

SET ERROR NOCONTINUE;
