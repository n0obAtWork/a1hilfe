# Zertifikate

<!-- source: https://amic.de/hilfe/_zertifikate.htm -->

Auf der Registerkarte „Zertifikate“ befinden sich folgende Bereiche.

[Zertifikate](./zertifikate.md#zertifikat_zertifikate)

[Nachhaltigkeit](./zertifikate.md#zertifikat_nachhaltigkeit)

Die [Nachhaltigkeit](../../vorgangsabwicklung/nachhaltigkeit/index.md) ist mit den Zertifikaten verknüpft. Das bedeutet zu einem Zertifikat können mehrere Nachhaltigkeiten gepflegt werden.

Dadurch lassen sich die Nachhaltigkeiten nach Jahren trennen. Für jedes Jahr werden Zertifikate und die dazugehörigen Nachhaltigkeitswerte eingetragen.

Zudem ist es möglich mehr als ein Zertifikat pro Jahr zu haben. Dadurch hat man die Möglichkeit zwei Zertifikate für unterschiedliche Artikel anzulegen oder zwei Zertifikate (z.B. eins für DE und eins für EU) mit demselben Artikel.

<p class="just-emphasize">Zertifikate</p>

Für die Verwaltung der Zertifikate eines Kunden steht eine Datentabelle zur Verfügung. Dabei können folgende Felder gepflegt werden.

| Feld | Beschreibung |
| --- | --- |
| **Zertifikate** | Hier kann der Typ des Zertifikats eingetragen werden.<br>Für die [Nachhaltigkeit](../../vorgangsabwicklung/nachhaltigkeit/index.md) gelten nur bestimmte Typen, die im Format [AF_NAHA_ZERT](../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md) nachgelesen werden können. |
| **Bemerkung** | Eigene Bemerkung / Beschreibung des Zertifikats oder z.B. die Nummer des Zertifikats. |
| **Gültig ab** | Ab wann das Zertifikat gilt, wird nichts eingetragen ist das Ab Datum unbegrenzt gültig. |
| **Gültig bis** | Bis wann das Zertifikat gilt, wird nichts eingetragen ist das Bis Datum unbegrenzt gültig. |
| **Zertifizierungsmethode** | Hier kann die Zertifizierungsmethode des Zertifikats eingetragen werden. (Format [AF_ZERTMETH](../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md)) |
| **Sortierung** | Bei mehreren Zertifikaten für den gleichen Zeitraum (z.B. DE und EU), kann hier die Sortierung eingetragen werden, damit zum Beispiel das DE Zertifikat immer zuerst ermittelt wird. |
| **Kategorie** | Hier kann die Kategorie des Zertifikats (Format [AF_ZERTKATEG](../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md)) eingetragen werden. |
| **Kontrollstelle** | Feld für zusätzliche Informationen (kann per [Einrichterparameter](../../firmenstamm/einrichterparameter/kunden_epa_tbkunstb.md) angezeigt werden) |
| **Kontrollnummer** | Feld für zusätzliche Informationen (kann per [Einrichterparameter](../../firmenstamm/einrichterparameter/kunden_epa_tbkunstb.md) angezeigt werden) |
| **Version** | Feld für zusätzliche Informationen (kann per [Einrichterparameter](../../firmenstamm/einrichterparameter/kunden_epa_tbkunstb.md) angezeigt werden) |
| **Level** | Feld für zusätzliche Informationen (kann per [Einrichterparameter](../../firmenstamm/einrichterparameter/kunden_epa_tbkunstb.md) angezeigt werden) |
| **Zusatzinformation** | Feld für zusätzliche Informationen (kann per [Einrichterparameter](../../firmenstamm/einrichterparameter/kunden_epa_tbkunstb.md) angezeigt werden) |

Für Nachhaltigkeitszertifikate müssen Zertifikate vom Typ 4 und 5 eingerichtet werden. Immer Typ 4 außer beim Systemkunden, den man in den Mandantstamm unter [MND] einträgt.

<p class="just-emphasize">Nachhaltigkeit</p>

Im Zusammenhang mit der [Nachhaltigkeit](../../vorgangsabwicklung/nachhaltigkeit/index.md) können in dieser Datentabelle unterschiedliche Status für diesen Kunden eingetragen werden. Dabei sind die Einträge dem aktuell ausgewählten Zertifikat zugeordnet. Je Zertifikat darf ein Artikelstamm in Kombination mit Anbauland nur einmal vorkommen, ansonsten wird eine Warnung ausgegeben.

Bei Eintragungen in dieser Datentabelle sollte beachtet werden, dass die Lizenz „[Nachhaltigkeit](../../firmenstamm/steuerparameter/lizenzen/nachhaltigkeit_lizenz_spa_715.md)“ aktiviert ist.

| Feld | Beschreibung |
| --- | --- |
| **Status** | Hier kann der Status der Nachhaltigkeit eingetragen werden. (Format [AF_NACHHSTAT](../../vorgangsabwicklung/nachhaltigkeit/stammdaten/formate.md)) |
| **Datum** | Informatorisches Datum |
| **Artikelstamm** | Artikelstamm für diesen Wert. Wird nichts eingetragen gilt der Wert für alle nicht eingetragenen Artikelstämme. |
| **Anbau THG-Wert** | Kundenindividueller Anbau THG-Wert für den Artikelstamm mit Anbauland |
| **Verarbeitung THG-Wert** | Kundenindividueller Verarbeitung THG-Wert für den Artikelstamm mit Anbauland |
| **Lieferung THG-Wert** | Kundenindividueller Lieferung THG-Wert für den Artikelstamm mit Anbauland |
| **Anbauland** | Nummer des [Anbaulandes](../../vorgangsabwicklung/nachhaltigkeit/stammdaten/faktor_thg_wert_anbauland.md#stamm_anbauland), welches als Vorbelegung an der Warenposition gelten soll. |
| **Indiv. THG-Werte** | Ja/Nein Feld. Standardmäßig Nein. Bei Nein lassen sich die kundenindividuellen THG-Werte nicht pflegen und sie werden in der Zeile rausgelöscht, wenn vorher welche drinstanden. Bei Ja werden die kundenindividuellen THG-Werte pflegbar. |
| **NUTS Nummer** | Darstellung der NUTS Nummer des Anbaulandes. |
| **Anbaulandbezeichnung** | Darstellung von Bezeichnung und Land des Anbaulandes. |
| **Auswahl** | Existieren zu diesem Kunden Nachhaltigkeitseinträge die keinem Zertifikat zugeordnet sind, können diese hier ausgewählt werden.<br>Beim Speichern werden die ausgewählten Werte dann dem aktuellen Zertifikat zugeordnet. |

Die Meldung „**Es existieren … nicht zugeordnete Nachhaltigkeiten.**“ bedeutet, dass Nachhaltigkeiten existieren, die keinem Zertifikat zugeordnet sind.

Um dieses Problem zu beheben, wählt man zuerst das Zertifikat aus, dem der Nachhaltigkeitseintrag zugewiesen werden soll. Danach wechselt man in das Feld „Auswahl“ der Datentabelle „Nachhaltigkeit“. Dort wählt man über die Itembox den Eintrag aus, welcher zugeordnet werden soll.

Falls mehrere Einträge existieren, wechselt man in die nächste Zeile und wählt wieder über das Feld „Auswahl“ einen Eintrag aus. Zum Schluss speichert man die Informationen am Kunden.
