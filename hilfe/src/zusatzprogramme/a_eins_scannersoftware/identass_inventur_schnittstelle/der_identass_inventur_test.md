# Der Identass Inventur Test

<!-- source: https://amic.de/hilfe/_identassinventurtest.htm -->

Für die Identass Inventur ist es möglich eine private Ableitung der Standard Scanner Prozedur einzurichten oder eine eigene private Prozedur zu verwenden.

Um diese zu testen, oder auch um überhaupt zu testen, besteht die Möglichkeit in der Mobilen Datenerfassung (MDE) mit der Funktion **Identass Inventur Test** Erfassungen vorzunehmen, ohne die Identass Software zu verwenden. Hier ist einzig A.eins aktiv.

Im Vorfeld richten Sie die Steuerparameter 809 und 810 sowie den Externen Namen des Bedieners wie bereits beschrieben ein. Gleiches gilt für die Einrichtung der Inventur und die Zuweisung über [VKONS].

Anschließend müssen Sie noch einen weiteren Steuerparameter einrichten. Im Steuerparameter 801 halten Sie fest, welche Prozedur verwendet werden soll. Hier haben Sie folgende Möglichkeiten:

- Nein
- Private Prozedur
- Indentass Inventur

Wählen Sie die gewünschte Einstellung und weisen Sie dieser dann die Scanner IP zu. Wird eine private Prozedur verwendet, so tragen Sie den Namen der Prozedur in das vorgesehene Feld des Steuerparameters ein.

Mit dem Funktionsaufruf für den Identass Inventur Test starten Sie eine neue Maske. Diese Maske simuliert den Scanner. Sie erkennen mehrere Felder.

| Feld | Wert |
| --- | --- |
| Zeit in ms | Benötigte Zeit in Millisekunden für Bearbeitung des Scann Vorgangs. |
| Artikel EAN | Eingabe einer Artikel EAN über ein geeignetes Eingabegerät (Scanner, Tastatur, usw.). |
| Menge | Angegebene Menge |
| Fehlercode | Zeigt eine Fehlernummer an. Die Null (0) steht für OK. |
| Fehlertext | Zeigt den Fehlertext zur Fehlernummer an. Bei Fehlercode Null (0) erscheint hier die Artikelnummer und dessen Bezeichnung.<br>Diese Daten können je nach Verwendung privater Prozeduren abweichen! |

Öffnen Sie nun zunächst die Einrichterparameter und geben für den Parameter „Scanner-ID“ den passenden Wert ein, den Sie auch in das Feld **Externer Name** und in den Steuerparametern verwenden.

Nun ist die Testmaske für Ihre Tests eingerichtet.
