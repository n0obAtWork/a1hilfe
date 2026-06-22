# Partieumbuchung

<!-- source: https://amic.de/hilfe/_partieerweiterungpartieumbuchung.htm -->

Hauptmenü > Partieverwaltung > Partie-Stammdaten > Partiegruppen

oder Direktsprung [PAR]

Allgemein

In der Variante „Partiebestand Details“ kann die Partieumbuchung durchgeführt werden. Dabei wird eine Partie in der Auswahlliste ausgewählt und mit der Funktion „Partie Umbuchung“ öffnet sich dann die Maske für die Partieumbuchung.

Maskenfelder

Die meisten Maskenfelder zeigen nur die Partieinformationen der Original Partie an.

Folgende Felder sind veränderbar:

| Maskenfelder | Bedeutung |
| --- | --- |
| Partiebezeichnung | In diesem Feld wird die neue Partiebezeichnung eingetragen. Wird das Feld nicht geändert und er Einrichterparameter „“ steht auf „Nein“, so kann keine neue Partie mit der gleichen Bezeichnung angelegt werden. |
| Menge | In das Feld wird die Menge eingetragen, die Umgebucht werden soll. |

Ablauf

**Neue Partie mit gleicher Partiebezeichnung**

Um eine neue Partie mit gleicher Bezeichnung anzulegen, muss nur die Menge in das Mengenfeld Eingetragen werden. Es darauf zu achten, dass der Einrichterparameter „“ auf „Ja“ steht.

**Neue Partie anlegen mit ungleicher Partiebezeichnung**

Um eine Partie mit neuer Partiebezeichnung anzulegen, wird einfach eine neue Partiebezeichnung eingetragen.

**Folgende Felder werden übernommen**.

Der Partiematchcode, die Belegreferenz, der Gültigkeitszeitraum und die Mengeneinheitsnummer werden mit in die neue Partie übernommen. Des Weiteren wird der das PartieStammAddon mit kopiert.

Als erstes wird eine neue Partie angelegt mit den Eingegebenen und den vorgegebenen Parametern. Danach wird eine Artikelumbuchung erstellt.

Es ist aber auch Möglich eine eigene Partieumbuchung(Artikelumbuchung) im Makro zu Implementieren. Dem Makro werden drei Parameter übergeben.

1. Die Vorgangsunterklasse

2. Die Partieid der neuen Partie

3. Die Partienummer der neuen Partie

Addon

Um AIS auf der Maske anzuzeigen gibt es zwei Felder für die Partieid auf der Maske

1. PartieIDORIG

2. n.PartieId$

Makrobeispiel

In diesem Beispiel wird eine Artikelumbuchung auf den gleichen Artikel mit unterschiedlicher Partie durchgeführt. Über die Funktion

```text
getldb("PARTIENUMMER$",par_abg);
```

wird der Wert aus dem Feld mit der Partienummer in die Variable par_abg gespeichert.

```text
PROGRAM TM_ARUPARUUMBU(
v_unterklasse; partieid; partienummer );
  const
  BUFLEN = 256;
  ID_LAGERNUMMER_ABG  = 1801;
  ID_LAGERNUMMER_ZUG  = 1804;

  ID_LAGERPLATZ_ABG   = 1802;
  ID_LAGERPLATZ_ZUG   = 1805;

  ID_PARTIENUMMER_ABG = 1910;
  ID_PARTIENUMMER_ZUG = 1911;

  var
  buf,
  lg_abg,
  lg_zug,
  art_abg,
  art_zug,
  lgpl_abg,
  lgpl_zug,
  par_abg,
  par_zug,
  menge,
  me_num,
  partiebezeich   : string;

  v,
  p,
  ok,
  lg_abg_i,

lg_zug_i        : integer;

menge_r         : real;

procedure init();
begin
  waitcursor(true);
  lg_abg   := alloc(128);
  lg_zug   := alloc(128);
  art_abg  := alloc(128);
  art_zug  := alloc(128);
  lgpl_abg := alloc(128);
  lgpl_zug := alloc(128);
  par_abg  := alloc(128);
  par_zug  := alloc(128);
  me_num   := alloc(128);
  partiebezeich := alloc(1000);
  menge    := alloc(128);
  buf      :=
alloc(BUFLEN);

end;

procedure aufraeumen();
begin
  free(buf);
  free(lg_abg);
  free(lg_zug);
  free(art_abg);
  free(art_zug);
  free(lgpl_abg);
  free(lgpl_zug);
  free(par_abg);
  free(par_zug);
  free(partieid);
  free(me_num);
  free(menge);
  free ( partiebezeich );
  waitcursor(false);
end;

begin
  init();
  waitcursor(true);
  getldb("PARTIENUMMER$",par_abg);
  strcpy(par_zug,par_abg);
  getldb("ARTIKELNUMMER$",art_abg);
  strcpy(art_zug,art_abg);
  getldb("QUELLLG$",lg_abg);
  strcpy(lg_zug,lg_abg);
  lg_abg_i:=strtoint(lg_abg);
  lg_zug_i:=strtoint(lg_zug);
  getldb("QUELLLGP$",lgpl_abg);
  getldb("ZIELLGP$",lgpl_zug);
  getldb("PARTIEBEZEICH$", partiebezeich );
  getldb("MENGE$",menge)

  waitcursor(true);
  menge_r:=strtoreal(menge);
  SetKlassNum ( 5120 );
  SetUKlassNum (v_unterklasse );
  v := StartVorgang( 0, 0 );   // neuer
leerer Vorgang
  if v <> 0 then
  {
    SetValue(v, ID_LAGERNUMMER_ABG,
lg_abg, 0);
    SetValue(v, ID_LAGERNUMMER_ZUG,
lg_zug, 0);
    p := Umbuchung (

v,
// Vorgangsreferenz

art_abg,          // ArtikelNr
Abgang

lg_abg_i,         // LagerNr

art_zug,          // ArtikelNr
Zugang

lg_zug_i,         // LagerNr

menge_r,          // Menge
      0
    );
    if p <> 0 then
    {
      SetValpos ( p ,
ID_LAGERPLATZ_ABG, lgpl_abg, 0 );
      SetValpos ( p ,
ID_LAGERPLATZ_ZUG, lgpl_zug, 0 );
      SetValPos ( p ,
ID_PARTIENUMMER_ABG, par_abg, 0 );
      SetValPos ( p ,
ID_PARTIENUMMER_ZUG, partienummer, 0 );
      PositionAdd(v);
    }
  }
  ok :=
BeendeVorgang(v,1,1);      // zum Schluß
abspeichern
  aufraeumen();
end.
```
