# Anwendung Formulararchiv

<!-- source: https://amic.de/hilfe/_anwendungformulararc.htm -->

Hauptmenü > Warenverkauf > Archiv > Archiv

Hauptmenü > Wareneinkauf > Archiv > Archiv

Hauptmenü > Büro und Internet > Büroumgebung > Archiv

Hauptmenü > CRM > Archiv > Archiv

Direktsprung **[FA]**

Hier findet sich die einzige Variante „Formulararchiv“.

Im Gegensatz zu [Formulararchiv-Administration](../formulararchiv_administration/index.md) handelt es sich bei dieser Variante um die mehr anwenderorientierte Präsentation des Formulararchivs.

| Felder | |
| --- | --- |
| KndNr. | Zuordnung des Beleges zu einer Kundennummer.<br>Die Versorgung der Kundennummer erfolgt je nach Ursprung des Beleges durch das System bzw. den Anwender.<br>Dabei können je nach Zulieferung (Druck, Import, Manuell, sonstige Programmteile, Fremdsoftware) eine 0 bzw. eine datentechnische NULL ins System eingestellt werden.<br>**Die KndNr. wird nicht länger durch fa_kundennummer (char 10) abgebildet, sondern intern durch fa_kundnummer (int).**<br>Fa_kundnummer kann technisch die -1 annehmen im Sinne von „Dokument hat keine Kundenzuordnung“. Dieses wird sowohl in den Auswahllisten als auch in Strg-F12 „Archiv anzeigen“ als Null dargestellt.<br>Somit ist es nun möglich in Strg-F12 bei der Spalteneingrenzung nach „null“ zu selektieren, die Auswahllisten haben eine Extra-Möglichkeit bekommen nach Dokumenten zu suchen denen keine Kundennummer zugeordnet ist. |
| Klassifizierung | Vom Anwender vorgegebene Klassifizierung des Beleges. |
| Beleg-Typ | Beleg-Typ |
| Beleg-Nr | Beleg-Nummer |
| Beleg-Datum | Beleg-Datum |
| Archiv/Druck-Datum | Zeitpunkt der Einstellung ins Archiv |
| Beleg-Referenz | Die Referenz die zusammengehörige Archiv-Einträge ausweist. |
| Herkunft | Ursprung des Beleges |
| Anleger | Der Kurzname des Anlegers des Archiv-Eintrages. |
| Beleg-Klasse | Vom System oder dem Anwender vorgegebene Typisierung des Beleges. |
| Inhalt | Technische Qualifizierung des Inhalts über den Mimetyp des Beleges |
| Mnd | Mandant |
| Autor, Betreff, Titel, Kommentar, Stichwörter | Unterstützende Felder zwecks Katalogisierung eines Beleges nach eben diesen Kriterien. |
| Formularnummer | Die Formularnummer des Druckes. |
| Barcode | Zugewiesener Barcode |
| Bedienerklasse | Bedienerklasse |
| Dateiname | Dateiname beim Import des Beleges |
| ID | Schlüssel-Identifikation des Beleges. Innerhalb eines Mandanten (MndNr!) eindeutig. |

| Funktionen | |
| --- | --- |
| ***Archiv anzeigen*** **CF12** | Das Dokument des selektierten Archiv-Eintrages anzeigen |
| Referenzen anzeigen | [Referenzen anzeigen](../formulararchiv_administration/recherche_nach_referenznummern.md) |
| ***Ändern*** **F5** | [Archiv ändern (Ansehen)](./archiv_aendern_ansehen.md) |
| ***Ansehen*** **F6** | [Archiv ändern (Ansehen)](./archiv_aendern_ansehen.md) |
| E-Mail senden | [Archiv Mail Versand](../../archiv_manager/archiv_mail_versand/index.md) |
| ***Hinzufügen …*** **F8** | [Archiv – Dokumente hinzufügen](../../archiv_dokumente_hinzufuegen.md) |
| ***Barcode zuweisen …*** **SF8** | [Archiv Barcode](../../archiv_barcode.md) |
| Referenzieren | [Referenzieren](../formulararchiv_administration/referenzieren.md) |
| Drucken | Druckt Archiv-Einträge – insoweit das möglich ist. |
| Pdf-Drucken … | Druckt Pdf-Dokumente (siehe [PDF-Drucken](../../dokumentenverwaltung_archiv_anzeigen/dokumentenverwaltung_multifunktionsleiste/pdf_drucken.md)) |

Zudem können sich in dieser Variante Funktionen - die über [Archiv Import](../../archiv_import/index.md) integriert werden können -befinden.

| Suchen | |
| --- | --- |
| Kundennummer | |
| Mit Kundennummer | Nein [Standard], Ja<br>Bei Nein werden die Dokumente gelistet, die im Selektionsbereich keine Kundennummernzuordnung haben.<br>Die in der Regel nicht existente Kundennummer 0 wird außen vorgelassen, da es in den Programm-Modulen und externen Anwendungen unüberschaubare Verwendungen für „0“ gibt. |
| Archivdatum | |
| Archivdatum Tage zurück | Da das Archiv u.U. sehr viele Einträge enthält besteht hier die Möglichkeit die Anzahl über eine Rückschau einzugrenzen. |
| Belegnummer | |
| Belegdatum | |
| Beleg-Referenznr. | |
| Herkunft | <ul><li>Intern</li><li>Extern</li><li>Manuell</li></ul> |
| Anleger | |
| Belegklasse | |
| Mandant | |
| Autor, Betreff, Titel, Kommentar, Stichwörter | |
| Formularnummer | |
| Barcode | |
| Bedienerklasse | |
| Dateiname | |
| Klassifizierung | |
| Gruppentyp | |

<p class="siehe-auch">Siehe auch:</p>

- [Archiv ändern (Ansehen)](./archiv_aendern_ansehen.md)
