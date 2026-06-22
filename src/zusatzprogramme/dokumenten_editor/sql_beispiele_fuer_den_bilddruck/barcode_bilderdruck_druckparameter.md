# Barcode/Bilderdruck-Druckparameter

<!-- source: https://amic.de/hilfe/_barcodedruckparam.htm -->

Die "DruckParameter" stellen eine Begrifflichkeit im Zusammenhang mit dem Aeins-Druck (Vorgangs- Kontrakt-Druck etc.) dar und ermöglichen auf ausgewählte Entitäten im Zusammenhang mit Drucker-Eigenschaften zuzugreifen.

Im Rahmen einer privaten Barcode- bzw. Bilderdruck-Procedure können diese Druckparameter übergeben und ausgewertet werden.

Anmerkung: Die Entitätsaufzählung ist vollzählig, es sollte deswegen aber nicht abgeleitet werden das auch jede Entität speziell in diesem Beritt sinnvoll bzw. notwendig ist. Als Beispiel sei die "FA_Id" genannt. Die kann zum Zeitpunkt des Druckes noch nicht bekannt sein, da die Archivierung erst danach stattfindet.

Hinweis: Gross-Kleinschrift ist nicht relevant.

| Entität | Beispiel |
| --- | --- |
| DruckerNummer | 6254 |
| DruSchacht | 0 |
| PreScript | |
| FormularId | 740 |
| AnzeigeDruckvorgang | false |
| KeineArchvierung | false |
| NurArchvieren | false |
| KeinDruckMerker | false |
| ZeigSQLKFehler | false |
| InterneImageErzeugung | false |
| FA_Id | 0 |
| FormularVerpostbar | false |
| Vorschau | false |
| DruckerQueue | Microsoft Print to PDF |
| DruckerKeinArchiv | false |
| DruckerSendenAn | false |
| DruckerNulldrucker | true |
| DruckerSendenAnProc | |
| IstLieblingsdrucker | true |
| LieblingsdruckerMakroScript | |
| Nacharchivierungs_Modus | false |
| LieblingsdruckerNurDrucken | 0 |

Folgender Beispiel-Aufruf stellt die „Druckparameter“ einer darauf vorbereiteten privaten Prozedure zur Verfügung:

```text
p_barcode_druckparam_beispiel('{druckparam}')
```

<details>
<summary>Beispiel für “Druckparam”</summary>

```sql
CREATE PROCEDURE
p_barcode_druckparam_beispiel( in in_druckparam long varchar )
Result
(
  code long varchar,
  codetype long varchar
)
Begin
  declare dc_code long varchar;
  declare dc_codetype long varchar;

  CALL sp_parse_json( 'dc_druckparam',
in_druckparam );

  set dc_codetype = 'qrcode';
  set dc_code = dc_druckparam.DruckerQueue;

  select dc_code as code, dc_codetype as
codetype;
End
```

</details>
