# Kostenobjekte: Einrichtung

<!-- source: https://amic.de/hilfe/_kostenobjekte_einrichtung.htm -->

### Einrichtungsschritte

1. Um das Kostenobjekt verwenden zu können, ist die [Kostenobjekt-Lizenz](../../../firmenstamm/steuerparameter/lizenzen/kostenobjekt_lizenz_spa_1064.md) notwendig.

2. In der allgemeinen Nummernkreiszuordnung <strong>[MNDNK]</strong> können Kostenobjekte einem Nummernkreis zugeordnet werden.

3. Die Kostenobjekte müssen im [Stammdatenpfleger „Kostenobjekte“](./kostenobjekte_pfleger.md) angelegt werden.

4. Im [Sachkontenstamm](../../stammdaten_der_fibu/sachkonten.md) **[SKS]** kann in den jeweiligen GuV-Konten im Feld „Sperre Kostenobjekt“ aus folgenden Möglichkeiten gewählt werden:

   - <strong>Gesperrt:</strong> Es wird kein Kostenobjekt abgefragt.
   - <strong>Kann:</strong> Es kann ein Kostenobjekt eingeben werden.
   - <strong>Muss:</strong> Es muss ein Kostenobjekt eingegeben werden.
   - **Fest** Es wird nur das im Sachkontenstamm festgelegte Kostenobjekt verwendet.

Im Feld „Kostenobjekt“ kann hier die Nummer eines Kostenobjektes eingegeben werden, das bei der Belegerfassung automatisch vorgeschlagen wird.

5. Damit auch Rechnungen aus der Warenwirtschaft beim Fibu -Übertrag automatisch in die Kostenobjektrechnung eingetragen werden können, ist es nötig, [Kostenobjektgruppen](../kostenobjektgruppe.md) **[KSOBG]** zu definieren, in denen die Kostenobjekte des Artikels für Einkauf und Verkauf angegeben werden können.  
Diese werden dann im [Artikel](../../../artikelstamm_und_artikel/artikel/index.md) **[AR]** über die Funktion ***Kostenst./Statistik/Abteil*** gepflegt. Wird der Artikel im Vorgang angesprochen, so wird das entsprechende Kostenobjekt bebucht.

6. Im [Mandantenstamm](../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU) **[MND]** sollte ein Fehlerkostenobjekt eingerichtet werden. Dieses Kostenobjekt wird herangezogen, wenn zu einem GuV-Konto versehentlich kein Kostenobjekt hinterlegt ist und die „Sperre Kostenobjekt“ des angesprochenen Kontos nicht auf **Gesperrt** oder **Fest** steht.

### Alternatives Label

In dem optionalen Parameter „Kostenobjekt_Label“ <strong>[OPT]</strong> kann ein alternatives Label für das Kostenobjekt eingetragen werden. Wird hier ein alternatives Label erfasst, so wird in den Labeln der Masken anstelle „Kostenobjekt“ der in den Optionen eingetragene Wert angezeigt. Das gleiche gilt für die Spaltenüberschriften in den Auswahllisten. Hiervon ausgenommen sind die Bezeichnungen der Auswahllisten und ihrer Varianten sowie die Bezeichnungen bzw. Beschriftungen von Funktionen im Hauptmenü, sie bleiben unverändert.

Wird das alternative Label aus den Optionen entfernt, so werden wieder die ursprünglichen Label in den Masken und Spaltenüberschriften in den Auswahllisten angezeigt.

**Hinweis zur Belegart „KO Kostenobjektumbuchung“**

Ist in der Option „Kostenobjekt_Label“ ein alternatives Label vorhanden, so wird dieses bei der Darstellung der Belegart berücksichtigt. Wurde in der Option z.B. „Projekt“ eingetragen, so wird anstelle der „Kostenobjektumbuchung“ die „Projektumbuchung“ u.a. in der [Belegerfassung](../../belegerfassung/index.md) und in den [FiBu-Nummernkreisen](../../stammdaten_der_fibu/nummernkreise/index.md) angezeigt. Die Kurzbezeichnung „KO“ bleibt jedoch unverändert.
