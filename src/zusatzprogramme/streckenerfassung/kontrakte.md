# Kontrakte

<!-- source: https://amic.de/hilfe/_vorgangsmappe_kontrakte.htm -->

Kontraktartikelausweichliste

Ist für einen Kontrakt eine Kontraktartikelausweichliste hinterlegt, so wird im *GFV* das Feld Artikelnummer farblich hinterlegt. Zusätzlich werden im *GMV* die Artikelnummern der Ausweichliste in neu hinzugefügten Zeilen angehängt, auch diese sind farblich hinterlegt. So hat der Bediener sofortigen Überblick über die vorhandenen Ausweichartikel.

Die Kontraktartikelausweichliste kann jetzt direkt bearbeitet werden, indem

der Cursor über dem markierten Feld Artikelnummer im *GFV* positioniert, das Kontextmenü geöffnet und der Kontextmenüpunkt [Kontraktartikelausweichliste bearbeiten](./kontextmenues.md#KontraktArtikelAusweichListe) ausgewählt wird.

Auswahl eines Kontraktes in der Strecke

Ist in dem [Profil](./profile/index.md#ProfileStreckenerfassung) für die Strecke der Schalter „Erweiterte Kontraktanzeige“ auf der Registerkarte [Griddefinition](./profile/index.md#registergriddefinition) auf „Ja“ gestellt, so werden die möglichen Kontrakte mit ihren Artikeln und so wie den Artikeln der [Kontraktausweichliste](../../kontrakt/kontraktausweichliste.md) angezeigt. Bei einer nachträglichen Artikelauswahl werden nur die Artikel angezeigt, welche dem Kontrakt zugeordnet worden sind. Wird ein Artikel aus der zugewiesenen [Ausweichliste](../../kontrakt/kontraktausweichliste.md) ausgewählt, so wird dieser bei der Vorgangserzeugung mit in die [Kontraktartikelliste](../../kontrakt/kontraktstammdaten/kontraktartikel/index.md) übernommen. Ist der Schalter „Fixpreis“ in der [Ausweichliste](../../kontrakt/kontraktausweichliste.md) auf „Nein“ gestellt, so wird als Kontraktpreis, der Preis des ersten Artikels aus [Kontraktartikelliste](../../kontrakt/kontraktstammdaten/kontraktartikel/index.md) genommen. Steht der Schalter auf „Ja“, so wird als Kontraktpreis, der Preis aus der [Ausweichliste](../../kontrakt/kontraktausweichliste.md) für den gewählten Artikel übernommen.

Ist der Schalter „Erweiterte Kontraktanzeige“ auf der Registerkarte [Griddefintion](./profile/index.md#registergriddefinition) auf „Nein“ gestellt, erfolgt die Standard Kontraktauswahl.

Folgende Felder werden durch die Auswahl eines Kontraktes vorbelegt.

Kontrakt, Artikel, Menge und Preis.
