# AMIC-Etikettendruck über JPP aufrufen

<!-- source: https://amic.de/hilfe/_AMICEtikettendruckJPP.htm -->

Um den AMIC-Etikettendruck programmgesteuert aufzurufen, existiert ein JPP-Objekt mit dem Namen **JEtikettendruck**.

| Methode | Parameter | Bedeutung |
| --- | --- | --- |
| Version | | Liefert die aktuelle Version. Aufruf:  
   
   
Dabei ist LDB_TRANSFER$VC das Feld auf der Maske, in der die Versionsnummer geschrieben werden soll. |
| Editieren | **LILAID** | Ruft den interaktiven Designer auf. Beispiel ( JPL-Syntax ):  
    
    
 |
| Vorschau | **LILAID**  
[procedurecall] | Öffnet den Report als Vorschau. Der optionale Parameter überschreibt den Viewnamen bzw. den Prozedurnamen, den man in der Definition angegeben hat.  
   
 |
| Drucken  
 | **LILAID** | Druck den Report. Beispiel:  
    
    
 |
| [ask] | 0 oder 1, je nachdem, ob vor dem Druck der Drucker bzw. das Ausgabeformat abgefragt werden soll(=1) oder nicht (=0). Standard ist Abfrage(also 1 ). |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [printerprofil] | Man gibt hier einfach das [Profil](./definition_in_a_eins.md#Druckerprofile) an, dass man bei der Bearbeitung der Definitionen in A.eins erstellt hat. |
| [printername] | Hier gibt man den Drucker direkt mit dem Namen an. |
| [fa_KundNummer]  
[fa_BelegNummer]  
[fa_BelegDatum]  
[fa_BelegReferenz]  
[fa_BelegTypText]  
[fa_BelegKlasse]  
[fa_Info_Betreff]  
[fa_Info_Kategorie]  
[fa_Info_Stichwoerter]  
[fa_Info_Autor]  
[fa_Info_Titel]  
[fa_Info_Kommentar] | Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet. |
| Exportieren  
 | **LILAID** | Exportiert den Report direkt im angegebenen Format. Ausgabeverzeichnis ist „..\\export\\lila“  
Beispiel:  
 |
| **Format** | Format kann folgernde Werte haben: „HTML“, „PDF“, „RTF“, „BMP“, „JPG“ |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [defaultApp] | Öffnet das Ergebnis mit dem Programm welches mit diesem Format verknüpft ist. |
| | | |
| Archivieren | **LILAID** | Archiviert den Report ohne vorher zu drucken. Beispiel:  
    
    
 |
| [procedurecall] | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde. |
| [fa_KundNummer]  
[fa_BelegNummer]  
[fa_BelegDatum]  
[fa_BelegReferenz]  
[fa_BelegTypText]  
[fa_BelegKlasse]  
[fa_Info_Betreff]  
[fa_Info_Kategorie]  
[fa_Info_Stichwoerter]  
[fa_Info_Autor]  
[fa_Info_Titel]  
[fa_Info_Kommentar] | Wird einer dieser optionalen Parameter verwendet, dann werden die Archivierungseinstellungen aus der Definition ignoriert und stattdessen diese Werte verwendet. |
