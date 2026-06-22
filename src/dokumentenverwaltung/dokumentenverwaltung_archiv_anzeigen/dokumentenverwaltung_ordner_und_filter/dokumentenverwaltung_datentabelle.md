# Dokumentenverwaltung- Datentabelle

<!-- source: https://amic.de/hilfe/dokumentenverwaltungdatentabel.htm -->

Aufbau der Datentabelle wird im Standard durch die Variante AW_FA_VIEW.VIEWDIALOG (\*) vorgegeben. Durch das Konzept der Ableitung besteht die einfache Möglichkeit Privatisierungen vorzunehmen.

Die Datentabelle unterstützt den Import von Mails aus den aktuellen Desktop-Outlook-Versionen per „Drag&Drop“.

Die Datentabelle unterstützt den Import von Dateien aus dem Windows-Explorer; soweit es sich um Dateien handelt die den Mime-Anforderungen von A.eins genügen.

(\*)Variante AW_FA_VIEW.VIEWDIALOG

```xml
<?xml version="1.0"
encoding="utf-8" standalone="yes"?>
 <Description Name="AW_FA_VIEW.VIEWDIALOG"
RowHeight="22" Version="">
  <Field Name="fa.fa_mime" Caption=" "
Mime="true" WidthDisplay="22" />
  <Field Name="fa.fa_kundennummer"
Caption="Kundennummer" />
  <Field Name="fa.fa_klasse"
Caption="Klassifizierung" Format="af_fa_klasse" Sql="isnull(fa.fa_klasse,0)"
/>
  <Field Name="fa.fa_belegtyptext"
Caption="Belegtyp" />
  <Field Name="fa.fa_belegnummer"
Caption="Belegnummer" />
  <Field Name="fa.fa_belegdatum"
Caption="Belegdatum" />
  <Field Name="fa.fa_druckdatum"
Caption="Archiv/Druckdatum" />
  <Field Name="fa.fa_belegreferenz"
Caption="Belegreferenz" />
  <Field Name="fa.fa_neuanlagebediener"
Caption="Anleger" />
  <Field Name="fa.fa_info_kommentar"
Caption="Kommentar" />
  <Field Name="fa.fa_info_titel"
Caption="Titel" />
  <Field Name="fa.fa_belegklasse"
Caption="Belegklasse" Format="faklasse" />
  <Field Name="fa.fa_herkunft"
Caption="Herkunft" Format="faherkunft" />
  <Field Name="fa.fa_info_betreff"
Caption="Betreff" />
  <Field Name="fa.fa_info_autor"
Caption="Autor" />
  <Field Name="fa.fa_dateiname"
Caption="Dateiname" />
  <Field Name="Versand" Caption="Versand"
Sql="(Select list(fv.Empfaenger||' '||fv.datum order by datum desc) from
fa_versandhistorie fv

where fv.fa_id=fa.fa_id and fv.fa_mndnr=fa.fa_mndnr)" />
  <Field Name="fa.fa_barcode" Caption="Barcode"
/>
  <Field Name="fa.fa_formularid"
Caption="Formularid" />
  <Field Name="fa.fa_id" Caption="FA-Id"
/>
  <Field Name="fa.fa_guid" Caption="FA Guid"
Visible="false" />
  <Field Name="fa.fa_mndnr" Caption="MndNr"
Visible="false" />
  <Field Name="fa.fa_mandant" Caption="Mandant"
Visible="false" />
  <Field Name="fa.fa_bedienerklasse"
Caption="Bedienerklasse" />
 <!-- With-Statement -->
  <With/>
  <FieldSql/>
  <!-- Limitation Statement -->
  <Limitation></Limitation>
  <!-- From-Statement -->
  <From>
    from formulararchiv fa
  </From>
  <!-- Join Statement -->
  <Join></Join>
  <!-- Where Statement -->
  <Where/>
  <!-- group by-Statement -->
  <GroupBy/>
  <!-- order by-Statement -->
  <OrderBy>order by fa.FA_Druckdatum
desc</OrderBy>
</Description>
```
