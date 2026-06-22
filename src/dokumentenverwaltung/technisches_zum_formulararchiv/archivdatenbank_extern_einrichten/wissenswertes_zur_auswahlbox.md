# Wissenswertes zur Auswahlbox

<!-- source: https://amic.de/hilfe/_wissenswerteszurausw.htm -->

Um die Problematik mit externen Formulararchiv und internen Joins in den Griff zu bekommen, bedient sich das Formulararchiv der Technik, die Daten in einer temporären Tabelle zusammenstellen und auf diesen die internen Joins ablaufen zu lassen.

Die Auswahlliste ist um diese Fähigkeit erweitert worden, um die stringgemäße Übergabe zu ermöglichen. Dazu expandiert das System an den Stellen wo „:§“ entsprechende Zeichenketten SQL-Syntax-gerecht auf.

```sql
// Auswahllistenfunktion :
Formulararchiv
TITLE FormularArchiv
INFO FormularArchiv
MASK AW_MASK
FIELD ID,
     FA_ID,
I4  , 8,HIDDEN
FIELD
KndNr.,
FA_Kundennummer,  I4,   8
FIELD
Beleg-Typ,
FA_BelegtypText,  char, 22
FIELD
Beleg-Nr,
FA_BelegNummer,   char, 10
FIELD Beleg-Datum,
FA_BelegDatum,    char, 10
FIELD
Archiv/Druck-Datum,
FA_DruckDatum,    char, 17
FIELD Beleg-Referenz,
FA_BelegReferenz, char, 10
FIELD Herkunft,
FA_Herkunft,      FS FAHerkunft
FIELD Anleger,
FA_NeuAnlageBediener,char,8
FIELD Inhalt,
FA_Mime,          char, 20
FIELD Beleg-Klasse,
FA_Belegklasse,   FS FAKlasse
FIELD
Mnd,
FA_Mandant,       char, 8
FIELD
MndNr,         FA_MndNr, I4,
8,HIDDEN
FIELD Autor      ,
fa_info_autor       , char, 16
FIELD Betreff    ,
fa_info_betreff     , char, 16
FIELD Titel      ,
fa_info_titel       , char, 16
FIELD Kategorie  , fa_info_kategorie   ,
char, 16
FIELD Kommentar  , fa_info_kommentar   ,
char, 16
FIELD Stichwörter, fa_info_stichwoerter, char, 16
SQL
 select :FIELDS from amic_fa('
 :§AUSW_1
 :§AUSW_2
 :§AUSW_3
 :§AUSW_4
 :§AUSW_5
 :§AUSW_6
 :§AUSW_7
 :§AUSW_8
 :§AUSW_9
 :§AUSW_10
 :§AUSW_11
 :§AUSW_12
 :§AUSW_13
 :§AUSW_14
 :§AUSW_15
 :§AUSW_16
 ')
  where ( 1= 1 )
 :ZUSATZ
 :ZUSATZ1
 :ZUSATZ2
 :ZUSATZ3
 and
(amic_fa_bedkl(db_bedienerklasse,fa_bedienerklasse) is not null)
 order by FA_Druckdatum desc
RETURN FA_Id,FA_Mandant,FA_MNDNR
IDENT FA_Id,FA_Mandant,FA_MNDNR
IDSQL select *
      from FormularArchiv
      where FA_ID=:ID1
      and
FA_Mandant=':ID2'
      and
FA_MNDNR=:ID3
```
