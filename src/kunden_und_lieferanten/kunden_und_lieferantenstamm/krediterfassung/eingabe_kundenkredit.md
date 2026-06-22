# Eingabe Kundenkredit

<!-- source: https://amic.de/hilfe/_eingabekreditlimit.htm -->

Hauptmenü > Stammdatenpflege > Kunden / Lieferanten > Kundenstamm

Direktsprung **[KU]**

Die Einrichtung der Kreditlimits geschieht in Abhängigkeit des [Steuerparameters 503](../../../firmenstamm/steuerparameter/kundenstammdaten/alle_kredite_als_summe_uebernehmen_spa_503.md) – „Alle Kredite als Summe übernehmen“.

Die Eingabe des Kreditlimits kann von zweiten Seiten aus geschehen. Zum einen kann das Limit vom Pfleger für **Kunden-/Lieferantenstammdaten** aus eingegeben werden und zum anderen über den Pfleger der **Kreditvergabe**.

Im Pfleger für **Kunden-/Lieferantenstammdaten** ist lediglich ein Feld „Kreditlimit“ vorhanden, welches je nach Steuerparameter (s.o.) für eine Bearbeitung freigeschaltet ist oder nicht.

Wird hier ein Wert geändert und gespeichert, so wird in Folge dessen ein Abgleich mit dem Pfleger der Kreditvergabe durchgeführt. Zunächst wird der Kredittyp ermittelt zu dem das neue Kreditlimit dort angelegt werden soll. Vorhandene Einträge werden auf den Status „abgelaufen“ gesetzt und das neue Limit wird eingetragen.

Hier ein wichtiger Hinweis zum [Kundenkreditlöschkennzeichen](./kundenkreditloeschkennzeichen.md).

Die Anwendung **Kreditvergabe** dient zur Einrichtung und Kontrolle der Kreditlimits für den gewählten Kunden. Sie zeigt alle aktiven und inaktiven Kreditlimits in Sortierung nach der Limitart bzw. des Kreditversicherers an.

Es ist möglich mehrere Einträge zu einem Kreditversicherer zu erzeugen. Dies ermöglicht zum Beispiel Planung eines Limits in der Zukunft.

Die Gültigkeit eines Kreditlimits hängt vom Datum der Genehmigung und des „Gültig Bis“ - Datum ab und davon, ob das Kreditlimit mit in die Summierung einfließen soll.

Wird für einen Kunden ein Kreditlimit gespeichert, so wird dieses gleichzeitig in dem zugehörigen Feld im Kunden-/Lieferantenstamm aktualisiert. Das Feld im Kunden-/Lieferantenstamm enthält immer den zur Zeit der Betrachtung korrekten Wert.

Wie dieser Wert bestimmt wird, lässt sich mit dem [Steuerparameters 503](../../../firmenstamm/steuerparameter/kundenstammdaten/alle_kredite_als_summe_uebernehmen_spa_503.md) – „Alle Kredite als Summe übernehmen“ – festlegen. Ist keine Summierung erwünscht, so ist das Feld „Kreditlimit“ in den Kunden-/Lieferantenstammdaten änderbar. Das hier direkt erfasste Kreditlimit wird dann beim Speichern in die Auflistung der aktiven Kreditlimits unter der als Standard angegeben [Limitart](./einrichtung_und_pflege_von_limitarten.md) übernommen.

| Feld | Beschreibung |
| --- | --- |
| Kunde | Kundennummer des Kunden |
| Name | Vor- und Nachname des Kunden |
| Straße | Straße und Hausnummer des Kunden |
| Ort | Postleitzahl und Ort |
| Telefon | Telefonnummer |
| Erfasst am | Erfassungsdatum des Kunden |
| Geändert am | Letztes Änderungsdatum des Kunden |
| Ansprechpartner | Ansprechpartner aus Anschrift |
| Zusatz1 | Zusatz 1 aus Anschrift |
| OP-Saldo | OP-Saldo des Kunden |
| Summierung | Steht auf Ja oder Nein.  
Ja zeigt an, dass diese Zeile zur Berechnung des gesamten Kreditlimits herangezogen wird.  
Das Feld wird berechnet mit Hilfe folgender Informationen:  
der Angabe beim Kredittyp, dem Löschkennzeichen, dem ‚Gültig bis‘-Datum, dem ‚Genehmigt ab‘-Datum und dem [Steuerparameter 503](../../../firmenstamm/steuerparameter/kundenstammdaten/alle_kredite_als_summe_uebernehmen_spa_503.md) (Alle Kredite als Summe übernehmen?) |
| Limitart | F3 Funktion zur Auswahl oder Angabe des Kredittypen |
| Bezeichnung | Bezeichnung des Kredittypen |
| Status | Aktiv: wird zur Berechnung des gesamten Kreditlimits herangezogen  
Abgelaufen: wird **nicht** zur Berechnung des gesamten Kreditlimits herangezogen und kann über einen Einrichterparameter (Nur aktive Datensätze anzeigen) ausgeblendet werden. Die Standard Vorbelegung für diesen Einrichterparameter ist Nein.  
Gelöscht: wird **nicht** zur Berechnung des gesamten Kreditlimits herangezogen und kann über einen Einrichterparameter (Nur aktive Datensätze anzeigen) ausgeblendet werden. Die Standard Vorbelegung für diesen Einrichterparameter ist Nein. |
| Referenznummer | Frei zu vergebene Nummer als Referenz der Kreditlimitart |
| Beantragt | Enthält den beatragten Wert des Kreditlimits |
| Beantragt am | Enthält das Datum der Beantragung |
| Genehmigt | Enthält den genehmigten Wert des Kreditlimits |
| Genehmigt am | Enthält das Datum der Genehmigung |
| Gültig bis | Enthält das Datum bis wann das Kreditlimit gültig sein wird. Kann auch leer bleiben wenn es kein Ablaufdatum geben soll. |
| Vertretergruppe | Anzeige der Vertretergruppe und Bezeichnung |
| Mahngruppe | Anzeige der Mahngruppe und Bezeichnung |
| Bonitätskennzeichen | Anzeige Bonitätskennzeichen |
| Zahlungsbedingungen | Anzeige der Zahlungsbedingungen des Kunden |
| Liefersperre | Anzeige Liefersperre |
| Mahnsperre | Anzeige Mahnsperre |
| Zinsgruppe | Anzeige Zinsgruppe und Bezeichnung |
| Kunden-Bemerkungen | Hier können frei zusätzliche Informationen erfasst werden, die für einen Kredit des Kunden relevant sind (z.B. Kunden­infor­mati­onssystem beachten). Es handelt sich hierbei um Kunden-Bemerkungen, nicht speziell um Bemerkungen zum Kredit! |

**Hilfe F1**

Ruft diese Hilfe auf.

**Speichern F9**

Speichert die Änderungen und verlässt die Anwendung.

**Zeile anfügen F8**

Erstellt an der markierten Position eine neue, leere Zeile.

**Kredittypen F10**

Wechselt in den Pfleger der für Kredittypen.

**Als gelöscht kennzeichnen F7**

Kennzeichnet den markierten Datensatz der Tabelle als gelöscht und setzt das „Gültig Bis“ Datum auf das Tagesdatum vom Vortag. Diese Änderung wird beim Verlassen der Anwendung oder durch Speichern übernommen.

**Als aktiv kennzeichnen F7**

Kennzeichnet den markierten Datensatz der Tabelle als aktiv. Diese Änderung wird beim Verlassen der Anwendung oder durch Speichern übernommen.

**Archiv anzeigen Strg+F12**

Öffnet das Archiv für den aktuellen Kunden.

**Ende ESC**

Verlassen der Anwendung und ggf. Speicherung der Änderungen

**Einrichterparameter**

Über einen Einrichterparameter, lassen sich hier die nicht aktiven Kreditlimits ausblenden.
