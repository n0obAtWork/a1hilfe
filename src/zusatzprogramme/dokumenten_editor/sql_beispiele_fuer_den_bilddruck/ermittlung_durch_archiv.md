# Ermittlung durch Archiv

<!-- source: https://amic.de/hilfe/_bilderdruckarchiv.htm -->

Bei diesen Codetypen ermittelt das Programm die zugehörigen Mimetypen automatisch.

| Inhalt von "codetype" | Bedeutung von "code" |
| --- | --- |
| archiv | Enthält den Primary Key der Relation "Formulararchiv" Fa_Id, Fa_MndNr als Zeichenkette durch Komma getrennt. |

<details>
<summary>Beispiel für Codetype 'Archiv'</summary>

```sql
procedure p_beispiel_file()
result (code long varchar, codetype long varchar)
begin
  select '17494,1' as code, 'archiv' as codetype
end
```

Einrichtung in Strichcode im Feld "Text": p_beispiel_archiv()

</details>
