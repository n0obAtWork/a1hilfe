# SKR03 / SKR04 Übernehmen

<!-- source: https://amic.de/hilfe/skr03skr04bernehmen.htm -->

**A.eins** bietet die Möglichkeit, die DATEV – Kontenpläne SKR03 und SKR04 zu übernehmen. Man erreicht den Programmpunkt über die Direktsprünge **[SKR03]** bzw. **[SKR04]**.

Beim Einspielen müssen folgende Punkte beachtet werden:

1. Die Einspielung kann nur dann geschehen, wenn noch keine Buchungen in der Fibu erfolgt sind. Eventuell alte Testdaten müssen vorher gelöscht werden. Die kann mit dem [Nullsetzer](../../zusatzprogramme/a_eins_nullsetzer/index.md) geschehen.

2. Durch die Einspielung werden die Sachkonten, ggf. Oberkonten, Druckpositionen, Forderungsgruppen, Erlöskennziffern und Steuersätze gelöscht und neu eingetragen. Einige Stammdaten müssen eventuell angepasst werden:

• Hausbanken

• Zinsgruppen / Zinsabschlag Stammdaten

• Mahnsätze

• Währungskurse

• Wechselbuchhaltung

• Mandantenstamm

• Erlöszuordnungen

• Forderungsgruppen

• Steuern

3. Überprüfen Sie die Zählkreise.  
Der Oberkontenzählkreis liegt nach der Einspielung im Bereich 900.000 - 999.999, der Sachkonten-Zählkreis im Bereich 1 - 9999.

Neben dem Kontenrahmen können auch zusätzliche Stammdaten übernommen bzw. angepasst werden.

| Tabelle | Beschreibung |
| --- | --- |
| Druckpositionen | Wird diese Option gewählt, so werden bestehende Druckpositionen gelöscht und durch die Druckpositionen für den SKR03/04 ersetzt. Ansonsten wird bei alle Druckpositionen im Sachkontenrahmen die 0 eingetragen.  
 |
| Oberkonten | Oberkonten können genau wie die Druckpositionen übernommen werden. D.h. alte Oberkonten werden gelöscht und durch die Oberkonten des SKR03/04 ersetzt. Will man die Oberkonten nicht übernehmen, so wird im Sachkontenstamm bei allen Oberkonten eine 0 eingetragen.  
 |
| Forderungsgruppen | Hier können die Forderungsgruppen des SKR03/04 übernommen werden. Dies geschieht nur, wenn nicht bereits Forderungsgruppen im Kundenstamm hinterlegt sind, die in den SKR03/04-Daten nicht existieren. Es erscheint in diesem Fall eine Fehlermeldung und die gesamte Übernahme wird nicht durchgeführt.  
 |
| Erlöskennziffern | Erlöskennziffern können dann übernommen (löschen der alten Daten und Eintragen der neuen), wenn alle Erlösklassen, Steuerschlüssel und Erlöskennziffern, die im SKR03/04 verwendet werden, als Stammdaten existieren. Auch dürfen im Artikel bzw. Artikelstamm keine Erlöskennziffern verwendet werden, die nicht den Daten des SKR03/04 entsprechen. Wenn eine dieser Bedingungen nicht erfüllt ist, so erscheint eine Fehlermeldung und die gesamte Übernahme wird nicht durchgeführt.  
 |
| Steuersätze | Es wird geprüft, ob alle bisher verwendeten Steuergruppen und Steuerschlüssel auch in den Steuerdaten des SKR03/04 existieren. Ist die nicht der Fall, erscheint ein Hinweis und die Übernahme wird nicht durchgeführt.  
 |
| Auswertungspositionen | Die in den Steuersätzen verwendeten Auswertungspositionen für die Umsatzsteuervoranmeldung können mit übernommen werden. Werden zwar die Steuern, jedoch nicht die Auswertungspositionen übernommen, so werden die entsprechenden Felder im Steuersatz auch nicht übernommen.  
 |
| Taxonomien | Hier wird die Kontozuordnung zu den Taxonomie eingespielt. Diese Kontozuordnung ist nur als Datengrundlage zu verstehen und muss noch angepasst werden.  
 |
| Mandantenstamm | Hier werden nur die Konten im Mandantenstamm ersetzt. Wählt man **Nein**, so werden die Konten auf 0 gesetzt.  
 |
| Währungsstamm | Die Kursgewinn-, Kursverlust- und das Ausgleichskonto werden ersetzt oder bei **Nein** auf 0 gesetzt. Es wird für alle eingetragenen Währungen jeweils das gleiche Kursgewinn-, Kursverlust- bzw. Ausgleichskonto verwendet.  
 |
| Mahnsätze | Das Konto wird ersetzt oder bei **Nein** auf 0 gesetzt. Es wird immer dasselbe Konto für jeden Mahnsatz eingetragen,  
 |
| Zinsgruppen | Die Konten werden ersetzt oder bei **Nein** auf 0 gesetzt. Es werden immer dieselben Konto für jeden Mahnsatz eingetragen,  
 |

Neben den hier aufgeführten Relationen werden noch die Konten im Hausbankenstamm, in den Wechselkosten und in den Zinsabschlagsstammdaten auf 0 gesetzt. Hierfür existieren keine Übernahmedaten.
