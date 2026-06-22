# Profile

<!-- source: https://amic.de/hilfe/_vorgangsmappe_profile.htm -->

Die Maske dient zur Konfiguration der [Streckenerfassung](../index.md). Es können diverse Einstellungen pro Profil in den Registerkarten der Maske konfiguriert werden.

Neben den Einstellungen des optischen Erscheinungsbildes der Maske (Breite, Anzahl der Zeilen der einzelnen Grids, etc.) sind viele spezielle Einstellungen möglich.

So können diverse Einstellungen gesetzt werden, die die Verarbeitung der Streckenerfassung beeinflussen. Dies ermöglicht ein Zusammenspiel mit anderen Programmen wie Microsoft MapPoint, AMIC Etikettendruck, Crystal Reports etc.

Folgende Registerkarten stehen für die Einstellungen zur Verfügung.

[Allgemein](./index.md#registerallgemein)

[Allgemein 2](./index.md#registerAllgemein2)

[Griddefinition](./index.md#registergriddefinition)

[Auswertungen](./index.md#registerAuswertungen)

[Kontrakte](./index.md#registerKontrakte)

[Kopiervorlagen](./index.md#registerKopiervorlagen)

[Benutzerfelder](./index.md#registerbenutzerfelder)

[Addonfelder](./index.md#registerAddonFelder)

[Buttons](./index.md#registerButtons)

<p class="just-emphasize">Registerkarte Allgemein ![](../../../ImagesExt/image8_1357.png)</p>

Hier können allgemeine Einstellungen der Streckenerfassung definiert werden.

| Feld | Beschreibung |
| --- | --- |
| Profil | Bezeichnung des Profils (Wird im Kopfbereich der Streckenerfassungsmaske angezeigt) |
| Standardprofil | Ein Profil kann als Standardprofil gekennzeichnet werden. Dieses wird unter anderem in Auswahllisten verwendet. |
| Maskenbreite | Die Breite der Maske Streckenerfassung |
| AIS – Breite | Befindet sich die Maske nicht im Vollbildmodus, kann hiermit die Breite der Maske vergrößert werden. Dadurch ist es möglich zusätzliche Informationen über das [AIS-System](../../ais_a_eins_informationssystem/index.md) anzuzeigen. |
| Vollbild | Hiermit kann eingestellt werden, dass die Maske im Vollbildmodus angezeigt wird. |
| Registerkarte (Zeile / Spalte) | Hiermit kann die Position der Registerkarte festgelegt werden. Default mäßig stehen die beiden Wert auf 0.<br>Nur wenn in beiden Feldern ein Wert größer 0 eingetragen ist, wird die Registerkarte verschoben. |
| Abhängigkeit | Hier kann die Abhängigkeit der Daten festgelegt werden.<br>\- Artikelabhängig<br>Die Daten werden bei dieser Einstellung abhängig von der ersten Datentabelle ermittelt. D.h. für die zweite und dritte Datentabelle sind die Artikel aus der ersten Datentabelle ausschlaggebend.<br>Befindet sich ein passender Artikel in der ersten Datentabelle wird der Vorgang in der zweiten Datentabelle angezeigt, ansonsten in der dritten. (dies gilt nicht bei Kontrakten)<br>\- Datentabellenabhängig<br>Bei dieser Einstellung werden die Daten entsprechend ihrer Eingabe angezeigt. D.h. Daten die in der zweiten Datentabelle angelegt wurden, werde auch dort angezeigt, unabhängig der Artikel in der ersten Datentabelle. |
| Ladeprozedur | Bezeichnung der Ladeprozedur (Standard: DISPO_VORGAENGE). Diese Prozedur lädt die Daten in die Maske. |
| Nummernkreis Strecke | Nummer des zu verwendenden Nummernkreises. |
| Nummernkreis Positionsstammsatz | Dieser Nummernkreis wird verwendet, wenn [Positionsstammsätze](../positionsstammsatz.md) angelegt werden. |
| Neuanlage Artikel -> Lager | Steht dieser Schalter auf „Ja“, wird bei Eingabe des Lagers geprüft ob der Artikel des Vorgangs in dem Lager vorhanden ist. Sollte dies nicht der Fall sein, wird nach entsprechender Abfrage der Artikel in das Lager kopiert. |
| Label für „Neu“ | Bezeichnung der Funktion mit der die Streckenerfassung im „Neu“-Modus mit diesem Profil aus der Auswahl heraus gestartet werden kann. |
| Label für „Ändern“ | Bezeichnung der Funktion mit der die Streckenerfassung im „Ändern“-Modus mit diesem Profil aus der Auswahl heraus gestartet werden kann. |
| AIS Gruppe Kunde | Auswahl einer individuell konfigurierten AIS-Maske ( [A.eins Informationssystem](../../ais_a_eins_informationssystem/index.md) ) beim Doppelklick auf die des Kundennamens. |
| AIS Gruppe Artikel | Auswahl einer individuell konfigurierten AIS-Maske ( [A.eins Informationssystem](../../ais_a_eins_informationssystem/index.md) ) beim Doppelklick auf die Spalte Artikel. |
| AIS Gruppe Kontrakt | Auswahl einer individuell konfigurierten AIS-Maske ( [A.eins Informationssystem](../../ais_a_eins_informationssystem/index.md) ) beim Doppelklick auf die Spalte Plandatum. |
| AIS Gruppe Partien | Auswahl einer individuell konfigurierten AIS-Maske ( [A.eins Informationssystem](../../ais_a_eins_informationssystem/index.md) ) beim Doppelklick auf die Spalte Partie. |
| Anzahl Tage gültig | Gültigkeitsdauer von Partien in Tagen |
| Manuelle Partierestmenge | Steht die Einstellung auf „Ja“, muss die Partierestmenge in der Maske „[Partiemengenverteilung](../maske_partiemengenverteilung.md)“ manuell eingegeben werden. Ansonsten berechnet das System die Restmenge automatisch. |
| Partiemenge ändert Positionsmenge | Wenn die Einstellung auf „Ja“ steht, wird Positionsmenge durch die Menge aus der „[Partiemengenverteilung](../maske_partiemengenverteilung.md)“ überschrieben. |
| *Nachkommastellen* Gebindeanzahl | Nachkommastellen für die Spalte Gebindeanzahl „G.Anz“. Die Einstellung gilt für alle drei Grids. |
| *Nachkommastellen* Gewicht | Nachkommastellen für die Spalte Gewicht. Die Einstellung gilt für alle drei Grids. |
| *Nachkommastellen* Mengen | Nachkommastellen für die Mengenspalten. Die Einstellung gilt für alle drei Grids. |
| *Nachkommastellen* Preis | Nachkommastellen für die Preisspalte. Die Einstellung gilt für alle drei Grids. |
| Label Nummerntext | Beschriftung für das Feld Streckennummer. |
| Vorgangsklammer aktiv | Steht die Einstellung auf „Ja“, wird auf der Maske eine Schaltfläche zum Bearbeiten der [Vorgangsklammer](../../../vorgangsabwicklung/vorgangsklammer.md#uebVorgangsklammer) angezeigt. |
| Positionsstammsatz kopieren | Hiermit kann bestimmt werden, ob beim Anlegen des Positionsstammsatzes die Daten aus dem Stammsatz kopiert werden sollen. |
| Rohwaredatum änderbar | Hiermit kann bestimmt werden, ob ein Rohwaredatum geändert werden kann. Dies gilt jedoch nur bei Belegklassen von 0-600 und 1000-1600.<br>*Des Weiteren ist zu beachten, dass für die Korrektur der Ursprüngliche Beleg storniert und ein neuer Beleg mit dem neuen Datum angelegt wird. Dabei werden nur die Daten übernommen die auch auf der Maske stehen.* |
| Funktionsansicht | Steht die Einstellung auf „Ja“, werden die Zusatzfunktionen (Doppelklick / linke Maustaste) nur zur Ansicht geöffnet, ansonsten zum Bearbeiten. |
| Mengenvorausberechnung aktiv | Steht die Einstellung auf „Nein“, wird die Mengenvorausberechnung im zweiten Grid unterdrückt.<br>Bei „Ja“ erfolgt eine Mengenvorausberechnung. Bei den Lieferungen werden stornierte Belege und die dazugehörigen Ursprungsbelege nicht mit einberechnet.<br>*Um die Mengen richtig zuzuweisen gilt grundsätzlich: Eine Zeile mit eingetragenem Kunden ist keine Vorausberechnungszeile.* |
| Vorausberechnungszeile anzeigen | Steht diese Einstellung auf „Ja“, wird bei der Mengenvorausberechnung in der mittleren Datentabelle in die unterste Zeile gescrollt, damit diese immer sichtbar ist. |
| Mengensummenklasse / 0:alle | In diesem Grid können die Klassen eingetragen werden, welche bei der Mengenvorausberechnung mit einbezogen werden sollen.<br>Sind keine Klasse eingetragen, werden alle Klassen mit einbezogen. |
| Welche Partie zieht bei Übermenge | Diese Einstellung sagt aus, welche Partie bei einer Mengenerhöhung entsprechend angepasst wird. (Format „[Partie bei Übermenge](./formate.md#formatPartiebeiUebermenge)“ ) |
| Lademittel aktiv | Steht die Einstellung auf „Ja“, wird ein spezieller Ausdruck von Lademitteln (auf dem dritten Grid) ausgeführt. |
| Storno- und Ursprungsbeleg | Steht diese Einstellung auf „Ja“, werden in diesem Profil auch Vorgänge und die dazugehörigen Stornobelege angezeigt. |
| Kopierfunktion deaktiviert | Steht diese Einstellung auf „Ja“, wird das Feld für die Anzahl von Kopien und die Kopierfunktion in der Optionbox nicht mehr angezeigt. |
| Rohwaredatum aus Plandatum | Steht diese Einstellung auf „Ja“, wird das Rohwaredatum des neuen Beleges nicht aus dem Feld „Belegdatum“ ermittelt, sondern aus dem „Plandatum“. |
| Umbuchung anzeigen | Hiermit kann eingestellt werden, ob Umbuchungen angezeigt werden sollen. |
| [Positionsbeziehungsübersicht](../positionsbeziehungsuebersicht.md) | Hiermit kann eingestellt werden, ob die Positionsbeziehungsübersicht aktiviert ist. |

<p class="just-emphasize">Registerkarte Allgemein 2 ![](../../../ImagesExt/image8_1357.png)</p>

Hier können weitere allgemeine Einstellungen vorgenommen werden.

Gültigkeit

In der Datentabelle der Gültigkeit lassen sich Vorschriften für das Anlegen und bearbeiten von Vorgängen innerhalb der Streckenerfassung festlegen.

Dabei sollte beachtet werden, dass sobald eine Gültigkeit eingetragen wurde, alle anderen nicht eingetragenen Kombinationen ungültig sind.

| Feld | Beschreibung |
| --- | --- |
| Klasse | Hier kann die Klasse eingetragen werden, wird nichts eingetragen, gilt diese Gültigkeit für alle Klassen. |
| Unterklasse | Hier kann die Unterklasse eingetragen werden, wird hier nichts eingetragen, gilt die Gültigkeit für alle Unterklassen der dazugehörigen Klasse. |
| Grid | Hier kann die Datentabelle für die Gültigkeit eingetragen werden, wird nichts eingetragen ist die Gültigkeit für alle Datentabellen gültig. |
| Streckenänderung | Hier wird festgelegt, ob der Vorgang innerhalb der Strecke geändert werden darf. |
| Neuanlage | Hier wird festgelegt, ob ein Vorgang mit der eingetragenen Klasse/Unterklasse/Datentabelle angelegt werden darf. |
| Vorgangsänderung | Hier kann festgelegt werden, ob ein Vorgang im Ansehen- oder Ändernmodus aufgerufen wird. |
| Vorgangszuordnung | Hier kann festgelegt werden, ob Vorgänge / Kontrakte zugeordnet werden dürfen. |
| Vorgangsstornierung | Hiermit wird festgelegt, ob ein Vorgang / Kontrakt storniert werden darf. |
| Vorgangspositionsstornierung | Hiermit wird festgelegt, ob eine Position im Vorgang / Kontrakt storniert werden darf. |

Einige Beispiele zur Gültigkeit können [hier](./beispiele.md#beispiele_gueltigkeit) gefunden werden.

Container

| Feld | Beschreibung |
| --- | --- |
| Containerbezeichnung | Hier kann die Containerbezeichnung eingegeben werden. Diese Bezeichnung wird dann in der Streckenerfassung für das Containerfeld angezeigt.<br>Wenn keine Containerbezeichnung angegeben wird, befindet sich auch kein Eingabefeld auf der Streckenerfassung. |
| Itembox für Container | Für das Containerfeld lässt sich hier eine Itembox festlegen. Wenn eine Itembox hinterlegt wird, muss der eingegebene Wert in der Itembox vorhanden sein, ansonsten kann jeder numerische Wert eingetragen werden. |

Die Itembox „IB_DISPO_KONTRAKTSTAMM“ / „IB_DISPO_KONTRAKTSTAMM_BEZ“ kann als Standarditembox für Kontrakte verwendet werden. (siehe dazu auch „[Itemboxparameter](./allgemeine_informationen.md#allgemein_info_Itembox_Parameter)“)

Kopieren

| Feld | Beschreibung |
| --- | --- |
| Prüfprozedur | Hier kann eine Prozedur hinterlegt werden, welche nach der Funktion „Strecke vervielfältigen“ aufgerufen wird. Übergabeparameter (IN-Parameter = „in_KlammerNr“) ist die ursprüngliche Nummer. |
| Prüfmakro | Hier kann ein Makro hinterlegt werden, welches nach der Funktion „Strecke vervielfältigen“ aufgerufen wird. Übergabeparameter ist die ursprüngliche Nummer. |

In der Prozedur kann auf die Nummern der erzeugten Klammern über ein SQL zugegriffen werden. In der Spalte „ident1“ ist die ursprüngliche Klammernummer enthalten. In der Spalte „ident2“ die neu erzeugte Klammernummer.

```sql
Select ident1, ident2
from gtt_amic_ident
where typ =
'5FE183B7-308D-4D66-BB14-A6333799D006'
```

Vorbelegung

In diesem Bereich lässt sich die Vorbelegung innerhalb der Streckenerfassung festlegen. Dafür stehen folgende Felder und Datentabellen zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Prozedur | Hier kann eine private Prozedur hinterlegt werden, welche die Vorbelegungswerte zurückgibt. |
| Feldname<br>*(Datentabelle „Übergabefelder“)* | In dieser Datentabelle kann der Feldname eines Übergabeparameterfeldes hinterlegt werden. Auf die Werte kann man innerhalb der privaten Prozedur zugreifen. *(exakte Schreibweise beachten)* |
| Feldname<br>*(Datentabelle „Ausführungsfelder“)* | In dieser Datentabelle stehen die Felder, welche beim Verlassen des Feldes die Vorbelegungsprozedur aufrufen. *(exakte Schreibweise beachten)* |

Neben den „Ausführungsfeldern“ wird die Prozedur auch beim ersten Betreten einer neuen Zeile aufgerufen.

Weitere Informationen und Beispiele befinden sich im Kapitel „[Vorbelegung](./vorbelegung.md)“.

<p class="just-emphasize">Registerkarte Griddefinition ![](../../../ImagesExt/image8_1357.png)</p>

Auf dieser Registerkarte können Einstellungen für die Grids in der Streckenerfassungsmaske vorgenommen werden. Dadurch kann z.B. die Zeilenanzahl, die Vorbelegung und weitere Einstellungen für die einzelnen Grids definiert werden.

Die Maske Streckenerfassung passt sich dann automatisch an die eingestellten Definitionen an.

<table class="AMIC-Tabelle" style="WIDTH: 100%; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="100%" border="0"><tbody><tr><td style="WIDTH: 31.22%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="31%"><p class="AMIC-Ueberschrift-Tabelle" style="TEXT-ALIGN: center" align="center"><span style="COLOR: white">Feld</span></p></td><td style="WIDTH: 68.78%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="68%"><p class="AMIC-Ueberschrift-Tabelle" style="TEXT-ALIGN: center" align="center"><span style="COLOR: white">Beschreibung</span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Zeilenanzahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Für jedes Grid kann hiermit die Zeilenanzahl separat eingestellt werden.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Vorgangsklasse</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Vorbelegung der Vorgangsklasse bei einer neuen Zeile im Grid.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Untervorgangsklasse</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Vorbelegung der Vorgangsunterklasse bei einer neuen Zeile im Grid.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Einzelvorgangsvorbelegung</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Vorbelegung für die Spalte Einzelvorgang „Evo.“</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Nur positive Partiemengen</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Steht die Einstellung auf „Ja“, sind nur positive Partiemengen möglich.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Lagerauswahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann festgelegt werden aus welchem Lager die Itembox der Spalte „Artikel“ den entsprechenden Artikel sucht. (Format „<a class="topic-link" href="./formate.md#formatLagerauswahl">Lagerauswahl</a>“)</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Artikelauswahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Je nach Einstellung wird bei der Artikelauswahl der Artikel gesucht. (Format „<a class="topic-link" href="./formate.md#formatLagerbezogeneArtikelauswahl">Lagerbezogene Artikelauswahl</a>“)</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Label Gridüberschrift</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier können die Gridüberschriften der einzelnen Grids festgelegt werden. Bei einer leeren Gridüberschrift wird die Standardeinstellung verwendet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Partiemengenverteilung aktiv</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Steht die Einstellung auf „Nein“, wird die Maske „<a class="topic-link" href="../maske_partiemengenverteilung.md"><span class="AMIC-Textkoerper-LinkZchn">Partiemengenverteilung</span></a>“ nur zur Ansicht geöffnet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Mengenänderung zulassen</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann mit der Einstellung „Ja“ definiert werden, dass Mengenänderungen zulässig sind.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Gridaktualisierung bei Mausklick</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Mit dieser Einstellung kann gesteuert werden, ob bei einem Mausklick in das Grid eine Aktualisierung der anderen Grids je nach Einstellung der jeweiligen Einstellung „<i>Aktualisierung im Grid</i>“ ausgeführt wird.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Aktualisierung im Grid</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Diese Einstellung steuert, ob das Grid aktualisiert wird, wenn die Einstellung „<i>Gridaktualisierung bei Mausklick</i>“ aktiv ist.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Erweiterte Kontraktanzeige</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Wird dieser Schalter auf „Ja“ gestellt, so werden bei der Kontraktauswahl in der Strecke alle Artikel eines Kontraktes inklusiv der dazugehörigen Kontraktausweichliste angezeigt. In der Artikelauswahl werden dann nur noch die Artikel des Kontraktes, so wie die der Ausweichliste angezeigt.</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Die Itembox „IB_Kontraktasuwahl_Strecke“ wird fest verwendet und übersteuert die im Feld IB-Kontraktauswahl hinterlegte Itembox.</span><span style="FONT-SIZE: 11pt"></span></p><p class="MsoNormal"><span style="COLOR: black">Die Itembox „I</span><span class="AMIC-TextkoerperZchn"><span style="COLOR: black">B_Strecke_KontraktArtikel_Erweitert“ wird fest verwendet und übersteuert die im Feld IB-Artikelauswahl hinterlegte Itembox.</span></span><span class="AMIC-TextkoerperZchn"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Kontraktzuordnung im Vorgang</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="MsoNormal"><span style="FONT-SIZE: 11pt; COLOR: black">In dieser Einstellung kann bestimmt werden, ob eine automatische Zuordnung erfolgt oder keine Zuordnung stattfinden soll.</span><span style="FONT-SIZE: 11pt"></span></p><p class="MsoNormal"><span style="FONT-SIZE: 11pt; COLOR: black">Wird bei der Anlage eines Vorgangs unabhängig von dieser Einstellung ein Kontrakt manuell angegeben, wird dieser in jedem Fall verwendet.</span><span style="FONT-SIZE: 11pt"></span></p><p class="MsoNormal"><span style="FONT-SIZE: 11pt; COLOR: black">Die Einstellung kann für jedes Erfassungsgrid separat vorgenommen werden.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">IB-Artikelauswahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alternative Itembox für die Artikelauswahl eingetragen werden. Ist nichts eingetragen, wird die Standarditembox verwendet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">IB-Kundenauswahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alternative Itembox für die Kundenauswahl eingetragen werden. Ist nichts eingetragen, wird die Standarditembox verwendet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">IB-Kontraktauswahl</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alternative Itembox für die Kontraktauswahl einer Position ausgewählt werden. Um eine ähnliche Auswahl, wie in der normalen Vorgangserfassung zu haben, sollte die Itembox &nbsp;„IB_KONTRAKTAUSWAHL_STRECKE“ verwendet werden.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">IB-Belegnummernauswahl Vorgang</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alternative Itembox für das Feld „Belegnummer“ bei Vorgängen hinterlegt werden. Ist nichts eingetragen, wird die Standarditembox verwendet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">IB-Belegnummernauswahl Kontrakt</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eine alternative Itembox für das Feld „Belegnummer“ bei Kontrakten hinterlegt werden. Ist nichts eingetragen, wird die Standarditembox verwendet.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.22%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Artikelinformationen für Itembox</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 68.78%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="68%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Hier kann eingestellt werden, ob vor dem Aufruf der Artikelauswahl-Itembox die Artikel-IDs aller Datentabellen in die temporäre Tabelle „GTT_AMIC_IDENT“ geschrieben werden sollen.</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">In der Tabelle „GTT_AMIC_IDENT“ stehen dann folgende Spalten zur Verfügung:</span><span style="FONT-SIZE: 11pt"></span></p><table class="MsoNormalTable" style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; BORDER-COLLAPSE: collapse; BORDER-BOTTOM: medium none; BORDER-LEFT: medium none" cellspacing="0" cellpadding="0" border="1"><tbody><tr><td style="BORDER-TOP: #548dd4 1pt solid; BORDER-RIGHT: medium none; WIDTH: 53.9pt; BORDER-BOTTOM: #4f81bd 2.25pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: #548dd4 1pt solid; PADDING-RIGHT: 5.4pt" valign="top" width="72">Spalte</td><td style="BORDER-TOP: #548dd4 1pt solid; BORDER-RIGHT: #548dd4 1pt solid; WIDTH: 270.5pt; BORDER-BOTTOM: #4f81bd 2.25pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="361">Beschreibung</td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 53.9pt; BACKGROUND: #c6d9f1; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: #4f81bd 1pt solid; PADDING-RIGHT: 5.4pt" valign="top" width="72"><span style="COLOR: black">TYP</span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 270.5pt; BACKGROUND: #c6d9f1; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="361"><span style="COLOR: black">Hier steht der Wert „VORGANGSMAPPE_GRID_ARTIKEL“ drin, über diesen Typ werden alle Artikel-IDs geholt.</span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 53.9pt; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: #4f81bd 1pt solid; PADDING-RIGHT: 5.4pt" valign="top" width="72">IDENT1</td><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 270.5pt; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="361">Enthält die Datentabellennummer des Artikels.</td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 53.9pt; BACKGROUND: #c6d9f1; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: #4f81bd 1pt solid; PADDING-RIGHT: 5.4pt" valign="top" width="72"><span style="COLOR: black">IDENT2</span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: #4f81bd 1pt solid; WIDTH: 270.5pt; BACKGROUND: #c6d9f1; BORDER-BOTTOM: #4f81bd 1pt solid; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="361"><span style="COLOR: black">Enthält die Artikel-ID</span></td></tr></tbody></table><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Zusätzlich wird der ITEM-PARAMETER „VMAPPE_AKTUELLES_GRID“ angelegt. Dieser enthält die Nummer der Datentabelle aus der die Itembox aufgerufen wurde.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr></tbody></table>

Zudem gibt es hier die Möglichkeit für die Kalkulation Daten zu hinterlegen.

| Feld | Beschreibung |
| --- | --- |
| Anwendung | ANWID der Anwendung, nur wenn eine Anwendung eingetragen ist, wird auch die Funktion in der Streckenerfassung angezeigt. |
| Variante | Variante die zur Anwendung gehört. |
| Profil | Name des Profils. |

Des Weiteren kann eine private Funktion angelegt werden, in der Anwendung / Variante / Profil angegeben werden können. Der Controlstring muss dabei wie folgt aussehen.

```text
^jpl
call_streckenkalkulation AnwendungID Variantenname Profilname
```

Diese Funktion wird dann mit in die Menüs der Streckenerfassung eingetragen.

<p class="just-emphasize">Registerkarte Auswertungen ![](../../../ImagesExt/image8_1357.png)</p>

Die Registerkarten für den AMIC Etikettendruck und „Crystal Reports“ sind der Registerkarte „Auswertungen“ gewichen. Statt wie bisher eine feste Anzahl von Auswertungen zu haben, können nun unterschiedlich viele Reports für die einzelnen Bereiche hinterlegt werden.

Beim Anlegen einer neuen Auswertung, wird die neue Funktion automatisch dem Untermenü „Drucken“ zugeordnet. Dabei sollte beachtet werden, dass bei bereits existierenden Funktionen das Untermenü nicht angepasst wird.

| Feld | Beschreibung |
| --- | --- |
| Reporttyp | Der Reporttyp entscheidet, welche Reporte im Feld Report zur Verfügung stehen. (Format „[Reporttyp](./formate.md#formatReporttyp)“) |
| Auswertungstyp | Der Auswertungstyp legt fest, welche Daten für die Auswertung zur Verfügung gestellt werden. (Format „[Auswertungstyp](./formate.md#formatAuswertungstyp)“) |
| Report | Identifikation des Reports |
| Reportbezeichnung | Bezeichnung des Reports |
| Vorgangsdruck | |
| Druckklassenbezeichnung | |
| Menübezeichnung | Bezeichnung des Reports wie sie im Kontextmenü der Streckenerfassung angezeigt werden soll. |
| Senden An | Steht dieses Feld auf „Ja“, wird der Report nicht angezeigt, sondern sofort gedruckt und im Archiv gespeichert. Der Archiveintrag wird dann mit dem „[Archiv Mail Versand](../../../dokumentenverwaltung/archiv_manager/archiv_mail_versand/index.md)“ aufgerufen. *(Gilt nur bei Crystal Report)* |
| Vorgangsklasse | |
| Vorgangsunterklasse | |
| Vorgangsunterklassenbezeichnung | |
| Formular | |
| Formularbezeichnung | |
| Drucker | |
| Druckerbezeichnung | |
| Anzahl | |

<p class="just-emphasize">Registerkarte Kontrakte ![](../../../ImagesExt/image8_1357.png)</p>

Auf dieser Registerkarte können die Voreinstellungen für Kontrakte vorgenommen werden.

| Feld | Beschreibung |
| --- | --- |
| Einzeilige Kontraktauswahl | Hier kann eingestellt werden, ob die Kontrakte ein-/ oder mehrzeilig dargestellt werden sollen. Bei der einzeiligen Darstellung können die Kontrakte innerhalb der Strecke nicht angelegt oder bearbeitet werden. |
| Einzel- / Gesamtmengen | Hier kann der verwendete Einzel-/Gesamtmengentyp eingestellt werden. (Format „Einzel-/Gesamtmengen“) |
| Mengen- / Wertkontrakt | Hier kann die verwendete „Mengen-/Wert“ Art eingestellt werden. (Format „Mengen-/Wertkontrakt“) |
| Standardkontraktvariante | Hiermit kann die Standardkontraktvariante vorbelegt werden. (Format „Standardkontraktvariante“ |
| Variante-Druck | Hier wird die Variante für den Kontraktdruck vorbelegt. |
| Anzahl Zeiteinheiten | Hier kann die Anzahl der Kontraktzeiträume festgelegt werden. |
| Zeiteinheit | Diese Einstellung legt fest um was für eine Kontraktzeitraumaufteilung es sich handelt. (Format „[Zeitintervall eines Kontrakts](./formate.md#formatZeitIntervalleinesKontraktes)“) |
| Plandatum als Bis-Datum | Hier kann eingestellt werden, ob das Plandatum bei der Kontraktneuanlage als Bis-Datum des Kontrakts verwendet werden soll. Als Voraussetzung ist, dass die Einstellung „Anzahl Zeiteinheiten“ den Wert „0“ hat. |
| Tag des aktuellen Monats | Mit dieser Einstellung kann der Starttag des Kontrakts vorbelegt werden. |
| Kontraktwährung übernehmen | Wird bei der Belegerfassung ein Kontraktausgewählt, kann hiermit festgelegt werden, dass die Kontraktwährung übernommen wird.<br>Dafür muss das „[UFLD-Feld](./index.md#registerbenutzerfelder)“ für die [Währung (1453)](./ufld_feldliste.md#verwendbareUFLDFelder) angezeigt werden. |
| Immer vor Laufzeit buchen | Hier kann eingestellt werden, dass ein Kontrakt auch vor der Laufzeit gebucht werden kann, unabhängig vom Steuerparameter [150](../../../firmenstamm/steuerparameter/kontraktwesen/vor_anfang_der_laufzeit_bebuchbar_spa_150.md). |

<p class="just-emphasize">Registerkarte Kopiervorlagen ![](../../../ImagesExt/image8_1357.png)</p>

Auf der Registerkarte „Kopiervorlagen“ lassen sich Vorlagen zum Kopieren einzelner oder mehrerer Spalten definieren. Dabei können Spalten von einer Datentabelle in eine andere oder in die gleiche Datentabelle kopiert werden.

| Feld | Beschreibung |
| --- | --- |
| Menübezeichnung | Hier kann die Bezeichnung der Kopierfunktion eingetragen werden. Dabei ist zu beachten, dass die Bezeichnung nicht existieren darf. |
| Startgrid | Hier kann eingestellt werden, aus welcher Datentabelle die Daten kopiert werden. |
| Zielgrid | Dies ist die Zieldatentabelle, wo die Daten hin kopiert werden. |
| Zielfeld | Hier kann das Feld angegeben werden, in dem man nach dem Kopieren landet. |

Drei weitere Grids geben die Möglichkeit zur Eingabe von

• Zu kopierenden Feldern der aktuellen Kopierfunktion

• Zu kopierende UFLD-Felder, die in beiden Grids zur Verfügung stehen

• Zu kopierende AddOn-Felder, die in beiden Grids zur Verfügung stehen

<p class="just-emphasize">Registerkarte Benutzerfelder ![](../../../ImagesExt/image8_1357.png)</p>

Auf dieser Registerkarte stehen drei Grids zur Verfügung um für jedes der drei Grids dynamisch UFLD-Felder anzulegen. Für die UFLD-Felder mit einer Itembox werden in der Streckenerfassungsmaske Bezeichnerfelder hinzugefügt.

Hierfür stehen auf jedem der Grids folgende Felder zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Feld | Nummer des UFLD-Feldes. |
| Bezeichnung | Bezeichnung für das Feld, dieser wird beim Auswählen des Feldes vorbelegt, kann jedoch angepasst werden. |
| Erfassungslevel | Hier muss der Erfassungslevel für das UFLD-Feld festgelegt werden. (Format „Erfassungslevel“) |

Damit die Daten auch in der Streckenerfassungsmaske geladen werden muss die Ladeprozedur des Profils angepasst werden. Die bisher verwendbaren UFDL-Felder stehen in der Liste „[Streckenerfassung UFLD-Feldliste](./ufld_feldliste.md)“.

<p class="just-emphasize">Registerkarte Addonfelder ![](../../../ImagesExt/image8_1357.png)</p>

| Feld | Beschreibung |
| --- | --- |
| Addonfelder aktivieren | Aktiviert oder deaktiviert die Anzeige der Addonfelder. |
| Herkunftsdaten-View (Grid 1) | Hier kann eine View hinterlegt werden, mit der die Felder und Daten für der Addonfelder ermittelt werden. Sollte keine View angegeben worden sein, wird die Tabelle „WARENBEWEGUNGADDON“ verwendet.<br>Um für eine bestimmte Datentabelle keine Addonfelder anzulegen, kann die View „STRECKE_EMPTY_WABEWADDON_VIEW“ verwendet werden.<br> <br>Weitere Informationen für die Felder können in den Tabellenaddons für die Tabelle „WARENBEWEGUNGADDON“ gepflegt werden. |
| Herkunftsdaten-View (Grid 2) | Siehe „Herkunftsdaten-View (Grid 1)“ |
| Herkunftsdaten-View (Grid 3) | Siehe „Herkunftsdaten-View (Grid 1)“ |

<p class="just-emphasize">Registerkarte Buttons ![](../../../ImagesExt/image8_1357.png)</p>

Auf dieser Registerkarte kann eingestellt werden, welche Buttons ausgeblendet werden soll.

| Feld | Beschreibung |
| --- | --- |
| Name | Name des Buttons |
| Anzeigen | Hier kann eingetragen werden, ob der Button „nicht“ angezeigt werden soll. |

<p class="siehe-auch">Siehe auch:</p>

- [Allgemeine Informationen](./allgemeine_informationen.md)
- [Beispiele](./beispiele.md)
- [Vorbelegung](./vorbelegung.md)
- [UFLD-Feldliste](./ufld_feldliste.md)
- [Formate](./formate.md)
