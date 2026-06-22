# Beispiel zum Anlegen einer Arbeitsregel

<!-- source: https://amic.de/hilfe/beispielzumanlegeneinerarbeits.htm -->

In diesem Beispiel aus der Praxis soll folgende organisatorische Anforderung umgesetzt werden: Es soll sichergestellt sein, dass Fremdwährungsbelege stets zu tagesaktuellen Kursen gerechnet werden. Im Unternehmen ist ein Mitarbeiter für die Pflege der Kurse zuständig. Nicht akzeptabel ist dabei, dass Rechnungen erst danach erfasst werden dürfen.

Ich benutze hier die Regeln 10 und 11.

**Regel 10** ist die Startregel, die unter FRZ eingetragen wird. Sie besagt inhaltlich: der Fremdwährungskurs stimmt mit dem Kurs überein, der exakt für das Belegdatum hinterlegt ist. Belege ohne Fremdwährung gelten immer als regelkonform. In dieser Regel sind keine Sperren etc. eingebaut. Die Nachfolgeregel wird mit der privaten Prozedur p_ARegel_Kurs bestimmt.

Arbeitsregel: 10  
Name: Währungskontrolle  
Kurzbezeichnung:WKO  
Register Nachfolgeregel  
Typ: Datenbank Funktion  
SQL / Funktion: p_ARegel_Kurs

**Regel 11** ist der Fehlerzustand: Der Kurs entspricht nicht dem Tageskurs. Auch in Regel 11 wird die Prozedur p_ARegel_Kurs zur Bestimmung der Folgeregel aktiviert. Bei Belegkorrektur wird dann ebenfalls auf den korrekten Kurs geprüft und ggf. wieder Regel 10 zugeordnet. In Regel 11 schaltet man diverse Sperren ein, die das Weiterverarbeiten des Vorgangs verhindern.

Arbeitsregel: 11  
Name: Währungskurs inkorrekt  
Kurzbezeichnung: wk err  
Register Sperren  
Druck – immer sperren  
Fibu-Übertrag – immer sperren  
Umwandlung – immer sperren  
Register Nachfolgeregel  
Typ: Datenbank Funktion  
SQL / Funktion: p_ARegel_Kurs

Nachfolgende Funktion ermittelt die korrekte Folgeregel ( 10 oder 11 )

\-- private Datenbankfunktion zur Ermittlung einer Nachfolgeregel

\-- angelegt am : dd.mm.yyyy für Regel 10 und 11 : Währungskontrolle

\--

\-- Regel 10 = Währung ist ok

\-- Regel 11 = Währung ist nicht ok

CREATE FUNCTION p_ARegel_Kurs

(

 in in_V_ID integer , -- V_Id des Vorgangs

 in in_REGEL_NUMMER integer , -- Nummer der aktiven Regel

 in in_BEDIENERID integer -- die aktuelle BedienerID

)

RETURNS integer

BEGIN

 declare neue_regel integer;

 declare dc_v_kurs numeric(15,6);

 declare dc_w_kurs numeric(15,6);

 declare dc_ok integer;

 declare dc_test integer;

 set dc_ok=null;

 -- Bestimme ob ein Fremdwährungsbeleg den Kurs zum aktuellen Datum enthält

 select first if v_klassnummer &lt; 1000

 then wk.waehrMultiVerkau

 else wk.waehrMultiAnkauf

 endif as w_kurs,

 vw.v_waehrKurs as v_kurs,

 if v_kurs is null or(w_kurs is not null and v_kurs = w_kurs)

 then 1

 else 0

 endif as wok into dc_w_kurs,

 dc_v_kurs,

 dc_ok from Vorgangstamm as vs left outer join v_waehrung as vw on vw.v_id = vs.v_id

 left outer join waehrungskurs as wk on wk.waehrnummer = vw.waehrnummer and waehrabdatum = vs.v_datum

 where vs.v_id = in_V_ID;

 \-- dc_ok ist NULL, wenn es kein Fremdwährungsbeleg ist, dann ist alles richtig

 -- dc_OK = 0, Kurs weicht ab, muss also in Regel 11 geschoben werden

 if(dc_ok is null or dc_ok = 1) then

 set neue_regel = 10; -- guter Zustand

 else

 set neue_regel = 11; -- Währung ist nicht ok

 end if;

 return neue_regel

exception

 when others then

 -- call AMIC_FEHLERPROT(20,'prozedur p_ARegel_Kurs :',ERRORMSG(\*));

 return 0 // 0 = Regelnummer wird nicht geändert!!!!

END
