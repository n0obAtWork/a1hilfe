# Einrichtung von Nummernkreisen

Zur allgemeinen Einrichtung der Nummernkreise gehört die Einrichtung von Nummernkreisen und ihren Zählkreisen. Anschließend können die Nummernkreise verschiedenen Vorgängen oder Stammdaten zugeordnet werden. Bei einer Neueinrichtung bzw. Erweiterung der Nummernkreise empfiehlt sich folgende Einrichtungsreihenfolge:

| Direktsprung | Beschreibung |
| :--- | :--- |
| **[NKZ]** | Unter **[NKZ]** können Zählkreise gepflegt werden. |
| **[NKS]** | Unter **[NKS]** gibt es die Möglichkeit Nummernkreise zu pflegen. Hier können Zählkreise über einen Gültigkeitszeitraum zu einem Nummernkreis zugeordnet werden. Außerdem können hier neue Zählkreise angelegt werden. |
| **[NKV]** | Vorgangszuordnung |
| **[NKF]** | Unter **[NKF]** werden Nummernkreise zu FiBu-Vorgängen zugeordnet (siehe Nummernkreiszuordnung Finanzbuchhaltung). |
| **[MND]** und **[MNDNK]** | Festlegung der Nummernkreise bei Personenkonten im Mandantenstamm |

<details>
  <summary>Nummernkreis</summary>

  Hauptmenü > Administration > Nummernkreise > Nummernkreise

  oder Direktsprung **[NKS]**

  <p class="just-emphasize pseudo-header">Kopfdaten</p>
  
  | Feld | Beschreibung |
  | :--- | :--- |
  | Nummernkreis | Hier wird eine eindeutige Nummer für den Nummernkreis festgelegt. Neben der Nummer kann hier eine Bezeichnung für den Nummernkreis vergeben werden. |
  | Gesperrt | <p>Mit diesem Kennzeichen können Nummernkreise gesperrt werden. Aus gesperrten Nummernkreisen kann keine Nummer bereitgestellt werden.</p><p>Der Standardwert ist „Nein“.</p> |
  | Nur für Journal | <p>Kennzeichen, ob es sich um ein Nummernkreis handelt, der nur für das Journal verwendet werden soll.</p><p>Der Standardwert ist „Nein“.</p> |
  | Nur für FiBu | <p>Kennzeichen, ob es sich um ein Nummernkreis handelt, der nur für die FiBu verwendet werden soll.</p><p>Der Standardwert ist „Nein“.</p> |

  <p class="just-emphasize pseudo-header">Datentabelle</p>

  Über die Datentabelle können Zählkreise zu einen Nummernkreis zugeordnet werden. Des Weiteren besteht die Möglichkeit hier direkt neue Zählkreise anzulegen.

  Einige Felder sind mit dem Hinweis „Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich“ versehen. Sollen diese Felder nachträglich geändert werden, so ist die Funktion **Markierter Zählkreis** zu verwenden. Über den Zählkreis-Pfleger können die Felder angepasst werden.

  | Feld | Beschreibung |
  | :--- | :--- |
  | Von | <p>Mit dem „Von“ - und „Bis“-Feld wird ein Zeitraum festgelegt, in dem der Zählkreis für den Nummernkreis gültig ist. In dem Feld „Von“ wird angegeben, ab wann ein Zählkreis für den Nummernkreis aktiv ist.</p><p>Sobald ein Zählkreis einem Nummernkreis zugeordnet wurde, kann das von-Datum nicht mehr geändert werden.</p> |
  | Bis | Mit dem „Von“ - und „Bis“-Feld wird ein Zeitraum festgelegt, in dem der Zählkreis für den Nummernkreis gültig ist. In dem Feld „Bis“ wird angegeben, bis einschließlich wann ein Zählkreis für den Nummernkreis aktiv ist. |
  | Zählkreis | <p>Hier können Zählkreise einem Nummernkreis zugeordnet werden. Dabei kann die Nummer eines vorhandenen oder eines neuanzulegenden Zählerkreises angegeben werden.</p><p>Welcher Zählkreis aktiv ist, hängt von dem Gültigkeitszeitraum ab. Existieren mehrere Zählkreise, die gültig sind, dann wird der Zählkreis gewählt, der über das aktuellere „Von“-Datum verfügt.</p><p>Alle zugeordneten Zählerkreise, die nicht mehr in dem Gültigkeitsbereich liegen oder die von einem anderen Zählerkreis abgelöst worden sind, werden nicht mehr gezogen und können mit dem Schalter „Inaktive Zählerkreise ausblenden“ in der Datentabelle ausgeblendet werden.</p> |
  | Bezeichnung | Bezeichnung des Zählkreises. |
  | Untergrenze | <p>Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Untergrenze bildet hierbei den Startwert.</p><p>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.</p> |
  | Obergrenze | <p>Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Obergrenze stellt die maximale Nummer dar, die bereitgestellt werden kann.</p><p>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.</p> |
  | Überlaufwarnung | <p>Siehe Überlaufwarnung.</p><p>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.</p> |
  | Waretext | Siehe Einrichtungstexte. |
  | Fibutext | Siehe Einrichtungstexte. |
  | Sperre | Siehe Zählkreissperre. |
  | Wrap around | <p>Siehe Wrap around.</p><p>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.</p> |
  | Stand | <p>Hier wird der aktuelle Stand des Zählkreises angezeigt.</p><p>Eingabe ist nur bei Neu-Anlage eines Zählkreises möglich.</p> |

  ![Nummernkreise](image8_26.png "Nummernkreise")
</details>

<details>
  <summary>Zählkreis</summary>

  Hauptmenü  Administration  Nummernkreise  Zählkreise

  oder Direktsprung **[NKZ]**

  | Feld | Beschreibung |
  | :--- | :--- |
  | Zählkreis | Hier wird eine eindeutige Nummer für den Zählkreis festgelegt. Neben der Nummer kann hier eine Bezeichnung für den Zählkreis vergeben werden. |

  <p class="just-emphasize pseudo-header">Register „Zählerstände“</p>

  | Feld | Beschreibung |
  | :--- | :--- |
  | Aktueller Zählerstand | <p>Hier wird der aktuelle Zählerstand des Zählkreises angezeigt. Der aktuelle Zählerstand stellt die nächste Nummer dar, die gezogen wird. Sobald eine Nummer aus dem Zählkreis gezogen wird, wird der Zählerstand automatisch um 1 hochgezählt.</p><p>Ausnahme: Befinden sich Nummern in der Reserveliste, so werden diese zuerst gezogen.</p><p class="just-emphasize">Hinweis</p><p>Ist der Zählerstand übergelaufen, so wird in dem Feld „Aktueller Zählerstand“ eine Nummer angezeigt, die um 1 größer als die Obergrenze ist. Außerdem wird das Label zum aktuellen Zählerkreis rot eingefärbt. Die „übergelaufene“ Nummer wird nicht gezogen! Sie dient nur zu Darstellungszwecken, um anzuzeigen, dass der Zählkreis übergelaufen ist.</p>![Zählerstand](image8_27.png "Zählerstände") |
  | Untergrenze Zähler | <p>Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Untergrenze bildet hierbei den Startwert.</p><p>Beim Ändern der Untergrenze ist zu beachten, dass ggf. der aktuelle Zählerstand angepasst werden muss (siehe aktueller Zählerstand). Außerdem werden Reservenummern, die nicht mehr in den Nummernbereich des Zählkreises passen, gelöscht.</p> |
  | Obergrenze Zähler | <p>Mit dem Setzen der Unter- und Obergrenze, wird ein Bereich festgelegt, aus dem die Nummern fortlaufend gezogen werden. Die Obergrenze stellt die maximale Nummer dar, die bereitgestellt werden kann.</p><p>Beim Ändern der Obergrenze ist zu beachten, dass ggf. der aktuelle Zählerstand angepasst werden muss (siehe aktueller Zählerstand). Außerdem werden Reservenummern, die nicht mehr in den Nummernbereich des Zählkreises passen, gelöscht.</p> |
  | Überlaufwarnung ab Zählerstand | Hier wird der Zählerstand eingetragen, ab dem eine Überlaufwarnung angezeigt soll. Mit der Überlaufwarnung soll der Anwender informiert werden, dass die Obergrenze des Zählkreises fast erreicht ist. |
  | Zählkreis „wrap around“ | <p>Steht dieses Kennzeichen auf „Ja“, so fängt der Zählkreis beim Überlaufen der Obergrenze automatisch wieder bei der Untergrenze an. Steht das Kennzeichen auf „Nein“ führt das Überlaufen des Zählkreises dazu, dass keine Nummer gezogen werden kann.</p><p>Der Standardwert ist „Ja“.</p> |
  | Zählkreissperre | <p>Steht die Zählkreissperre auf „Ja“, so wird der Zählkreis gesperrt. Dadurch können keine weiteren Nummern aus dem Zählkreis gezogen werden.</p><p>Der Standardwert ist „Nein“.</p><p class="just-emphasize">Hinweis</p><p>Beim Setzen der Zählkreissperre ist zu beachten, dass alle Nummernkreise, die diesen Zählkreis aktiv verwenden, blockiert werden können. Das kann dazu führen, dass aus diesen Nummernkreisen keine Nummer bereitgestellt werden kann!</p> |
  | Reservenummern | <p>Nicht (mehr) genutzte Nummern können in eine Reserveliste geschrieben und damit wieder freigegeben werden. Wird eine Nummer aus dem Zählkreis gezogen, so werden zuerst die Nummern aus der Reserveliste genommen.</p><p>Beim Ziehen einer Nummer aus der Reserveliste, wird der Zählerstand nicht hochgezählt.</p> |

  <p class="just-emphasize pseudo-header">Register „Verwendung“</p>

  | Feld | Beschreibung |
  | :--- | :--- |
  | Verwendung | Hier werden alle Nummernkreise aufgelistet, in denen der Zählkreis verwendet wird. |

  <p class="just-emphasize pseudo-header">Register „Einrichtungstexte“</p>

  | Feld | Beschreibung |
  | :--- | :--- |
  | Ware | Hier kann eine alphanumerische Belegnummer für die Warenwirtschaft eingerichtet werden (siehe Syntax). |
  | FiBu | Hier kann eine alphanumerische Belegnummer für die Finanzbuchhaltung eingerichtet werden (siehe Syntax). |
  | Syntax | <p>`[{Nr,Länge}][Festtext]`</p><table><thead><tr><th>Nr.</th><th>Bedeutung</th></tr></thead><tbody><tr><td>1</td><td>Belegnummer ohne führende Nullen</td></tr><tr><td>2</td><td>Belegnummer mit führenden Nullen</td></tr><tr><td>3</td><td>Nummernkreis ohne führende Nullen</td></tr><tr><td>4</td><td>Nummernkreis mit führenden Nullen</td></tr><tr><td>5</td><td>Rechter Nummernanteil ohne führende Nullen</td></tr><tr><td>6</td><td>Rechter Nummernanteil mit führenden Nullen</td></tr><tr><td>11</td><td>Bediener Identifikation</td></tr><tr><td>12</td><td>Bediener Kurzbezeichnung</td></tr></tbody></table><p>Beispiel:<br>Beleg-Nummer 97/000100 Einrichtungstext = 97/{2,6}<br>**Achtung**: Obiges Beispiel kann zu Problemen bei der Anzeige in Listen, Auswertungen wegen der Länge führen!</p>|

  ![Zählkreise](image8_28.png "Zählkreise")
</details>

<details>
  <summary>Vorgangszuordnung</summary>

  Hauptmenü > Administration > Nummernkreise > Vorgangszuordnung
  
  oder Direktsprung **[NKV]**

  Hier werden pro Bedienerklasse und Vorgang bzw. Vorgangsunterklasse die entsprechenden Nummernkreise zugeordnet. In der Basis-DB sind die vorhandenen Bedienerklassen zugeordnet.

  Der Eintrag von **Bedienerklasse 0** führt zur generellen Gültigkeit des Nummernkreises für diesen Vorgangstyp (z.B. Nummer Lieferscheine).

  Prinzipiell können verschiedene Bedienerklassen auch mit getrennten Nummernkreisen arbeiten.

  ![Zuordnung](image8_29.png "Zuordnung")
</details>

<details>
  <summary>FiBu-Vorgangszuordnung</summary>

  Hauptmenü > Administration > Nummernkreise > Fibu-Vorgangszuordnung

  oder Direktsprung **[NKF]**

  Hier werden Nummernkreise zu FiBu-Vorgängen zugeordnet (siehe Nummernkreiszuordnung Finanzbuchhaltung).
</details>

