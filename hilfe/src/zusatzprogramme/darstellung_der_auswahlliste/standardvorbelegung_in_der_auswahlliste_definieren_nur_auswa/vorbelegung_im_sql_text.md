# Vorbelegung im SQL-Text

<!-- source: https://amic.de/hilfe/vorbelegungimsqltext.htm -->

Um die Vorbelegung zu definieren verwendet man das Schlüsselwort DEFAULT. Hinter diesem Schlüsselwort folgt dann der Feldname den man vorbelegen will und durch ein Gleichheitszeichen „=“ getrennt der Wert. Bei diesem Wert kann es sich auch um einen Select-Befehl handeln.

DEFAULT fa_info_autor=“select bedienername from bedienerstamm where bedienerid=db_bedienerid“, fa_belegklasse=1400

Mehr Informationen dazu unter „[Default im Gestaltungsdialog](./vorbelegung_im_gestaltungsdialog.md)“
