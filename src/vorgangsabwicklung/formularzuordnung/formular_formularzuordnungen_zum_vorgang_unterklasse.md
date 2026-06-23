# Formular - Formularzuordnungen zum Vorgang/Unterklasse

<!-- source: https://amic.de/hilfe/formularformularzuordnungenzum.htm -->

Diese Seite bestimmt die Formulare für die Vorgangsunterklasse.

Druck, Vorschau und Bildschirm Formulare können unabhängig voneinander definiert werden. Gleiches gilt auch für die Archivierung.

Mit dem Parameter ‚Artikeltextvariante’ kann für eine Vorgangsunterklasse IMMER eine bestimmte Artikeltextvariante gezogen werden. Diese Einstellung übersteuert die lagerspezifische Textvariante!

Die Artikeltextvariante wird bei jedem Vorgangsunterklassenwechsel neu ausgewertet und dem Artikel ggf. der neue Variantentext zugeordnet.

- Unterklassen wechseln
- Belegumwandlungen

Außerdem lassen sich hier die Zuordnungen von AIS-Gruppen zu Vorgangsunterklassen einstellen. Für genauere Informationen dazu siehe [Beispiel eines Informationsfeldes in Vorgängen](../../zusatzprogramme/ais_a_eins_informationssystem/beispiel_eines_eingabefeldes_in_vorgaengen.md).

<p class="just-emphasize">Einstellungen der Formulare</p>

| Formular | Beschreibung |
| --- | --- |
| Druck | Wird zum Druck des Beleges verwendet |
| Vorschau | Wird für die Anzeige einer Vorschau verwendet |
| Bildschirm | Wird zur Anzeige auf dem Bildschirm verwendet |
| Auftrag Formular | |
| Angebot Formular | |
| Archivierung | Dieses Formular wird verwendet, um ungedruckte Belege vor dem FiBu-Übertrag noch zu archivieren. Das Formular wird nicht verwendet, wenn ein regulärer Druck stattfindet! |
| Referenzdrucker (für Archivierung) | Im Zusammenhang mit dem obigen Formular sollte ein Drucker eingerichtet sein, der dessen Druckeinstellungen und Formatierungen für die Erstellung des Archiveintrags beinhaltet. Der Drucker wird physikalisch nicht verwendet, muss jedoch erreichbar sein. Deshalb kann hier auch ein virtueller Drucker angegeben werden. |
| Rechnungsformular im Barverkauf | Dieses Formular wird für den Druck einer Rechnung in der Marktkasse ab einem im Steuerparameter [867 .- Rechnungsdruck](../../firmenstamm/steuerparameter/kasse_barverkauf/rechnungsdruck_bei_barverkauf_spa_867.md) bei Barverkauf festgelegten Grenzwert verwendet. Der zu verwendende Drucker wird in der Kassensystemverwaltung festgelegt.<br>Die Adresse des zugrundeliegenden Kunden wird als neue Vorgangsversandanschrift kopiert und als neue Rechnungsadresse vorbelegt.<br> |
| Gelangensmahnformular | Hier kann ein Formular für den Mahnschreibendruck der [Gelangensbestätigung](../../zusatzprogramme/gelangensbestaetigung.md) eingetragen werden. |

<p class="just-emphasize">Artikeltextvariante</p>

Gibt an, welche Textvariante für Artikel in der Vorgangsunterklasse verwendet werden soll. Ist hier eine 0 eingetragen, so wird die Standard-Artikeltext-Variante herangezogen.

Zu beachten ist jedoch der Steuerparameter (SPA) [Artikeltext-Variante des Artikels (231)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_warenposition/artikeltext_variante_des_artikels_spa_231.md) der Steuerparametergruppe „Vorgangsbearbeitung Warenposition“.

<p class="just-emphasize">Positionsteil</p>

| Feld | Beschreibung |
| --- | --- |
| Kopflänge aus Formular | Gibt an, ob bei der Erfassung die Größe der Anzeige einer Position fest auf 3 Zeilen definiert oder aus dem Formular dynamisch vorbelegt werden soll |
| | |
