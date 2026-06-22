# Allgemein – Allgemeine und generelle Vorgangszuordnungen

<!-- source: https://amic.de/hilfe/allgemeinallgemeineundgenerell.htm -->

| Feld | Beschreibung |
| --- | --- |
| Periodenbehandlung | F3 Funktion zur Periodenbehandlung  
• Jahresgrenzen/Inventurgrenzen  
• Lieferdatum nie in geschlossener Periode  
 |
| Lager / Strecke | F3 Funktion zur Auswahl Lager oder Strecke  
 |
| Barabwicklung Vorgang (ohne Kassensystem) | F3 Auswahl Ja oder Nein  
 |
| (Kassen)Konto für Barabwicklung | F3 Auswahl für Konto Barabwicklung  
 |
| Eigener Nummernkreis bei Stornobelegen | Auswahl Nummernkreis bei Stornobelegen  
 |
| Arbeitsregelnummer | Auswahl für Arbeitsregeln  
 |
| Rohware Vorerfassung | Kennzeichen für Lieferscheine im Ein- und Verkauf:  
\- ohne: Keine [Rohware-Wandlung](../vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lieferschein/rohware_wandlung.md) möglich  
\- möglich: [Rohware-Wandlung](../vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lieferschein/rohware_wandlung.md) möglich kann erfolgen,  
wenn bestimmte Voraussetzungen vorhanden sind  
\- geprüft: Der Beleg wird bei Belegabschluss auf Einhaltung der Voraussetzung für die [Rohware-Wandlung](../vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lieferschein/rohware_wandlung.md) geprüft.  
 |
| Kokore Druckverhalten | Kokore Druckverhalten  
 |
| Artikel Itembox | Angabe einer alternativen Itembox  
 |
| Kunden Itembox | Hier kann eine alternative Itembox angegeben werden, die in der Vorgangserfassung zur Verfügung stehen soll, um Kunden auszuwählen. So kann z.B. eingerichtet werden, dass bei Angeboten eine Interessenten-Auswahlliste, bei Rechnungen nur eine Liste der Kunden angezeigt wird, die Ware erhalten haben.  
 |
| Nach Druck korrigierbar  
 | Hier kann pro Vorgangsklasse und Unterklasse eingestellt werden ob der Vorgang nach dem Druck korrigierbar ist.  
Diese Funktionalität wurde aus dem Steuerparameter 67 ‚Rechn./Gutschr. nach Druck korrigierbar’ hierher übertragen und der Steuerparameter wurde deaktiviert.  
Die Einstellung aus dem Steuerparameter wurde für die Vorgangsklassen 700-890, 1700-1890 und >5000 übernommen.  
Vorbelegung ist Nein.  
 |
| Sofort Fibu Übertrag beim Drucken (nur für Druckmodul)  
 | Hier wird entschieden, ob beim Drucken sofort ein Fibu Übertrag stattfindet.  
Diese Einstellung kann als Ergänzung zum Steuerparameter 337 ‚Rechnungsdruck m. Fibu-Übertrag-Abfrage’ aus der Parametergruppe ‚Vorgangsbearb. Spezialitäten’ gesehen werden.  
Vorbelegung ist Nein.  
Das Feld ist nur sichtbar bei den Vorgangsklassen 700-890 und 1700-1890.  
Beispiel:  
Stellt man den Steuerparameter 337 auf Nein und dieses Feld auf Ja, dann besteht die Möglichkeit beim Drucken sofort einen Fibu-Übertrag auszulösen, ohne dass noch eine Abfrage erscheint.  
 |
| Bei Einzelumwandlung Addon kopieren | Wird der Beleg umgewandelt (nicht Sammelumwandlung) oder mittels Auswahllistenfunktion teildisponiert, so werden aus dieser Quellvorgangsunterklasse mit dieser Einstellung Vorgangsaddons in den neuen beleg kopiert. |
| Kontraktexportprozedur | Dieses Feld erscheint nur bei den Vorgangsklassen Einkaufskontrakt, Verkaufskontrakt sowie bei Rohwarekontrakten. Diese Funktion wird aufgerufen, Sobald ein Kontrakt geändert wird. Neben möglichen Exporten von Kontraktänderungen steuert diese Funktion zusätzlich, ob die Änderungs-Historie geschrieben wird. Dazu muss sie als Rückgabewert die 1 liefern und auf dem Register „Partie“ muss bei „Export im Tagebuch“ **Ja** eingetragen sein  
   
 |
| Quellbeleg freigeben bei Stornobeleg | (0) – Nein – Beim Erstellen eines Stornobelegs werden die Quellbelege grundsätzlich nicht reaktiviert  
(1) – Ja – Beim Erstellen eines Stornobelegs werden die Quellbelege grundsätzlich reaktiviert  
(2) – Entscheidung – Quellbelege werden bei der Erstellung des Stornobelegs nur freigegeben, wenn dies auf der Umwandlungs-Maske eingeschaltet wird. Eine Freigabe des Quellbelegs bei Erstellung eines Stornobelegs mit Kopie ist nicht möglich |
| Warnhinweis Permanente Inventur | Soll eine Warnung ausgegeben werden, wenn der Artikel bereits einer permanenten Inventur zugewiesen wurde?  
(0) – Warnung wird auf dem Bildschirm ausgegeben  
(1) – Warnung wird nur ins Fehlerprotokoll geschrieben  
(2) – Warnung wird komplett ignoriert |
