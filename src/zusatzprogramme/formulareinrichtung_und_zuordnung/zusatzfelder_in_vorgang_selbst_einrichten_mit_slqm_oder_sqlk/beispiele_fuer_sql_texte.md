# Beispiele für SQL-Texte

<!-- source: https://amic.de/hilfe/beispielefrsqltexte.htm -->

// ermittelt den Saldo des Kunden

// auf Basis gebuchter Saldo

```sql
select
sum(kontosumgebhaben)-sum(kontosumgebsoll) tuwas
from    kontosummen
where   kontonummer =
        (select
kontonummer from kundenstamm where kundid =:KUNDID)

// lädt die Bemerkungszeile 1
//
SELECT BemerkPosition.BemerkText
FROM KundenStamm
INNER JOIN (BemerkStamm INNER JOIN BemerkPosition
 ON BemerkStamm.BemerkId =
BemerkPosition.BemerkId)
 ON KundenStamm.KundBemerk =
BemerkStamm.BemerkId
WHERE BemerkZeile=1
AND   Kundid=:KUNDID

// Privater SQL Text
SQLK_Text_Gu_Re     ---
SELECT (V_WertNetto + V_WertSteuer) AS
Bruttobetrag,
       (IF Bruttobetrag
< 0 THEN 'RECHNUNG'
         ELSE
'GUTSCHRIFT' ENDIF) AS Belegtext
FROM   Vorgangstamm
WHERE V_Id=:V_Id

// Privater SQL Text
sqlk_Saldo_erfasst     ---
select sum(kontosumerfsoll) - sum(kontosumerfhaben) as
saldo_erfasst,
from    kontosummen
where   kontonummer =
        (select
kontonummer from kundenstamm where kundid = :KUNDID)

// effektiver Artikel Bestand
//
SELECT
( ArtiBestMenge + ArtiBestKorr ) as Bestand
FROM
Artikel
LEFT OUTER JOIN ArtiBestand
ON
Artikel.ArtikelID = ArtiBestand.ArtikelID
WHERE
Artikel.ArtikelID=:ARTIKELID
```
