# Mehrmandant Transfer

<!-- source: https://amic.de/hilfe/mehrmandanttransfer.htm -->

### Zentralmandant

In der zweiten Variante “Mehrmandant Transfer“ werden alle Datenbanktabellen angezeigt die von dem Mehrmandanten System unterstützt werden. An dieser Stelle kann Einfluss auf die zu Exportierenden Daten genommen werden. Durch die Erstellung einer Privaten View kann Einfluss darauf genommen werden, welche Daten an den Untermandant weitergegeben werden soll. Durch das Auswählen der Funktion mms_transfer_stop wird die Relation aus der Export Bedingung entfernt.

### Achtung:

***Beim Erstellen einer Privaten View ist auf jedem Fall darauf zu achten, dass nur die Daten der Hauptrelation zurückgegeben werden. Die Privaten Views haben eine vom System her festgeschriebene Namensgebung.***

***Diese Namensgebung lautet.: „p_mmsxml_view_“Relationsname.***

***Für die Relation Artikel z.B.: „p_mmsxml_view_Artikel“***

### Untermandant

Auf der Untermandant Seite stehen nur Prozeduren zur Verfügung. Sollen übermittelte Daten nicht in eine Relation eingespielt werden, so wählen Sie bitte die Funktion mms_transfer_stop aus.

### Auswählen einer Privaten Prozedur oder View

Um einer Relation eine Prozedur oder View hinzuzufügen wählen Sie bitte die Relation in der Auswahlliste aus und drücken dann die Taste F5. Je nach dem auf welchen Mandanten sich kann dort eine Prozedur oder ein View ausgewählt und bearbeitet werden. Mit der Taste F8 kann eine neue Relation hinzugefügt werden ist nur für den Zentralmandanten wichtig. Dies ist für den Fall besonders wichtig, wenn für Bestimmte Untermandanten nur bestimmte Artikel transportiert werden dürfen. Das bedeutet, es kann auf der Senderseite mehrere Views für eine Relation definieren und anlegen werden. Beim Export wird für jedem Untermandaten die dementsprechende View aufgerufen.

### Achtung:

**Wenn ich über F8 eine neue Relation hinzufügen möchte so ist darauf zu achten, dass der Tabellenname mit dem Alias Name für den Untermandaten betitelt wird.**

**z.B.**

***Artikel p_mmsxml_view_Artikel für einen Untermandaten***

***Artikel_UM1 p_mmsxml_view_ArtikelUM1 für den zweiten Untermandaten***

***Artikel_UM1 p_mmsxml_view_ArtikelUM2 für den dritten Untermandaten***

| Maskenfelder | Bedeutung |
| --- | --- |
| Tabellenname | Zu bearbeitende Relation |
| Viewname | Name der Privaten View oder der Prozedur |
| Benutzer | Mitarbeiter die gerade auf der Relation stehen |
| Trigger | Trigger die auf die Relation wirken |
| Nicht Übernehmen | Dieser Haken bewirkt, dass die Relation nicht exportiert oder importiert wird |

### Die Liste der von A.eins bislang unterstützten Relationen und ihrer IDs:

| Relationsname | Abhängige ID/Nummern |
| --- | --- |
| Artikel | ArtikelId |
| Artikelstamm | ArtistammId |
| Artikeladdon | ArtikelId |
| Artikelstammaddon | ArtistammId |
| Artikelmaskedaten | ArtikelId |
| Artikelinfogruppe | ArtiInfoGruppe |
| Artikeltext | ArtistammId |
| Artikeltextblob | ArtistammId |
| Artiauspraeg | ArtistammId |
| Artiauspgebinde | ArtistammId |
| Artibestauspr | ArtistammId |
| Artigefahrklasse | ArtistammId |
| Artiherstell | ArtistammId |
| Artiherstpreis | ArtistammId |
| Artiintrastat | ArtistammId |
| Artikundarnr | ArtistammId |
| Artilieferant | ArtistammId |
| Artistamgebinde | ArtistammId |
| Artizusammensetz | ArtistammId |
| Artiladeartilink | ArtikelId |
| Artigebinde | ArtikelId |
| Artimerkmal | ArtikelId |
| Artiausweich | ArtistammId |
| Artiverpackelement | ArtistammId |
| Artistueckkomp | ArtikelId |
| ArtiFolgeArtikel | ArtistammId |
| ArtiBonusGruppe | ArtiBonusGruppe |
| AbteilArtiGruppe | AbteilGruppe |
| ArtiGruppe | ArtGrNummer |
| AbteilGruplink | AbteilGruppe |
| ArtiBestGruppe | ArtikelId |
| ArtifrachtGruppe | ArtiFraGruppe |
| ArtiladeGruppe | ArtiLadeGruppe |
| ArtirabattGruppe | ArtiRabGruppe |
| ArtizuabGruppe | ArtiZAGrNummer |
| ArtibewGruppe | BewertGrNummer |
| Artilpreismatrix | PreisMatNummer |
| ArtilpreisGruppe | ArtiLisPrGruppe |
| ArtikoststGruppe | ArtiKstGrNummer |
| ArtikstrGruppe | ArtiKSTRGrNummer |
| Artipool | ArtiPoolNummer |
| DSD_MaterGruppe | DSD_MatGrNummer |
| DSD_VolumGruppe | DSD_VolGrNummer |
| Fruchtartmerkmale | saatfrunummer |
| GefahrToxiKlasse | ArtistammId |
| GefahrBrandKlasse | ArtistammId |
| GefahrGutKlasse | ArtistammId |
| InventurGruppe | ArtikelId |
| InventurMengabwGruppe | InvMAbwGrpId |
| MengeinhGruppe | ArtistammId |
| Mengeinheit | ArtistammId |
| PartieGruppe | PartieGrNummer |
| RezepturGruppe | ArtikelId |
| RohwarenGruppe | ArtikelId |
| SaatFruchtArt | saatfrunummer |
| SaatFruchtSorte | ArtikelId |
| SaatFrSortPosi | SaatSorteId |
| SaatFrSortPositAddon | SaatSorteId |
| SaatFruchtSorteAddon | SaatSorteId |
| SaatKategorie | SaatKateNummer |
| SaatAnerkZuOrd | SaatAnZuOId |
| Sekundschluessel | ArtistammId |
| Sinfos1 | ArtistammId |
| Steuerschluessel | SteuerSchluessel |
| VertProvGruppe | ArtikelId |
| VerpackGruppe | VerpackGruppe |
| WarenGuppe | ArtistammId |

### Beispiel für eine Private Prozedur

```sql
create procedure p_Erloeskennziffernichtueberschreiben (
in in_tabelle char(255) default '',
            in in_parentid integer default 0 )
begin
  declare dc_ekz_nummer integer;
  declare dc_ekz_nummer_orginal integer;
  -- Ekznummerneintrag mit übernehmen, wenn nicht vorhanden.
  --
  set dc_ekz_nummer = 0;
  select first wert into dc_ekz_nummer from tabellenstruktur where
     feldname='ekz_nummer' and TabellenName = 'Artikel';
  if ( dc_ekz_nummer = 0 ) then
    select first wert into dc_ekz_nummer from tabellenstruktur where
      feldname='ekz_nummer' and TabellenName = 'ArtikelStamm';
  end if;
  set dc_ekz_nummer_orginal = 0;
  select first ekz_nummer into dc_ekz_nummer_orginal from Erloeskennziffer
     where ekz_nummer = dc_ekz_nummer;
  if ( dc_ekz_nummer_orginal  = 0  ) then
    select 0 as retval;
  else
    select 1 as retval;
  end if;
end
```

| Rückgabewert | Bedeutung |
| --- | --- |
| Rückgabewert 0 | Bedeutet, dass die Daten nicht in das Zielsystem übernommen werden. |
| Rückgabewert 1 | Bedeutet, dass die Daten in das Zielsystem übernommen werden |

Soll ein Datensatz komplett gelöscht werden, bevor dieser in das Ziel System eingespielt wird, so kann dieser in der Temporären Relation Tabellenstruktur eindeutig über den Parameter in_parentid identifiziert werden.

```sql
delete from tabellenstruktur where parentid = in_parentid;
```

### Beispiel für eine Private View

```sql
create view  p_mmsxml_view_ warengruppe as
     select warengruppe.*,ars.artistammid as view_artistammid
     from warengruppe warengruppe
     join artikelstamm ars  on warengruppe.wagrunummer = ars.wagrunummer
```

### Achtung:

**Der Aliasname der Relation muss immer so heißen, wie die Relation selbst. Es ist auch darauf zu achten, dass der Aliasname klein geschrieben wird. Des Weiteren ist darauf zu achten, das der Aliasname des Artikelstamm immer “ars“ heißt und der Aliasname des Artikels “ar“. Die Artikelid oder die Artistammid werden immer als view_ArtiStammid oder view_ArtikelId mitgegeben. Diese werden später wieder ausgefiltert.**

### Aufruf aus dem „ArtikelExportXML“

```sql
select warengruppe.* from p_mmsxml_view_warengruppe warengruppe where
   view_artistammid  = dc_artikelstammid for xml auto, elements
```
