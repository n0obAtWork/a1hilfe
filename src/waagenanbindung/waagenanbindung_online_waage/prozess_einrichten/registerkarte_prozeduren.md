# Registerkarte Prozeduren

<!-- source: https://amic.de/hilfe/_prozess_prozeduren.htm -->

Lagerumbuchung / Rohwarenlieferschein

In diesem Feld kann eine private Prozedur hinterlegt werden, die steuert, ob eine Lagerumbuchung anstelle eines Rohwarenlieferscheins erzeugt werden soll. Diese Funktion wird benötigt, wenn die Ware eigentlich auf Lager 99 angeliefert werden sollte, diese aber stattdessen auf Lager 7 angeliefert worden ist.

Als Übergabe Parameter wird die OWaage_Id erwartet.

| Eingabe Parameter | Feldtyp |
| --- | --- |
| in_owaage_id | integer |

Die Prozedur muss folgende Parameter zurückliefern.

| Parameter | Feldtyp |
| --- | --- |
| Klasse | integer |
| Unterklasse | integer |
| Ursprungslager | Integer |
| BuchungsTyp | integer |
| Planlieferdatum | date |
| BewertungsPreis | numeric(15,4) |

<p class="just-emphasize">Klasse</p>

Wird die Klasse mit einer 0 zurückgegeben so wird ein Lieferschein erstellt, wird die Klasse mit 5110 zurückgegeben, so wird die Lagerumbuchung erstellt.

<p class="just-emphasize">Unterklasse</p>

Hier wird die Unterklasse der Lagerumbuchung zurückgegeben.

<p class="just-emphasize">Ursprungslager</p>

Das Ursprungslager ist das Lager, auf dem eigentlich die Ware angeliefert werden sollte. Das Ziel Lager ist das Lager aus dem Waagen Satz.

<p class="just-emphasize">Buchungstyp</p>

Der Buchungstyp für die Lagerumbuchung.

• 0 für Angebot

• 1 für Auftrag

• 2 für Rechnung

<p class="just-emphasize">Planlieferdatum</p>

Hier kann ein spezielles Planungslieferdatum angegeben werden bleibt dieser Parameter leer so wird das Tagesdatum genommen.

<p class="just-emphasize">Bewertungspreis</p>

Wird ein Bewertungspreis > 0 zurückgegeben so wird dieser Bewertungspreis genommen. Ansonsten startet die normale Preisfindung des Beleges.

<p class="just-emphasize">Beispielprozedur</p>

```sql
CREATE PROCEDURE p_liefer_lagerumbuch
(

in in_owaage_id  integer

)

Result (

Klasse
integer

,Unterklasse     integer

,Ursprungslager  integer

,BuchungsTyp     integer

,Planlieferdatum date

,BewertungsPreis numeric(15,4)

)
//
BEGIN
// Kommentar
// -------------------------
//

if in_owaage_id >
0then
    select 5110  as
Klasse

,0
as Unterklasse

,666     as Ursprungslager

,1       as BuchungsTyp

,today() as Planlieferdatum

,3.5     as BewertungsPreis

from dummy;

else
    select 0   as Klasse

,0       as Unterklasse

,0       as Ursprungslager

,1       as BuchungsTyp

,today() as Planlieferdatum

,3.5     as BewertungsPreis

 from dummy;

end if;
end
```

Private Prüfprozedur

Hier kann eine private Prozedur hinterlegt werden, die das Abschließen einer Wiegung aktiv verhindert. Die Prozedur liefert zwei Werte zurück einmal das Feld Fehler und den Fehlertext.

Der Fehler kann zwei Zustände annehmen.

• Zustand 0 ist kein Fehler Wiegung kann abgeschlossen werden es erfolgt keine Meldung.

• Zustand 1 ist ein Fehler und die Fehlermeldung wird angezeigt.

Sollte in der Prozedur ein technischer Fehler auftreten (wie z.B. ein Feld wurde nicht deklariert), so muss auch ein Fehler samt Fehlertext zurückgegeben werden, damit der Benutzer eine Meldung erhält, warum die Wiegung nicht abgeschlossen werden konnte. Dies bedeutet in der Exception muss es eine Fehler-Behandlung geben.

Als Übergabe Parameter wird die OWaage_Id erwartet.

| Eingabe Parameter | Feldtyp |
| --- | --- |
| in_owaage_id | integer |

Die Prozedur muss folgende Parameter zurückliefern.

| Parameter | Feldtyp |
| --- | --- |
| Fehler | integer |
| Fehlertext | char(255) |

Beispiel für eine Private Prozedur:

```sql
REATE PROCEDURE
p_owaage_private ( in in_owaage_id integer )
Result
(

fehler integer

,fehlertext char(255)

)
BEGIN
  declare
dc_Fehler
integer;
  declare
dc_FehlerText       char(255);
  declare
dc_kunde
integer;
  declare ECX_ERR_NOTFOUND
EXCEPTION FOR SQLSTATE '02000';
  declare
dc_ErrorMsg         LONG VARCHAR;
  declare
dc_SQLCODE          integer;
  declare
dc_SQLSTATE         CHAR(10);
--
--
  if in_owaage_id = 13321 then
    set dc_Fehler = 1;
    set dc_FehlerText = 'Es ist ein
Fehler ist aufgetreten';
  else
    set dc_Fehler = 0;
  end if;
  set dc_kunde = 1;
  select dc_Fehler as Fehler, dc_FehlerText as
fehlertext from dummy;
EXCEPTION
  when others then
    Select  ERRORMSG()

,SQLCODE

,SQLSTATE
    into
dc_ErrorMsg

,dc_SQLCODE

,dc_SQLSTATE;
--
    call AMIC_FEHLERPROT( 20

,amic_func_sprachtexte ('a', 'b', 'Prozedur', -1)

,amic_func_sprachtexte ('a','b','Beim Ausführen der Prozedur %s ist ein Fehler
aufgetreten.', -1, 'p_owaage_private')

|| '\n'

|| amic_func_sprachtexte('a','b','Parameter (%s):: %s', -1, 'in_owaage_id',
in_owaage_id )

|| '\n\n'

|| 'SQLCODE:: ' || dc_SQLCODE

|| ' [' || dc_SQLSTATE || ']'

|| '\n'

|| dc_ErrorMsg

,null

);
    select 2 as fehler, dc_ErrorMsg ||'
' || dc_SQLCODE || ' ' || dc_SQLSTATE as fehlertext from dummy;
END
```

Private Prüfprozedur Siloverwaltung

In diesem Feld kann eine Private Prozedur hinterlegt werden, die in der Validierungsfunktion des Ladeträgers / Silo wirkt. Liefert die Prozedur ein 0 (FALSE) zurück, so wird das Feld nicht validiert, bei dem Rückgabewert 1 (True) wird das Feld validiert.

Übergabeparameter an die Prozedur

Die Parameter müssen genauso heißen wie die Parameter in der nachfolgenden Tabelle.

| Parameter | Bedeutung |
| --- | --- |
| In_ArtikelId | Artikel der aktuellen Wiegung |
| In_OwaageId | OwaageId der aktuellen Wiegung |
| In_Ladeträger | Aktueller Ladeträger / Silo |
| In_PartieId | Die Partie-Id der aktuellen Wiegung. Ist keine Partie vorhanden, wird eine 0 übermittelt |

Rückgabewerte der Prozedur

| Parameter | Bedeutung |
| --- | --- |
| retval | Gibt an, ob das Feld validiert werden darf<br>1. 0 False<br>2. 1 True |
| retvaltext | Hier kann noch ein Fehlertext angegeben werden, der im False-Fall mit ausgegeben werden soll. |

Beispiel Einer privaten Prozedur, diese gibt immer ein TRUE zurück.

```sql
CREATE procedure
p_silo_prozedure (  in in_ArtikelId integer

,in in_OwaageID integer

,in in_Ladetraegernr integer

,in in_PartieID integer

)

result (   retval integer

,retvaltext char(255)

)
--
BEGIN
--Sollten stehen gelassen werden, Variablen werden im
WHEN OTHERS verwendet
  declare ECX_ERR_NOTFOUND EXCEPTION FOR SQLSTATE
'02000';
  declare dc_ErrorMsg LONG VARCHAR;
  declare dc_SQLCODE integer;
  declare dc_SQLSTATE CHAR(10);
  declare dc_retval integer;
  declare dc_retvaltext char(255);
  set dc_retval  = 1;
  if in_Ladetraegernr = 501 then
    set dc_RetvalText = 'Fehler
Juhu';
    set dc_retval  = 0;
  end if;
  select dc_retval  as retval, dc_RetvalText
as retvaltext
EXCEPTION
```

 when others then

```sql
Select
ERRORMSG(), SQLCODE, SQLSTATE into dc_ErrorMsg, dc_SQLCODE, dc_SQLSTATE;
    call AMIC_FEHLERPROT( 20,
amic_func_sprachtexte('a', 'b', 'Prozedur', -1) ,
amic_func_sprachtexte('a','b','Beim Ausführen der Prozedur %s ist ein Fehler
aufgetreten.', -1, 'p_silo_prozedure')
    || '\n\n'|| 'SQLCODE:: ' ||
dc_SQLCODE|| ' [' || dc_SQLSTATE || ']'|| '\n'|| dc_ErrorMsg, null);
END
```

Mengenbestimmungsprozedur für die Rohwarenbelege

In diesem Feld ist es möglich, eine Funktion zu hinterlegen, welche eine alternative Menge zurückgibt. Die Prozedur wird nach dem Eintragen der Wiegedaten in die Relationen Rohwarehauptsatz_Waage und RohwareZusatzQualität_Waage aufgerufen. Gibt diese Funktion eine Menge größer als 0 zurück, wird die Menge in der Relation Rohwarehauptsatz_Waage überschrieben. In das Feld WaagenOriMenge wird das Originalgewicht der Waage eingetragen. Sollte die Menge in Abhängigkeit von schon erfassten Qualitätswerten stehen, so müssen diese Qualitäten in der Relation Rohwarehauptsatz_Waage auf 0 gesetzt werden, da diese ansonsten bei der Rohwarenbelegerzeugung wieder abgerechnet werden.

Folgende Übergabeparameter muss die Funktion haben

| Parameter | Bedeutung |
| --- | --- |
| In_OwaageId | OwaageId der aktuellen Wiegung |
| In_SatzId | SatzId der Daten in der Relation RohwareHauptSatz_Waage |

Als Rückgabewert wird ein Wert des Typs Numeric(15,4) erwartet.

Eigene Qualitätsanzeige

Hier kann eine Prozedur hinterlegt werden, welche die Qualitätstabellen mit Wert füllt. Die Standartprozedur ist **OWaageArtikelDaten.**

Folgende Übergabe Parameter muss die Funktion haben

| Parameter | Bedeutung |
| --- | --- |
| In_OwaageId | OwaageId der aktuellen Wiegung |

Rückgabewerte der Prozedur

| Parameter | Datentyp | Bedeutung |
| --- | --- | --- |
| Nummer | integer | |
| Bezeichnung | Char(40) | Bezeichnung der Qualität |
| UntererBasisWert | Numeric(15,4) | |
| ObererBasisWert | Numeric(15,4) | |
| Typ | intger | |
| Einheit | Char(10) | |
| UntererBasisWertDefault | Numeric(15,4) | |
| ObererBasisWertDefault | Numeric(15,4) | |
| QualitaetsWert | Numeric(15,4) | |
| RohSorQuWaagPos | integer | |
| Sortierung | Numeric(15,4) | |
| Bestandteilnutzung | integer | |

Eigene Speicherprozedur

In diesem Feld kann eine Prozedur hinterlegt werden, die nach dem Speichern des Waagensatzes aufgerufen wird. Nach dem Aufruf der Prozedur wird die Waagenmaske neu geladen.

Folgende Übergabe Parameter muss die Funktion haben

| Parameter | Bedeutung |
| --- | --- |
| In_OwaageId | OwaageId der aktuellen Wiegung |

Hinweisfeld

In diesem Feld kann eine Prozedur hinterlegt werden, welche das Hinweisfeld auf der Waagenmaske befüllt. Die Beispielprozedur ist **OwaageHinweis.**

Der Text kann mehrzeilig zurückgegeben werden.

Die Prozedur wird an folgende Stellen aufgerufen.

1. Nach der Eingabe des Kontraktes

2. Nach der Eingabe des Artikels

3. Nach der Eingabe des Kunden

4. Nach jeder Wiegung

5. Nach der Eingabe des Ladeträgers

6. Nach der Eingabe der Partie

7. Nach dem Öffnen der Maske im Bearbeiten Modus

Folgende Übergabe Parameter muss die Funktion haben

| Parameter | Bedeutung |
| --- | --- |
| In_OwaageId | OwaageId der aktuellen Wiegung |
| In_KtrId | Hier wird die Kontraktid des Waagensatzes übergeben |
| In_Kundeid | Kundid des Waagensatzes |
| in_ArtikelId | Artikelid des Waagensatzes |

Rückgabewerte der Prozedur

| Parameter | Datentyp | Bedeutung |
| --- | --- | --- |
| Hinweis | Char(1000) | Auszugebender Text |
| Farbe | Char(20) | Hintergrundfarbe |
| Schriftfarbe | Char(40) | Farbe der Schrift |
| Bitmap | integer | • 0 zeigt das Ampelbild nicht an<br>• 1 zeigt das Ampelbild an |

Private Prozedur Verkaufsbeschränkung

In diesem Feld kann eine private Prozedur hinterlegt werden, die in der Validierungsfunktion des Artikels wirkt. Ermittelt wird der Status und die Meldung für die Verkaufsbeschränkung.

Wenn keine Prozedur ausgewählt wurde, wird die Prozedur „Amic_WaageVerkaufsbeschraenkung“ angesprochen.
