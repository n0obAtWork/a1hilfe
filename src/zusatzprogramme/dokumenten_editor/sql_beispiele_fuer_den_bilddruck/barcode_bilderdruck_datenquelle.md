# Barcode/Bilderdruck-Datenquelle

<!-- source: https://amic.de/hilfe/_barcodedruckquelle.htm -->

Die "Datenquelle" ist eine Begrifflichkeit im Zusammenhang mit dem A.eins-Druck (Vorgangsdruck, Kontrakt-Druck etc.) und stellt dort die Möglichkeit auf bestimmte Entitäten zuzugreifen.

Im Rahmen einer privaten Barcode- bzw. Bilderdruck-Prozedure kann die Datenquelle übergeben und ausgewertet werden.

Anmerkung: Die Entitätsaufzählung ist vollzählig, es sollte deswegen nicht erwartet werden, dass auch jede Entität speziell in diesem Beritt sinnvoll bzw. notwendig ist.

Hinweis: Gross-Kleinschrift ist nicht relevant.

| Entität | Vorgangsdruck | Kontraktdruck |
| --- | --- | --- |
| DQBelegQuelle | z.B. 1 | z.B. 12 |
| DQBelegTyp | Vorgangsklasse. Z.B. 700 | 1 |
| DQBelegId | v_id | Die interne, für den Anwender unsichtbare, Identifikation des Kontraktes für sämtliche externen Verweise (**KtrId**). |
| DQBelegNummer | v_numnummer | Die vom Anwender vergebene, eventuell aus einem Nummernkreis gekommende, logische Identifikation des Kontraktes bzw. deren eindeutiger numerischer Teil (**KtrNummer**). |
| DQKundenNummer | | |
| DQSammeldruck | Kennzeichen | \- |
| DQPDFA | \- | \- |
| DQBelegreferenz | | Archiv-Referenz |
| DQFA_Belegreferenz | Archiv-Referenz | |
| DQAttachment | | |
| DQV_id | | \- |
| DQFormularid | | |
| DQFormularidZ | | |
| DQV_Klassnummer | | \- |
| DQV_UKlassnummer | | \- |
| DQV_Jahrnummer | | \- |
| DQV_Unternummer | | \- |
| DQVerpostung | | |
| DQVerposter | | |
| DQBelegDatum | | |
| DQReason | | |
| DQLocation | | |
| DQContactInfo | | |
| DQFA_Druckdatum | | |
| DQFA_NeuanlageBedienerId | | |
| DQBelegTypText | z.B. „Rechnung windows“ | |
| DQBelegTypKlasse | z.B. „700“ | |

Folgender Beispiel-Aufruf stellt die „Datenquelle“ einer darauf vorbereiteten privaten Prozedure zur Verfügung:

```text
p_barcode_datenquelle_beispiel('{datenquelle}')
```

<details>
<summary>Beispiel für “Datenquelle”</summary>

```sql
CREATE PROCEDURE p_barcode_datenquelle_beispiel( in in_datenquelle long varchar )
Result
(
  code long varchar,
  codetype long varchar
)
Begin
  declare dc_code long varchar;
  declare dc_codetype long varchar;
  CALL sp_parse_json( 'dc_datenquelle', in_datenquelle );
  set dc_codetype = 'qrcode';
  set dc_code = dc_datenquelle.DQBelegId;
  select dc_code as code, dc_codetype as codetype;
End
```

</details>
