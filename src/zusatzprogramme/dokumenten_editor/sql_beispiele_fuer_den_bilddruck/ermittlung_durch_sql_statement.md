# Ermittlung durch Sql-Statement

<!-- source: https://amic.de/hilfe/_bilderdrucksql.htm -->

Wird ein unterstützter Mimeytp in "codetyp" übermittelt dann wird der Wert in "code" als Sql-Statement zur Ermittlung des Bild-Inhaltes interpretiert.

| Unterstützte Mimetypen in "codetyp" |
| --- |
| image/bmp |
| image/ jpeg |
| image/png |
| image/tiff |
| image/gif |
| image/x-icon |

<details>
<summary>Beispiel-Prozedur für den Fall Mimetyp</summary>

```sql
procedure p_beispiel_mime()
result (code long varchar, codetype long varchar)
begin
  select 'select i_image from bitimages where
imageid=117' as code,
  (select i_mime from bitimages where imageid=117)
as codetype
end
```

Einrichtung in Strichcode im Feld "Text": p_beispiel_mime()

</details>
