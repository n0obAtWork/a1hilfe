# SQL-Variablen in der Auswahlliste verwenden

<!-- source: https://amic.de/hilfe/sqlvariableninderauswahllistev.htm -->

Wenn in der Auswahlliste Spalten fehlen, ist es nicht unbedingt notwendig, eine private Ableitung der Auswahlliste zu bilden. Man kann sich eigene SQL-Variablen erstellen, die dann wie ein Subselect in das SQL-Statement eingefügt werden. Wenn man den Reiter SQL-Variablen das erste Mal aufruft, sieht man nur eine Liste, in der in der ersten Zeile „(Neu)“ steht. Klick man in diese Zeile, so öffnet sich eine weitere Maske, in der dann die notwendigen Informationen angeben werden kann:

| | **Bedeutung** |
| --- | --- |
| Überschrift | Die Überschrift, die über der neuen Spalte erscheinen soll.  
 |
| Typ | Der Typ des Feldes, wie er auch in der FIELD-Anweisung verwendet wird. Eine Auswahl der möglichen Typen ist mit F3 möglich. Hier können sowohl Basistypen als auch FS-Formate ausgewählt werden.  
 |
| FS-Format | Wenn man als Typen FS ausgewählt hat, kann man hier ein Format angeben. Eine Auswahl des FS-Formats ist mit F3 möglich.  
 |
| Aktiv | Wenn man ein Subselect momentan nicht ausführen möchte, aber die Arbeit, die man in die Formulierung gesteckt hat, nicht über den Haufen werden will, so kann man hier die SQL-Variable einfach deaktivieren. Sie wird dann komplett ignoriert.  
 |
| Herkunft | Dies ist nur ein Anzeigefeld und kann vom Anwender nicht direkt beeinflusst werden. Es hat folgernde Ausprägungen:  
• Anwender: Dies ist die Standardeinstellung, wenn die Variable über den Gestaltungsdialog angelegt wurde.  
• Steuerparameter: Der Steuerparameter „Reklamationsmaßnahmen“ (SPA 1040) nutzt IBVariablen um verwendete Spalten automatisch der Anwendung „Reklamationen“ hinzuzufügen. Diese können zwar hier bearbeitet aber nicht gelöscht werden.  
 |
| SQL-Ausdruck | Der SQL-Befehl, der in die Auswahlliste eingebaut werden soll. Hier kann man auf alle Felder zugreifen, die auch in dem SQL-Statement stehen. Nur wenn hier etwas eingetragen wurde, erscheint die Spalte auch in der Auswahlliste. Hier einige Beispiele:  
   
SQL in der Auswahllisten:  
   
In dem SQL oben fehlt unter anderem die Bezeichnung des Kontos. Diese kann man dann wie folgt ergänzen:  
   
Man kann sich das Ganze am besten klarmachen, wenn man sich vorstellt, dass intern die Auswahlliste wie folgt erweitert wird:  
 |

Die so neu angelegten Spalten stehen zuerst immer ganz rechts außen, können aber genau wie alle anderen Spalten mit verschoben werden. Auch stehen, nachdem diese SQL-Variable erst einmal in der Auswahlliste erschienen ist, alle anderen Funktionen des Gestaltungsdialoges – Sortierung, Farbgestaltung, Feldauswahl und Summierung - für sie zur Verfügung.
