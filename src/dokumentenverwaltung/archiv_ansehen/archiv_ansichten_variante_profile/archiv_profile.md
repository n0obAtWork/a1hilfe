# Archiv-Profile

<!-- source: https://amic.de/hilfe/_archivprofilpflege.htm -->

Hier werden die Profile gepflegt.

| Felder | |
| --- | --- |
| Name | Ansichtsprofil-Identifikation<br>Dieser Profilname wird an den betreffenden Stellen auf Masken und Funktionsbezeichnungen dargestellt. |
| Bezeichnung | Alternative Bezeichnung für Funktionen im Hauptmenü. |
| Funktion | Funktions-Identifikation (\*)<br> <br>F3-Auswahl auf [Freigegebene Archiv-Editoren](./archiv_profile.md#archiv_profil_editoren) |
| Optionbox | (\*):<br>Funktion und Optionbox bestimmen den Kontext der durch das Profil aufgerufenen A.eins-Funktionalität. Somit geben Sie zusätzlich auch die Berechtigungsrolle vor.<br> <br>Erläuterungen zu den Berechtigungen:<br>Der Rollen-Kontext ah_archivbelegfluss/OB_ARCHIV.VIEWDIALOG steuert ob die Funktion Belegfluss ![view\_refresh](../../../ImagesExt/image8_935.png "view_refresh") überhaupt sichtbar ist.<br>Der Rollen-Kontext dieser **Funktion** der der angegebenen **Optionbox** bestimmt, ob der Archiv-Editor ausgeführt werden darf.<br>Somit ist es möglich die Daten des Belegflusses einzusehen, aber rollentechnisch zu verhindern, dass der Archiv-Editor auf diesen Daten aufgerufen werden kann |
| Direktsprung | Hier lässt sich ein Direktsprung angeben. |
| Profil-Zuordnung | Belegfluss-Archivanzeigen unterstützen Profile.<br>Eine Profil-Zuordnung ist technisch über die Kombination Anwendung/Variante/Besitzer gegeben und kann hier eindeutig zugeordnet werden.<br>Damit steht dann die Funktion ![window\_options](../../../ImagesExt/image8_936.png "window_options") in der Belegfluss-Archivanzeige zur Verfügung.- |
| Felder<br> Ableitung | Hier findet sich einer [XML-Beschreibung](./ableitung_xml_beschreibung.md) die Datengewinnung der anzuzeigenden Archiv-Daten wieder. |
| Sonstiges<br> Archiv anzeigen mit Belegfluss-Funktion | Bestimmt ob in bestimmten Szenarien der „Belegfluss-Button“ beim „Archiv anzeigen“ aktiv ist – oder nicht.<br> |
| Sonstiges<br> Archiv anzeigen mit Löschen-Funktion | Bestimmt ob in bestimmten Szenarien der „Löschen-Button“ beim „Archiv anzeigen“ aktiv ist – oder nicht.<br> |
| Sonstiges<br> Where-Klausel(\*\*) | Veraltet. Wird durch ***Felder Ableitung*** abgelöst.<br> <br>Der SQL-Schnipsel der an das Archiv-SQL angehängt wird der die Ermittlung der Daten durchführt.<br>Es wird dabei von ausgegangen das es sich vom Typus um eine And-Erweiterung handelt, diese muss also entsprechend angegebene werden, also zum Beispiel:<br>„and 1=1“<br> <br>Ein leeres SQL-Statement ist ein gültiges. |
| Sonstiges<br> Join-Klausel | Obsolet. |
| Sonstiges<br> With-Klausel | Obsolet. |
| Tiptext Titel/Tiptext | Hier werden die Haupt-Menü-üblichen Tipptexte der (privaten) Funktion vorgegeben. |
| Icon | In Weiter-Entwicklung<br>Hier wird ein Icon aus einer fest vergebenen Auswahl von Icons auswählbar sein. |
| (\*\*)<br>Folgende JVARS sind in den Ausdrücken verwendbar | |
| :!jvars_5001_ZW1 | Entspricht dem Wert der Einrichtung aus Bedienerstamm/Kundenindiv. SQL Anpassung/WWW_BELEGFLUSS |
| :!jvars_5001_ZW2 | Ist die Aufbereitung des :!jvars_5001_ZW1 in Form von<br>select if ':ZW1' = '' then -1 else '0:ZW1' endif wert<br>Für nicht eingerichtete Belegflüsse ist er somit -1. |

| Funktionen | |
| --- | --- |
| Stammdaten pflegen | Im jeweiligen Bearbeitungsmodus stehen die gängigen Bearbeitungs-Funktionen zur Verfügung |
| SQL prüfen | In Weiter-Entwicklung!<br> <br>Führt eine syntaktische Überprüfung durch.<br>(Diese Überprüfung macht allerdings nur Sinn solange keine Ableitungen involviert sind die weitere Aliase als FA zur Verfügung stellen.<br>Aus diesem Grunde wird auch nicht sofort die Eingabe des Sql validiert) |
| Speichern unter … | Beim „Speichern unter“ von AMIC-Vorlagen, also solchen die mit AMIC anfangen, wird eben dieses vorangestellte AMIC automatisch entfernt. |

Freigegebene Archiv-Editoren

\*Veraltet!\*

Das Zusammenspiel von Archiv und Aeins im Belegfluss basiert auf durch Aeins freigegebene Archiv-Editoren. Das sind Programmteile die ein bestimmtes Kommunikationsprotokoll implementiert bekommen haben. Diese werden indirekt über die sie repräsentierenden Aeins-Funktionen referenziert. Damit bestimmt AMIC die Freigabe.

| Funktion | | Freigabe durch AMIC |
| --- | --- | --- |
| **archivbelegfluss** | **Belegfluss** | **Ja** |
| MENU_224_10033_ERE | Eingangsrechnung | Nein |
| MENU_5_9787_Beleg | Fibubeleg | Nein |

Diese befinden sich in der ausgelieferten Tabelle „Archivprofileditor“.
