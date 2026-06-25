# Eingabe - Eingabefelder

<!-- source: https://amic.de/hilfe/_formularzuordnung_tabeingabefelder.htm -->

Auf der Registerkarte „Eingabefelder“ stehen folgende Felder zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| 1\. Eingabefeld | Die Eingabefelder legen die Reihenfolge der Vorgangsfelder fest. |
| 2\. Eingabefeld | Siehe „1. Eingabefeld“ |
| 3\. Eingabefeld | Siehe „1. Eingabefeld“ |
| 4\. Eingabefeld | Siehe „1. Eingabefeld“ |
| 5\. Eingabefeld | Siehe „1. Eingabefeld“ |
| Startfeld | Das Startfeld legt fest, in welchem Feld man beim Starten eines neuen Beleges anfängt. |
| Liefer-/Plan-Datum pro Artikel | Festlegung ob das Liefer-/Plandatum pro Artikel änderbar sein soll.<br> |
| Preisdatum pro Artikel | Festlegen, ob das Preisbezugsdatum pro Artikel änderbar sein soll. |
| … mit Warnung? | Gibt bei Änderungen am Preisdatum des Artikels eine entsprechende Meldung auf dem Bildschirm aus. |
| Abgrenzungsdatum pro Artikel | Festlegen, ob das Abgrenzungsdatum pro Artikel änderbar sein soll. |
| … mit Warnung? | Gibt bei Änderungen am Abgrenzungsdatum des Artikels eine entsprechende Meldung auf dem Bildschirm aus. |
| Datum der Verfügbarkeit übernehmen | Dieses Feld ist nur bei „Angebot“, „Auftrag“, „Bestellanfrage“ und „Bestellung“ eingebbar. Hier wird festgelegt, ob nach einer Änderung des Plandatums in der Warenposition das neue Datum auch im Vorgangskopf übernommen werden soll:<br><ul><li>&nbsp;&nbsp;&nbsp; Nur in der Warenposition ändern</li><li>&nbsp;&nbsp;&nbsp; Auch im Vorgang setzen</li><li>&nbsp;&nbsp;&nbsp; Nur Im Vorgang setzen, wenn das neue Datum größer ist<br>&nbsp;</li></ul> |
| Liefernummer auf Position nachtragen | Hier kann festgelegt werden, ob in Abhängigkeit von der Einstellung des Steuerparameters [826 - Liefernummer auf Position eingeben](../../firmenstamm/steuerparameter/vorgangsbearbeitung_positionen/liefernummer_auf_position_eingeben_spa_826.md) eine [Erfassung von Lieferscheinnummern auf der Warenpositionsmaske](../erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/lieferscheinnummer.md) zulässig sein soll.<br>Diese Einstellung ist erst ab Stufe Rechnung / Eingangsrechnung verfügbar. |
| Kontraktanzeige in Warenmaske | Die Information zu Kontrakten wird angezeigt<br><ul><li>&nbsp;&nbsp;&nbsp; Nur wenn Kontrakt vorhanden</li><li>&nbsp;&nbsp;&nbsp; Immer</li></ul> |
| Kontrakteingabe auf Warenmaske | Steuert, ob die Kontraktnummer manuell eingegeben werden kann oder lediglich aus der Itembox ausgewählt werden darf |
| Kontraktautomatik auf Warenmaske | Hier gibt es die Möglichkeit, die Steuerung für die Kontraktauswahl abweichend von Steuerungsparameter 64 – Automatische Auswahl bei Vorgangsbearbeitung die Einstellung auf Ebene der Vorgangsunterklasse erfolgen.<br><ul><li>&nbsp;&nbsp;&nbsp; Wie SPA 64</li><li>&nbsp;&nbsp;&nbsp; Ab 2 Kontrakten – Es wird ab 2 Kontrakten eine Auswahl angeboten</li><li>&nbsp;&nbsp;&nbsp; Generell – Es wird stets eine Kontraktauswahl angeboten</li><li>&nbsp;&nbsp;&nbsp; Nie – Es wird der erste gültige Kontrakt verwendet.</li></ul> |
| Zugang/Abgang immer separat | (NUR BEI UMBUCHUNGSVORGÄNGEN im Positionsteil)<br>Ist diese Einstellung „JA“, so wird stets bei Umbuchungen Zugang und Abgang zur separaten Erfassung angezeigt. |
| Gebinde erfassen | (NUR BEI UMBUCHUNGSVORGÄNGEN im Positionsteil)<br>Werden Artikel mit Gebinde umgebucht, so geht bei Einstellung „JA“ die Gebindererfassungsmaske auf und erwartet die Eingabe von Gebindeinformationen. |
| Bei Neuerfassung (und immer separat ein) aus Abgang vorbelegen | (NUR BEI UMBUCHUNGSVORGÄNGEN im Positionsteil)<br>Ist die Option „Zugang/Abgang immer separat“ auf „JA“ gestellt, so wird im NEU-Fall die Eingabe im Abgang automatisch in die Eingabefelder des Zugangs kopiert. |
| Gebinde im Zugang verwenden | (NUR BEI UMBUCHUNGSVORGÄNGEN im Positionsteil)<br>Ist diese Option aktiviert, so wird der Zugang mit Gebindeinformationen erfasst (sofern möglich). Ist die Option deaktiviert (NEIN), so wird der Zugang als Vorbelegung in der Ergebnismengeneinheit des Gebindes geführt werden. |
| Gebinde Preismengenbezugsübernahme | Hiermit kann eingestellt werden, dass die [Preismengenbezugsübernahme](../erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/gebindebearbeitung.md#GebindebearbPreismengenbezug) für diese Klasse / Unterklasse erfolgen soll. |
| Stammpfleger Merkmalsleiste | Dieses Kennzeichen entscheidet darüber, ob bei einer Merkmalsleistenverarbeitung im Bereich Artikeleingabe sofort der Stammdatenpfleger des Artikelstamms aufgerufen wird, um den neu erzeugten Artikel (und nur in diesem Fall) sofort zur Bearbeitung angeboten zu bekommen (Fall : Ja) oder nicht (Fall : Nein). |
| Gebinde Preismengenbezugsübernahme | |
| Verkaufsbeschränkungen auswerten | Hier kann eingestellt werden, on Verkaufsbeschränkungen, die im [Artikelstamm auf der Registerkarte Konstanten](../../artikelstamm_und_artikel/parameter_des_artikelstamms/registerkarte_konstanten.md) eingestellt werden in dieser Vorgangsunterklasse auch ausgewertet werden. Die Einstellung ist nur aktiv, wenn die Verwendung im [Steuerparameter 900 – Verkaufsbeschränkung](../../firmenstamm/steuerparameter/artikel_und_artikelstammdaten/verkaufsbeschraenkung_spa_900.md) eingeschaltet wurde. |
| Mengeneinheit und Gebindefaktoren aus kunden-/lieferantenspezifischer Artikelzuordnung verwenden | Bei der Einstellung ‚Ja‘ werden optionale Mengeneinheiten, Mengen-/Gebindeneinheitsbezeichnungen und Gebindefaktoren sowie deren Änderbarkeitskennzeichen zur Vorbelegung bei der Erfassung einer Warenpositionen herangezogen, wenn dem Artikel eine kunden-/lieferantenspezifische Mengeneinheit zugeordnet ist. Bei Folgeartikeln, Komponenten von Handelsstücklisten sowie Produktions- und Rohwarevorgängen ist diese Option nicht wirksam.<br>Siehe auch [kunden-/lieferantenindividuelle Artikelnummern](../../artikelstamm_und_artikel/individuelle_artikelnummern.md) |
