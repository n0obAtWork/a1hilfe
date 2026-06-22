# Archiv-Import über JPP-Methode JVAR_IMPORT aus JFA_Import

<!-- source: https://amic.de/hilfe/_archivimportberjppme.htm -->

Diese Methode wird u.a. im Hinzufügen-Dialog als Basis-Methode des Archivs eingesetzt. Als JPP-Methode steht sie GUI-los auch zum Costumizing bereit.

Diese Methode wird im Rahmen des Integrationstestes amic_test_jarchivexport_tofile verwendet.

| Parameter: |
| --- |
| Owner | Pflichtfeld | Gibt den JVAR-Owner vor, in dem die folgenden dynamischen Parameter gesucht werden. |
| $file | Pflichtfeld | Dateipfad |
| $delete | Optional | Löschen der Datei nach Import<br>Standard 0 = Nein |
| fa_mandant | Optional | |
| fa_kundenummer | Optional | |
| fa_belegtyptext | Optional | |
| fa_belegnummer | Optional | |
| fa_belegreferenz | Optional | |
| fa_info_autor | Optional | |
| fa_info_betreff | Optional | |
| fa_info_kategorie | Optional | |
| fa_info_stichwoerter | Optional | |
| fa_info_kommentar | Optional | |
| fa_info_titel | Optional | |
| fa_belegdatum | Optional | |
| fa_mndnr | Optional | |
| fa_barcode | Optional | |
| fa_klasse | Optional | Standard ist 0 |
| fa_belegklasse | Optional | Standard ist 0 |
| fa_bedienerklasse | Optional | Standard ist die Bedienerklasse des ausführenden Users |

Bei erfolgreicher Archivierung befindet sich systemüblich in

| 5000 | JVARS_LAST_FA_ID |
| --- | --- |
| 5000 | JVARS_LAST_FA_MNDNR |

der Primary-Key des hinzugefügten Archiv-Dokuments der Relation Formulararchiv.
