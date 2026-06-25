# FIREO

<!-- source: https://amic.de/hilfe/fireo.htm -->

Für die Finanzbuchhaltung kann der Test der Bewegungsdaten einfach per Event gestartet werden. Die Reorganisation wird bewusst nicht angeboten, da eventuell auftretende Fehler korrekt abgearbeitet werden sollen und zur Not der AMIC-Hotline gemeldet werden sollen. Um die Vorlage für dieses Event zu aktivieren, muss auf dem Register „Vorlagen“ Fireo auf **Ja** gestellt werden. Der Prozeduraufruf, der auf dem Register „Verarbeitungsroutine“ zu finden ist, muss noch ein wenig angepasst werden.

Für diese Automation kann die [Ausgabedatei](../../finanzbuchhaltung/fibu_reorganisator/automation.md) unter Optionen **[OPT]** festgelegt werden

**Funktionsweise des Fireo:**

Das A.eins Programmstart.vbs Skript kopiert die Aeins.exe und benennt diese dann in den Wert des Übergabeparameters um. Danach werden alle aktiven A.eins-Prozesse auf dem Datenbank Server gekillt. Erst danach startet das A.eins Programm mit der ausgewählten Reorganisation.

**Achtung der Parameter „Kill“ beendet alle A.eins-Prozesse die auf dem Datenbankserver laufen.** **Deswegen wird der Parameter nicht Vorbelegt. Um zu testen, ob der FIREO Aufruf funktioniert, kann dieser unter OSQL getestet werden. Aber beim Test muss der Kill Parameter leer sein.**

**Aufruf der Fireo Prozedur**

```sql
begin
  call fireo_automatisch(
               in_systempfad   = 'c:\\aeins\\bin\\aeins_programmstart.vbs',
               in_section      = 'entw',
               in_schema       = '',
               in_schemadatei  = '',
               in_user         = '<User>',
               in_passwort     = '<Password>',
               in_kill         = '',
               in_exe          = 'A1entw',
               in_dsn          = 'entw',
               in_dauer        = 0,
               in_fireo_aufruf = '')
end
```

| **Parameter Übersicht** | **Bedeutung** |
| --- | --- |
| in_systempfad | Pfad zum VBS Script A.eins Programmstart. Da das Event auf dem Datenbankserver läuft, muss dies das A.eins-Verzeichnis sein, auf das man vom Datenbankserver zugreifen kann. |
| in_section | Section der amicconf.ini |
| in_schema | Schema des Datenbank Servers |
| in_schemadatei | Schemadatei des Datenbank Servers |
| in_user | Anmeldungs Benutzer |
| in_passwort | Anmeldungs Passwort |
| in_kill | Beendet alle A.eins-Prozesse auf dem Datenbankserver |
| in_exe | Name der Exe unter der das Fireo laufen soll |
| in_dsn | DSN Eintrag |
| in_dauer | Parameter Timeout nach dem der Prozess beendet wird |
| in_fireo_aufruf | Welcher Test soll ausgeführt werden. Dabei sind folgende Schlüsselwörter möglich:<br>• Stammdaten<br>• Bewegungsdaten<br>• Jahreswechsel PK<br>• Jahreswechsel BK<br>• Zinssaldo<br>• Waehrung ( Achtung: Währung nicht mit Umlaut )<br>• Fragmente<br>Diese Schlüsselwörter können - durch Komma getrennt - beliebig kombiniert werden. So würde die Kombination in_fireo_aufruf =“Bewegungsdaten,Waehrung“ den Test Bewegungsdaten und den „Test Währung“ durchführen.<br> <br>Wird kein Wert angegeben, so werden nur der Test Stammdaten und der Test Bewegungsdaten ausgeführt. |
