# Archiv-Ansicht Standard-Auslieferung: Kunden, Vorgang

<!-- source: https://amic.de/hilfe/archivansichtstandardausliefer.htm -->

Die Standard-Auslieferungen sind so konstruiert, dass sie auch noch auf sehr großen Archiv-Beständen akzeptable Zugriffszeiten liefern.

**Kundenauswahlliste: AMIC_KUNDE**

| Var | Herkunft | Wert |
| --- | --- | --- |
| ZW1 | AL Return | KundNummer |
| KUNDNUMMER | AL Return | KundNummer |
| Freies UND | Konstante | fa.fa_kundennummer = ':!jvars_5001_ZW1' |

**Kundendialog: AMIC_KUNDE_DLG**

| Var | Herkunft | Wert |
| --- | --- | --- |
| ZW1 | Maskenfeld | h.KundNummer$ |
| REFERENZ | Maskenfeld | h.Fa_Belegreferenz$ |
| KUNDNUMMER | Maskenfeld | h.KundNummer$ |
| Freies UND | Konstante | fa.fa_kundennummer = ':!jvars_5001_ZW1' |

**Vorgangsauswahllisten: AMIC_VORGAWL**

| Var | Herkunft | Wert |
| --- | --- | --- |
| ZW1 | AL Return | v_id |
| REFERENZ | SQL | select fa_belegreferenz wert from vorgangstamm where v_id = :ZW1 |
| KUNDNUMMER | SQL | select KUNDNUMMER wert from vorgangstamm where v_id = :ZW1 |
| Freies UND | Konstante | fa.fa_belegreferenz &lt;> '' and fa.fa_belegreferenz in (select fa_belegreferenz from vorgangstamm where v_id=:!jvars_5001_ZW1<br>union select ktr.fa_belegreferenz from v_posikontrakt vpktr join kontraktstamm ktr on ktr.ktrid = vpktr.ktrid where vpktr.v_id=:!jvars_5001_ZW1<br>union select p.fa_belegreferenz from v_posipartie vpp join partiestamm p on p.partieid = vpp.partieid where vpp.v_id=:!jvars_5001_ZW1<br>) |

Signifikante Geschwindigkeitsvorteile konnten mit folgender Ableitung festgestellt werden:

```xml
<?xml version="1.0"
encoding="utf-8"?>
<Description Name="AMIC_LGU" RowHeight="22"
Version="8.1.1.256">
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
fa_versandhistorie fv where fv.fa_id=fa.fa_id and fv.fa_mndnr=fa.fa_mndnr)"
/>
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
  <With>
    with x(fa_belegreferenz) as
    (
      select fa_belegreferenz
from vorgangstamm where v_id=:!jvars_5001_ZW1 and isnull(fa_belegreferenz,'') !=
''
      union
      select
ktr.fa_belegreferenz from v_posikontrakt vpktr join kontraktstamm ktr on
ktr.ktrid = vpktr.ktrid

where vpktr.v_id=:!jvars_5001_ZW1 and isnull(ktr.fa_belegreferenz,'') != ''
      union
      select
p.fa_belegreferenz from v_posipartie vpp join partiestamm p on p.partieid =
vpp.partieid
      where
vpp.v_id=:!jvars_5001_ZW1 and isnull(p.fa_belegreferenz,'') != ''
    )
  </With>
  <!-- Limitation Statement -->
  <Limitation></Limitation>
  <!-- From-Statement -->
  <From>
    from formulararchiv fa
  </From>
  <!-- Join Statement -->
  <Join>
    join x on (x.fa_belegreferenz =
fa.fa_belegreferenz)
  </Join>
  <!-- Where Statement -->
  <Where>where (1=1)  and isnull(
fa.fa_progintern , 0 ) in ( -1 , 0 )
  and ((select fab_wer from formulararchivbediener
where  fab_wer=:!jvars_3561_jvar_system_status_bedienerklasse and
fab_darf=fa.fa_bedienerklasse) is not null) or ( 1 = 0 )</Where>
  <!-- group by-Statement -->
  <GroupBy />
  <!-- order by-Statement -->
  <OrderBy>order by fa.FA_Druckdatum
desc</OrderBy>
</Description>
```

**Vorgangsdialoge: AMIC_SV**

| Var | Herkunft | Wert |
| --- | --- | --- |
| REFERENZ | SVMAIN | ID_FA_BELEGREFERENZ |
| KUNDNUMMER | SQL | ID_KUNDNUMMER |

Man erkennt ganz deutlich, dass somit möglicherweise im Vorgangsdialog andere Belege als in der dazugehörigen Auswahlliste recherchiert werden.
