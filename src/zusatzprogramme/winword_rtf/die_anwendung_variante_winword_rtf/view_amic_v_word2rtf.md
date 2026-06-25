# View Amic_V_Word2Rtf

<!-- source: https://amic.de/hilfe/viewamicvword2rtf.htm -->

Die View dient als Grundlage der Datengewinnung und wird bei weiteren Tabellen entsprechend über "Union" erweitert.

```sql
CREATE VIEW amic_v_word2rtf
AS
SELECT
  'Anschriftnotizen'  AS tabelle,
'winword'           AS
winwordspalte,
'textblob'          AS
rtfspalte,
'adressid'          AS
pkspalte,
adressid            AS
pkwert,
len(winword)        AS
winworddatalen,
len(textblob)       AS rtfdatalen,
  'cast(winword as long binary)' AS
feld_schnipsel,
  'where adressid=' || adressid  AS
where_schnipsel,
ifnull(textblob,0,1)
AS konvert_status
FROM
anschriftnotizen;
```
