# Die Auswahlliste Formulararchiv-Anzeige

<!-- source: https://amic.de/hilfe/dieauswahllisteformulararchiva.htm -->

Diese Auswahlliste wird nach Auslösen der Aktion ["Archiv anzeigen"](../index.md#aktion_archiv_anzeigen) geöffnet.

<p class="just-emphasize">Archiv-Anzeige ohne Vorschau</p>

Die Felder sind mittels ["Variante"](../../../archiv_import/archiv_dokumenten_import.md#archiv_ansichts_definition_variante) in den [Archiv-Ansicht-Definitionen](../../archiv_ansicht_definition/index.md) gegebenen Möglichkeiten einzurichten.

Die Felder in der Standard-Auslieferung der Archiv-Anzeige ohne Vorschau sind wie folgt:

| **Felder** |
| --- |
| KndNr | Zugeordnete Kundennummer |
| Beleg-Typ | Zugeordneter Textueller Beleg-Typ |
| Beleg-Nr | Zugeordnete Belegnummer |
| Beleg-Datum | Zugeordnetes Beleg-Datum |
| Archiv/Druck-Datum | Zugeordnetes Archivierungsdatum bzw. Druck-Datum |
| Beleg-Referenz | Zugeordnete Archiv-Referenz |
| Mnd | Zugeordneter Mandant |
| Herkunft | Zugeordnete Herkunft |
| Betreff | Zugeordneter Betreff |
| Autor | Zugeordneter Autor |
| Barcode | Zugeordneter Barcode |
| Bedienerklasse | Zugeordnete Bedienerklasse |
| Formularid | Zugeordnete Formularid |
| Fa-Id | Zugewiesene technische Formulararchiv-Id |
| Dateiname | Zugewiesener Dateiname |

| **Funktionen** |
| --- |
| Senden an … | Senden an |
| Archiv anzeigen [**Strg F12**] | Archiv anzeigen |
| Ändern | [Archiv-Stammdatenpfleger](../../../archiv_stammdatenpfleger.md) |
| Ansehen | [Archiv-Stammdatenpfleger](../../../archiv_stammdatenpfleger.md) |
| Hinzufügen | [Archiv – Dokumente hinzufügen](../../../archiv_dokumente_hinzufuegen.md) |
| Barcode zuweisen … | [Archiv Barcode](../../../archiv_barcode.md) |
| Drucken | Es wird ein Druck des Archiv-Inhaltes über das Windows-System eingeleitet.<br> <br>Technische Erläuterung:<br>Dabei wird von A.eins eine temporäre Datei im Temp-Verzeichnis erstellt und diese dem Windows-System zum Drucken über die Methode „print“ übergeben.<br>Über Systemsteuerung > Programme > Standardprogramme können Sie mittels ![](../../../../ImagesExt/image8_908.png) <br>auf Ihrem System nachverfolgen welche Applikation mit der Extension verbunden ist. |
| Ansicht Information | Diese Funktion teilt in einem Dialog mit, welche [Archiv-Ansicht-Definition](../../archiv_ansicht_definition/index.md) zum Aufbau dieser Auswahlliste verwendet wurde. |
| Archiv Eintrag löschen | [Archiveinträge löschen](../../../archiv_administration/index.md) |

<p class="just-emphasize">Neue Archiv-Anzeige mit Vorschau</p>

Ist für die Ansicht der „Vorschau“-Modus aktiviert, dann gestaltet sich die „Archiv-Anzeige“ als Dialog in neuer Optik mit neuen Möglichkeiten:

![](../../../../ImagesExt/image8_909.jpg)

1) Der Dialog ist in der Größe veränderbar und auch insgesamt maximierbar.

2) Im Grid werden die bekannten Archiv-Inhalte dargestellt.

3) Im oberen Bereich befindet sich eine „Multifunktionsleiste“ (Ribboncontrol) das „Office-like“ neben dem Kontext-Menü (rechte Maustaste) ausgewählte Funktionalitäten zur Verfügung stellt. In der jetzigen Auslieferung ist das vorerst die Funktionalität ***Ansicht***.

Über ![](../../../../ImagesExt/image8_910.png) lässt sich die „Multifunktionsliste“ dauerhaft aufklappen, ansonsten muss mit der Maus der Punkt „Ansicht“ selektiert werden. Damit erhält man folgende Ansicht

![](../../../../ImagesExt/image8_911.jpg)

Mit der Funktion ***Unter der Multifunktionsleiste anzeigen*** lässt sich festlegen, wo die „Multifunktionsleisten-Steuerung“ angezeigt werden soll.

4) Im unteren Bereich findet sich die Information, um welche Ansicht es sich handelt (in diesem Beispiel um die Archiv-Ansicht „AMIC_KUNDE“ mit der Ansichts-ID 421. Des Weiteren hat das System 291 Archiv-Einträge komplett geladen.

Mit Hilfe der Funktion ***Aktualisieren*** werden die Daten neu ermittelt und geladen.

5) In den Spalten-Köpfen können Sie Sortierungen vornehmen und via Drag&Drop Spaltenreihenfolgen bestimmen.

![](../../../../ImagesExt/image8_912.jpg)

Hier wurde z.B. aufsteigend nach „Beleg-Datum“ sortiert und die „Beleg-Nr“-Spalte per Maus hinter die „Beleg-Datum“ positioniert.

Außerdem lassen sich die Breiten der Spalten per Maus festlegen. Spaltenreihenfolgen und Spaltenbreiten werden sich übergreifend gemerkt, Sortierungen nicht.

6) Positionieren Sie die Maus z.B. auf die „Beleg-Datum“-Spalte und betätigen Sie das kleine Dreieck ![](../../../../ImagesExt/image8_913.png), dann können Sie eine Auswahl der angezeigten Daten bestimmen:

![](../../../../ImagesExt/image8_914.png)

Mit Hilfe von „Text Filter“ können Sie weitere Spezialisierungen vornehmen.

Diese Eingrenzungsmöglichkeiten sind spaltenübergreifend nutzbar und damit können vielfältigste Recherchen durchgeführt werden.

7) Gibt es eine Spalte „Container-Inhalt“ die den Archiv-Eintrag icon-mäßig visualisiert.

![](../../../../ImagesExt/image8_915.jpg)

8) Hinzufügen via Drop&Drag

Sie können aus Outlook heraus Mails in diesen Dialog ziehen und diese so dem Aeins-Archiv-kontext hinzufügen.

Wenn Sie das getan haben, fragt Aeins, ob Sie noch eine nachträgliche Katalogisierung vornehmen möchten:

![](../../../../ImagesExt/image8_916.png)

Bejahen Sie dies, haben Sie die Gelegenheit, die neu hinzugefügten Archiv-Einträge archiv-technisch nachzubearbeiten:

![](../../../../ImagesExt/image8_917.png)

Diesem Beispiel entnehmen Sie bitte, das das Betreff-Feld schon automatisch mit dem Betreff der Mail initialisiert und gespeichert wurde.

**9) Archiv-Ableitung**

Die Praxis hat gezeigt, dass in einigen Fällen eine volle Kontrolle über das Datengewinnungs-Sql verfügbar sein muss. Mit Hilfe der Funktion ***Ableitung*** **[SF2]** erhält man individuelle Zugriffsmöglichkeiten auf eine gesamte Archiv-Ansicht, also Auslierung und Privatisierungen über alle Bedienerklassen.

Man hat also die Möglichkeit eine private „Ableitung“ zu definieren. Diese Ableitung bleibt über Updates hinaus bestehen.

Sie definieren eine Ableitung über die Pflege eines XML-Dokumentes im Editor.

Ist noch keine Ableitung gespeichert stellt sich das zum Beispiel für „AMIC_KUNDEN“ so dar:

```xml
<?xml version="1.0" encoding="utf-8"?>
<Description Name="AMIC_KUNDE" RowHeight="60" Version="7.8.5.157">
  <Field Name="fa.fa_kundennummer" Caption="Knd.Nr." />
  <Field Name="fa.fa_belegtyptext" Caption="Beleg-Typ" />
  <Field Name="fa.fa_belegnummer" Caption="Beleg-Nr" />
  <Field Name="fa.fa_belegdatum" Caption="Beleg-Datum" />
  <Field Name="fa.fa_druckdatum" Caption="Archiv/Druck-Datum" />
  <Field Name="fa.fa_belegreferenz" Caption="Beleg-Referenz" />
  <Field Name="fa.fa_neuanlagebediener" Caption="Anleger" />
  <Field Name="fa.fa_bedienerklasse" Caption="Bedienerklasse" />
  <Field Name="fa.fa_info_kommentar" Caption="Kommentar" />
  <Field Name="aeinspic.i_image" Caption="Containerinhalt" Bitmap="true" />
  <Field Name="fa.fa_info_titel" Caption="Titel" />
  <Field Name="fa.fa_belegklasse" Caption="Beleg-Klasse" Format="faklasse" />
  <Field Name="fa.fa_herkunft" Caption="Herkunft" Format="faherkunft" />
  <Field Name="fa.fa_info_betreff" Caption="Betreff" />
  <Field Name="fa.fa_info_autor" Caption="Autor" />
  <Field Name="fa.fa_barcode" Caption="Barcode" />
  <Field Name="fa.fa_formularid" Caption="Formularid" />
  <Field Name="fa.fa_id" Caption="FA-Id" />
  <Field Name="fa.fa_guid" Caption="FA Guid" Visible="false" />
  <Field Name="fa.fa_mndnr" Caption="MndNr" Visible="false" />
  <Field Name="fa.fa_mandant" Caption="Mandant" Visible="false" />
  <Field Name="fa.fa_dateiname" Caption="Dateiname" />
  <!-- With-Statement -->
  <With />
  <!-- Limitation Statement -->
  <Limitation></Limitation>
  <!-- From-Statement -->
  <From>
    from formulararchiv fa
    left outer join amic_v_images aeinspic on (aeinspic.i_mime=fa.fa_mime)
  </From>
  <!-- Join Statement -->
  <Join>left outer join amic_fa_fibu(:!jvars_5001_ZW1) aff on ( fa.fa_id=aff.fa_id and fa.fa_mndnr=aff.fa_mndnr)
left outer join amic_fa_crw(:!jvars_5001_ZW1) fagcrw on (fagcrw.fa_id=fa.fa_id and fagcrw.fa_mndnr=fa.fa_mndnr )</Join>
  <!-- Where Statement -->
  <Where>where (1=1)  and isnull( fa.fa_progintern , 0 ) in ( -1 , 0 ) and ( ( ( (isnull(fa.fa_kundennummer,0) = :!jvars_5001_ZW1
or isnull(fa.fa_belegreferenz,'') = ':!jvars_5001_ZW2')
or (aff.fa_id=fa.fa_id and aff.fa_mndnr=fa.fa_mndnr)
or ( fagcrw.fa_id=fa.fa_id and fagcrw.fa_mndnr=fa.fa_mndnr) )  ) and ((select fab_wer from formulararchivbediener where  fab_wer=:!jvars_3561_jvar_system_status_bedienerklasse and fab_darf=fa.fa_bedienerklasse) is not null) or ( 1 = 0 ) )</Where>
  <!-- group by-Statement -->
  <GroupBy />
  <!-- order by-Statement -->
  <OrderBy>order by fa.FA_Druckdatum desc</OrderBy>
</Description>
```

| Description - Attribute | | |
| --- | --- | --- |
| Name | Name der zugrunde liegenden Archiv-Anischt. | Dokumentatorischer Charakter. |
| RowHeight | Bestimmt die Höhe der Archiv-Ansichtszeilen im Grid. | Standard sind 60 Pixel, ohne Grafiken wird 22 empfohlen. |
| Version | Die A.eins-Version bei erstmaliger Erstellung der Ableitung. | Dokumentatorischer Charakter. |

| Description- Nodes |
| --- |
| Field | Name | Name der Sql-Spalte | |
| | Caption | Beschriftung der Sql-Spalte | |
| | Bitmap | true oder false, Standard ist false | Gibt an, ob sich beim Inhalt der Sql-Spalte um eine Grafik handelt. |
| | Format | Name des A.eins-Format | Ähnlich wie bei der A.eins-Auswahlliste und den A.eins-Itemboxen kann man hier eine „textuelle Entsprechung“ des Wertes der Sql-Spalte angegeben. |
| | Visible | true oder false, Standard ist true | Nicht alle Felder will man in jedem Falle auch anzeigen, gleich wohl wird der Wert eines solchen Feldes vielleicht für weitere Verwendungen (Funktionen) benötigt. |
| | Sql | Sql-Statement zur Gewinnung des Wertes der Sql-Spalte | |
| With | | Sql-Statement für ein optionales With | Weitere Erläuterungen entnehmen Sie bitte der Sybase-Dokumentation. |
| Limitation | | Sql-Anweisung für eine optionale Limitierung des Resultsets. | |
| From | | Sql-Anweisung für From | |
| Join | | Optionale Erweiterung des obigen „From“ | Bei der Erst-Initialisierung einer Ableitung belegt A.eins diese mit dem Resultat der Archiv-Ansicht vor! |
| Where | | Where-Klausel des Sql-Statements | Bei der Erst-Initialisierung einer Ableitung belegt A.eins diese mit dem Resultat der Archiv-Ansicht vor!<br>Beachten Sie unbedingt das Sie bei Änderungen hinsichtlich **fa_progintern** und dem Sichtschutz-Konzept über **formulararchivbediener** alleinverantwortlich<br>handeln. |
| GroupBy | | Optionale GroupBy-Klausel des Sql-Statements | |
| OrderBy | | Optionale OrderBy-Klausel des Sql-Statements | |

Sie können eine Ableitung löschen, indem Sie sämtlichen Text im Editor löschen und dann Speichern.

Das System erkennt Änderungen bzw. reagiert nur dann, wenn Sie auch Änderungen durchführen!

Das eine Ableitung „aktiv“ ist erkennen Sie visuell auf der Maske:

![](../../../../ImagesExt/image8_918.png)

Mit Hilfe der Funktion ***Ableitung Export*** können Sie die Ableitung exportieren.

<p class="siehe-auch">Siehe auch:</p>

- [„Archiv ansehen“](./archiv_ansehen.md)
