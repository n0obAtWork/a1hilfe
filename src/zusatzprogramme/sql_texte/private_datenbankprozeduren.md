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
| Neu (F8) | Es öffnet sich ein Dialog in dem sich ein privater Arbeitsname unter „Sql-Text“ angeben lässt. Empfohlen ist privaten Arbeitsnamen ein **p_** voranzustellen.<br>Mittels „Template“ lässt sich als Vorlage der Text einer bestehenden privaten Datenbank-Prozedure auswählen.<br><br>Wird das Feld „Template“ leer gelassen öffnet sich der Editor mit einem Vorschlag.<br><br>Als Beispiel für die fiktive private Datenbank-Funktion „P_Beispiel“:<br><br><br><br><pre><code>-- Priv. Prozedur p_beispiel --- Streckeunit&#10; 20.11.2023&#10;--&#10;--&#10; Beschreibung&#10;--&#10;--&#10;--&#10;CREATE PROCEDURE&#10; p_beispiel ( )&#10;--&#10;BEGIN&#10;-- Hier kann die&#10; Verarbeitung beginnen&#10;--&#10;--&#10;--&#10;EXCEPTION&#10; when others&#10; then&#10; &#10; call amic_exception( ERRORMSG() &#124;&#124; CHAR(10) &#124;&#124; CHAR(13) &#124;&#124; TRACEBACK(),&#10; SQLCODE , SQLSTATE , 'p_beispiel' , -1 , in_commit = 1);&#10; --&#10; ggf. sofortiges commit mit in_commit=0 unterbinden (z.B. bei Verwendung in&#10; Trigger, Atomic)&#10;ENDD</code></pre> |
| Editieren (F5) | Der Editor öffnet sich mit dem Text der Datenbank-Prozedure (siehe „Source“) |
| Löschen (F7) | Entfernt die Datenbank-Prozedure aus dem System-. |
| Create (F10) | Legt das Datenbank-Objekt gemäß der „Source“ an. |
| Drop (F11) | Entfernt ggf. ein vorhandenes Datenbank-Objekt. |
| Export (Umschalt F8) | Exportiert die Informationen der Datenbank-Prozedure in eine Datei, die zur Herstellung der so gespeicherten Datenbank-Prozedure wieder herangezogen kann. (Einspielung mittels **[OSQL]** und Funktion „Sql-Texte ausführen“) |
