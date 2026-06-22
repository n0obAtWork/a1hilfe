# zu Stapel hinzufügen

<!-- source: https://amic.de/hilfe/zustapelhinzufgen.htm -->

Durch Aufruf der Funktion „***zu Stapel hinzufügen***“ **Strg+F8** werden automatisch die ausgewählten Datensätze zu einem Stapel hinzugefügt. Es ist dazu nicht zwingend notwendig, vorher einen Stapel anzulegen. Dabei wird nach folgender Logik der Stapel bestimmt:

1. Existiert noch kein Stapel, so wird automatisch ein Stapel mit dem Namen „Stapel_1“ angelegt.

2. Existiert genau ein Stapel, so wird dieser verwendet.

3. Ansonsten öffnet sich eine F3-Auswahl, aus der man einen der Stapel auswählen kann.

Sind Vorgänge der Warenwirtschaft in einem Stapel zusammengefast, so werden bei bestimmten Aktionen – z.B. Umwandeln Rechnung aus Lieferschein – automatisch Vorgänge dem Stapel hinzugefügt Wenn man also aus der Stapelverarbeitung heraus einen Lieferschein in eine Rechnung umwandelt, so wird die Rechnung automatisch dem ausgewählten Stapel zugeordnet. Ist ein Stapel aktiv – unabhängig, ob man im Bearbeitungsmodus der Stapelverarbeitung ist oder nicht - und man bearbeitet einen Vorgang, so wird dieser Vorgang automatisch dem Stapel hinzugefügt.

Zusätzlich sind Varianten zur Vorgangsbearbeitung so eingerichtet, dass Vorgänge, die einem Stapel zugeordnet sind, mit einem Icon **![](../../../ImagesExt/image8_1305.png)**gekennzeichnet werden. Dies wir über FIELD mit dem Name **Stapelicon** und dem [Feldtyp](../../private_varianten_und_sql_texte/feldtyp_im_sql_text.md) **ICON** gesteuert. Dabei erkennt die Auswahlliste an dem Name **Stapelicon**, dass es sofort das Icon darstellen soll, und zwar ohne die Daten komplett neu zu laden.

```text
FIELD
,StapelICON,ICON,2,TIPTEXT="Zeigt an, ob der Vorgang in einem
Stapel ist."
```

Diese Kennzeichnung kann mit dem Steuerparameter 1175 „Stapel in Anwendungen anzeigen“ deaktiviert werden.
