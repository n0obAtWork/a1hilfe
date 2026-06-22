# Archiv Volltext-Recherche

<!-- source: https://amic.de/hilfe/archivvolltextrecherche.htm -->

| Archivmanager | Registerkarte | Volltext-Recherche |
| --- | --- | --- |
| Volltext-Lizenz (914) | Ja/Nein | Lizenzspa „Volltextrecherche“ |
| Archivtext-Einträge | Anzahl der Einträge der Relation „Archivtext“ | Informatorisch |
| Archivtext-Index | Name des Volltext-Index | Informatorisch, Standard: „ArchivTextIndex“<br> <br>Der Name ist auch in der JVar 2014, „JVAR_ARCHIV_VOLLTEXT_NAME“ verfügbar |
| Aktualisierungsfunktion | Name der Aktualisierungsfunktion | Standard: „volltextrecherche_progress“<br>***Hinweis:***<br>***Hier bei dieser Datenbank-Funktion geht es lediglich darum den nächsten Kandidaten für die Erzeugung des Archivtextes anzugeben!***<br>***Die eigentliche Volltext-Index-Aktualisierung erfolgt immer durch A.eins-Programmcode bzw. durch interne Sybase-Routinen.***<br> |
| Letzte Aktualisierung | Zeitstempel der letzten System-Volltext-Index-Aktualisierung | |
| Volltext-Index-Einträge | Anzahl der in den Volltext-Index einfließenden Dokumente | |

Die Aktualisierungsfunktion ermittelt die nächsten zu verarbeitenden Kandidaten für die Aufbereitung des Archivtextes. Die von A.eins ausgelieferte Funktion kann privatisiert werden um auf besondere Umstände reagieren zu können.

```sql
Auslieferung  :
volltextrecherche_progress
---<summary>Liefert eine vorgebbare
maximal-Anzahl zu verarbeitender
Formulararchiv-Einträge</summary>
---<returns>Key
Formulararchiv und einem Problem-Status, dieser ist ungleich 0 wenn ein Problem
aufgetreten ist.
---</returns>
---<param
name="in_anzahl"></param>
---<param
name="in_zeitschranke"></param>
CREATE PROCEDURE
volltextrecherche_progress( IN in_anzahl INTEGER DEFAULT 1 , IN in_zeitschranke
INTEGER DEFAULT 1 )
RESULT
(

fa_id    INTEGER,
  fa_mndnr
INTEGER,
  problem  INTEGER
)
BEGIN
  DECLARE DC_HOUR
INTEGER;

  SET DC_HOUR =
DATEPART( HOUR , NOW() );

  IF ( DC_HOUR
< 5 OR DC_HOUR > 19 ) OR ( in_zeitschranke != 1 ) THEN
    IF (
NOT EXISTS( select * FROM amic_status_relation( 'archiv' ) WHERE status IN ( 0 , 1 ) ) )
THEN
      SELECT -1 AS fa_ID , -1
AS fa_mndnr , 1 AS problem;

ELSE
      BEGIN
        DECLARE
LOCAL TEMPORARY TABLE TMP_DATEN

(

fa_id    INTEGER ,

fa_mndnr INTEGER ,

problem  INTEGER
        ) ON COMMIT
PRESERVE ROWS;

INSERT INTO TMP_DATEN( fa_id , fa_mndnr , problem )
        SELECT TOP
in_anzahl
        fa.fa_id ,
fa.fa_mndnr , 0 FROM formulararchiv fa
        JOIN
amic_mime am ON ( am.am_code = fa.fa_mime AND am.am_archivvolltext = 1 AND
ISNULL( fa.fa_progintern , 0 ) != -2 )
        LEFT OUTER
JOIN archivtext at ON ( at.archiv_guid = fa.fa_guid )
        LEFT OUTER
JOIN archivtextbad atb ON ( atb.archiv_guid = fa.fa_guid )
        WHERE
at.archiv_guid IS NULL

AND atb.archiv_guid IS NULL

AND fa.fa_guid IS NOT NULL;
        SELECT
fa_id , fa_mndnr , problem FROM TMP_DATEN
      END
    END
IF
  END
IF
EXCEPTION
   WHEN
OTHERS THEN
   BEGIN

     CALL amic_exception(
ERRORMSG() || '\n' || TRACEBACK() , SQLCODE ,
SQLSTATE , 'volltextrecherche_progress' , -30000
);

END;
   SELECT -1
AS fa_id , -1 AS fa_mndnr , 1 AS problem;
END
```
