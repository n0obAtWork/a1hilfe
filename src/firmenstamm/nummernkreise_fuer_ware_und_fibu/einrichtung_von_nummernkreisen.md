# Einrichtung von Nummernkreisen

<!-- source: https://amic.de/hilfe/_einrichtungvonnummer.htm -->

Zur allgemeinen Einrichtung der Nummernkreise gehört die Einrichtung von Nummernkreisen und ihren Zählkreisen. Anschließend können die Nummernkreise verschiedenen Vorgängen oder Stammdaten zugeordnet werden. Bei einer Neueinrichtung bzw. Erweiterung der Nummernkreise empfiehlt sich folgende Einrichtungsreihenfolge:

| **Direktsprung** | **Beschreibung** |
| --- | --- |
| **[NKZ]** | Unter **[NKZ]** können Zählkreise gepflegt werden.<br> |
| **[NKS]** | Unter **[NKS]** gibt es die Möglichkeit Nummernkreise zu pflegen. Hier können Zählkreise über einen Gültigkeitszeitraum zu einem Nummernkreis zugeordnet werden. Außerdem können hier neue Zählkreise angelegt werden.<br> |
| **[NKV]** | Vorgangszuordnung<br> |
| **[NKF]** | Unter **[NKF]** werden Nummernkreise zu FiBu-Vorgängen zugeordnet (siehe [Nummernkreiszuordnung Finanzbuchhaltung](../../finanzbuchhaltung/stammdaten_der_fibu/nummernkreise/nummernkreiszuordnung_finanzbuchhaltung.md)).<br> |
| **[MND]** und ****[MNDNK]**** | Festlegung der Nummernkreise bei Personenkonten im Mandantenstamm<br> |

<details>
<summary>Nummernkreis</summary>

Hauptmenü > Administration > Nummernkreise > Nummernkreise

oder Direktsprung **[NKS]**

<p class="just-emphasize">Kopfdaten</p>

| **Feld** | **Beschreibung** |
| --- | --- |
| Nummernkreis | Hier wird eine eindeutige Nummer für den Nummernkreis festgelegt. Neben der Nummer kann hier eine Bezeichnung für den Nummernkreis vergeben werden.<br> |
| Gesperrt | Mit diesem Kennzeichen können Nummernkreise gesperrt werden. Aus gesperrten Nummernkreisen kann keine Nummer bereitgestellt werden.<br>Der Standardwert ist „Nein“.<br> |
| Nur für Journal | Kennzeichen, ob es sich um ein Nummernkreis handelt, der nur für das Journal verwendet werden soll.<br>Der Standardwert ist „Nein“.<br> |
| Nur für FiBu | Kennzeichen, ob es sich um ein Nummernkreis handelt, der nur für die FiBu verwendet werden soll.<br>Der Standardwert ist „Nein“.<br> |

<p class="just-emphasize">Datentabelle</p>

Über die Datentabelle können Zählkreise zu einen Nummernkreis zugeordnet werden. Des Weiteren besteht die Möglichkeit hier direkt neue Zählkreise anzulegen.

Einige Felder sind mit dem Hinweis „Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich“ versehen. Sollen diese Felder nachträglich geändert werden, so ist die Funktion ***Markierter Zählkreis*** zu verwenden. Über den Zählkreis-Pfleger können die Felder angepasst werden.

| **Feld** | **Beschreibung** |
| --- | --- |
| Von | Mit dem „Von“ - und „Bis“-Feld wird ein Zeitraum festgelegt, in dem der Zählkreis für den Nummernkreis gültig ist. In dem Feld „Von“ wird angegeben, ab wann ein Zählkreis für den Nummernkreis aktiv ist.<br>Sobald ein Zählkreis einem Nummernkreis zugeordnet wurde, kann das von-Datum nicht mehr geändert werden.<br> |
| Bis | Mit dem „Von“ - und „Bis“-Feld wird ein Zeitraum festgelegt, in dem der Zählkreis für den Nummernkreis gültig ist. In dem Feld „Bis“ wird angegeben, bis einschließlich wann ein Zählkreis für den Nummernkreis aktiv ist.<br> |
| Zählkreis | Hier können Zählkreise einem Nummernkreis zugeordnet werden. Dabei kann die Nummer eines vorhandenen oder eines neuanzulegenden Zählerkreises angegeben werden.<br>Welcher Zählkreis aktiv ist, hängt von dem Gültigkeitszeitraum ab. Existieren mehrere Zählkreise, die gültig sind, dann wird der Zählkreis gewählt, der über das aktuellere „Von“-Datum verfügt.<br>Alle zugeordneten Zählerkreise, die nicht mehr in dem Gültigkeitsbereich liegen oder die von einem anderen Zählerkreis abgelöst worden sind, werden nicht mehr gezogen und können mit dem Schalter „Inaktive Zählerkreise ausblenden“ in der Datentabelle ausgeblendet werden.<br> |
| Bezeichnung | Bezeichnung des Zählkreises.<br> |
| Untergrenze | Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Untergrenze bildet hierbei den Startwert.<br>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.<br> |
| Obergrenze | Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Obergrenze stellt die maximale Nummer dar, die bereitgestellt werden kann.<br>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.<br> |
| Überlaufwarnung | Siehe [Überlaufwarnung](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Ueberlaufwarnung).<br>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.<br> |
| Waretext | Siehe [Einrichtungstexte](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Einrichtungstexte).<br> |
| Fibutext | Siehe [Einrichtungstexte](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Einrichtungstexte).<br> |
| Sperre | Siehe [Zählkreissperre](./einrichtung_von_nummernkreisen.md#Zaehlkreissperre).<br> |
| Wrap around | Siehe [Wrap around](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Wrap_Around).<br>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.<br> |
| Stand | Hier wird der aktuelle Stand des Zählkreises angezeigt.<br>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.<br> |

![Ein Bild, das Tisch enthält. Automatisch generierte Beschreibung](../../ImagesExt/image8_26.png "Ein Bild, das Tisch enthält. Automatisch generierte Beschreibung")

</details>

 

<details>
<summary>Zählkreis</summary>

Hauptmenü > Administration > Nummernkreise > Zählkreise

oder Direktsprung **[NKZ]**

| **Feld** | **Beschreibung** |
| --- | --- |
| Zählkreis | Hier wird eine eindeutige Nummer für den Zählkreis festgelegt. Neben der Nummer kann hier eine Bezeichnung für den Zählkreis vergeben werden.<br> |

<p class="just-emphasize">Register „Zählerstände“</p>

| **Feld** | **Beschreibung** |
| --- | --- |
| Aktueller Zählerstand | Hier wird der aktuelle Zählerstand des Zählkreises angezeigt. Der aktuelle Zählerstand stellt die nächste Nummer dar, die gezogen wird. Sobald eine Nummer aus dem Zählkreis gezogen wird, wird der Zählerstand automatisch um 1 hochgezählt.<br>Ausnahme: Befinden sich Nummern in der [Reserveliste](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Reservenummern), so werden diese zuerst gezogen.<br> <br>**Hinweis**<br>Ist der Zählerstand übergelaufen, so wird in dem Feld „Aktueller Zählerstand“ eine Nummer angezeigt, die um 1 größer als die Obergrenze ist. Außerdem wird das Label zum aktuellen Zählerkreis rot eingefärbt. Die „übergelaufene“ Nummer wird nicht gezogen! Sie dient nur zu Darstellungszwecken, um anzuzeigen, dass der Zählkreis übergelaufen ist.<br>![](../../ImagesExt/image8_27.png)<br> |
| Untergrenze Zähler | Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Untergrenze bildet hierbei den Startwert.<br>Beim Ändern der Untergrenze ist zu beachten, dass ggf. der aktuelle Zählerstand angepasst werden muss (siehe [aktueller Zählerstand](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Zaehlerstand)). Außerdem werden Reservenummern, die nicht mehr in den Nummernbereich des Zählkreises passen, gelöscht.<br> |
| Obergrenze Zähler | Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Obergrenze stellt die maximale Nummer dar, die bereitgestellt werden kann.<br>Beim Ändern der Obergrenze ist zu beachten, dass ggf. der aktuelle Zählerstand angepasst werden muss (siehe [aktueller Zählerstand](./einrichtung_von_nummernkreisen.md#Zaehlkreis_Zaehlerstand)). Außerdem werden Reservenummern, die nicht mehr in den Nummernbereich des Zählkreises passen, gelöscht.<br> |
| Überlaufwarnung ab Zählerstand | Hier wird der Zählerstand eingetragen, ab dem eine Überlaufwarnung angezeigt soll. Mit der Überlaufwarnung soll der Anwender informiert werden, dass die Obergrenze des Zählkreises fast erreicht ist.<br> |
| Zählkreis „wrap around“ | Steht dieses Kennzeichen auf „Ja“, so fängt der Zählkreis beim Überlaufen der Obergrenze automatisch wieder bei der Untergrenze an. Steht das Kennzeichen auf „Nein“ führt das Überlaufen des Zählkreises dazu, dass keine Nummer gezogen werden kann.<br>Der Standardwert ist „Ja“.<br> |
| Zählkreissperre | Steht die Zählkreissperre auf „Ja“, so wird der Zählkreis gesperrt. Dadurch können keine weiteren Nummern aus dem Zählkreis gezogen werden.<br>Der Standardwert ist „Nein“.<br> <br>**Hinweis**<br>Beim Setzen der Zählkreissperre ist zu beachten, dass alle Nummernkreise, die diesen Zählkreis aktiv verwenden, blockiert werden können. Das kann dazu führen, dass aus diesen Nummernkreisen keine Nummer bereitgestellt werden kann!<br> |
| Reservenummern | Nicht (mehr) genutzte Nummern können in eine Reserveliste geschrieben und damit wieder freigegeben werden. Wird eine Nummer aus dem Zählkreis gezogen, so werden zuerst die Nummern aus der Reserveliste genommen.<br>Beim Ziehen einer Nummer aus der Reserveliste, wird der Zählerstand nicht hochgezählt.<br> |

<p class="just-emphasize">Register „Verwendung“</p>

| **Feld** | **Beschreibung** |
| --- | --- |
| Verwendung | Hier werden alle Nummernkreise aufgelistet, in denen der Zählkreis verwendet wird.<br> |

<p class="just-emphasize">Register „Einrichtungstexte“</p>

<table class="AMIC-Tabelle" style="WIDTH: 100%; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="100%" border="0"><tbody><tr><td style="WIDTH: 20.98%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="20%"><p class="AMIC-Textkoerper" style="TEXT-ALIGN: center" align="center"><b><span style="COLOR: white">Feld</span></b></p></td><td style="WIDTH: 79.02%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="79%"><p class="AMIC-Textkoerper" style="TEXT-ALIGN: center" align="center"><b><span style="COLOR: white">Beschreibung</span></b></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 20.98%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="20%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Ware</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 79.02%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="79%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alphanumerische Belegnummer für die Warenwirtschaft eingerichtet werden (siehe Syntax).</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 20.98%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="20%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">FiBu</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 79.02%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="79%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alphanumerische Belegnummer für die Finanzbuchhaltung eingerichtet werden (siehe Syntax).</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 20.98%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="20%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Syntax</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 79.02%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="79%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">[{Nr,Länge}][Festtext]</span><span style="FONT-SIZE: 11pt"></span></p><table class="AMIC-Tabelle" style="BORDER-COLLAPSE: collapse; MARGIN-LEFT: 15.9pt" cellspacing="0" cellpadding="0" border="0"><tbody><tr><th style="WIDTH: 23.05pt; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="31"><b><span style="COLOR: white">Nr</span></b><span style="COLOR: white">.</span></th><th style="WIDTH: 721.15pt; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="962"><b><span style="COLOR: white">Bedeutung</span></b></th></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">1</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Belegnummer ohne führende Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">2</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Belegnummer mit führenden Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">3</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Nummernkreis ohne führende Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">4</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Nummernkreis mit führenden Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">5</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Rechter Nummernanteil ohne führende Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">6</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Rechter Nummernanteil mit führenden Nullen</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">11</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Bediener Identifikation</span><span style="FONT-SIZE: 11pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23.05pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31"><span style="FONT-SIZE: 11pt; COLOR: black">12</span><span style="FONT-SIZE: 11pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 721.15pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="962"><span style="FONT-SIZE: 11pt; COLOR: black">Bediener Kurzbezeichnung</span><span style="FONT-SIZE: 11pt"></span></td></tr></tbody></table><p class="AMIC-Textkoerper"><u><span style="FONT-SIZE: 11pt; COLOR: black">Beispiel:</span></u><span style="FONT-SIZE: 11pt; COLOR: black"></span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Beleg-Nummer 97/000100 Einrichtungstext = 97/{2,6}</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><b><span style="FONT-SIZE: 11pt; COLOR: black">Achtung:</span></b><span style="FONT-SIZE: 11pt; COLOR: black"> Obiges Beispiel kann zu Problemen bei der Anzeige in Listen, Auswertungen wegen der Länge führen!</span><span style="FONT-SIZE: 11pt"></span></p></td></tr></tbody></table>

![](../../ImagesExt/image8_28.png)

</details>

<details>
<summary>Vorgangszuordnung</summary>

Hauptmenü > Administration > Nummernkreise > Vorgangszuordnung

oder Direktsprung **[NKV]**

Hier werden pro Bedienerklasse und Vorgang bzw. Vorgangsunterklasse die entsprechenden Nummernkreise zugeordnet. In der Basis-DB sind die vorhandenen Bedienerklassen zugeordnet.

Der Eintrag von **Bedienerklasse 0** führt zur generellen Gültigkeit des Nummernkreises für diesen Vorgangstyp (z.B. Nummer Lieferscheine).

Prinzipiell können verschiedene Bedienerklassen auch mit getrennten Nummernkreisen arbeiten.

![Ein Bild, das Text, Screenshot, Display, Software enthält. Automatisch generierte Beschreibung](../../ImagesExt/image8_29.png "Ein Bild, das Text, Screenshot, Display, Software enthält. Automatisch generierte Beschreibung")

</details>

<details>
<summary>FiBu-Vorgangszuordnung</summary>

Hauptmenü > Administration > Nummernkreise > Fibu-Vorgangszuordnung

oder Direktsprung **[NKF]**

Hier werden Nummernkreise zu FiBu-Vorgängen zugeordnet (siehe [Nummernkreiszuordnung Finanzbuchhaltung](../../finanzbuchhaltung/stammdaten_der_fibu/nummernkreise/nummernkreiszuordnung_finanzbuchhaltung.md)).

</details>
