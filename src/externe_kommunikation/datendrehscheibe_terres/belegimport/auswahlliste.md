# Auswahlliste

<!-- source: https://amic.de/hilfe/_terres_belegimportAuswahlliste.htm -->

Nach dem erfolgreichen Import der Eingangsbeleg in das A.eins System werden diese in der Anwendung Terres Belegimport angezeigt. In der Auswahlliste können folgende Felder farblich nach dem Import dargestellt sein. Die Zusammenfassung eines Beleges wird in einer gelb Markierten Zeile dargestellt.

<p class="just-emphasize">Rechnung</p>

| ![\*](../../../ImagesExt/image8_1557.jpg "*") Farbe | ![\*](../../../ImagesExt/image8_1557.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Rot | ![\*](../../../ImagesExt/image8_1556.jpg "*") In dem Beleg kommen unterschiedliche Lagerort vor |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Weiß | ![\*](../../../ImagesExt/image8_1556.jpg "*") Der Beleg enthält nur ein Lagerort. |

<p class="just-emphasize">Aeins-Art.</p>

| ![\*](../../../ImagesExt/image8_1558.jpg "*") Farbe | ![\*](../../../ImagesExt/image8_1558.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Rot | ![\*](../../../ImagesExt/image8_1556.jpg "*") Der Artikel ist A.eins Pool nicht vorhanden |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Gelb | ![\*](../../../ImagesExt/image8_1556.jpg "*") Der Artikel ist nicht auf dem A.eins Lager vorhanden. |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Weiß | 1. Der Artikel ist auf dem Lager vorhanden.<br>2. Der Terres Artikel ist nicht im A.eins Artikelpool(Gruppenartikel) vorhanden wurde aber in diverse A.eins Artikel aufgeteilt.<br>3. Der Artikel ist im A.eins Artikelpool oder nicht vorhanden wurde aber einem andern Artikel zugeordnet. |

<p class="just-emphasize">A.eins Lager</p>

| ![\*](../../../ImagesExt/image8_1557.jpg "*") Farbe | ![\*](../../../ImagesExt/image8_1557.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Rot | ![\*](../../../ImagesExt/image8_1556.jpg "*") Es existiert keine Zuordnung eines Lagers aus der Eingangsrechnung zu einem A.eins Lager. Diese kann im [Importumsetzer](../index.md#ueb_bereich_importumsetzer) hinterlegt werden |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Weiß | ![\*](../../../ImagesExt/image8_1556.jpg "*") Es existiert eine Zuordnung zwischen dem Lagerortcode und einem A.eins Lager |

<p class="just-emphasize">Gelöschte Position / Gelöschter Beleg</p>

| ![\*](../../../ImagesExt/image8_1558.jpg "*") Farbe | ![\*](../../../ImagesExt/image8_1558.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Rot | ![\*](../../../ImagesExt/image8_1556.jpg "*") Die Position wurde als gelöscht gekennzeichnet und wird nicht mit in die Eingangsrechnung übernommen. Position belibt in der XML enthalten. |
| ![\*](../../../ImagesExt/image8_1556.jpg "*") Weiß | ![\*](../../../ImagesExt/image8_1556.jpg "*") Die Position wird mit übertragen. |

<p class="just-emphasize">Folgende Funktionen stehen zur Bearbeitung der Eingangsbelege zur Verfügung:</p>

<p class="just-emphasize">XML-Daten anzeigen</p>

Mit dieser Funktion werden die XML-Daten der zuvor markierten Belege im Standardbrowser angezeigt.

<p class="just-emphasize">Kennzeichen Zu/Abschlag</p>

Diese Funktion kennzeichnet die zuvor markierten Positionen als Zu/Abschlagsposition.

*Zurzeit wird dieses Kennzeichen noch nicht verwendet.*

<p class="just-emphasize">Kennzeichen Normalposition</p>

Diese Funktion setzt das Zu/Abschlagskennzeichen für die zuvor markierten Positionen wieder zurück.

<p class="just-emphasize">Position umbuchen</p>

Diese Funktion ruft die Maske „TERRES – Positionsumbuchung“ auf. Auf dieser Maske kann für jede Position ein Artikel aus dem A.eins Pool eingetragen werden, welcher anstelle des TERRES Artikels verwendet werden soll. Es können nur Artikel des gleichen Lagers und mit gleichem Steuersatz ausgewählt werden, andere Artikel stehen nicht zur Verfügung.

<p class="just-emphasize">Position aufteilen</p>

Diese Funktion ruft die Maske „TERRES – Positionsaufteilung“ auf. In dieser kann die markierte Position auf mehrere unterschiedliche Artikel aufgeteilt werden.

Dabei muss die Menge und der Wert komplett aufgeteilt werden. Bei der Belegerzeugung wird anders als bei nicht aufgeteilten Positionen, das Lager des Artikels verwendet und nicht das Lager aus der Vorgangskonstante [**VKONS**].

Aufgeteilte Positionen können hier auch wieder rückgängig gemacht werden, in dem die Datentabelle geleert wird.

<p class="just-emphasize">Position löschen</p>

Mit dieser Funktion wird bei allen zuvor markierten Positionen das Löschkennzeichen gesetzt. Diese Positionen werden dann bei der Belegerzeugung nicht angelegt.

<p class="just-emphasize">Beleg löschen</p>

Mit dieser Funktion wird bei allen zuor manuell markierten Belegen das Löschkennzeichen gesetzt. Der Beleg kann dann nicht mehr angelegt werden.

<p class="just-emphasize">Beleg erzeugen</p>

Diese Funktion erzeugt alle manuell markierten Belege, so dass diese in A.eins zur weiteren Bearbeitungen zur Verfügung stehen. Über die Steuerparameter lässt sich die Belegerzeugung [individualisieren](./individualisierung.md#terres_beleg_indiv_belegerz).

<p class="just-emphasize">Bestimmung der Vorgangsunterklasse</p>

Im Standard wird die Vorgangsunterklasse 0 gewählt. Soll dem Beleg eine andere Unterklasse zugeordnet werden, so ist dies im Steuerparameter „[829](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegimport_spa_829.md)“ einzutragen. Dort wird eine Umschlüsselklasse des [Importumsetzters](../../importumsetzer.md) hinterlegt. In der Umschlüsselklasse kann dann eine Zuordnung zwischen dem A.eins Lager und der Vorgangsunterklasse hergestellt werden.

<p class="just-emphasize">Artikelumbuchung</p>

Mit dieser Funktion kann ein Artikel aus der Terresrechnung auf ein Lagerangelegt werden, wenn dieser auf dem Ziellager nicht vorhanden ist. Der Artikel muss aber im A.eins Artikelpool vorhanden sein. Das Ziellager für den Artikel ist immer das A.eins Lager welches in der Auswahlliste angezeigt wird.

<p class="just-emphasize">Import Event anlegen</p>

Mit dieser Funktion wird der Importevent „TerresImportBeleg“ angelegt. In den [Events](../../../zusatzprogramme/events/starten_von_events.md) kann dieser angepasst oder gelöscht werden.

Der Event importiert die Daten aus einem Verzeichnis, welches im Steuerparameter „[829](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/belegimport_spa_829.md)“ hinterlegt werden kann oder über eine [individualisierte](./individualisierung.md#terres_beleg_indiv_import) Prozedur.
