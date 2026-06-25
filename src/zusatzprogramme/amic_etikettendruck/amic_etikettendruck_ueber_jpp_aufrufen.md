# AMIC-Etikettendruck über JPP aufrufen

<!-- source: https://amic.de/hilfe/_AMICEtikettendruckJPP.htm -->

Um den AMIC-Etikettendruck programmgesteuert aufzurufen, existiert ein JPP-Objekt mit dem Namen **JEtikettendruck**.

| Methode | Parameter | Bedeutung |
| --- | --- | --- |
| Version | | Liefert die aktuelle Version. Aufruf:<br> <br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_DO ("AED", "VERSION", "LDB_TRANSFER$VC")&#10; call JPP_DEL("AED")</code></pre><br> <br>Dabei ist LDB_TRANSFER$VC das Feld auf der Maske, in der die Versionsnummer geschrieben werden soll. |
| Editieren | **LILAID** | Ruft den interaktiven Designer auf. Beispiel ( JPL-Syntax ):<br><br><br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )&#10; call JPP_EX( "AED", "Editieren" )&#10; call JPP_DEL("AED")</code></pre><br> |
| Vorschau | **LILAID**<br>[procedurecall] | Öffnet den Report als Vorschau. Der optionale Parameter überschreibt den Viewnamen bzw. den Prozedurnamen, den man in der Definition angegeben hat.<br> <br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )&#10; call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )&#10; call JPP_EX( "AED", "Vorschau" )&#10; call JPP_DEL("AED")</code></pre><br> |
| Drucken<br> | **LILAID** | Druck den Report. Beispiel:<br><br><br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )&#10; call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )&#10; call JPP_IN( "AED", "fa_KundNummer", "10000" )&#10; call JPP_IN( "AED", "fa_BelegDatum", "30.12.2017" )&#10; call JPP_IN( "AED", "fa_BelegTypText", "Kontoblatt" )&#10; call JPP_IN( "AED", "ask", "0" )&#10; call JPP_EX( "AED", "Drucken" )&#10; call JPP_DEL("AED")</code></pre><br> |
| [ask] | 0 oder 1, je nachdem, ob vor dem Druck der Drucker bzw. das Ausgabeformat abgefragt werden soll(=1) oder nicht (=0). Standard ist Abfrage(also 1 ). |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [printerprofil] | Man gibt hier einfach das [Profil](./definition_in_a_eins.md#Druckerprofile) an, dass man bei der Bearbeitung der Definitionen in A.eins erstellt hat. |
| [printername] | Hier gibt man den Drucker direkt mit dem Namen an. |
| [fa_KundNummer]<br>[fa_BelegNummer]<br>[fa_BelegDatum]<br>[fa_BelegReferenz]<br>[fa_BelegTypText]<br>[fa_BelegKlasse]<br>[fa_Info_Betreff]<br>[fa_Info_Kategorie]<br>[fa_Info_Stichwoerter]<br>[fa_Info_Autor]<br>[fa_Info_Titel]<br>[fa_Info_Kommentar] | Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet. |
| Exportieren<br> | **LILAID** | Exportiert den Report direkt im angegebenen Format. Ausgabeverzeichnis ist „..\\export\\lila“<br>Beispiel:<br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_IN( "AED", "LILAID", "EAN_ETIKETT" )&#10; call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )&#10; call JPP_IN( "AED", "Format", "PDF" )&#10; call JPP_EX( "AED", "Exportieren" )&#10; call JPP_DEL("AED")</code></pre><br> |
| **Format** | Format kann folgernde Werte haben: „HTML“, „PDF“, „RTF“, „BMP“, „JPG“ |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [defaultApp] | Öffnet das Ergebnis mit dem Programm welches mit diesem Format verknüpft ist. |
| | | |
| Archivieren | **LILAID** | Archiviert den Report ohne vorher zu drucken. Beispiel:<br><br><br><pre><code>call JPP_NEW("AED","JEtikettenDruck")&#10; call JPP_IN( "AED", "LILAID", "EAN_ETIKETT")&#10; call JPP_IN( "AED", "procedurecall", "p_kontoblatt(10000,10000,2017,12)" )&#10; call JPP_IN( "AED", "fa_KundNummer", "10000" )&#10; call JPP_IN( "AED", "fa_BelegDatum", "30.12.2017" )&#10; call JPP_IN( "AED", "fa_BelegTypText", "Kontoblatt" )&#10; call JPP_EX( "AED", "Archivieren" )&#10; call JPP_DEL("AED")</code></pre><br> |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [fa_KundNummer]<br>[fa_BelegNummer]<br>[fa_BelegDatum]<br>[fa_BelegReferenz]<br>[fa_BelegTypText]<br>[fa_BelegKlasse]<br>[fa_Info_Betreff]<br>[fa_Info_Kategorie]<br>[fa_Info_Stichwoerter]<br>[fa_Info_Autor]<br>[fa_Info_Titel]<br>[fa_Info_Kommentar] | Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet. |
