# Ausbuchen Fremdware/-lager SF5

<!-- source: https://amic.de/hilfe/_ausbuchenfremdwarela.htm -->

Diese Funktion im Kontraktstamm **[KTR]** macht es möglich, Verkaufskontrakte Fremdware oder Einkaufskontrakte Fremdlager, die genau einen Kontraktartikel beinhalten, auszubuchen.

Dafür müssen zuerst alle Kontrakte, die ausgebucht werden sollen, in der Auswahlliste markiert werden. Nach Aufruf der Funktion ***Ausbuchen Fremdware/-lager*** **SF5** öffnet sich eine Maske, in die alle markierten Fremdware/-lager Kontrakte übernommen werden. Kontrakte, die nicht ausgebucht werden können, werden im unteren Teil der Maske gesondert in einem Grid ausgewiesen. Anschließend wird für jeden gültigen Kontrakt der Ausbuchungspreis ermittelt. Die Art der Preisermittlung ist auf der Maske einstellbar, die Vorbelegung erfolgt aus den Einrichterparametern. Folgende Preisvorbelegungen sind möglich:

**Ohne Preis**

Der Preis wird auf 0 gesetzt

**Vom Beleg**

Der Ausbuchungspreis wird mit dem Preis aus dem zugehörigen Voreinkauf / Vorverkauf vorbelegt.

**Aus Preisliste**

Bei dieser Variante ist es notwendig in den Einrichterparametern eine Preislistennummer zu hinterlegen. Diese Preisliste wird dann zur Preisermittlung aus artikelspezifischen Preisen herangezogen.

Die ermittelten Preise können manuell geändert werden. Ferner stehen zwei Funktionen zur Verfügung, mit denen alle Preise oder der Preis der aktiven Zeile neu ermittelt werden können. Dabei wird jeweils der aktuelle Stand in ‚Art der Preisermittlung‘ ausgewertet.

Für Kontrollzwecke steht eine Belegvorschau zur Verfügung, die den zugehörigen Beleg der aktiven Zeile präsentiert.

 Das Ausbuchen erfolgt dann anschließend durch den Aufruf der Funktion ***Start Ausbuchen*** **F9**. Dabei wird für jeden einzelnen Kontrakt eine Rechnung mit entsprechender Vorgangsklasse und einer Positionszeilen erzeugt. Die Unterklasse wird über die Steuerparameter 601 (Unterklasse für Fremdlager ausbuchen) und 602 (Unterklasse für Fremdware ausbuchen) aus der Parametergruppe Kontraktwesen festgelegt.  
Das mit dem aktuellen Tagesdatum vorbelegte Vorgangsdatum für die einzelnen Ausbuchungsrechnungen kann für jede Ausbuchung individuell auf der Maske angegeben werden.

Bei Verwendung von Partien spielt der Einrichterparameter „Partie mit rückbuchen“ (Vorbelegung = JA) eine Rolle. Ist für den Kontraktartikel eine Partie angegeben, kann diese beim Erzeugen der Rechnung in der Positionszeile mit Kontrakt berücksichtigt werden.

Nach erfolgreicher Erstellung der Rechnung wird das Sperrkennzeichen des Kontraktes automatisch auf ‚Ja’ gesetzt.  
Der Einrichterparameter ‚ Erledigungskennzeichen sofort mit setzen’ legt fest, ob auch das Erledigt-Kennzeichen mit gesetzt werden soll (Vorbelegung = JA).

Wird der Einrichterparameter „Vorgang sofort öffnen nach Ausbuchung“ (Vorbelegung = NEIN) auf JA gesetzt, werden die erzeugten Rechnungen sofort im Anschluss an das Ausbuchen geöffnet.
