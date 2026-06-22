# Vorbelegung

<!-- source: https://amic.de/hilfe/_vorgangsmappeProfile_vorbelegung.htm -->

Für die Vorbelegung innerhalb der Strecke wird eine private Prozedur benötigt. Dabei müssen jedoch einige Richtlinien beachtet werden.

[IN-Parameter](./vorbelegung.md#vorb_IN_Parameter)

[Übergabefelder](./vorbelegung.md#vorb_uebergabe)

[RESULT-Klausel](./vorbelegung.md#vorb_RESULT_Klausel)

[Beispielprozedur](./vorbelegung.md#vorb_Beispielprozedur)

<p class="just-emphasize">IN-Parameter![](../../../ImagesExt/image8_1357.png)</p>

Folgende IN-Parameter werden beim Aufruf der Prozedur übergeben.

| Parameter | Beschreibung |
| --- | --- |
| in_KlammerNr | Klammernummer der Strecke. |
| in_Grid | Nummer der Datentabelle. |
| in_Occ | Zeile innerhalb der Datentabelle. |
| in_Profil | Name des aktuell verwendeten Profils. |
| in_Spalte | Name der Spalte, durch die die Prozedur aufgerufen wird. |
| in_Wert | Wert der übergebenen Spalte. |
| in_OccChange | 0 Die Prozedur wurde durch ein „Ausführungsfeld aufgerufen.  
1 Die Prozedur wurde durch eine neue Zeile aufgerufen. |

<p class="just-emphasize">Übergabefelder![](../../../ImagesExt/image8_1357.png)</p>

Die [Übergabefelder](./index.md#registerAllgemein2_Vorbelegung) können innerhalb des Profils festgelegt werden. Diese werden vor dem Aufruf der Prozedur in eine globale Tabelle geschrieben. Aus dieser Tabelle können dann die übergebenen Werte innerhalb der Prozedur wieder ausgelesen werden.

Die verwendete Tabelle für die Übergabeparameter nennt sich „GTT_AMIC_IDENT“. Folgende Spalten werden für die Übergabe verwendet.

| Spalte | Beschreibung |
| --- | --- |
| TYP | Da die Tabelle nicht nur in der Streckenerfassung verwendet wird, gibt dieses Feld den Datensatztyp an. Für die Übergabewerte wird der Typ "STRECKENERFASSUNG_VORBELEGUNGSPARAMETER" verwendet. |
| TEXT1 | In dieser Spalte befindet sich der Spaltenname des übergebenen Feldes. |
| TEXT2 | In dieser Spalte befindet sich der Wert des übergebenen Feldes. |

Zum Auslesen aus der Tabelle kann dann folgendes Statement verwendet werden.

```sql
Select cast(text2 as
char(255))
Into dc_ArtikelNummer
from gtt_amic_ident
where typ = 'STRECKENERFASSUNG_VORBELEGUNGSPARAMETER'

and text1 =
'Artikelnummer'
```

Dabei sollte nicht vergessen werden, den Wert in den richtigen Datentypen umzuwandeln (CAST).

<p class="just-emphasize">RESULT-Klausel![](../../../ImagesExt/image8_1357.png)</p>

Die „RESULT-Klausel“ ist die Beschreibung, welche Daten zurückgegeben werden.

| Parameter | Beschreibung |
| --- | --- |
| Spalte | Name der Spalte für die die Vorbelegung erfolgen soll. |
| Wert | Wert der Vorbelegung |
| Ueberschreiben | 0 Befindet sich in der Spalte bereits ein Wert, so wird dieser nicht überschrieben.  
1 Der zurückgegebene Wert wird immer in die Spalte übernommen. |

Da es sich um eine RESULT-Klausel handelt, können mehrere Spalten zur gleichen Zeit zurückgegeben werden. Entweder kann dafür eine lokale Tabelle oder ein „UNION“-Statement verwendet werden.

<p class="just-emphasize">Beispielprozedur![](../../../ImagesExt/image8_1357.png)</p>

In diesem Beispiel kann man erkennen, wie der Kopf und die Rückgabewerte der privaten Prozedur aussehen müssen.

```sql
CREATE PROCEDURE
p_VorbelegungStreckenerfassung
(
  in_KlammerNr   integer
  ,in_Grid
integer

,in_Occ        integer
  ,in_Profil     char(255)
  ,in_Spalte     char(255)
  ,in_Wert
char(255)
  ,in_OccChange  integer
)
result
(

Spalte
char(255)

,Wert
char(255)
  ,Ueberschreiben   integer
)
BEGIN
--
Select 'Einzelvorgang'
       ,'0'
       ,0
union all
Select 'WabewMenge'
       ,123654.8596
       ,0
union all
Select 'ArtikelNummer'
       ,'Tacrin'
       ,0;
END
```
