# Neues Kennzeichen in der Warenbewegung

<!-- source: https://amic.de/hilfe/_wabewktrbuchkorr.htm -->

Es wurde ein neues Kennzeichen in der Warenbewegung **WabewKtrBuchkorr** angelegt. Dieses Kennzeichen wird im Moment nur in Vorgängen gesetzt die beim Washout oder Circle Geschäft erstellt werden. Dies bedeutet, es werden keine Vorgänge mehr mit Menge > 0 und Wert = 0 zum ausbuchen der Mengen erstellt. Es werden jetzt Vorgänge mit Wertartikel angelegt. Das Kennzeichen wird dabei auf 1 gesetzt ist. Damit können Vorgänge mit einem Wertartikel in den Kontraktbewegungen angezeigt werden.

Private Auswertung die die **ktrbewmenge** und **wabewsignimengen** berücksichtigen müssen abgeändert werden. Damit die privaten Auswertungen weiterhin ordnungsgemäß funktionieren.

Beispiel aus der Prozedur amic_func_ktr_calc_all_ratierlich

```text
sum((isnull(kb.ktrbewmenge,0) - isnull(kb.ktrbewdispmenge,0)) * wb.wabewVorzeichen *
if isnull(wb.WabewKtrBuchkorr,0) = 1 then 1 else
wb.wabewSigniMengen endif )
```
