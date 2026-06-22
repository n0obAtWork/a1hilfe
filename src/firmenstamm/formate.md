# Formate

<!-- source: https://amic.de/hilfe/_fsformate.htm -->

Hauptmenü > Administration > Steuerung > Benutzer Feldsteuerung

Direktsprung [FORMA]

In A.eins gibt es an vielen Stellen Entscheidungsfelder, deren Wert sich aus einer vorgegebenen Liste auswählen lässt. Diese Listen heißen in A.eins „FORMAT“. Oder „FSFormate“

In der Regel sind diese Listen in Form von Systemformaten vorgegeben und für den Bediener nicht änderbar. Es gibt jedoch zwei Formatgruppen, die auch oder ausschließlich vom Bediener gepflegt werden: Benutzerformate und Anwendungsformate.

Systemformate

Diese Formate werden ausschließlich von der Entwicklung erstellt und gepflegt.

Benutzerformate

Diese Formate werden ausschließlich in der Installation lokal verwendet. Diese Formate werden zum Beispiel in AIS-Masken verwendet. Die Inhalte dieser Formate werden vom Anwender bzw. Supporter individuell eingetragen.

Die Namenskonvention gebietet hier den Präfix „BF_“.

Anwendungsformate

Hier werden in der Regel einige Basiselemente der Liste vorgegeben und deren Nummerierung ist bis zu einem bestimmten Wert für die Verwendung den Entwicklern des Systems vorbehalten. Weitere Werte können oberhalb des gesperrten Bereichs eingetragen werden.

Die Namenskonvention gebietet hier den Präfix „AF_“.

Pfleger:

| Bezeichnung | Inhalt |
| --- | --- |
| Eigentümer | Systemformat, Benutzerformat oder Anwendungsformat |
| Formatname | Name des Formats |
| Bezeichnung | Hinweis für den Verwender des Formats |
| Nicht übersetzen | Wenn hier „Ja“ angegeben wird, der Text dieses Formates nicht durch Übersetzungen überschrieben. |
| Nummern reservieren bis | Wird nur von Entwicklern von Anwendungsformaten angezeigt, um festzulegen, welche Nummern für die Entwicklung reserviert bleiben sollen. |
| Nr | Interne Nummer z.B. einer Enumeration (c#Enum) |
| Textersetzung | Dargestellter Text für diesen Wert |
| Kommentar, Schnipsel | Wird dieses Format in einem Auswahllistenfilter verwendet, so muss hier ein Schnipsel hinterlegt werden, der bestimmt, wie dieser Wert im SQL gefiltert werden soll. |
| Aktiv | Hier wird entschieden, ob der Eintrag überhaupt angezeigt und verwendet werden soll.  
• Ja = uneingeschränkt verwendbar  
• Nein = nicht verwendbar  
• Kann nur angezeigt, aber nicht ausgewählt werden. (z.B. um einen interims-Status anzuzeigen) |
| ENUM | Sofern dieses Format von AMIC ausgeliefert wird, so ist für c#-Implementationen eine Enumeration mit dem Namen FS_&lt;Formatname> im Namespace A.eins verfügbar, die die eingetragenen ENUM-Werte beinhaltet.  
Dies gilt für immer mehr Systemformate und den vorausgelieferten Teil der Anwendungsformate.  
Der Bediener selbst kann diese Spalte nicht verwenden. |
