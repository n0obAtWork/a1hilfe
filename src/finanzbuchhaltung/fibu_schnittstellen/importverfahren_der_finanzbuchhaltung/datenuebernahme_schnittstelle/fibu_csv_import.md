# FIBU-CSV-Import

<!-- source: https://amic.de/hilfe/fibucsvimport.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Datenübernahme

Direktsprung **[DUEB]**

Um für CSV-Dateien einen Datenimport in die Finanzbuchhaltung zu definieren, kann eine Datenübernahme definiert werden. Es finden hier keine Tests statt, ob eine Datei bereits eingelesen wurde. Dies muss durch geeignete betriebliche Maßnahmen sichergestellt werden.

Um die CSV-Verarbeitung zu aktivieren, müssen im Block „Prozeduren“ folgende Werte eingetragen werden:

![Ein Bild, das Text, Screenshot, Reihe, Schrift enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_786.png "Ein Bild, das Text, Screenshot, Reihe, Schrift enthält. Automatisch generierte Beschreibung")

Steht man im Feld „Programm 1“, so können mit F3 auch die hier einzutragenden Werte ausgewählt werden. „Programm 1“ und „Programm 2“ sind von AMIC bereitgestellte Funktionen, die die Verarbeitung steuern.

Das Programm „dueb_import2db_csv“ importiert die Dateien in das Formulararchiv. Dort sind sie unter dem Belegtypen „Fibu-Datenübername CSV“ wiederzufinden. Gleichzeitig wird in der Tabelle „dueb_import“ die Fa_id und die Fa_mndnr des Archiveintrags vermerkt.

Das Programm „dueb_import_csv“ versucht mithilfe der unter Prozedur eingetragenen privaten Datenbankprozedur die Daten zu verarbeiten. Dabei werden vor der Belegerstellung erst alle Daten auf Konsistenz geprüft und erst danach werden die Belege erstellt. Dadurch ist sichergestellt, dass nicht nur Teile aus der Datei verarbeitet werden.

In das Feld Prozedur muss eine private Prozedur eingetragen werden, die ein Resultset mit den erforderlichen Daten zurückliefern muss. Mit der Funktion ***Prozedur bearbeiten*** kann die Funktion direkt bearbeitet oder neu angelegt werden. Bei der Neuanlage wird ein Gerüst mit dem benötigten Resultset vorgegeben.

Beispieldaten:

```text
ident;telefon;mail;klasse;referenznr;beledatum;hauptkonto;gegenkonto;betrag;sh
1;0431 99020;Support@amic.de;ZA;ABC;20.09.2023;1010;10111;500;S
1;0431 99020;Support@amic.de;ZA;ABC;20.09.2023;1010;10123;200;S
1;0431 99020;Support@amic.de;ZA;ABC;20.09.2023;1010;10000;300;H
```

Beispiel Prozedur zum Einlesen der Beispieldaten. Zu bearbeiten ist in den meisten Fällen nur der individuelle Teil. In dem vorgegebenen Gerüst sind die Stellen mit TODO gekennzeichnet:

```sql
create procedure p_fibu_csv_import(in in_fa_id integer, in_fa_mndnr integer)
  result(
          uebertragungsnummer  char(60),
          uebertragungskennung char(30),
          erstelltam           date,
          erstelltvon          char(20),
          ident                integer,
          poszaehler           integer,
          fibuv_klasse         integer,
          fibuv_herktyp        smallint,
          fibuv_fremdnr        char(255),
          numkreisnummer       integer,
          fibuv_numnummer      integer,
          fibuv_nummer         char(20),
          fibuv_datum          date,
          fibuv_eingangsdatum  date,
          jahrnummer           smallint,
          perinummer           integer,
          hauptkonto           integer,
          hauptkoststel        integer,
          hauptkstr            integer,
          fibuvp_haupttext     char(100),
          FiBuV_PaginierNr     char(255),
          mahndatum            date,
          mahnstufe            integer,
          gegenkonto           integer,
          koststelnummer       integer,
          kstrnummer           integer,
          fibuvp_betrag        numeric(15,4),
          fibuvp_sollhaben     integer,
          fibuvp_valdatum      date,
          zahlbednummer        integer,
          fibuvp_skodatum      date,
          fibuvp_skosatz       numeric(15,4),
          fibuvp_skobetrag     numeric(15,4),
          steuerklasse         integer,
          steuergrnummer       integer,
          steuerschluessel     integer,
          steuerabdatum        date,
          fibuvp_steuwert      numeric(15,4),
          fibuvp_steusatz      numeric(15,4),
          FiBuVP_Text          char(100),
          SteuerGruppeTest     integer,
          SteuerGruppeAusKu    integer
        )
begin
  declare dc_err_notfound exception for sqlstate value '02000';
  declare dc_data long varchar;
  declare local temporary table tempImport
  (
    ident                integer,
    poszaehler           integer,
    fibuv_klasse         integer,
    fibuv_herktyp        smallint,
    fibuv_fremdnr        char(255),
    numkreisnummer       integer,
    fibuv_numnummer      integer,
    fibuv_nummer         char(20),
    fibuv_datum          date,
    fibuv_eingangsdatum  date,
    jahrnummer           smallint,
    perinummer           integer,
    hauptkonto           integer,
    hauptkoststel        integer,
    hauptkstr            integer,
    fibuvp_haupttext     char(100),
    FiBuV_PaginierNr     char(255),
    mahndatum            date,
    mahnstufe            integer,
    gegenkonto           integer,
    koststelnummer       integer,
    kstrnummer           integer,
    fibuvp_betrag        numeric(15,4),
    fibuvp_sollhaben     integer,
    fibuvp_valdatum      date,
    zahlbednummer        integer,
    fibuvp_skodatum       date,
    fibuvp_skosatz       numeric(15,4),
    fibuvp_skobetrag     numeric(15,4),
    steuerklasse         integer,
    steuergrnummer       integer,
    steuerschluessel     integer,
    steuerabdatum        date,
    fibuvp_steuwert      numeric(15,4),
    fibuvp_steusatz      numeric(15,4),
    FiBuVP_Text          char(100),
    primary key (ident, poszaehler)
  );
  set dc_data = (select AMICBLOB from amic_fa_get_from_key(in_fa_id, null, in_fa_mndNr));
--Hier beginnt der Individuelle Teil:
  insert into tempImport
  (
    ident,
    poszaehler,
    fibuv_klasse,
    fibuv_fremdnr,
    fibuv_datum,
    hauptkonto,
    gegenkonto,
    fibuvp_betrag,
    fibuvp_sollhaben
  )
  SELECT ident,
         number(*),
         (select fibuv_klasse from fibuvorgklasse where fibuv_klkurzbez=klKurz),
         fibuv_fremdnr,
         fibuv_datum,
         hauptkonto,
         gegenkonto,
         cast(trim(betrag)as numeric(15,4)),
         (if sollhaben='S' then 1 else 2 endif)
  from openstring(value dc_data)
  WITH ( ident              integer,
         filler(),                           -- Nicht benötigte spalte „telefon“ ignorieren
         filler(),                           -- Nicht benötigte spalte „mail“ ignorieren
         klKurz             char(20),
         fibuv_fremdnr      char(255),
         fibuv_datum        date,
         hauptkonto         integer,
         gegenkonto         integer,
         betrag             char(20),
         sollhaben          char(1)
  )
  option (delimited by ';' QUOTE '"' skip 1)           -– delimited by = Trennzeichen festlegen,
                                                       -– Quote        = Textbegrenzer festlegen
                                                       -– Skip         = Zeilen überspringen
  as daten;
-- Hier endet der Individuelle Teil:
  select
    in_fa_id as uebertragungsnummer,
    in_fa_mndnr as uebertragungskennung,
    today(*),
    USER,
    ident,
    poszaehler,
    fibuv_klasse,
    fibuv_herktyp,                                     --Ist der Herkunftstyp nicht angegeben, wird 30 für Import eingetragen. Eigene Herkunftstypen sollten ab 100 gewählt werden.
    fibuv_fremdnr,
    numkreisnummer,
    fibuv_numnummer,
    fibuv_nummer,
    fibuv_datum,
    fibuv_eingangsdatum,
    jahrnummer,
    perinummer,
    hauptkonto,
    hauptkoststel,
    hauptkstr,
    fibuvp_haupttext,
    FiBuV_PaginierNr,
    mahndatum,
    mahnstufe,
    gegenkonto,
    koststelnummer,
    kstrnummer,
    fibuvp_betrag,
    fibuvp_sollhaben,
    fibuvp_valdatum,
    zahlbednummer,
    fibuvp_skodatum,
    fibuvp_skosatz,
    fibuvp_skobetrag,
    steuerklasse,
    steuergrnummer,
    steuerschluessel,
    steuerabdatum,
    fibuvp_steuwert,
    fibuvp_steusatz,
    FiBuVP_Text,
     0 as SteuerGruppeTest,
     0 as SteuerGruppeAusKu
    from tempImport
    order by ident, poszaehler;
end
```

Der eigentliche Import läuft dann über die Standardschnittstelle. Die Bedeutung und Besonderheiten der Spalten ist unter „[Resultset der FIBU-Datenübernahme](./resultset_der_fibu_datenuebernahme.md)“ beschrieben.
