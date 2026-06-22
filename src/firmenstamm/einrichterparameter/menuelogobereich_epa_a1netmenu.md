# Menülogobereich (EPA A1NETMENU)

<!-- source: https://amic.de/hilfe/_A1NETMENU.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| ArchivAnzeige | Ja | Wird hier Nein eingeben wird die Uhr dargestellt |
| ArchivAnzeigeFunktion | AMIC_MENU_ARCHIVANZEIGE | Diese Datenbank-Procedure gibt die fa_id, fa_mndnr zurück, dessen Bild-Dokument im Menülogobereich dargestellt werden soll.  
Signatur ist  
CREATE PROCEDURE AMIC_MENU_ARCHIVANZEIGE( IN in_bedienerklasse integer )  
RESULT  
(  
 fa_id integer,  
 fa_mndnr integer  
) |
