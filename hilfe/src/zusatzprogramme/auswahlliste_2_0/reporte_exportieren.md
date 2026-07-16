# Reporte exportieren

<!-- source: https://amic.de/hilfe/reporteexportieren.htm -->

Hat man einen Report zu einer Auswahlliste erstellt, so kann man diesen nicht nur drucken, sondern auch in vielfacher Form exportieren. Dieser Export lässt sich auch automatisieren um ggf. nachts die Daten zur Analyse vorzubereiten. Die geschieht mit folgendem Controlstring:

```text
^jpl AWEport (Anwendung, Variante, Profil, Ansicht, Reportbezeichnung, Ausgabeformat, Ausgabedatei)
```

| | **Bedeutung** |
| --- | --- |
| Anwendung, Variante | Hiermit wird beschrieben, welche Anwendungsvariante die Daten liefert<br> |
| Profil | In diesem Profil hat man die Eingrenzung festgelegt.<br> |
| Ansicht | Die Ansicht ist Optional. Wenn man hier einen Leerstring übergibt, so wird die Standardansicht verwendet.<br> |
| Reportbezeichnung<br> | Die Bezeichnung des Reports, den man unter ***Report bearbeiten*** angegeben hat und der unter ***Report drucken*** zu sehen ist.<br> |
| Ausgabeformat | Hier muss eine Zahl angegeben werden. Es stehen folgende Formate zur Verfügung:<br> <br> Pdf = 0,<br> Html = 1,<br> Rtf = 2,<br> Bitmap = 3,<br> MetaFile = 4,<br> Tiff = 5,<br> MultiTiff = 6,<br> Jpeg = 7,<br> Png = 8,<br> Xls = 9,<br> Xlsx = 10,<br> Docx = 11,<br> Xps = 12,<br> Mhtml = 13,<br> Xhtml = 14,<br> Svg = 15,<br> Jqm = 16,<br> Xml = 17,<br> Text = 18,<br> TextLayout = 19,<br> Tty = 20,<br> Preview = 21,<br> Pptx = 22<br> |
| Ausgabedatei | Optional. Wenn nichts angegeben, wird die Ausgabe ins Export-Verzeichnis von A.eins geschrieben. Das angegebene Verzeichnis muss existieren, es wird **nicht** automatisch angelegt. Der Dateiname bildet sich dann wie folgt:<br>Reportbezeichnung _ AnwVarSystemProfil _ AnwVarawBox _ AnwVarBesitzer + Extension<br>Die Datei wird überschrieben, wenn sie bereits existiert.<br> |

**Hinweis:** Fehler stehen im Fehlerprotokoll (Direktsprung [FEHLP]).
