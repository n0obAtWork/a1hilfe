# Archiv-Ansichten-Variante: Profile

<!-- source: https://amic.de/hilfe/_archivansichtprofil.htm -->

Dokumentenverwaltung > Dokumentenverwaltung > Ansichten > Archiv – Profile

Direktsprung **[FAA]**

Dokumentenverwaltung > Dokumentenverwaltung > Einrichtung Archiv-Profile > Archiv-Profile

Direktsprung **[ARPRO]**

Hier werden die Archiv-Profile der Dokumentenverwaltung gepflegt. Die Archiv-Profile steuern zunächst vornehmlich im Belegfluss Filter, Funktionen, Direktsprung und weitere Parameter.

| Felder | |
| --- | --- |
| Profilname | Ansichtsprofil-Identifikation  
Dieser Profilname wird an den betreffenden Stellen auf Masken und Funktionsbezeichnungen dargestellt. |
| **Funktion** | Funktions-Identifikation (siehe Erläuterungen zur Funktion und Optionbox) |
| **Optionbox** | Optionbox-Identifikation (siehe Erläuterungen zur Funktion und Optionbox) |
| | **Erläuterungen zur Funktion und Optionbox:**  
Funktion und Optionbox bestimmen den Kontext der durch das Profil aufgerufenen Aeins-Funktionalität, geben als auch die Berechtigungsrolle vor.  
Die auslösende Funktion in der Dokumenten-Verwaltung ist ![view\_refresh](../../../ImagesExt/image8_935.png "view_refresh"). Ihre „Sichtbarkeit“ wird durch die üblichen Rollen-Einstellungen gesteuert.  
(der betreffende Kontext ist ah_archivbelegfluss/OB_ARCHIV.VIEWDIALOG)  
   
Der Rollen-Kontext dieser **Funktion** und der angegebenen **Optionbox** bestimmt allerdings ob der Archiv-Editor – also die eigentliche Belegfluss-Aktivität - ausgeführt werden darf. (Somit ist es möglich die Daten des Belegflusses einerseits einzusehen, aber rollentechnisch zu verhindern, dass der Archiv-Editor auf diesen Daten aufgerufen werden kann) |
| Sql … | Der Sql-Schnipsel der an das Archiv-Sql angehängt wird der die Ermittlung der Daten durchführt.  
Vorteil der Anzeige des Sql-Schnipsels:  
Die Anzeige des Sql-Schnipsels ermöglicht die Suche nach bestimmten Kriterien.  
Nachteil der Anzeige des Sql-Schnipsels:  
1) Der tatsächlich eingerichtete Sql-Schnipsel kann durchaus länger als die maximalmögliche Anzahl von 255 Zeichen die in der Auswahlliste angezeigt werden sein.  
2) Es existieren noch weitere Sql-Schnipsel für Join, with, etc. pp. Diese werden nicht angezeigt und sich auch so nicht „durchsuchbar“. |

| Suchen | |
| --- | --- |
| Suchen … | Sucht in den Feldern Profilname, Funktion, Optionbox und Sql |

| Funktionen | |
| --- | --- |
| **Stammdaten pflegen**  
**F5, F6, F7 und F8** | Stammdatenpflege über Archiv-Profile |
| **Menüfunktion anlegen** | Es wird im Aeins-Hauptmenü unter Dokumentenverwaltung > Belegfluss eine private Aeins-Funktion angelegt, die die Archiv-Anwendung „Belegfluss“ mit dem selektierten Profil aufruft.  
Als Bezeichnung der Funktion werden die ersten 40 Zeichen des Profilnamens genommen.  
Als eindeutige Identifikation der Funktion wird zu pf_XXX festgelegt, wobei XXX die Guid-Repräsentation des Schattenschlüssels aus der Relation ArchivProfil (Spalte ProfilGuid) ist.  
   
Der ControlString wird (zum Beispiel) zu  
^CS ArchivProfilAnzeige 64c6c56853d34ffebbfc6746904bbc6d  
   
Wird das Profil gelöscht, so wird die private Funktion gelöscht.  
   
Eine bestehende Menü-Funktion wird aktualisiert.  
   
*Hinweis: Die meisten Änderungen sind nur nach einem erneuten Menü-Aufbau – also Neustart des Aeins-Clienten – sichtbar.* |
| **Menüfunktion löschen** | Löscht die über „Menüfunktion anlegen“ generierte Funktion. |
| **Archivprofil ausführen** | Startet das Archivprofil direkt aus der Auswahlliste heraus.  
Bei Einrichtungen muss also nicht erst der Umweg über das hauptmenü getätigt werden. |

<p class="siehe-auch">Siehe auch:</p>

- [Archiv-Profile](./archiv_profile.md)
- [Ableitung (XML-Beschreibung)](./ableitung_xml_beschreibung.md)
