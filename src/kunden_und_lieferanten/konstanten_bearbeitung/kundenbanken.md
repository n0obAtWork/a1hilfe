# Kundenbanken

<!-- source: https://amic.de/hilfe/_kundenbanken.htm -->

Hauptmenü > Stammdaten > Konstanten Kundenstamm > Kundenbanken

oder Direktsprung **[KUBA]**

Alle Grunddaten der Banken, mit denen das System zu tun hat, werden im allgemeinen Bankenstamm hinterlegt, egal ob es sich um eigene Hausbanken oder um Banken von Kunden, Lieferanten, Vertretern... handelt. Hierdurch wird vermieden, dass immer wiederkehrende Informationen, wie die Bankleitzahl, wiederholt werden müssen.

Die spezifischen Eingaben der Kundenbankdaten werden im Auswahlbildschirm Kundenbank **erfasst,** der über den Direktsprung **[KUBA]** zu erreichen ist.

In der Stammdatenmaske können folgende Daten eingegeben werden.

| Felder |
| --- |
| Kunde | Hier kann aus dem Kundenstamm ein Kunde ausgewählt werden, für den die folgenden Einstellungen gemacht werden sollen.  
 |
| IBAN | Die „International Bank Account Number“ - kurz IBAN- wird im Zahlungsverkehr immer wichtiger. In dem ab dem 28.01.2008 gestarteten SEPA Verfahren wird sie an Stelle der Kontonummer verwendet. Bei der Erfassung der Kundenbanken wird die IBAN für deutsche, österreichische und belgische Banken anhand eines Prüfzifferverfahrens überprüft.  
Der Test der IBAN kann entweder für jede [Bank](../../finanzbuchhaltung/zahlungsverkehr/stammdaten_zahlungsverkehr/bankenstamm.md) oder global per [Steuerparameter](../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) abgeschaltet werden.  
In der IBAN ist die Bankleitzahl und Kontonummer enthalten. Anhand der Bankleitzahl wird der Bankenstamm durchsucht und dann die Bank und Kontonummer eingetragen. Wird keine Bank vorgeschlagen ist entweder der Bankenstamm nicht korrekt gepflegt oder die IBAN ist nicht korrekt aufgebaut.  
   
Die IBAN kann nachträglich über ein Funktion „Generiere IBAN“ für alle Kundenbanken mit eingetragener Bank und Kontonummer erzeugt werden.  
 |
| Bank | Banknummer und Text. Eine Auswahl mit **F3** ist möglich  
 |
| BIC / BLZ | Hier wird die BIC(Swift) der Bank wird angezeigt. Steht der [Steuerparameter](../../firmenstamm/steuerparameter/optionen_global/bankleitzahl_und_kontonummer_anzeigen_spa_1121.md) 1121 „Bankleitzahl und Kontonummer anzeigen“ auf **Ja**, so wird hier auch die BLZ angezeigt.  
 |
| Kontonummer | Kontonummer des Bankkontos. Dieses Feld wird nur angezeigt, wenn der [Steuerparameter](../../firmenstamm/steuerparameter/optionen_global/bankleitzahl_und_kontonummer_anzeigen_spa_1121.md) 1121 „Bankleitzahl und Kontonummer anzeigen“ auf **Ja** steht.  
 |
| | |
| Empfänger/Zahlungspflichtiger | Hier kann ein abweichender Empfänger eingetragen werden. Ist dieses Feld leer, so wird beim DTA die Bezeichnung des Kunden/Lieferanten herangezogen.  
 |
| Beschreibung | Hier kann ein Informationstext hinterlegt werden, um ggf. anderen Mitarbeitern die Bedeutung der Bank zu erläutern.  
 |
| Währung | Nummer und Text der Währung, in der dieses Konto geführt wird. Eine Auswahl mit **F3** ist möglich  
 |
| Sperre des Kontos | Wenn ja, ist das Konto für weitere Verarbeitungen gesperrt.  
 |
| Standardbank | Wenn zu einem Kunden/Lieferanten mehrere Banken eingerichtet sind, so wird die Bank, bei der hier **Ja** eingetragen ist beim automatischen Zahlungsverkehr verwendet.  
 |
| Gültig bis | Termin, zu dem das Konto ausläuft.  
 |
| Max. Abbuchbetrag | Maximale Abbuchung auf ein Kunden-/Lieferantenkonto. Bei mehreren Konten werden die Beträge ggf. geteilt. Ist nur ein Konto vorhanden, erfolgt die Gesamtbuchung auf jedem Fall auf das eine Konto. Findet beim automatischen Zahlungsverkehr Berücksichtigung.  
 |
| Max. Einzahlbetrag | Maximaler Einzahlungsbetrag auf ein Kun­den/Lieferantenkonto.  
 |

Zu den Kundenbanken existieren [Einrichterparameter](../../firmenstamm/einrichterparameter/kundenbanken_epa_tbkuba.md), die die Erfassungsmöglichkeiten steuern.

Kontraktabtretungskonto

Unter Umständen kann es dazu kommen, dass die Banknummer und Kontonummer nicht mehr geändert oder das Konto gelöscht werden darf. Dieser Fall liegt vor, wenn in einem Kontrakt ein Abtretungskonto eingetragen, der Kunde in der Kontraktgruppe des Kontrakts ist und die Banknummer und Kontonummer dieselben sind.
