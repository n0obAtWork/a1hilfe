# Ausführen von SQL-Texten

<!-- source: https://amic.de/hilfe/ausfhrenvonsqltexten.htm -->

Wenn man auf einem Aktionsfeld lediglich ein unter **[SQLK]** gespeichertes Statement ausführen will, so kann man dies mit folgendem Controlstring erreichen:

```text
^dbx_select
( "SQLK", "#(sqlk_test,1)", "TMP" )
```

Die Funktion hat folgende Parameter

| Parameter | Im Beispiel | Beschreibung |
| --- | --- | --- |
| Cursorname<br> | "SQLK" | Name des Cursors. Dieser ist relativ beliebig, darf jedoch nicht länger als 30 Zeichen sein. Man findet diesen Namen z.B. im Tracefile wieder.<br> |
| Statement | "#(sqlk_test,1)" | Dies ist eine spezielle Syntax um dem System zu sagen, dass das auszuführende Statement aus den SQL-Texten kommt.<br>Dabei ist das Zeichen **#** sozusagen das Kommandowort. Der erste Parameter ist der Name des SQL-Textes und der zweite der Besitzer. Für unter SQLK erfasste Texte muss hier immer eine <strong>„1“</strong> stehen.<br> |
| Option<br> | "TMP" | Sorgt dafür, dass der Cursor automatisch wieder freigegeben wird.<br> |
