# Buchungsadministratoren

<!-- source: https://amic.de/hilfe/_buchungsadmin.htm -->

Hauptmenü > Administration > Geschäftsjahr / Perioden

oder Direktsprung **[PERBA]**

In gesperrte Perioden kann nicht mehr gebucht werden. Um nachträglich Buchungen in bereits gesperrte Perioden durchzuführen, müssen diese erneut geöffnet werden. Dabei werden die Perioden für alle Bediener freigegeben.

In dem Modul “Buchungsadministratoren“ können gezielt Bediener bestimmt werden (sog. Buchungsadministratoren), die in eine gesperrte Periode buchen dürfen. Die Periode muss dazu nicht wiedereröffnet werden. Für Buchungsadministratoren verhält sich die gesperrte Periode wie eine geöffnete Periode. Für alle anderen Bediener bleibt die Periode weiterhin bis zur Wiedereröffnung für jegliche Buchungen gesperrt.

Buchungsadministratoren werden für eine spezielle Periode angelegt. Das bedeutet, dass der Buchungsadministrator nur in die gesperrte Periode buchen kann, für die er angelegt wurde. Soll ein Bediener die Berechtigung für mehrere Perioden haben, so muss er für jede einzelne Periode als Buchungsadministrator eingetragen werden.

<details>
<summary>Felder der Buchungsadministratoren</summary>

| **Bezeichnung** | **Beschreibung** |
| --- | --- |
| Periodenbereich | Gibt an, ob die Periode für die Ware oder für Fibu ist. |
| Wirtschaftsjahr | Das Wirtschaftsjahr, auf das sich diese Periode bezieht. |
| Periode | Gibt die Periode als Monatszahl aus |
| Bezeichnung | Gibt die Periode als Monatsnamen aus |
| Bedienerkurzname | Zeigt an, welcher Bediener der Periode als Buchungsadministrator zugewiesen wurde. (Kürzel) |
| Bedienername | Zeigt an, welcher Bediener der Periode als Buchungsadministrator zugewiesen wurde. (Ganzer Name) |
| BedienerId | Zeigt an, welcher Bediener der Periode als Buchungsadministrator zugewiesen wurde. (Bediener ID) |

*Tipp: **Wenn man Bediener bei der Neuanlage nicht immer wieder neu eingeben möchte, so kann man mit der Funktion** **Speichern unter** **einen bereits bestehenden Datensatz als Vorlage nehmen.***

</details>

**Hinweis zu abgeschlossenen Perioden:**

*Man beachte, dass es für bereits abgeschlossene Perioden keine Möglichkeit gibt, weitere Buchungen vorzunehmen. Dies gilt auch für Buchungsadministratoren.*

<details>
<summary>Funktionalität</summary>

Folgende Funktionalitäten stehen den Buchungsadministratoren zur Verfügung:

- Buchungsadministratoren können in die gesperrte Periode buchen, für die sie angelegt wurden. Der Status der Periode bleibt dabei auf Buchungsschluss (gesperrt).
- Für Buchungsadministratoren verhält sich die gesperrte Periode wie eine geöffnete Periode:
  - Das Feld „Periode“ kann mit der gesperrten Periode vorbelegt werden.
  - Die gesperrte Periode kann für eine Buchung ausgewählt werden.
  - Ist an dem Feld eine **F3** - Auswahl angebunden, kann der Buchungsadministrator die gesperrte Periode via **F3** - Taste selektieren.
  - Der Buchungsadministrator bekommt weder beim Auswählen der Periode noch beim Buchen eine Meldung, dass es sich um eine gesperrte Periode handelt.

**Einschränkungen:**

Die Implementierung der Funktionalitäten erfolgte in den Standardanwendungen von A.eins. Für private Anwendungen und Funktionen gibt es keine Gewährleistung, dass diese Funktionalitäten zur Verfügung stehen.

</details>

<details>
<summary>Hinweis zu Perioden– und Inventurabschluss</summary>

Um eine Periode oder Inventur abzuschließen, müssen zunächst alle Buchungsadministratoren für die entsprechende Periode entfernt werden. Sonst kann der jeweilige Abschluss nicht erfolgen.

Sollten bei einem Perioden- oder Inventurabschluss noch Buchungsadministratoren eingetragen sein, wird man von der jeweiligen Maske darauf hingewiesen.

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Buchungsadministratoren: Pfleger](./buchungsadministratoren_pfleger.md)
