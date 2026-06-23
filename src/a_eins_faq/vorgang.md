# Vorgang

<!-- source: https://amic.de/hilfe/_faq_vorgang.htm -->

<details>
<summary>Frage: In der Rechnungsbearbeitung [REB] fehlt eine Belegnummer. Was ist mit dieser Rechnung?</summary>

<p class="just-emphasize">Antwort:</p>

Wenn in einer Auswahlliste ein Beleg nicht wie erwartet angezeigt wird, kann es mehrere Gründe geben. Prüfen Sie als erstes die Auswahlbedingungen im F2-Filter. Vielleicht wurde hier eine unzutreffende Eingrenzung vorgenommen.  
Eine andere Möglichkeit ist, dass der Beleg über die Funktion „Stornieren“ (F7) aus dem System entfernt worden ist. Unter dem Direktsprung STOPO (Stornoprotokoll) sind die Belegnummern und weitere Eckdaten aufgeführt. Der Beleg ist dann nicht mehr existent und aufrufbar. Die Einstellung des Steuerparameters [SPA] Nr. 490 „Storno-Belegnummern wieder reaktivieren“ auf den Wert „Nein“ verhindert die erneute Vergabe der Belegnummer.

</details>

<details>
<summary>Frage: Beim Erfassen von Eingangsrechnungen ist bei uns für den Administrator „Zeilen-Zu-/Abschlag“ freigeschaltet. Der normale Bediener sieht diese Zeile in der Option Box unten links nicht. Wo kann ich für Normalbediener alle Funktionen in der Option Box freischalten?</summary>

<p class="just-emphasize">Antwort:</p>

Die Berechtigungen für Funktionen in Option Boxes kann man ändern, wenn man als Administrator in der betreffenden Option Box auf den untersten Eintrag navigiert: „Dieses Menü“.

Entweder filtert man dann mit **F2** den Text der Beschriftung oder wählt direkt aus der Auswahlliste.

Ist der entsprechende Datensatz markiert, so öffnet sich mit ***„Ändern“*** **(F5)** eine Maske, auf der für die Bedienerklassen ablesbar und einstellbar ist, ob ihnen die Funktion zur Verfügung steht.

Die Einträge in der Spalte „Soll“ können hier angepasst werden. Die Zuweisung der Berechtigungen (Ja/Nein) erfolgen immer je Bedienerklasse. Angezeigt werden auch die Kurzbezeichnungen der dieser Klasse zugehörigen Bediener. Die Namen zu den Bediener-Kurzbezeichnungen findet man im Bedienerstamm **[BD]**.

Zusatzhinweis: Wenn es die Kombination aus Bedienerklassen bereits gibt, dann wird die entsprechende Rollenbezeichnung oben im Feld „Rolle“ eingesetzt. Gibt es für diese Kombination noch keine Rolle, so wird automatisch eine neue Rolle angelegt. Man erkennt dies auch am „Ja“ im Feld „neue Rolle“. Die Bezeichnung ist dann im Feld „Rolle“ frei eingebbar.

Bereits bestehende Rollen können übrigens auch an dieser Stelle der Funktion zugeteilt werden anstatt über das Grid.

Weitere Informationen: [Rollenstamm](../firmenstamm/firmenkonstanten/zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenpflegerstamm.md)

</details>

<details>
<summary>Frage: Was passiert mit dem Lieferschein, wenn man eine Rechnung storniert (bzw. löscht) mit F7?</summary>

<p class="just-emphasize">Antwort:</p>

Nach dem Löschen einer Rechnung in der Rechnungsbearbeitung mit der Funktion „Stornieren“ F7 kann sich der zugehörige Lieferschein unterschiedlich verhalten in Abhängigkeit davon, ob es sich um eine Teilumwandlung oder eine (komplette) Umwandlung handelt und bei letzterer je nach Einstellung im Steuerparameter 987.

Exkurs zur Teilumwandlung: Standard-Teildisposition und Mehrfachteildisposition erzeugt man in einem Beleg im Positionsteil mit den gleichlautenden Funktionen. Weitere Informationen: [Standard Teildisposition/Mehrfachdispositionen](../vorgangsabwicklung/standard_teildisposition_mehrfachteildisposition/index.md)

Beim Stornieren einer Rechnung mit (mehrfach) teildisponierten Positionen werden diese Warenpositionen im Lieferschein in der entsprechenden Menge wieder frei. Dieses Verhalten trifft auch dann zu, wenn sich alle Warenpositionen eines einzigen Lieferscheins auf einer einzigen Rechnung befinden, die Anlage jedoch über die Teilumwandlung stattgefunden hat.

Wurde ursprünglich aus der Auswahlliste Lieferscheinbearbeitung heraus mit der Funktion „Rechnung aus Lieferschein“ die Rechnung erzeugt, dann ist das Verhalten abhängig vom Steuerparameter 987. Steht dieser auf „Nein“, dann bleibt der Lieferschein als ganz abgearbeitet im System stehen. Ist dieser SPA auf „im Einkauf und Verkauf“ eingestellt, dann steht der Lieferschein nach dem F7-Storno der Rechnung wieder so im System, als hätte gar keine Rechnungserzeugung stattgefunden.

Weitere Informationen: [SPA987](../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/quellbelegreaktivierung_bei_stornieren_loeschen_von_warebele.md)

Ergänzung: Wie hier für Rechnung und Lieferschein beschrieben, verhält es sich immer in Bezug zum Beleg der Vorstufe. Basiert die Rechnung auf einem Auftrag und wurde die Zwischenstufe Lieferschein übersprungen, dann trifft das beschriebene Verhalten auf die Warenposition(en) des Auftrags zu. Gleiches gilt bspw. wenn noch keine Rechnung erstellt wurde und ein Lieferschein mit Stornieren gelöscht wird.

</details>

 

<details>
<summary>Frage: Wie richte ich einen TSE-QRCode ein?</summary>

<p class="just-emphasize">Antwort:</p>

Voraussetzungen:

- SPA 1058 (TSE Lizenz)
- Ein Formular mit der Formularbereich 860 (Kassenfuß) oder 902 (Abschluss und Fuß)

    
Der erster Schritt muss ein Formular für den QRCode eingerichtet werden. Dafür navigiert man in die Anwendung Formulare **[FRM]**. Hier mit Neu **(F8)** einen neuen Datensatz anlegen. Im neuen Datensatz sowohl eine ID, als auch eine Bezeichnung vergeben. Dann den Datensatz Speichern **(F9)** und diesen mit Bearbeiten **(F5)** wieder öffnen. Als nächstes den Formulartyp wählen (Voraussetzung beachten) und unter dem Tab ***„Formularbereiche“*** den Bereich 860 oder 902 hinzufügen und wie gewünscht konfigurieren (Maße). Den Formularbereich Einrichten **(F6)** und hier die Position 46 (TSE_QR_CODE) hinzufügen und die Maße wie gewünscht konfigurieren. Als Text kann entweder der AMIC Etiketten-Standard für genutzt werden („AMIC_TSE_QRCODE“), oder man richtet sich selbst eine private Konfiguration ein.

    
Für eine Anpassung der Maße des TSE-QRCodes navigiert man in die Anwendung Etikettendruck mit dem Direktsprung **[LILA]** in die Variante 2 „Vorlagen Etiketten“. Dann wählt man den Datensatz mit dem Namen „*AMIC_TSE_QRCODE*“ aus und führt die Funktion ***Vorlage übernehmen*** **(F9)** aus. Nun öffnet sich ein Pfleger, in welchem lediglich erneut (F9) zum übernehmen gedrückt werden muss, um eine private ID zu vergeben z.B: „*P_TSE_QRCODE*“. Nach Abschluss der Übernahme navigiert man in die Variante 1. „Private Etiketten“. Hier den erstellten Datensatz auswählen und auch das Funktionslabel anpassen. Danach mit **(F6)** den QR-Code bearbeiten. In der Anwendung muss lediglich auf den QRCode geklickt werden, um dann links unter der Kategorie „*Position*“ die Dimensionen anzupassen. (Bitte beachten, dass ein QRCode im Verhältnis 1:1 bleiben muss, also Breite = Höhe). Oben links in der Anwendung ist ein Speicher Symbol, diese Funktion ausführen und die Anwendung schließen.

    
Als letztes muss das Formular lediglich hinzugefügt werden:

- Für Vorgänge: [FRZ] -> Vorgang auswählen -> (F5) -> Tab: Formulare -> Feld: Druckformular -> eingerichtetes Formular eintragen
- Für Finanzvorgänge: Hauptmenü -> Barvorgänge -> Kassenverwaltung -> Kasse auswählen -> (F5) -> Tab: Formulare -> Kategorie: Finanzvorgänge -> Feld: gewünschter Finanzvorgang -> eingerichtetes Formular eintragen

</details>

<details>
<summary>Frage: Wie richte ich einen EPC-QRCode ein?</summary>

<p class="just-emphasize">Antwort:</p>

Voraussetzungen:

- Ein Formular mit der Formularbereich 902 (Abschluss und Fuß) oder 1100 (SQL Liste)

    
ein Formular für den QRCode eingerichtet werden. Dafür navigiert man in die Anwendung Formulare **[FRM]**. Hier mit Neu **(F8)** einen neuen Datensatz anlegen. Im neuen Datensatz sowohl eine ID, als auch eine Bezeichnung vergeben. Dann den Datensatz Speichern **(F9)** und diesen mit Bearbeiten **(F5)** wieder öffnen. Als nächstes den Formulartyp wählen (Voraussetzung beachten) und unter dem Tab ***„Formularbereiche“*** den Bereich 902 oder 1100 902 hinzufügen und wie gewünscht konfigurieren (Maße). Den Formularbereich Einrichten **(F6)** und hier die Position 45(EPC_QR_CODE) hinzufügen und die Maße wie gewünscht konfigurieren. Als Text kann entweder der AMIC Etiketten-Standard für genutzt werden („AMIC_EPC_QRCODE“), oder man richtet sich selbst eine private Konfiguration ein.

    
Für eine Anpassung der Maße des EPC-QRCodes navigiert man in die Anwendung Etikettendruck mit dem Direktsprung **[LILA]** in die Variante 2 „Vorlagen Etiketten“. Dann wählt man den Datensatz mit dem Namen „*AMIC_EPC_QRCODE*“ aus und führt die Funktion ***Vorlage übernehmen*** **(F9)** aus. Nun öffnet sich ein Pfleger, in welchem lediglich erneut (F9) zum übernehmen gedrückt werden muss, um eine private ID zu vergeben z.B: „*P_EPC_QRCODE*“. Nach Abschluss der Übernahme navigiert man in die Variante 1. „Private Etiketten“. Hier den erstellten Datensatz auswählen und auch das Funktionslabel anpassen. Danach mit **(F6)** den QR-Code bearbeiten. In der Anwendung muss lediglich auf den QRCode geklickt werden, um dann links unter der Kategorie „*Position*“ die Dimensionen anzupassen. (Bitte beachten, dass ein QRCode im Verhältnis 1:1 bleiben muss, also Breite = Höhe). Oben links in der Anwendung ist ein Speicher Symbol, diese Funktion ausführen und die Anwendung schließen.

Als letztes muss das Formular lediglich hinzugefügt werden:

- Für Vorgänge: [FRZ] -> Vorgang auswählen -> (F5) -> Tab: Formulare -> Feld: Druckformular -> eingerichtetes Formular eintragen
- Für Finanzvorgänge: Hauptmenü -> Barvorgänge -> Kassenverwaltung -> Kasse auswählen -> (F5) -> Tab: Formulare -> Kategorie: Finanzvorgänge -> Feld: gewünschter Finanzvorgang -> eingerichtetes Formular eintragen

</details>

<details>
<summary>Frage: In der Rechnungsbearbeitung [REB] wird für einen Beleg „Datenvorbereitung fehlerhaft“ angezeigt. Was ist der Fehler?</summary>

<p class="just-emphasize">Antwort:</p>

Die Meldung „Datenvorbereitung fehlerhaft“ weist darauf hin, dass das System keine Mailadresse ermitteln kann. Die Mailadresse wird aus der im Vorgang gespeicherten AdressID gezogen. Wenn Sie den SPA 574 auf „Hauptanschrift archivieren“ stehen haben, wie dies üblicherweise der Fall ist, dann wird bei Änderungen an der Hauptanschrift diese nicht einfach überschrieben, sondern die alte erhalten und eine neue AdressID vergeben. Im Vorgang können Sie über „Hauptanschrift aktualisieren“ die alten Daten mit den neuen Daten aus dem Anschriftenstamm überspeichern.

</details>

<details>
<summary>Frage: Wie verändere ich das Verhalten von Sammelumwandlungen von Bestellungen zu Eingangslieferscheinen und Eingangsrechnungen?</summary>

<p class="just-emphasize">Antwort:</p>

Man muss bei **[SPA]** mindestens einen Trennungssteuerparameter (90-96) auf „Trennen“ setzen. Falls das betroffenen Feld bei **[BSB]** Bestellungen fehlt, muss man dieses unter **[UFLD]** einrichten. Je nach Sammelumwandlung muss der Steuerparameter 1137 oder 1138 auf „Mit Trennung“ gesetzt werden. Die Funktion ***Sammel-EL aus Best.*** läuft über den SPA 1137 und ***Sammel-EK aus Best.*** über den SPA 1138. Durch das Setzen auf „Mit Trennung“ werden aus mehreren Bestellungen ein Zieldokument(Eingangslieferschein, Eingangsrechnung) erstellt.

</details>
