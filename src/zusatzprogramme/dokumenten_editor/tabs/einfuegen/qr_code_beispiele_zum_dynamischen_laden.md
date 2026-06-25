# QR-Code Beispiele zum dynamischen Laden

<!-- source: https://amic.de/hilfe/_dokumenthilfe_qr_dyn.htm -->

Neben den üblichen Textgestaltungsmöglichkeiten lassen sich über die Punkte „Einfügen“ („Illustrationen“) mit „Bild“ und „Strichcode“ jeweils statische Elemente integrieren.  
Für „Bild“ siehe „Bild…“.

Für „Strichcode“ siehe im Kontext-Menü „Formatieren…“ und dort unter „Strichcode Layout“ im Register „Typ und Farbe“ und da unter „Typ“ die Elemente „Kodierung“ und „Text“.

Im Zusammenhang mit einer geeigneten privaten Datenbank-Routine lässt sich der Strichcode „dynamisieren“. Zum Zeitpunkt der Druckaufbereitung wird die Datenbank-Routine mit der WabewId der zugehörenden Warenpositionszeile aufgerufen. Anhand dieser kann dann ein Text für den Strichpunkt aufbereitet und zurückgegeben werden.

Aktiviert wird diese Mechanik durch eintragen des Namens der Strichcode-Datenbank-Routine im Element „Text“

Zu beachten ist die Konvention das eine solche Routine den Prefix „p_“ hat, also zum Beispiel: p_Barcode

<p class="just-emphasize">Beispiel für eine solche Strichcode-Datenbank-Routine:</p>

```sql
---<summary>Ermittelt aus
der WabewId den zugehörigen Artikel-Code und gibt den Code-Typen
bekannt</summary>
---<param
name="in_wabewid">wabewid</param>
---<returns>
---code    : der zugeordnete Code
---codetyp : der zugeordnete Codetyp
---</returns>
Create Procedure p_Barcode( in in_wabewid integer )
Result
(
    code long
varchar,
    codetype
char(32)
)
Begin
  select 'Democode für QrCode-Beispiel' || ', wabewid=' || in_wabewid as code,
'QrCode'
as
codetype
End
```
