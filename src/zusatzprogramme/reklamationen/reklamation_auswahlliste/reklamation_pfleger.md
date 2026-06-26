# Reklamation Pfleger

<!-- source: https://amic.de/hilfe/reklamationpfleger.htm -->

<details>
<summary>Kasten Reklamation</summary>

| Felder | Beschreibung |
| --- | --- |
| Nummer | In diesem Feld wird die aktuelle Reklamationsnummer angezeigt. Diese wird sich automatisch aus dem Nummernkreis gezogen, welcher im [Steuerparameter 1036](../../../firmenstamm/steuerparameter/optionen_global/allgemeiner_steuerparameter_fuer_die_reklamation_spa_1036.md) zu geordnet ist. |
| Datum | Hier wird das Datum eingetragen, zu welchem Datum die Reklamation erfasst worden ist. Das Datum wird mit dem Tagesdatum vorbelegt. |
| Bearbeiter | Gibt den Benutzer an, welcher für Reklamation verantwortlich ist. |
| Geschäftsbereich | Hier wird Geschäftsbereich hinterlegt |
| Grund 1 - 3 | In diesen drei Feldern können bis zu drei Reklamationsgründe eingetragen werden. Die Reklamationsgründe sind im [Anwenderformat](../../../firmenstamm/formate.md#Anwendungsformate) „af_reklamati“ hinterlegt, dieses kann um weitere Reklamationsgründe erweitert werden. |
| Beschreibung | In dieses Feld wird die Beschreibung zu dieser Reklamation eingetragen. |
| Erstellt von | Benutzer, welcher die Reklamation erstellt hat. |
| am | Datum der Erstellung |

</details>

<details>
<summary>Reklamierer/Verursacher</summary>

| Felder | Beschreibung |
| --- | --- |
| Konto-Nr. | Kunden/Lieferantennummer |
| Vorgang | Hier wird der betreffende Eingangslieferschein oder die betreffende Eingangsrechnung zu diesem Lieferanten / Verursacher ausgewählt.<br>Hier wird der betreffende Lieferschein oder die betreffende Rechnung zu diesem Kunden / Reklamierer ausgewählt. |
| Datum | Belegdatum |
| Vorgangsklasse | Vorgangsklasse des ausgewählten Vorgangs |
| Unterklasse | Unterklasse des ausgewählten Vorgangs |
| Ansprechpartner | Hier kann ein Ansprechpartner hinterlegt werden. Dieser muss bereits bei Kunden angelegt sein. Ansonsten empfiehlt sich das Bemerkungsfeld. |
| Bearbeiter | Hier kann ein A.eins-Bediener angegeben werden. |
| Bemerkung | Freitext für Zusatzinformationen. |
| Erledigt | Mit dem Schalter kann eine Reklamation auf erledigt gesetzt werden. |
| Erledigt von | Kürzel des Bedieners, der den Schalter gesetzt hat |
| am | Datum an dem der Schalter gesetzt wurde. |
| Button: Vorgang erzeugen | Erzeugt den Vorgang. |
| Vorgang | Vorgang des von hier aus erzeugtem Vorgang. |
| Datum | Belegdatum. |
| Vorgangsklasse | Hier kann vor dem Erstellen des Vorgangs die gewünschte Vorgangsklasse eingetragen werden.<br>Nach dem Erstellen wird hier die Vorgangsklasse des erzeugten Vorgangs angezeigt<br>(Vorbelegung kann im SPA 1036 gesetzt werden). |
| Unterklasse | Hier kann vor dem Erstellen des Vorgangs die gewünschte Unterklasse eingetragen werden.<br>Nach dem Erstellen wird hier die Vorgangsklasse des erzeugten Vorgangs angezeigt<br>(Vorbelegung kann im SPA 1036 gesetzt werden). |
| Button: Bleistift hinter Vorgang | Öffnet den Vorgang im Editier-Modus. |
| Button: Lupe hinter Vorgang | Öffnet das Archiv des Vorgangs (Lizenz für das Formulararchiv ist notwendig). |

</details>

<details>
<summary>Artikel – Reklamierer/Verursacher</summary>

| Felder | Beschreibung |
| --- | --- |
| Artikelnummer | In diesem Feld kann ein Artikel aus einem zugeordneten hinzugefügt werden.<br>Die ItemBox bietet eine Variante, welche nur Artikel aus dem Vorgang anzeigt und eine, welche alle Artikel anzeigt.<br>Die Daten der Tabelle sind die Grundlage für den erzeugten Beleg. |
| Rek bzw. Ver | Gibt an, ob der Artikel beim Erstellen eines Vorgangs berücksichtigt wird. |
| Wert | Gibt an, ob es sich um einen Wertartikel handelt. |
| Menge | In diesem Feld wird die Reklamationsmenge eingetragen.<br>Die Menge bezieht sich falls vorhanden auf die Gebinde-ME. Anderenfalls auf die reguläre ME. |
| ME | Mengeneinheit des Artikels. Hier können keine Gebinde erfasst werden. |
| Preis | Wird mit dem Preis aus der dazu gehörigen Warenposition vorbelegt. |
| Pr. Einh | Wird mit der Preiseinheit der dazu gehörigen Warenposition vorbelegt. |
| Partie-Nr | Partienummer, falls der Artikel eine Partie besitzt. |
| LG-Nr. | Nummer des Lagers, auf welches gebucht werden soll. Es können nur Läger ausgewählt werden auf den der Artikel angelegt ist. |

</details>

### Funktionen

| Funktion | Beschreibung |
| --- | --- |
| Archiv | Zeigt die Reporte an, welche gedruckt wurden. (Muss möglicherweise unter **[ANWR]** eingerichtet werden)<br>Außerdem können hier sonstige Dateien der Reklamation zugeordnet werden wie beispielsweise Bilder (Lizenz für das Formulararchiv ist notwendig). |
| Maßnahmen (F5) | Maßnahmen für die Reklamation, welche für den Report Druck nötig sind. |

### Maßnahmen (Report Druck)

Die hier angegeben Felder sind die Standard Einstellungen und können im [Steuerparameter 1040](../../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/reklamationsmassnahme_spa_1040.md) vollständig individualisiert werden.

<details>
<summary>Kasten Maßnahmen gegenüber Verursacher</summary>

| Felder | Beschreibung |
| --- | --- |
| Schriftliche Stellungnahme | |
| Schadensprotokoll schicken, Rechnung | |
| Nachlieferung bis | |
| Keine Nachlieferung/Umtausch | |
| Umtausch bis | |
| Annahme verweigert | |
| Abholung bis | |
| Partie-Ware verbleibt beim Kunden | |
| Ware zurück an Verladeort | |
| Entsorgung | |
| Unabhängiger Sachverständiger | |
| Nachuntersuchung/Analyse | |
| Preisnachlass | |
| Haftpflichtversicherung | |
| Gutschriftverwaltung Lieferant | |
| Druck Reklamierer- oder manuelle Adresse | |
| Ware wird umgeleitet an | |
| Druck manueller Kostentext | |
| Andere Beschreibung | |

</details>

 

<details>
<summary>Interne Maßnahmen</summary>

| Felder | Beschreibung |
| --- | --- |
| Reklamation akzeptieren | |
| Ware ist verkehrsfähig | |
| Reklamation ist versichert | |
| Reklamation ist meldepflichtig | |

</details>

 

<details>
<summary>Erledigung der Reklamation gegenüber Kunden</summary>

| Felder | Beschreibung |
| --- | --- |
| Ersatz | |
| Kulanz | |
| Gutschrift | |
| Stornierung | |
| Preisnachlass | |

</details>

 

### Funktionen

| Funktion | Beschreibung |
| --- | --- |
| Report – Details (F10) | Erstellt die Vorschau zum Drucken des Reports. (Solange nicht anders im Steuerparameter gepflegt: „Reklamationsreport_Detail“) Diese werden dem Archiv der Reklamation zugeordnet. |
