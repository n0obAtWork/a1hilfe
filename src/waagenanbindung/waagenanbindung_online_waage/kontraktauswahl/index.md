# Kontraktauswahl

<!-- source: https://amic.de/hilfe/_waage_kontraktauswahl.htm -->

Um Kontrakte in der Waage verwenden zu können, muss der Einrichterparameter Kontrakt anschließen auf Ja stehen.

Man kann in dem Feld Kontrakt entweder eine Kontraktnummer oder den Namen des Kontraktekunden eingeben. Die Angaben aus dem Kontrakt (z.B. Artikel, Kunde) werden nach der Auswahl in die Waagenmaske übernommen.  
    

Beim Wiegetyp Warenausgang oder Rohwarenausgang werden bei der **F3-**Auswahl im Feld Kontrakt nur Kontrakte der Kontraktklassen &lt; 10 angezeigt, beim Eingang nur Kontrakte der Klassen > 10 und für Lohn/Schüttwiegung Kontrakte aller Klassen.  
    

Nach der Auswahl von Rohwarenkontrakten auf dem Feld Kontrakt der Waagenmaske wird das Feld Wiegetyp entsprechend gesetzt, damit die Rohwarenbelegerzeugung richtig funktioniert.

Bei einem Kontrakt der Kontraktklasse 3 (Verkaufskontrakt Rohware) auf Rohwarenausgang;

bei einem Kontrakt der Kontraktklasse 13 (Einkaufskontrakt Rohware) auf Rohwareneingang.

Anlage eines Kontraktes

Es besteht die Möglichkeit auf der Waagemaske einen [Kontrakt](../../../kontrakt/index.md) neu anzulegen. Dazu wird der Kunde / Lieferant ausgewählt dann muss noch ein Artikel ausgewählt werden und in das Feld Dispo-Menge wird die Kontraktmenge eingetragen. Dann wird per Funktion ***Kontraktauswahl*** der Kontrakt als gesamt Mengen Kontrakt angelegt. Der Kontrakt wird dann automatisch in das Feld Kontraktnummer auf der Waagemaske übernommen und es wird die erste Zeile in dem Grid [Kontraktverteilung](../pflichtfelder_an_der_waagenmaske.md) vorbelegt.

Hinzufügen eines Artikels zu einem Kontrakt

Es besteht die Möglichkeit, den Artikel des Kontraktes zu wechseln, dann wird an den Kontrakt der neue Artikel angefügt. Dazu muss der Einrichterparameter „Alternativartikel im Kontrakt zulassen“ auf Ja gestellt. Dann wird beim Speichern der Daten der Artikel in den Kontrakt übernommen.

Artikelauswahl bei Kontraktzuordnung

Im Standard wird die Itembox 'IB_Kontrakt_Artikel_Waage' verwendet. Diese Itembox wirkt auf dem Artikelfeld und zeigt alle Kontraktartikel und die Artikel aus der Kontraktausweichliste an. Dabei ist zu beachten, dass nur bei einem Gesamtmengen-Kontrakt der Kontrakt auch dem Vorgang zugeordnet wird. Ist der Kontrakt ein Einzelmengen-Kontrakt, so muss der Schalter 'Kontrakt überziehen' im Prozess auf 'Ja' gestellt werden, damit der Kontrakt bei der Vorgangserzeugung gezogen wird.  
    

<p class="siehe-auch">Siehe auch:</p>

- [Kontraktverteilung in der Waage](./kontraktverteilung_in_der_waage.md)
- [Verkaufskontrakt (1) / Einkaufskontrakt (11)](./verkaufskontrakt_1_einkaufskontrakt_11.md)
- [Verkaufskontrakt Rohware (3) / Einkaufskontrakt Rohware (13)](./verkaufskontrakt_rohware_3_einkaufskontrakt_rohware_13.md)
- [Verkaufskontrakt Fremdlager (2) / Einkaufskontrakt Fremdlager (12)](./verkaufskontrakt_fremdlager_2_einkaufskontrakt_fremdlager_12.md)
