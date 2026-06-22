# Ermittlung durch Datei

<!-- source: https://amic.de/hilfe/_bilderdruckdatei.htm -->

| Inhalt von "codetype" | Inhalt von "codetype" | Beispiel |
| --- | --- | --- |
| file | Enthält den Pfad auf die Bild-Datei | '\\\\amrum\\aeins\\bin\\druck.jpg' |
| | | 'C:\\Users\\beispiel\\Pictures\\butterfly.bmp' |

| Unterstützte Bild-Formate (Bild-Datei-Erweiterungen) | Alternative | Beschreibung |
| --- | --- | --- |
| bmp | | Windows Bitmap |
| jpg | jpeg | JPG oder JPEG ist das häufigste Format für Dateien mit digitalen Fotos. Ob eine Datei .jpg oder .jpeg heißt, ist egal; .jpg ist nur die bei Dateinamen übliche Verkürzung von .jpeg auf drei Buchstaben. |
| gif | | Graphics Interchange Format |
| wmf | | Windows Metafile |
| exif | | Exchangeable Image File Format |
| emf | | Windows Enhanced Metafile |
| png | | Portable Network Graphics |
| ico | | Windows-Format für Icons |
| tif | tiff | Tagged Image File Format<br>TIF und TIFF sind genau dasselbe. TIF wird in älteren Dateisystemen verwendet, die die 8.3-Namenskonvention verwenden, während TIFF in neueren Dateisystemen verwendet wird, die lange Dateinamen erlauben. |

<details>
<summary>Beispiel für Codetype 'File'</summary>

```sql
procedure p_beispiel_file()
result (code long varchar, codetype long varchar)
begin
  select 'C::\Users\ah\Pictures\butterfly.bmp' as
code, 'file' as codetype
end
```

Einrichtung in Strichcode im Feld "Text": p_beispiel_file()

</details>
