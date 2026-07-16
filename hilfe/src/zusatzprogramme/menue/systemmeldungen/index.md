# Systemmeldungen

<!-- source: https://amic.de/hilfe/systemmeldungen1.htm -->

Administration > Menü > Systemmeldungen

oder Direktsprung [**MENU**]

Systemmeldungen sind gemäß folgenden Regeln aufzubauen. Diese Regeln werden von A.eins zur Laufzeit geprüft, und die entsprechenden Systemmeldungen werden dann im [Hauptmenü-Systemmeldebereich](../das_hauptauswahlmenue/hauptmenue_systemmeldebereich.md) dargestellt.

Die Systemmeldungen sind über [Steuerparameter 893](../../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/systemmeldungen_dynamisch_spa_893.md) zu parametrisieren.

| **Felder** | |
| --- | --- |
| Name | Eindeutiger technischer Name der Systemmeldung. |
| Aktiv | Ja/Nein<br>Bestimmt ob die Systemmeldung überhaupt aktiv sein soll, d.h. ob die Bedingungen für eine Anzeige beim Programmstart überhaupt geprüft werden sollen. |
| Beschriftung | Die explizite Beschriftung der Systemmeldung.<br>Hinweis: Es handelt sich hierbei nicht um die Beschriftung der Funktion. |
| Funktion | Die Funktion, die ausgeführt werden soll wenn ein User die Systemmeldung klickt.<br>Sie können hier private Funktionen anbinden. |
| Funktionsart | Informatorisch die Funktionsart. |
| Rolle | Die Anzeige der Systemmeldung und das Recht der Ausführung der Funktion hängt ab vom Rollenkontext der Funktion. |
| Exklusiv-User | Gemäß Rollenkontext kann es Bedienerklassen geben, denen die Systemmeldung vorlegt wird. (Rolle)<br>Durch Angabe eines Kurznamens lässt sich die Systemmeldung weiter einschränken. (Es kann auch durch komma-getrennte Liste von Kurznamen angegeben werden) |
| Sortierung | Kriterium für die Reihenfolge der Abarbeitung der Systemmeldungen. |
| Vorlage vorhanden? | Es existiert eine Kopiervorlage mit deren Hilfe Sie die Systemmeldung auf Standard-Auslieferung zurücksetzen können.<br>Ein „Ja“-Eintrag wird informatorisch gelb markiert, wenn sich das [Systemmeldungsstatement](./systemmeldungspfleger.md#Systemmeldungstatement) von dem der Vorlage unterscheidet. |

| **Funktionen** | |
| --- | --- |
| Pflege-Funktionen<br>[**F8**],[**F5**],[**F6**],[**F7**] | Neu, Ändern, Ansehen, Löschen |
| Funktionen ansehen/bearbeiten [**F11**] | Ruft den Anseh- bzw. Pflegedialog für die angegebene Funktion der Systemmeldung direkt auf. |
| Export | Funktion um Systemmeldungen für weitere Verwendung zu exportieren. |
| Rollenkontext [**SF5**] | Ruft den Rollenkontextpfleger für die angegebene Funktion der Systemmeldung auf. |
| Funktion Informationen [**F5**] | Ruft den Funktions-Informationsdialog für die angegebene Funktion der Systemmeldung auf. |
| Neu aus Vorlage … [**SF8**] | AMIC liefert Vorlagen für System-Meldungen aus.<br>Hier können Sie eine Vorlage auswählen. Sollte die Systemmeldung zur Vorlage schon existieren, dann können Sie die Standardwerte für gewisse Felder wiederherstellen. Existiert die Systemmeldung noch nicht, haben Sie die Möglichkeit im Systemmeldungspfleger die generierte Systemmeldung zu ändern. |
| Vorlage zeigen [SF6] | Zeigt die Vorlage im Ansichts-Modus an.<br>([Systemmeldungs-Vorlagepfleger](../systemmeldungen_vorlage/systemmeldungs_vorlage_pfleger.md)) |
| Vorlage bearbeiten [SF5] | Öffnet den Bearbeitungsdialog für die Vorlage.<br>(Steht ausschließlich der Entwicklung zur Verfügung)<br>([Systemmeldungs-Vorlagepfleger](../systemmeldungen_vorlage/systemmeldungs_vorlage_pfleger.md)) |
| Aktiv &lt;-> Nicht aktiv | Je nachdem wie der aktuelle Aktiv-Status ist, wird entweder von Ja auf Nein oder von Nein auf Ja geschaltet. |

| **Suchen** | |
| --- | --- |
| Name | Name like |
| Aktiv | Ja, Nein, egal |
| Beschriftung | Beschriftung like |
| Funktion | Funktion like |

<p class="siehe-auch">Siehe auch:</p>

- [Systemmeldungspfleger](./systemmeldungspfleger.md)
