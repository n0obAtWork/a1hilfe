# Ermittlung des Nachhaltigkeitsstatus und der THG – Werte

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_ermittlung_der_werte.htm -->

<p class="just-emphasize">Ermittlung des Nachhaltigkeitsstatus</p>

Die Ermittlung des Nachhaltigkeitsstatus erfolgt aktuell über vier Ebenen. Wenn die jeweilig vorherige Ebene keinen Status angegeben hat, folgt die nächste Ebene.

| Ebene | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 1 | Artikelstamm Vorbelegung Warenbewegung | Auf der ersten Ebene gilt die Einstellung am Artikelstamm für die Vorbelegung der Warenbewegung. Dies gilt nur wenn man als Vorbelegung „Nicht Nachhaltig“ auswählt. Man kann also nur nachhaltige Waren künstlich zu „Nicht Nachhaltig“ machen und nicht andersherum |
| 2 | Kontrakt | Auf der zweiten Ebene gilt die Kontrakteinstellung. |
| 3 | Kunde / Mandant | Auf der dritten Ebene gilt die Einstellung des Kunden. |
| 4 | | Auf der vierten Ebene gilt, wenn keine der vorherigen Ebenen einen Wert hat, so hat diese Bewegung keine Nachhaltigkeitsinformationen. |

<p class="just-emphasize">Ermittlung der THG – Werte</p>

Die Ermittlung der THG – Werte ist abhängig vom Nachhaltigkeitsstatus. Es werden nur THG-Werte gezogen, wenn der Nachhaltigkeitsstatus „Nachhaltig“ ist. Es gilt aber auch hier wieder, wenn die jeweilig vorherige Ebene keine Werte angegeben hat, folgt die nächste Ebene. Keine Werte heißt nicht „0“, sondern es darf kein Wert eingetragen sein.

| Ebene | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 1 | Kontrakt | Auf der ersten Ebene gelten die Werte am Kontraktartikel. |
| 2 | Kunde / Mandant | Auf der zweiten Ebene gelten die Werte am Kunden. |
| 3 | Anbauland artikelspezifisch | Auf der dritten Ebene gelten die Werte aus dem Anbauland, wo artikelspezifische Werte hinterlegt worden sind. |
| 4 | Anbauland global | Auf der vierten Ebene gelten die Werte aus dem Anbauland, wo keine artikelspezifischen Werte hinterlegt worden sind. |
| 4 | | Wurde kein Wert auf den vorherigen Ebenen gefunden, hat der THG – Wert keinen Wert. |

<p class="just-emphasize">Ermittlung des Anbaulandes</p>

Die Ermittlung des Anbaulandes erfolgt auch unabhängig von den anderen Ermittlungen. Die THG-Werte werden hierbei insoweit nicht beeinflusst, dass die Ebene des Anbaulandes nicht die Ebene der THG-Werte beeinflusst. Ist jedoch ein Anbauland angegeben, werden die THG-Werte unter Umständen gezogen.

| Ebene | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 2 | Kontrakt | Auf der ersten Ebene gelten die Werte am Kontraktartikel. |
| | Anschrift | Auf der zweiten Ebene gelten die Werte in der Anschrift. Siehe Erläuterung zur Anschrift unten. |
| 3 | Kunde / Mandant | Auf der dritten Ebene gelten die Werte am Kunden. |
| 4 | | Wurde kein Wert auf den vorherigen Ebenen gefunden, hat das Anbauland keinen Wert. |

Es ist möglich einen Artikel aus mehreren Anbauländern zu beziehen oder zu produzieren.

Damit das gewünschte Anbauland vorbelegt wird, kann man auf der Einkaufsseite entweder in der Anschrift des Belegkunden [ANSCH] oder Kundenversandanschrift [KUVS] das gewünschte Anbauland auswählen und abspeichern. Die Kundenversandanschrift hat eine höhere Priorität und ignoriert die Anschrift.

Für den Verkauf kann man nur im Lagerstamm [LGS] in der Lageranschrift ein Anbauland setzten. Dieses wird dann, wenn möglich als Vorbelegung gezogen.

Voraussetzung ist, dass das in der Anschrift, Kundenversandanschrift oder Lageranschrift eingetragene Anbauland auch im jeweiligen Kundenzertifikat hinterlegt ist.

Ist dies nicht der Fall, wird das erst beste passende Kundenzertifikat mit einem anderen Anbauland vorbelegt.

Es ist möglich ein manuelles Anbauland anzugeben. Dadurch wird die Anbaulandherkunft zu manuell und werden die Nachhaltigkeitseinrichtungen nach den THG-Werten durchsucht und gezogen.

<p class="just-emphasize">Berechnung des CO2 eq (kg/t)</p>

Die Berechnung des „CO2 eq (kg/t)“ ist abhängig davon ob mit massebezogenem Werten für Zwischenprodukte gerechnet wird oder nicht (Einstellung [Steuerparameter 844](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/nachhaltigkeitseinstellungen_spa_844.md)). Wenn nicht mit den Zwischenprodukten gerechnet wird, erfolgt die Berechnung über folgende Formel:

```text
Thg-Wert /
(Allokationsfaktor * Konversionsfaktor)
```

Sollte einer der Werte „0“ haben, kann keine Berechnung erfolgen, dann müssen die Stammdaten ([Artikelstamm](../../../artikelstamm_und_artikel/parameter_des_artikelstamms/registerkarte_zertifiakte.md#ars_nachhaltigkeit) -> [THG – Wert](../stammdaten/faktor_thg_wert_anbauland.md)) oder die THG – Werte angepasst werden.

Wird hingegen mit Zwischenprodukten gerechnet erfolgt keine Berechnung, da das Zwischenprodukt schon den „CO2 eq (kg/t)“ darstellt.

<p class="just-emphasize">Herkunftsfelder</p>

Fast jedes Feld der Nachhaltigkeitsbewegung hat ein zusätzliches Herkunftsfeld. In diesem wird sich gemerkt, woher der Wert ermittelt wurde. Zum Beispiel wurde der Status am Kontrakt auf „Nicht nachhaltig“ gesetzt. Im Herkunftsfeld für den Status würde dann der Wert „Kontrakt“ enthalten sein. Wenn dieser Wert jedoch an der Warenposition von Hand auf „Nachhaltig“ gesetzt wird, enthält das Herkunftsfeld den Wert „Manuell“.

Folgende Werte kann das Herkunftsfeld enthalten.

| Wert | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Unbekannt | Dieser Wert kann durch eine interne Umstellung aufgetreten sein. Sollte aber unter normalen Umständen nicht existieren. Dies kann auch auftreten, wenn im Anbauland keine THG-Werte eingetragen worden sind. Dann sind die THG-Werte leer und die Herkunft der THG-Werte ist unbekannt. |
| 1 | Manuell | Gibt an, dass der Wert durch eine manuelle Eingabe überschrieben wurde. |
| 2 | Anbauland | Gibt an, dass der Wert aus den Anbaulandinformationen kommt. |
| 3 | Kontrakt | Gibt an, dass der Wert vom Kontrakt gezogen wurde. |
| 4 | Kunde | Gibt an, dass der Wert vom Kunden ermittelt wurde. |
| 5 | Artikelstamm | Gibt an, dass der Wert vom Artikelstamm kommt. Diese Herkunft kann nur noch auftreten, wenn man auf der Artikelstammmaske unter [ARS] auf dem Tabreiter Konstanten im Grid Vorbelegung Warenbewegung Einkauf oder Verkauf auf „Nicht Nachhaltig“ setzt. Dann ist der Nachhaltigkeitsstatus „Nicht Nachhaltig“ und die Herkunft aus dem Artikelstamm |
| 6 | Hauptanschrift | Gibt an, dass das Anbauland bei Einkauf aus der Belegkundenanschrift kommt und beim Verkauf aus der Beleglageranschrift. |
| 7 | Versandanschrift | Gibt an, dass das Anbauland aus der Versandanschrift des Belegkunden kommt. |
