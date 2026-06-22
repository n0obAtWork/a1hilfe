# Objekt-Stamm

<!-- source: https://amic.de/hilfe/_baustelle.htm -->

Auf der Anwendung Objekt-Stamm kann man Projekte anlegen (für ein oder mehrere Kunden mit einem oder mehreren Lieferanten).

Hauptmenü > Nebenbuchhaltungen > Objekte > Objektverwaltung oder Direktsprung **[OBJ]**

<details>
<summary>Tabreiterübergreifende Felder</summary>

Oben links sind Felder, welche dauerhaft eingeblendet sind

Es gibt folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Nummer | Wird automatisch mit einer freien Objektnummer gefüllt. |
| Unter- Nr. | Es kann eine Unternummer eingetragen werden, wenn man mehrere Unterobjekte mit der gleichen Objektnummer haben möchte. |
| Bezeichnung | Bezeichnung für das Objekt. |
| Matchcode | Suchbegriff für das Objekt. |

Das Feld Unter- Nr. ist nur sichtbar, wenn der Steuerparameter SPA_BAUSTELLE_UNTERBAUSTELLE_AKTIV mit der Nummer 172 gültig ist.

Das Feld Matchcode wird automatisch mit dem Inhalt des Feldes Bezeichnung befüllt. In das Feld kann etwas anderes als die Bezeichnung eingetragen werden.

</details>

<details>
<summary>Allgemein</summary>

Auf dem Tabreiter „Allgemein“ sind Felder, welche teilweise schon vorausgefüllt sind

Es gib folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| gültig ab | Wird automatisch mit dem heutigen Tag gefüllt |
| gültig bis | Wird automatisch mit dem heutigen Tag + 1 Jahr gefüllt |
| Archivreferenz | Wird automatisch mit 00BS0000 + dem Eintrag im Nummernfeld gefüllt |
| Kunde | Die Kundennummer der Kunden. Mittels F3 ist eine Auswahlhilfe verfügbar. Bei mehr als einem Kunden wechselt man in den Tabreiter Kundenliste und trägt dort im Grid einen weiteren Kunden ein. Falls alle Kunden betroffen sind, lässt man das Feld leer. |
| Lieferant | Die Kundennummer der Lieferanten. Mittels F3 ist eine Auswahlhilfe verfügbar. Bei mehr als einem Lieferanten wechselt man in den Tabreiter Lieferantenliste und trägt dort im Grid einen weiteren Lieferanten ein. Falls alle Lieferanten betroffen sind, lässt man das Feld leer. |
| Rechnungsempfänger | Die Kundennummer des Rechnungsempfängers, auch als Oberkunde bekannt. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Zahlkunde | Die Kundennummer des Zahlkunden. Dieser begleicht alle Rechnungen. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Ansprechpartner | Die Kundennummer des Ansprechpartners, auch als Objektunternehmer bekannt. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Objekt erledigt | Zeigt an, ob das Objekt erledigt ist. Es kann nur Ja oder Nein eingetragen werden. |
| Abrechnung möglich | Zeigt an, ob die Abrechnung des Objektes möglich ist. Es kann nur Ja oder Nein eingetragen werden. |
| Preisgültigkeit | Ist vorbelegt mit Nein und kann auf Ja geändert werden. Wenn auf Ja geändert wird, lässt sich das Datum rechts davon ändern. Zeigt an, ob sich der Preis ändern kann, und wie lange der Preis gültig ist. |
| Lagernummer | Wird automatisch mit dem Wert aus dem Steuerparameter SPA_BAUST_LAGERNUMMER_FEHL vorbelegt. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Vertretergruppe VK | Ist vorbelegt mit dem Wert 0. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Musterobjekt | Ist vorbelegt mit Nein und kann auf Ja geändert werden. Ein Musterobjekt kann benutzt werden als Vorlage für das Duplizieren. |
| ZuAbschl.Klasse VK | Ist vorbelegt mit 0. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Budget | Budget des Objektes in Euro mit 2 Nachkommastellen. |

Im Neu-Anlegen Fall wird nach Auswahl des Hauptkunden mit F3 teilweise automatisch der Rechnungsempfänger und Zahlkunde ausgefüllt. Dies muss vorher eingerichtet worden sein.

Der Kunde und der Lieferant wird automatisch in das Grid auf dem Tabreiter Kundenliste und Lieferantenliste eingetragen.

Wenn man mehr als einen Kunden und oder Lieferanten im Projekt haben möchte, muss man diese im Tabreiter Kundenliste und Tabreiter Lieferanten in das Grid eintragen.

Wenn das Musterobjekt auf Ja geändert wurde, dann wird diese Information durch das Duplizieren nicht dupliziert.

Das Lager aus SPA_BAUST_LAGERNUMMER_FEHL muss existieren.

</details>

<details>
<summary>Zusätzliche Kennzeichen</summary>

Auf dem Tabreiter „Zusätzliche Kennzeichen“ sind Felder, welche teilweise schon vorausgefüllt sind

Es gibt folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Fremdartikel zulässig | Zeigt an, ob Fremdartikel für das Objekt erlaubt sind. Ist vorbelegt mit Ja und kann auf Nein geändert werden. |
| spez. EK-Preise möglich | Zeigt an, ob spez. EK-Preise möglich sind. Ist vorbelegt mit Ja und kann auf Nein geändert werden. |
| spez VK-Preise möglich | Zeigt an, ob spez. VK-Preise möglich sind. Ist vorbelegt mit Ja und kann auf Nein geändert werden. |
| Zahlungsbedingungen VK | Zahlungsbedingungen des Verkaufes. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Zahlungsbedingungen EK | Zahlungsbedingungen des Einkaufes. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Bonusabrechnung | Ist vorbelegt mit Standard und kann auf Nein und Immer geändert werden |
| Versandart | Versandart des Objektes. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Sammelrech.Kennz | Ist vorbelegt mit „laut Kundenstamm und SPA“ und wird mit F3 mit einem Sammelrechnungskennzeichen gefüllt. |

Die Versandart steht auf 0 und ist damit die Standardversandart. Die Versandarten müssen vorher eingerichtet werden.

</details>

<details>
<summary>Kundenliste</summary>

Auf dem Tabreiter „Kundenliste“ ist das Kundengrid.

Mit F3 füllt man die Spaltenfelder der Kundennummerspalte.

Die erste Zeile wird automatisch gefüllt, wenn in das Feld Kunde auf dem Tabreiter Allgemein ein Kunde eingetragen wird.

Wenn mindestens 2 Kunden im Grid eingetragen sind, wird im Tabreiter Allgemein das Feld Kunde gesperrt und im Feld steht „->Liste“, was signalisiert, dass die Kunden auf dem Tabreiter Kundenliste einsehbar sind.

Wenn man dann den Hauptkunden ändern möchte, muss man mit der Tastenkombination Strg+ Shift + Entf im Grid alle Kunden oder alle Kunden bis auf den Hauptkunden löschen.

Dann kann man im Grid einen neuen Kunden mit F3 auswählen oder auf dem Tabreiter Allgemein das freigeschaltete Kundenfeld mit F3 befüllen.

Es dürfen keine Mehrfacheinträge von Kunden existieren. Man wird gezwungen einen anderen Kunden einzutragen, wenn der Kunde vorher schon eingetragen wurde.

</details>

<details>
<summary>Lieferantenliste</summary>

Auf dem Tabreiter „Lieferantenliste“ ist das Lieferantengrid.

Mit F3 füllt man die Spaltenfelder der Lieferantennummerspalte.

Die erste Zeile wird automatisch gefüllt, wenn in das Feld Lieferant auf dem Tabreiter Allgemein ein Lieferant eingetragen wird.

Wenn mindestens 2 Lieferanten im Grid eingetragen sind, wird im Tabreiter Allgemein das Feld Lieferant gesperrt und im Feld steht „->Liste“, was signalisiert, dass die Lieferanten auf dem Tabreiter Lieferantenliste einsehbar sind.

Wenn man dann den Hauptlieferanten ändern möchte, muss man mit der Tastenkombination Strg+ Shift + Entf im Grid alle Lieferanten oder alle Lieferanten bis auf den Hauptlieferanten löschen.

Dann kann man im Grid einen neuen Lieferanten mit F3 auswählen oder auf dem Tabreiter Allgemein das freigeschaltete Lieferantenfeld mit F3 befüllen.

Es dürfen keine Mehrfacheinträge von Lieferanten existieren. Man wird gezwungen einen anderen Lieferanten einzutragen, wenn der Lieferant vorher schon eingetragen wurde

</details>

<details>
<summary>Futter-App</summary>

Auf dem Tabreiter „Futter-App“ kann man den Futterbedarf bei vorhandener Lizenz verwalten

</details>

<details>
<summary>Privatisierbares Addon</summary>

Auf dem Tabreiter „Privatisierbares Addon“ kann man sich eigene Felder mit AIS anlegen.

</details>

<details>
<summary>Funktion des Objektstammes</summary>

| Funktion | Beschreibung |
| --- | --- |
| Mengenzeitr. **(F10)** | Öffnet die Maske Objektverwaltung mit den Werten aus den tabreiterübergreifenden Feldern. Es kann der Zeitraum und die Zu - und Abgänge verwaltet werden. |
| Anschriften **(SF6)** | [Anschriften](../kunden_und_lieferanten/kunden_und_lieferantenstamm/anschriften/registerkarten_in_anschriften/allgemein.md) |
| Bemerkung **(F2)** | [Kontraktbemerkungen](../kontrakt/kontraktvarianten/bereiche.md) |
| Rabatt/Preise **(SF12)** | Öffnet die Maske Objektverwaltung mit den Werten aus den tabreiterübergreifenden Feldern.<br>Es können für Artikel und Warengruppen bis zu 3 Rabatte verwaltet werden.<br>Wenn in den Rabattfeldern jeweils 0,00 steht, dann würde - sofern angelegt - der Standardrabatt (allgemeiner/individueller Rabatt) gezogen werden. <br>Ein ¨Ja¨ im Feld ¨Null¨ verhindert, dass die Standard-Rabattierung stattfindet. Typ der Behandlung, wenn alle drei Rabattsätze 0 sind:<br>\- 0 (Ja) = es gibt keine Rabatte.<br>\- 1 (Nein) = es sollen die Standardrabatte verwendet werden.<br>Das Feld Ausschluss gibt an, dass alle außer der ausgewählten (Haupt-/Ober-)Warengruppe ausgewählt werden. Wenn also ein Ja in Aus steht, wird diese (Haupt-/Ober-)Warengruppe ausgeschlossen.<br> |
| DSGVO-Liste | Öffnet die DSGVO-Maske mit den Daten aus der Anschriftenmaske. Man kann die Daten mit F7 anonymisieren. |
| Archiv anzeigen **(CF12)** | [Archiv](../dokumentenverwaltung/archiv_dokumente_hinzufuegen.md) |
| Duplizieren **(F4)** | Öffnet die Duplizierenmaske mit der man entscheiden kann, welche Daten von einem vorhandenen Projekt in ein neues Projekt dupliziert werden sollen. |
| Standardkonditionen **(SF7)** | Wenn für einen Kunden Standardkonditionen eingerichtet worden sind, kann man diese über die Funktion laden |

</details>
