# Behandlungsschema

<!-- source: https://amic.de/hilfe/_behandlungsschemata.htm -->

Administration > Formulare/Abläufe > Behandlungsschema

Mit dem Direktsprung [BEH] Behandlungsschema können Sie den Behandlungsschemapfleger aufrufen. Es wird eine Standardbehandlung ausgeliefert, die Sie für Ihre Anwendung modifiziert ablegen können.

Administration > Formulare/Abläufe > Formularzuordnung/Vorgangsunterklasse

Welches Behandlungsschema für welche Vorgangsunterklasse verwendet wird, legen Sie in der Formularzuordnung [FRZ] auf der Registerkarte Abwicklung fest.

Das Behandlungsschema gibt Ihnen die Möglichkeit, bestimmte Vorgehensweisen bei der Kundennummernänderung auszuschließen, Meldungen ein- oder auszuschalten und eine Behandlungsvorgabe für bestimmte Fälle vorzugeben.

Da ein Behandlungsschema unter Umständen auch von einem Makro aufgerufen wird, ist es stets möglich, Meldungen abzuschalten.

| Behandlungsschemakriterien |
| --- |
| **Kriterium** | **Werte** | **Verfahrensweise** |
| Beteiligung von Kontrakten | Weiter oder Abbruch je mit oder ohne Meldung | Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Kontrakten verbieten oder zumindest zur Meldung bringen |
| Kundenwechsel unter Kunden gleicher Kontraktgruppe | Weiter oder Abbruch je mit oder ohne Meldung | Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Kontrakten verbieten oder nur zur Meldung bringen, wenn es sich im Kunden mit gleicher Kontraktgruppe handelt. |
| Kundenwechsel unter Kunden ungleicher Kontraktgruppe | Kontrakt entfernen oder Abbruch je mit oder ohne Meldung | Sie können hier bestimmen, ob ein Kontrakt im Fall ungleicher Kontraktgruppen der Kunden abgewählt wird, oder in diesem Fall der Kundenwechsel verboten ist. |
| Kontraktneufindung starten | Ja/Nein | Wird aus der vorherigen Einstellung ein Kontrakt abgewählt, so kann er mit dieser Einstellung neu für den neuen Kunden vorbelegt werden, sofern ein Kontrakt vorhanden ist |
| Beteiligung von Partien | Weiter oder Abbruch je mit oder ohne Meldung | Sie können hier grundsätzlich die Kundennummernänderung bei Beteiligung von Partien verbieten oder zumindest zur Meldung bringen |
| Abwahl von Partien ohne Zwang | Ja/Nein | Ist die Partie einer Warenposition nicht kundenspezifisch, so muss sie nicht abgewählt werden. Dies kann jedoch auf Wunsch dennoch geschehen. |
| Partie-Neufindung wenn nötig | Ja/Nein | Hat ein Artikel kein Partiezwang und wird die Partie abgewählt, weil sie nicht zum neuen Kunden zugeordnet werden kann, so kann hier definiert werden, ob eine neue Partie gesucht und ggf. zugeordnet werden soll |
| Manuelle Konditionen | Beibehalten/Entfernen mit und ohne Warnung | An dieser Stelle können Sie definieren, ob manuell eingegebene Konditionen beibehalten oder entfernt werden sollen. Dies kann jeweils mit oder ohne Meldung geschehen |
| Abgeänderte automatische Konditionen | Beibehalten/Entfernen mit und ohne Warnung | An dieser Stelle wird definiert, ob abgeänderte automatische Konditionen beibehalten oder entfernt werden. Dies kann jeweils mit oder ohne Meldung geschehen |
| Preisbehandlung manuelle Preise | Beibehalten/Entfernen mit und ohne Warnung | Diese Einstellung regelt die Neufindung bzw. Beibehaltung manuell abgeänderter Preise (nicht bei Kontraktpreisen!) |
| Behandlung manueller Gesamtbeträge | Beibehalten/Entfernen mit und ohne Warnung | Diese Einstellung regelt die Neufindung von Preisen bzw. Beibehaltung von manuell geänderten Beträgen (Gesamtpreisen). |
| Preisbehandlung automatische Preise | Beibehalten oder neu finden | Wird der Kundenwechsel auf einen Kunden mit abweichender Preisklasse vollzogen, so können Preise beibehalten oder neu ermittelt werden. |
| Auto Preise ggf. auch in der Kasse | Ja / Nein | Sollen automatische Preise neu gefunden werden, so muss hier festgelegt werden, ob dies auch in der Kasse passieren soll. |
| Wechsel zu Kunden mit abweichender Währung | Währung beibehalten/Abbruch mit und ohne Warnung | Wird der Kundenwechsel auf einen Kunden mit abweichender Währung vollzogen, so kann die Währung beibehalten oder der Kundenwechsel abgebrochen werden |
| Beteiligung von Strecke führt zu | Entfernen / Abbruch mit und ohne Warnung | Ist eine Strecke an diesem Beleg beteiligt, so kann diese entfernt oder der Wechsel abgebrochen werden. |
| Nullpreis im Ziel | Beibehalten / Abbruch / Akzeptieren mit und ohne Warnung | Ist in einer Warenposition nach dem Kundenwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden. |
| Abweichende Zahlungsbedingung | Beibehalten / Abbruch / Akzeptieren mit und ohne Warnung | Wird durch den Kundenwechsel eine andere Zahlungsbedingung gewählt, als im Vorgang definiert wurde, so kann entschieden werden, ob dies erlaubt sein soll, die alte Zahlungsbedingung erhalten bleiben soll oder die neue verwendet werden soll. |
| Behandlung der Versandanschrift | Verwerfen der Versandanschrift / Erhalten der Versandanschrift | Ist in einer Warenposition nach dem Kundenwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden. |
| Verkaufsbeschränkungen nicht aufgelöst | Abbruch oder weiter mit Warnung wenn möglich | Bestehen für Artikel des Belegs Verkaufsbeschränkungen, für die der neue Kunde kein Zertifikat hat, so muss eine Entscheidung getroffen werden:  
   
**Einstellung "Abbruch":**  
Abbruch des Kundenwechsels mit der Angabe der betroffenen Artikel.  
   
**Einstellung „Weiter mit Warnung wenn möglich“:**  
In Abhängigkeit der Einstellung des Steuerparameters 1001 wird verfahren:  
\* Die Einstellung "Erfassung abweisen" sorgt für einen Abbruch  
\* Die Einstellung "Abfrage in der GUI" sorgt dafür, dass der Kundenwechsel vollzogen wird, aber für die entsprechenden Artikel eine Warnung ausgegeben wird.  
Der Anwender muss in diesem Fall die Kaufberechtigung des Kunden persönlich prüfen. |
| Nachhaltigkeit Statusänderung | Weiter oder Abbruch je mit oder ohne Meldung | Wird der Status einer oder mehrerer Positionen von Nachhaltigkeit zu Nicht-Nachhaltigkeit oder umgekehrt geändert, so wird hier gewarnt und ggf. auch abgebrochen. |
| | | |
