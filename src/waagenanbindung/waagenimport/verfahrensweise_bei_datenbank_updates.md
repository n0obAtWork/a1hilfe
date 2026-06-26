# Verfahrensweise bei Datenbank-Updates

<!-- source: https://amic.de/hilfe/verfahrensweisebeidatenbankupd.htm -->

Bei einem Datenbank-Update auf einer Kundendatenbank müssen alle mit ScriptSystem=0 und außerdem alle mit ScriptBesitzer=1 gekennzeichneten Datensätze erhalten bleiben.

Alle Datensätze beider Relationen mit ScriptSystem=1 werden (unter Direktsprung AMICR eingerichtet) gelöscht und neu eingespielt.

In Zukunft soll ein Foreign Key die Relationen verbinden: Wegen des Foreign Keys müssen dann zuerst die Datensätze der Relation ScriptParamPar gelöscht, bzw. zuerst die Datensätze der Relation ScripParam eingespielt werden.

Die Datensätze deren ScriptSystem-Flag nicht gesetzt ist, werden **nicht** automatisch bei allen Kunden eingerichtet.

Mögliche Problemquellen:

Derzeit kann nicht verhindert werden, dass Detailsätze mit ScriptSystem=1 eingespielt werden, deren Kopfsatz mit ScriptSystem=0 nicht auf der Kundendatenbank existiert.

**Wichtiger Hinweis**

Bis Aeins-Version 4.2.2.181 (22.6.99) werden inaktive ScriptParameter nicht zuverlässig erkannt. Daher kann es erforderlich sein, nach Beendigung der Parametrisierungen das SQL-Skript WAAGKORR.SQL aufzurufen. Besser ist es, vorläufig keine Parameter als inaktiv zu markieren. Das SQL-Skript hat folgenden Inhalt:

delete from ScriptParamPar WHERE ScriptPId='WaagenImport' and ScriptPPAktiv=0;

commit;
