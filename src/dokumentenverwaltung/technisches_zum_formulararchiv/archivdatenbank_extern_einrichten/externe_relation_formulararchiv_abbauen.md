# Externe Relation Formulararchiv abbauen

<!-- source: https://amic.de/hilfe/_externerelationformu.htm -->

Bei sehr großen Formulararchiv-Relationen haben wir angeraten, die Relation Formulararchiv samt Massendaten extern zu halten. Weiterentwicklungen in A.eins führen jedoch dazu, dass diese Maßnahme nicht länger nötig ist, ja sie sogar unbedingt rückgängig gemacht werden sollte.

Waren „früher“ noch alle Daten des Archivs in der einen Relation Formulararchiv konzentriert, so ist der Stand heute, dass die binären Dokumente des Formulararchivs – und das ist der weitaus größte Datenanteil – sich in der dafür extra geschaffenen Relation Archiv befinden. Rahmen- und Steuerdaten befinden sich nun in der Relation Formulararchiv. Dieses Vorgehen hat enorme positive Auswirkungen auf die Geschwindigkeit, mit der das Basissystem Recherchen anstellen kann. Weiterhin sind so Nachforschungen möglich, die mit einer einzelnen externen Relation Formulararchiv-Relation nicht machbar wären.

Somit steht man vor der Aufgabe ggf. eine externe Relation Formulararchiv in eine interne Relation zu überführen.

Bitte überprüfen Sie zunächst ob die Relation formulararchiv auch extern ist.

Kommando hierfür:

```sql
select
remote_location from sys.systable where table_name = 'formulararchiv'
```

Existiert ein nichtleere remote_location, ist die Relation Formulararchiv extern und es sind weitere Schritte nötig.

Wir werden nun eine leere Relation formuararchiv_neu anlegen (Schritt1), die Daten von formulararchiv in diese Relation transferieren (Schritt2) und zum Schluß die Relation formulararchiv droppen und die Relation formulararchiv_neu in die relation formulararchiv umbenennen. (Schritt3)

Restaurieren der Indizes (Schritt4)

Als letzte Maßnahme wird eine Reorganisation der Datenbank empfohlen.

*Bevor Sie weitermachen kommen Sie bitte Ihrer Sorgfaltspflicht nach und überzeugen sich, dass sie eine lauffähige Sicherung der beteiligten Datenbanken haben, um im Bedarfsfalle möglicherweise auftretende Problemfälle notfalls dadurch rückgängig machen zu können, dass Sie die Sicherung einspielen können.*

**Schritt1:**

Direktsprung **[OSQL]**, ***Sql-Statement*** **F5**, Formulararchiv eingeben und structure anklicken. Es geht dann ein Notepad mit dem Anlagestatement auf.

Kopieren Sie aus dieser Datei sinngemäß

```sql
create table
admin.FormularArchiv (
FA_Id integer NOT NULL,
fa_MndNr integer NOT NULL default 0,
// hier aus Übersichtsgründen gelöscht
fa_verpostung_vorgesehen integer default 0
, primary key( FA_Id
,fa_MndNr  )  );
```

und ändern sie formulararchiv in formulararchiv_neu um: Also aus „create table admin.Formulararchiv“ wird „create table admin.Formulararchiv_Neu”.

Das so veränderte Statement kopieren Sie bitte ins OSQL und führen es dort aus.

Schritt2: Die Daten von formulararchiv nach Formularachiv_Neu transferieren.

```sql
insert into
formulararchiv_neu with auto name select * from formulararchiv
```

Vergleichen Sie hiernach die Anzahlen in den jeweiligen Relationen:

```sql
Select
count(*) from formulararchiv
```

```sql
Select
count(*) from formulararchiv_neu
```

Schritt3: formulararchiv droppen und formulararchiv_neu umbenennen

```sql
Drop table
formulararchiv
```

```sql
Alter table
formulararchiv_neu rename formulararchiv
```

Schritt4: Restaurieren der Indizes.

```sql
create  index
FormularArchiv_Kundennummer on FormularArchiv ( FA_Kundennummer ASC );
create  index FormularArchiv_Druckdatum on
FormularArchiv ( FA_Druckdatum ASC );
create  index FormularArchiv_MD5 on FormularArchiv
( FA_MD5 ASC );
create  index FormularArchiv_belegreferenz on
FormularArchiv ( FA_BelegReferenz ASC );
create  index ixc_Profile_Vorschlag2 on
FormularArchiv ( FA_BelegKlasse ASC,FA_NeuanlageBediener ASC );
create  index ixc_ProfileVorschlag_3 on
FormularArchiv ( fa_adressid ASC );
create  index fa_guid_index on FormularArchiv (
fa_guid ASC );
create  index fa_vorgrecherche on FormularArchiv (
fa_v_id ASC,fa_formularid ASC,fa_formularz ASC );
create  index fa_proginternindex on FormularArchiv
( fa_progintern ASC );
create  index I_formulararchiv_fa_v_id on
FormularArchiv ( fa_v_id ASC );
create  index
ix_barcode on FormularArchiv ( fa_barcode ASC );
```
