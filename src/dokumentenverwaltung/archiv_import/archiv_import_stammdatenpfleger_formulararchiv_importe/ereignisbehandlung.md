# Ereignisbehandlung

<!-- source: https://amic.de/hilfe/_ereignisbehandlung.htm -->

In den Fällen in denen die Komplexität den jetzigen Rahmen sprengt, lässt sich nun noch die Verarbeitung oder auch Vervollständigung mit Hilfe eines VBA-Skriptes nachbessern bzw. überhaupt durchführen.

Das System führt dabei auf Wunsch zu den 3 Zeitpunkten, nämlich bevor der Import startet, während jedes Datei-Importes und nachdem der Import durchgeführt worden ist, 3 angebbare VBA-Skripte durch. Damit man die Möglichkeit hat, einen Import-Vorgang in einem großen Skript zu verarbeiten, ist die Möglichkeit Parameter zu übergeben, integriert worden.

Im obigen Beispiel wird das vba-Script fa_import jeweils mit entsprechenden Parametern aufgerufen. Wie üblich und vereinbarungsgemäß sollten dabei „private“ Parameter mit „p_“ anfangen. Das garantiert keine Überschneidung mit vom System verwendeten Namen.

Fa_import_Test hat folgenden Inhalt:

```sql
option explicit

' Binärer Import ins Formulararchiv

dim owner
dim p_status
dim fam_ref_vorg
dim p_referenz
dim belegnummer
dim klassnummer
dim referenz

owner = Aeins.ScriptParam( "owner" )
p_status = Aeins.ScriptParam( "p_status" )

if p_status = 2 then
  ' das A.eins-System stellt die Aufbereitung der
Referenznummer per Datenbank-Funktion zur Verfügung
  ' Der A.eins-Anwender hat im Archiv-Mananger die
Möglichkeit eine eigene Datenbank-Funktion einzurichten,
  ' deshalb muss hier nun diese Datenbank-Funktion
zunächst ermittelt werden
  p_referenz = Aeins.ScriptParam( "p_ref" )
  Aeins.jpp_new "x1" , "JFA_Manager"
  Aeins.jpp_in "x1", "Merkmal" , p_referenz
  fam_ref_vorg = Aeins.jpp_do( "x1" , "GetMerkmal"
)
  Aeins.jpp_delete "x1"

  ' Aufbereitet wird nun die Belegreferenz mittels
dieser Datenbank-Funktion. Die Funktion erwartet als
  ' Parameter die Belegnummer und die
Klassennummer
  ' Die Belegnummer steht in der JVARS-Variable
"2", die Klassennummer in der JVARS-Variable "1" bereit

  belegnummer = Aeins.JVARS_GET(owner , "2")
  klassnummer = Aeins.JVARS_GET(owner , "1")

  Aeins.jpp_new "x2", "JDBX"
  Aeins.jpp_in "x2",  "sql"  , "select "
& fam

_ref_vorg  & "(" & klassnummer & ","
& belegnummer & ") as Referenz"
  Aeins.jpp_do "x2" , "exec"

  Aeins.jpp_in "x2", "col", "Referenz"
  Aeins.jpp_in "x2", "fld", "LDB_TRANSFER$VC"
  referenz = CStr(Aeins.jpp_do( "x2", "get" ))

  Aeins.jpp_delete "x2"

  ' 3 : Belegreferenz
  Aeins.JVARS_SET owner, "3" , referenz
end if
if p_status = 1 then
' Aeins.QuitApp( 0 )
end if
if p_status = 3 then
end if
```

Das Script demonstriert

- die Verwendung der Ereignisse.
- Wie man die Aufbereitung der Referenz-Nummer einfach nutzen kann

Bemerkenswert ist das Kommando Aeins.QuitApp( zahl ), wobei eine Zahl ungleich 0 dem Import-Skript signalisiert das der Import zu beenden ist bzw. nicht durchgeführt werden soll.

Die Übergabe der Kerndaten erfolgt durch JVARS. Die Kerndaten werden zunächst durch die Kriterien der Verschlagwortung versucht zu ermitteln. Das ergibt ein gewisses Resultat. Dieses wird an das VBA-Script übergeben, kann von diesem wie gewünscht aufbereitet werden und wird dann retour an den Import zur weiteren Verarbeitung bzw. Behandlung zurückgegeben.

**Damit steht also eine frei programmierbare Erweiterung des Imports zur Verfügung!**
