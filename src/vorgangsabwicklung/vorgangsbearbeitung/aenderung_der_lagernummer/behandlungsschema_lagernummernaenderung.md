# Behandlungsschema Lagernummernänderung

<!-- source: https://amic.de/hilfe/_behandlungsschema.htm -->

Administration > Formulare/Abläufe > Behandlungsschema

Mit dem Direktsprung **[BEH]** Behandlungsschema können Sie den Behandlungsschemapfleger aufrufen. Es wird eine Standardbehandlung ausgeliefert, die Sie für Ihre Anwendung modifiziert ablegen können.

Administration > Formulare/Abläufe > Formularzuordnung/Vorgangsunterklasse

Welches Behandlungsschema für welche Vorgangsunterklasse verwendet wird, legen Sie in der Formularzuordnung **[FRZ]** auf der Registerkarte Abwicklung fest.

Das Behandlungsschema gibt Ihnen die Möglichkeit, bestimmte Vorgehensweisen bei der Lagernummernänderung auszuschließen, Meldungen ein- oder auszuschalten und eine Behandlungsvorgabe für bestimmte Fälle vorzugeben.

Da ein Behandlungsschema unter Umständen auch von einem Makro aufgerufen wird, ist es stets möglich, Meldungen abzuschalten.

| Behandlungsschemakriterien |
| --- |
| **Kriterium** | **Werte** | **Verfahrensweise** |
| Artikelfindung | • Identische Artikelnummer suchen – Verwenden Sie diese Option, wenn Sie in allen Lägern die gleichen Artikelnummern verwenden.  
• Identische Artikelnummer + Lagerstamm verproben – Wir empfehlen diese Option, wenn Sie in allen Lägern identische Artikelnummern verwenden und diese den gleichen Lagerstammeintrag haben  
• Über den Artikelstamm suchen – Verwenden Sie diese Option, wenn Sie unterschiedliche Artikelnummern pro Lager verwenden, die jedoch einen gemeinsamen Artikelstamm verwenden. Voraussetzung ist jedoch, dass zu einem Artikelstamm nur ein Eintrag pro Lager existiert. | Auf welche Weise sollen Artikel im neuen Lager gefunden werden |
| Kontraktbehandlung | Beibehalten/Entfernen mit und ohne Warnung | Sie bestimmen hier, ob ein ausgewählter Kontrakt beibehalten werden soll, wenn es möglich ist (Lagerspezifische Kontrakte werden entfernt) oder Kontrakte grundsätzlich beim Lagernummernwechsel entfernt werden. |
| Ungültige/abgewählte Kontrakte neu finden | Ja/Nein | Wird aus der vorherigen Einstellung ein Kontrakt abgewählt, so kann er mit dieser Einstellung neu für den neuen Kunden vorbelegt werden, sofern ein Kontrakt vorhanden ist |
| Manuelle Preise bei abweichender Preisgruppe | Beibehalten/Entfernen mit und ohne Warnung | Pro Lager können unterschiedliche Preise (Preisgruppen) gelten. Definieren Sie, ob manuell eingegebene Preise bei abweichender Preisgruppe beibehalten oder entfernt werden sollen. |
| Manuelle Preise bei identischer Preisgruppe | Beibehalten/Entfernen mit und ohne Warnung | Der manuell eingegebene Preis kann eine Spezialität für das Lager sein, der bei Lagerwechsel entfernt oder beibehalten werden soll. |
| Automatische Preise bei abweichender Preisgruppe | Beibehalten/Entfernen mit und ohne Warnung | Pro Lager können unterschiedliche Preise (Preisgruppen) gelten. Definieren Sie, ob automatisch ermittelte Preise bei abweichender Preisgruppe beibehalten oder entfernt werden sollen. |
| Nullpreis im Ziel | Jeweils mit und ohne Warnung  
• Beibehalten  
• Abbruch  
• Akzeptieren | Ist in einer Warenposition nach dem Lagernummernwechsel ein Null-Preis, weil u.U. keine Preiseinrichtung existiert, so kann dieser Nullpreis akzeptiert, durch den ursprünglichen Preis überschrieben oder der Wechsel abgebrochen werden. |
| Partiebehandlung | Jeweils mit und ohne Warnung:  
• Abbruch – Bei Partiebeteiligung wird abgebrochen  
• Beibehalten – Partie wird beibehalten wenn möglich. Lagerspezifische Partien werden entfernt.  
• Artikel in Partie anlegen – Der Artikel des neuen Lagers wird wenn nötig der Partie hinzugefügt.  
• Partie immer abwählen – ebendies | Wie ist bei Beteiligung von Partien zu verfahren? |
| Manuelle Konditionen | Beibehalten/Entfernen mit und ohne Warnung | An dieser Stelle können Sie definieren, ob manuell eingegebene Konditionen beibehalten oder entfernt werden sollen. Dies kann jeweils mit oder ohne Meldung geschehen |
| Abgeänderte automatische Konditionen | Beibehalten/Entfernen mit und ohne Warnung | An dieser Stelle wird definiert, ob abgeänderte automatische Konditionen beibehalten oder entfernt werden. Dies kann jeweils mit oder ohne Meldung geschehen |
| Bei Folgeartikel abbrechen | Ja/Nein | ebendies |
| Artikel im Ziellager anlegen | Weiter/Abbruch mit/ohne Warnung | Wenn der Artikel nicht im Ziel-Lager existiert müsste dieser dort angelegt werden. Dies kann erlauft sein (Weiter) oder eben nicht (Abbruch) Optional kann eine zusätzliche Warnmeldung ausgegeben werden. |
