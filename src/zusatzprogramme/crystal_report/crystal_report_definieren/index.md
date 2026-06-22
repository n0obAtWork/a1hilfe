# Crystal Report definieren

<!-- source: https://amic.de/hilfe/crystalreportdefinieren.htm -->

Hauptmenü > Administration > Werkzeuge > Anwendung Reports

Direktsprung **[ANWR]**.

Das Design und die Eingrenzung der Daten der Reporte werden in A.eins vom Programm gesteuert. Dazu sind einige Informationen notwendig, die über diese Anwendung hinterlegt werden.

| Feld | Bedeutung |
| --- | --- |
| Ident | Die eindeutige Identifikation des Reports. Diese ist vierzig Stellen lang. Bei der Erfassung wird geprüft, ob ein Report mit dieser Ident existiert und ggf. eine Meldung ausgegeben.  
Mit der Funktion aw_list kann der Report dann aufgerufen werden. Man gibt dazu diese Ident als ersten Parameter an.  
Private Reporte müssen mit **PR_** beginnen.  
 |
| Name/Titel | Titel des Reports, wie er in der Auswahlliste erscheint. Dieser Titel wird über die Formel „TITEL“ an den Report übergeben und kann dort z.B. als Überschrift verwendet werden.  
 |
| Aktiver Report | Dies ist nur ein Anzeigefeld. Der unter **Reportdateien** aktivierte Report wird hier angezeigt.  
 |
| NICHT übersetzen | Reporte, die nicht in die Übersetzung mit einfließen sollen ( z.B. technische Reporte ), können hier von der Übersetzung ausgenommen werden. Dazu ein **Ja** eintragen.  
 |

<p class="siehe-auch">Siehe auch:</p>

- [Basisdaten](./basisdaten.md)
- [Reportdateien](./reportdateien.md)
- [Bedingungen](./bedingungen.md)
- [Info](./info.md)
- [CRW-Archivdefinition](./crw_archivdefinition.md)
- [Abweichendes Firmenlogo](./abweichendes_firmenlogo.md)
- [Funktionen zur Reportbearbeitung](./funktionen_zur_reportbearbeitung.md)
