# Partiestamm

<!-- source: https://amic.de/hilfe/_partiestammpar.htm -->

Hauptmenü > Partieverwaltung > Chargen / Partien > Partie-Stammdaten

oder Direktsprung **[PAR]**

In der Variante Partieübersicht ist bei artikel-spezifischen Partien die zugeordnete Lagernummer nicht sichtbar, wenn noch kein Partiebestand gebucht ist. Die zugewiesenen Lager leben in dieser Fassung erst auf, wenn auch Bestände gebucht sind.

Partien können auf Basis Artikelstamm oder auf Basis Artikel/Lager angelegt werden. Der [Steuerparameter 277](../../firmenstamm/steuerparameter/partiewesen/typ_vorbelegung_bei_partieartikel_anlage_spa_277.md) „Typ-Vorbelegung bei Partieartikel-Anlage“ legt fest wie das Feld „Typ der Zuordnung“ vorbelegt werden soll. Zur Auswahl stehen Artikelstamm oder Artikel/Lager. Die Standardeinstellung des Steuerparameters 277 ist Artikelstamm.

Partien werden in der Praxis hauptsächlich während der Belegerfassung selbst angelegt. Dies kann aber auch über die Partiestammdatenverwaltung erfolgen.

Der [Steuerparameter 1084](../../firmenstamm/steuerparameter/partiewesen/artikel_mehrfach_in_partie_erlaubt_spa_1084.md) „Artikel mehrfach in Partie erlaubt“ legt fest, ob im Partiestamm-Pflegemodul ein Artikel oder Artikelstamm mehreren Artikelposition der Partie zugeordnet werden kann (Einstellung: **Ja**).

Bei der Einstellung **Nein** kann ein Artikel oder Artikelstamm nur einer Partieartikelposition zugeordnet werden. Es ist dann auch nicht möglich, einer Position einen Artikel zuzuordnen, dessen Artikelstamm bereits einer anderen Position zugeordnet wurde und umgekehrt.

Die Unterschiede dieser zwei Möglichkeiten werden nachfolgend dargestellt.

<p class="siehe-auch">Siehe auch:</p>

- [Anlegen einer Partie über die Vorgangserfassung](./anlegen_einer_partie_ueber_die_vorgangserfassung.md)
- [Anlegen einer Partie über Partiestammdatenverwaltung](./anlegen_einer_partie_ueber_partiestammdatenverwaltung.md)
- [Partiestammsatz löschen](./partiestammsatz_loeschen.md)
- [Druckerstatus-Etikettendruck](./druckerstatus_etikettendruck.md)
