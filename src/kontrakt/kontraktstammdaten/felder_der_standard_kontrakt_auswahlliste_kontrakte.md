# Felder der Standard-Kontrakt-Auswahlliste (Kontrakte)

<!-- source: https://amic.de/hilfe/_kontrakt_auswahllist_felder.htm -->

Hauptmenü > Kontraktverwaltung > Kontraktstammdaten

oder Direktsprung **[KTR]**

Variante: *Kontrakte*

**Nachfolgend werden einige Felder der Kontraktstamm-Standardauswahlliste beschrieben:**

| Variante: Kontrakte |
| --- |
| Kontraktklasse | Kontraktklasse des Kontrakts |
| Kontraktunterklasse | Kontraktunterklasse des Kontrakts |
| Nummer | Kontraktnummer |
| Bezeichnung | Kontraktbezeichnung |
| Artikelnummer | Die Nummer eines Artikels der Kontraktartikel |
| Weitere Artikel | Liste der Nummern der gegebenenfalls vorhandenen weiteren Artikel des Kontrakts |
| Pricing | Pricing-Angabe eines Kontraktartikels |
| Sollmenge | Summe aller Zeitraum-Sollmengen des Kontrakts |
| Waagemenge | Summe der per Kontraktzuordnung in der Onlinewaage zugordneten Mengen zum Kontrakt |
| Restmenge | Summe aller Zeitraum-Restmengen des Kontrakts<br>**Zu beachten:** Steuerungsparameter zur Berücksichtigung von Restmengen erledigter Kontrakte |
| gebuchte Menge | Summe aller gebuchten Mengen des Kontrakts |
| Mengeneinheit Menge | Mengeneinheit der Mengendarstellungen |
| Kontraktpreis | Der angegeben Preis im Kontrakt |
| Mengeneinheit Preis | Mengeneinheit der Preisdarstellungen |
| geliefert | Summe aller gelieferten Mengen des Kontrakts |
| disponiert | Summe aller disponierten Mengen des Kontrakts |
| Hauptkunde | Nummer des Hauptkunden der dem Kontrakt zugeordneten [Kontraktgruppe](../kontraktgruppen.md) nebst dessen Bezeichnung |
| Staat | ISO-Bezeichnung des Staats der Kontrakt-Versandanschrift beziehungsweise Haupanschrift |
| Ort<br> | Ort der Kontrakt-Versandanschrift beziehungsweise Haupanschrift |
| Ernte | Das dem Kontrakt zugeordnete Erntejahr |
| Start<br> | Das Startdatum der Gültigkeit des Kontrakts |
| Ende | Das reguläre Enddatum Gültigkeit des Kontrakts |
| Disp. Gruppe | Dispositionsgruppe des Kontrakts |
| beteiligte Kunden | Liste der Kundennummern der der Kontraktgruppe zugeordneten Kunden |
| Bestätigung | Druckkennzeichen der Kontraktbestätigung |
| Storno | Stornokennzeichen |
| Muster | Kennzeichen, ob es sich um einen Musterkontrakt handelt |
| Löschkennzeichen | Kennzeichen, ob der Kontrakt gelöscht ist |
| Variante | Variante für den Kontraktdruck |
| überfällig seit | Bei überfälligen kontrakten das maximale Ende-Datum der Laufzeit des Kontrakts |
| Abschluss | Das Datum des Abschlusses des Kontrakts (Kontrakt-Datum) |
| Vertreter/-gruppe | Der dem Kontrakt zugeordnete Vertreter beziehungsweise der zugeordneten Vertretergruppe |
| Streckennummer | Liste der Klammernummern (Strecken) aus den Mengenzeiträumen des Kontrakts |
| Parität | Nummer der Parität des Kontrakts |
| Paritätbezeichnung | Bezeichnung der Parität |
| Warengruppe | Warengruppe eines Artikels der Kontraktartikel |
| PLZ | Postleitzahl der Kontrakt-Versandanschrift beziehungsweise Haupanschrift |
| Buchungstyp | Buchungstyp des Kontrakts (normal, Vorverkauf, Voreinkauf, Einlagerung, Kommission) |
| Betrag | Nettosumme des Kontrakts |
| Erledigt | Erledigungskennzeichen des Kontrakts |
| individuelle Nachhaltigkeit | Nur bei vorhandener Nachhaltigkeit-Lizenz:<br>Im Kontraktartikel angegebenes Nachhaltigkeitskennzeichen eines im Artikelstamm als nachhaltig gekennzeichneten Kontraktartikels |
| Gruppe | Nummer der Kontraktgruppe |
| Gruppenbezeichnung | Bezeichnung der Kontraktgruppe |

**Bei Einstellung des** [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/berechnung_fuer_ratierliche_kontraktmengen_aktiv_spa_701.md)s 701 „Berechnung für ratierliche Kontraktmengen aktiv“ **mit dem Wert Ja sind zusätzliche Spalten verfügbar. Die Überschriften der Spalten der Mengen der Form „mm/jj Menge x“ enthalten jeweils die Angaben mm = Monat, jj = Jahr und x = laufende Nummer ab 1 bis zur im** [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/berechnung_fuer_ratierliche_kontraktmengen_aktiv_spa_701.md)s 698 „Maximale Vorausmonate für die ratierlichen Kontraktmengen“ **angegebener Anzahl. Zu jeder Mengenspalte gehört zudem die Spalte der korrespondierenden Restmenge sowie die Spalte der bis dahin kumulierten Restmengen.** Wenn im [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 846 „Ratierliche Einstellungen“ **die Option „MENGEUEBER“ mit dem Wert Ja eingestellt ist, werden negative Restmengen eines Monats mit 0 dargestellt und mit der jeweiligen tatsächlichen Restmenge des folgenden Monats verrechnet. Die Darstellung von Sollmenge und Restmenge der vor dem aktuellen Monat liegenden Monate ist abhängig von der Einstellung der Option „Vorzeitraummenge beibehalten“ im** [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 846 „Ratierliche Einstellungen“ **(siehe unten).**

| Spalten der ratierlichen Kontraktmengen |
| --- |
| Menge Vormonate | In dieser Spalte wird die Summe der Sollmengen der vor dem aktuellen Monat vorhandenen Kontrakt-Zeiträume ausgewiesen, sofern im [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 846 „Ratierliche Einstellungen“ die Option „MENGEUEBER“ mit dem Wert Nein oder die Option „Vorzeitraummenge beibehalten“ mit dem Wert Ja eingestellt ist. Andernfalls wird die nicht verbrauchte Menge (Soll-Menge minus Restmenge) der Menge des aktuellen Monats zugeschlagen, so dass als Menge der Vormonate die verbrauchte Menge dargestellt wird. |
| Restmenge Vormonate | In dieser Spalte wird die Summe der Restmengen der vor dem aktuellen Monat vorhandenen Kontrakt-Zeiträume ausgewiesen sofern im [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 846 „Ratierliche Einstellungen“ die Option „MENGEUEBER“ mit dem Wert Nein oder die Option „Vorzeitraummenge beibehalten“ mit dem Wert Ja eingestellt ist. Andernfalls wird bei tatsächlicher positiver Restmenge diese auf 0 reduziert und die tatsächliche Restmenge zur Restmenge des aktuellen Monats addiert. Bei negativer Restmenge und der Einstellung der Option „MENGEUEBER“ mit dem Wert Ja wird diese mit 0 dargestellt und mit der Restmenge des aktuellen Monats verrechnet angezeigt**.** |
| mm/jj Menge x | Ratierliche Menge des Monats mm im Jahr jj |
| Restmenge x | Ratierliche Restmenge des Monats. **Bei negativer Restmenge und der Einstellung der Option „MENGEUEBER“** im [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 846 „Ratierliche Einstellungen“ **mit dem Wert** **Ja** **wird diese mit 0 dargestellt und mit der Restmenge des folgenden Monats verrechnet.** |
| kum. Rest | Summe der Restmengen bis einschließlich des Monats x |
