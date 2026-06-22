# Private Datenbankprozeduren

<!-- source: https://amic.de/hilfe/_sqltexte_sqlpp.htm -->

Hauptmenü > Administration > Werkzeuge > SQL Textmanager und dann Variante „Private Datenbankprozeduren“

oder Direktsprung **[SQLPP]**

| Auswahlliste | Bedeutung |
| --- | --- |
| SQL Text | Eindeutiger Arbeitsname innerhalb der Datenvorhaltung. |
| Datenbankname | System-Name des Objektes in der Datenbank (kann sich - sollte aber gemäß Empfehlung nicht - vom Arbeitsnamen unterscheiden) |
| Definition | Der Text der Prozedure gemäß System-Datenbank-Tabelle „sysprocedure“ |
| Source | Der Quelltext der Prozedure gemäß A.eins-Datenbank-Tabelle „Sql_Stamm“ |

| Bereichsauswahl/Filter | Bedeutung |
| --- | --- |
| Textname | Suchen im Arbeitsnamen „SQL Text“ |

| Funktionen | Bedeutung |
| --- | --- |
| Neu (F8) | Es öffnet sich ein Dialog in dem sich ein privater Arbeitsname unter „Sql-Text“ angeben lässt. Empfohlen ist privaten Arbeitsnamen ein **p_** voranzustellen.  
Mittels „Template“ lässt sich als Vorlage der Text einer bestehenden privaten Datenbank-Prozedure auswählen.  
    
Wird das Feld „Template“ leer gelassen öffnet sich der Editor mit einem Vorschlag.  
    
Als Beispiel für die fiktive private Datenbank-Funktion „P_Beispiel“:  
    
    
 |
| Editieren (F5) | Der Editor öffnet sich mit dem Text der Datenbank-Prozedure (siehe „Source“) |
| Löschen (F7) | Entfernt die Datenbank-Prozedure aus dem System-. |
| Create (F10) | Legt das Datenbank-Objekt gemäß der „Source“ an. |
| Drop (F11) | Entfernt ggf. ein vorhandenes Datenbank-Objekt. |
| Export (Umschalt F8) | Exportiert die Informationen der Datenbank-Prozedure in eine Datei, die zur Herstellung der so gespeicherten Datenbank-Prozedure wieder herangezogen kann. (Einspielung mittels **[OSQL]** und Funktion „Sql-Texte ausführen“) |
