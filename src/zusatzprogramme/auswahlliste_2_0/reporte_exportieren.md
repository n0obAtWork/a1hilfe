# Reporte exportieren

<!-- source: https://amic.de/hilfe/reporteexportieren.htm -->

Hat man einen Report zu einer Auswahlliste erstellt, so kann man diesen nicht nur drucken, sondern auch in vielfacher Form exportieren. Dieser Export lässt sich auch automatisieren um ggf. nachts die Daten zur Analyse vorzubereiten. Die geschieht mit folgendem Controlstring:

```text
^jpl AWEport (Anwendung, Variante, Profil, Ansicht,
Reportbezeichnung, Ausgabeformat, Ausgabedatei)
```

| | **Bedeutung** |
| --- | --- |
| Anwendung, Variante | Hiermit wird beschrieben, welche Anwendungsvariante die Daten liefert  
 |
| Profil | In diesem Profil hat man die Eingrenzung festgelegt.  
 |
| Ansicht | Die Ansicht ist Optional. Wenn man hier einen Leerstring übergibt, so wird die Standardansicht verwendet.  
 |
| Reportbezeichnung  
 | Die Bezeichnung des Reports, den man unter Report bearbeiten angegeben hat und der unter Report drucken zu sehen ist.  
 |
| Ausgabeformat | Hier muss eine Zahl angegeben werden. Es stehen folgende Formate zur Verfügung:  
   
 Pdf = 0,  
 Html = 1,  
 Rtf = 2,  
 Bitmap = 3,  
 MetaFile = 4,  
 Tiff = 5,  
 MultiTiff = 6,  
 Jpeg = 7,  
 Png = 8,  
 Xls = 9,  
 Xlsx = 10,  
 Docx = 11,  
 Xps = 12,  
 Mhtml = 13,  
 Xhtml = 14,  
 Svg = 15,  
 Jqm = 16,  
 Xml = 17,  
 Text = 18,  
 TextLayout = 19,  
 Tty = 20,  
 Preview = 21,  
 Pptx = 22  
 |
| Ausgabedatei | Optional. Wenn nichts angegeben, wird die Ausgabe ins Export-Verzeichnis von A.eins geschrieben. Das angegebene Verzeichnis muss existieren, es wird **nicht** automatisch angelegt. Der Dateiname bildet sich dann wie folgt:  
Reportbezeichnung _ AnwVarSystemProfil _ AnwVarawBox _ AnwVarBesitzer + Extension  
Die Datei wird überschrieben, wenn sie bereits existiert.  
 |

**Hinweis:** Fehler stehen im Fehlerprotokoll (Direktsprung [FEHLP]).
