# Inventurstamm - Pfleger

<!-- source: https://amic.de/hilfe/_inventurstammpfleger.htm -->

<p class="just-emphasize">Hinweis</p>

Folgende Angaben sind wichtig:

Inventurgruppe: 1

Inventurstichtag: Erhebungsdatum

Bezeichnung: Freier Text

Typ der Inventur: 1 (Hauptinventur mit Jahreswechsel)

Art der Inventur: 1 (oder 2)

Erhebung am: Erhebungsdatum

<details>
<summary>Felder des Inventurstamm-Pflegers</summary>

<p class="just-emphasize">Identifikation</p>

| Feld | Beschreibung |
| --- | --- |
| Gruppe | Hier wird bestimmt, für welche Inventurgruppe die Inventur erfolgen soll.<br>Z. B. 1 = Hauptinventur JW, 10 = Teilinventur WG 10, 20 = Teilinventur WG 20 |
| Stichtag | Datum, zu dem die Inventur erfolgen soll. |

</details>

<details>
<summary>Information</summary>

| Feld | Beschreibung |
| --- | --- |
| Eröffnungsvortrag | Das Auswahlfeld gibt an, ob Eröffnungsvortrag gelaufen ist. |
| Vorläufig eingespielt | Das Auswahlfeld für Inventuren gibt an, ob die bereits vorläufig eingespielt worden sind. Dies ist auch für noch nicht abgeschlossene Inventuren möglich. |
| Abgeschlossen | Die Kennzeichnung, ob die Inventur abgeschlossen ist. Nur als abgeschlossene, gekennzeichnete Inventuren können endgültig eingespielt oder (bei Zwischeninventuren) gelöscht werden. |
| Eingespielt | Das Auswahlfeld, ob die Inventur bereits endgültig eingespielt worden ist. Das kann nur bei abgeschlossenen Inventuren der Fall sein. |
| Eingespielt am | Tag der Inventureinspielung |
| Gelöscht | Kennzeichnung für gelöschte Inventuren; Sämtliche Inventurbelege sind dann beseitigt, nur der Inventur-Stammsatz bleibt als Nachweis, dass es diese Inventur mal gegeben hat, erhalten. |

</details>

<details>
<summary>Allgemein</summary>

| Feld | Beschreibung |
| --- | --- |
| Bezeichnung | Freier Text für die Beschreibung der Inventur |
| Typ der Inventur | **1** – Jahreswechselinventur<br>Es werden Bestandsbuchungen (mengen u. wertmäßige Ein- / Ausbuchungen) erzeugt, und für das neue Wirtschaftsjahr vorgetragen.<br>**2** – Zwischeninventur<br>Durch die Aufnahme kann der Inventurbestand mit dem Buchbestand abgeglichen und eventuelle Differenzen ermittelt werden. Auch die Zwischeninventur muss abgeschlossen u. eingespielt werden. |
| Art der Inventur | **1** – Erhebung am Stichtag (“Normal-Inventur“)<br>**2** – Erhebung und Stichtag versetzt (verschobene Inventur)<br>Die Inventuraufnahme kann vor oder nach dem Stichtag erfolgen. Die Inventurmenge / Wert wird dann fortgeschrieben / zurückgerechnet. |
| Nummernkreis | Dies ist der Nummernkreis der Inventurbelegnummer. Es muss ein Belegkreis mit dem Vorgangstyp “Inventur” sein. Natürlich muss er vom Datum her auch gültig sein (in der Basis-DB = Nr.-Kreis 130). |
| Erhebung am | Datum, zu dem die Bestände aufgenommen werden. |
| Automatische Inventurbelegnummer | Die einzelnen Inventuraufnahmen werden, wenn hier auf “**Ja**” geschaltet, automatisch durchnummeriert.<br>Die Lückenlosigkeit der Erfassung kann damit sichergestellt und überwacht werden. |
| Automatische Positionsnummer | Bei der Erfassung werden, wenn “**Ja**”, automatisch Zeilennummern je erfasster Position vergeben. |
| Bewertung abfragen | Kennzeichen, ob die Bestandsbewertung belegweise (anderenfalls nur je Gesamtbestand) erfolgen soll. |
| Vortrag | Permanente Inventur vorgetragen |

</details>

<details>
<summary>Funktionen des Inventurstamm-Pflegers</summary>

| Funktion | Beschreibung |
| --- | --- |
| Neu(Alle Inventurgruppen) **(Shift + F8)**<br>[Nur im „***Ändern***“-Modus aufrufbar] | Legt für diesen Stichtag alle Inventurgruppen an. |

</details>
