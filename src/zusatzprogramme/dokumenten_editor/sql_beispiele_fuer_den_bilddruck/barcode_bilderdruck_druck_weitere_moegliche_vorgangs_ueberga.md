# Barcode/Bilderdruck-Druck: Weitere mögliche Vorgangs-Übergabe-Parameter

<!-- source: https://amic.de/hilfe/_barcodedruckweitere.htm -->

Folgende Parameter können in der Form {...} als Parameter an die dafür angepasste private Prozedure übergeben werden:

| Parameter | Syntax | Typ | |
| --- | --- | --- | --- |
| v_id | {v_id} | Zahl | |
| wabewid | {wabewid} | Zahl | |
| v_posizaehler | {v_posizaehler} | Zahl | |
| DRUCKPARAM | {DRUCKPARAM} | Zeichenkette | Druckparameter |
| DEVICE | {DEVICE} | Zeichenkette | Druckerqueue |
| JVAR_BIN_PATH | { JVAR_BIN_PATH} | Zeichenkette | Pfad auf das A.eins-Bin-Verzeichnis |
| Instanz | {Instanz} | Zeichenkette | Eindeutige Kennung des zur Zeit aktiven A.eins. |
| DatenQuelle | {datenquelle} | Zeichenkette | Datenquelle des Drucks |
| Specials | {specials} | Zeichenkette | „Private“ Kennzeichen |

Hinweis: Gross-Kleinschrift ist nicht relevant.

**Erläuterungen zu “Specials”:**

Die Methodik erlaubt es “private Daten” an die private Prozedure als Parameter gesammelt zu übergeben. Die “privaten Daten” können über die JVars mit dem Owner **3568** von Vorläufen, Makros, jegliche Makros im Formulardruck, Scripten, …bereitgestellt werden. Neben der eigentlichen Auswertung in der Datenbank-Prozedure sind Nutzer dieser Möglichkeit auf für die Pflege der JVars 3568 verantwortlich, z.B. Abräumen.

Die Übergabe der Werte zu den Jvar berücksichtig grundsätzlich die Möglichkeit in den Jvar „Stacks“ zu verwenden.

Das Beispiel wird demonstriert wie man auch nicht alltägliche Jvar-Namen und „JVar-Stacks“ adressiert.

Folgende Beispiel-Vorbelegung in einem Makro-Auszug sei gegeben:

```text
// …
  jvarsset( 3568 , "JVars Variablen
Key" , "JVars Variablen
Wert" );
// …
```

Folgender Beispiel-Aufruf sei im Dokument im „Barcode“ hinterlegt:

```text
p_barcode_specials_beispiel('{specials}')
```

<details>
<summary>Beispiel für “Specials”</summary>

```sql
CREATE PROCEDURE
p_barcode_specials_beispiel ( IN in_specials long varchar )
Result
(
  code long varchar,
  codetype long varchar
)
Begin
  declare dc_code long varchar;
  CALL sp_parse_json( 'dc_specials', in_specials
);
  set dc_code = dc_specials ."JVars Variablen
Key"[[1]];
  IF dc_code != 'JVars Variablen Wert' THEN
    CALL amic_exception( 'Übergabe
Dokument-Specials nicht geklappt!' , SQLCODE , SQLSTATE ,
'p_barcode_specials_beispiel', -999999 );
  END IF;
  select dc_code as code, 'QrCode' as
codetype;
End
```

</details>
