# Zeilenumbruch in Multiline Texten

<!-- source: https://amic.de/hilfe/zeilenumbruchinmultilinetexten.htm -->

Will man die Darstellung in Multiline-Textfeldern formatieren, so kann man die einen Zeilenumbruch dadurch erzwingen, indem man ‚\\n‘ in den Text einfügt:

```sql
Select
‘Zeile1\nZeile2\nZeile3 und jetzt kommt erst Zeile4\nZeile4‘ as
Ergebnis
```

Das Ergebnis sieht dann folgendermaßen aus:

![](../../../ImagesExt/image8_1076.png)
