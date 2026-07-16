# Mein Tracefile in der Datenbank

<!-- source: https://amic.de/hilfe/meintracefileinderdatenbank.htm -->

Direktsprung [**TRAW**]

Mit Hilfe dieser Variante lassen sich auf übersichtliche Weise Recherchen innerhalb eines eingelesenen Tracefile anstellen.

Zum jetzigen Zeitpunkt muss dazu über [OSQL] das entsprechende Tracefile eingelesen werden. Der Inhalt wird dann in dieser Variante zur Ansicht und Auswahl bereitgestellt.

| Felder | Auswahlliste |
| --- | --- |
| wer | Bediener |
| Zeit | Zeitstempel |
| CurNo | |
| Maske | Aeins-Maske |
| Verbrauch | Anweisungsdauer in Millisekunden |
| Err | Rückgabe-Status der Anweisung<br> <br>So bedeutet „100“ z.B. dass der selektierte Datensatz nicht gefunden wurde. Es kommt sehr auf den jeweiligen Kontext an um zu beurteilen ob das ein Fehler ist oder nicht! |
| SQL Ausdruck | Informatorisch die SQL-Anweisung<br> <br>Achtung, die wirkliche SQL-Anweisung kann wesentlich länger sein als das was in der Auswahlliste aus technischen Gründen maximal angezeigt werden kann! |
| Plan | Datenbank-Ausführungsplan der Anweisung |
| Status | Technischer Aeins-Status |
| Id | Technischer Schlüssel |

| Auswahlbedingungen |
| --- |
| SQL Textschnipsel wie |
| Nur Verbrauch > … ms |
| Nur Fehlercode &lt; … |
| Nur Maske wie |

| Funktionen | |
| --- | --- |
| Ansehen [F6] | Dialog „Traceeintrag“ |
| Löschen [F7] | Löscht selektierte Einträge |

<p class="siehe-auch">Siehe auch:</p>

- [Dialog „Traceeintrag“](./dialog_traceeintrag.md)
