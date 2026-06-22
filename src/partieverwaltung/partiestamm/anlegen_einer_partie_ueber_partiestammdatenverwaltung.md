# Anlegen einer Partie über Partiestammdatenverwaltung

<!-- source: https://amic.de/hilfe/_anlegeneinerpartiebe1.htm -->

Hauptmenü > Partieverwaltung > Chargen / Partien > Partie-Stammdaten

oder Direktsprung **[PAR]**

Über die Funktion ***Neu*** **F8** wird eine neue Partie angelegt.

| Felder |
| --- |
| Partienummer | Vorschlag einer automatischen Systemnummer, die überschrieben werden kann (über Mandantennummernkreis **[MNDNK]** wird ein Zählkreis den Partien zugeordnet) |
| Bezeichnung | Bezeichnung der Partie |
| Matchcode | Matchcode der Partie |
| Laufzeit | Zeitraum, in dem die Partie bebucht werden kann |
| Gesperrt | Gesperrte Partien können nicht bebucht werden. Durch private Varianten ist es möglich, gesperrte Partien zu selektieren. |
| Bestandsprüfung aussetzen | Wird die Bestandsprüfung ausgesetzt, dann wird beim Zuordnen der Partie nicht überprüft ob genügend Bestand vorhanden ist. |
| Qualitaetsstatus | Hier wird festgelegt, ob noch eine Qualitätsuntersuchung der Partie sinnvoll ist oder nicht. |
| Erledigung | Die Partie wird nicht zur Auswahl angeboten bzw. berücksichtigt. |
| Kundenbereich (Verkauf) | alle: alle Kunden können aus dieser Partie beliefert werden  
Liste: Es kann eine Liste derjenigen Kunden aufgebaut werden die aus dieser Partie beliefert werden dürfen. Diese Eingabemaske wird per Knopf ***Kunden*** in der Optionbox geöffnet. |
| Lieferantenbereich (Eink) | alle: alle Lieferanten können dieser Partie liefern  
Liste: Es kann eine Liste derjenigen Lieferanten aufgebaut werden die diese Partie liefern dürfen. Diese Eingabemaske wird per Knopf ***Lieferanten*** in der Optionbox geöffnet. |
| Warengruppenbereich | Es können Warengruppen dieser Partie zugeordnet werden, deren Artikel aus dieser Partie beliefert werden dürfen. |
| Fremdartikel zulässig | deaktiv |
| Fixpreise im Verk./Eink. | Es können Preise in den Partien hinterlegt werden, die bei der Vorgangserfassung für den Einkauf sowie den Verkauf automatisch vorgeschlagen werden. |
| Währung | Währung der Partie |
| Mengeneinheit | Grundmengeneinheit der für diese Partie relevanten Artikel. In der Folgemaske kann eine Artikelauswahl über alle Artikel getroffen werden die diese Grundmengeneinheit besitzen. Wenn dies nicht gewünscht ist, kann hier mit einer Grundmengeneinheit = 0 gearbeitet werden (dies verursacht eine Meldung, die ignoriert werden kann). |
| Minderwertigkeit-Kennzeichen(in %) | Hier kann man minderwertige Partien in prozentualen Werten angeben. |
| Archiv-Referenz | Nummer wird automatisch beim Anlegen der Partie vorbelegt. Die Partienummer kommt in der Referenznummer vor. |

| Menü |
| --- |
| ***Ende*** **ESC** | Abbruch der Erfassung mit Abfrage (Maske wird verlassen) |
| ***Speichern*** **F9** | Speichern der erfassten Partie |
| ***Anschrift*** **F4** | Hinterlegung der Partieanschrift (evtl. als Hilfsmittel für Formulareinrichtung) |
| ***Bewegungen*** **SF9** | Übersicht der Partiebewegungen in Menge, Wert, Artikel und Belegnummern |
| ***Artikel*** **F2** | Nachdem diese Partie mit ***Speichern*** **F9** gesichert wurde, erscheint eine weitere Funktion ***Artikel*** **F2**. Über diese Funktion können dieser Partie der/die Artikel zugeordnet werden. Die Vorbelegung „Typ der Zuordnung“ wird über den Steuerungsparameter 9 (siehe Steuerungsparameter [SPA] Partieverwaltung) gesteuert.  
Ist die Zuordnung erfolgt, können weitere Funktionen für die Artikel angewendet werden:  
• Mengen: Erfassung der Sollmengen- und Werte  
• Bewegungen: Informationen über die bereits vorhandenen Bewegungen des markierten Partieartikels |
| ***Kunden*** | Erscheint im Menü erst, wenn im Feld Kundenbereich „Liste“ ausgewählt wurde. Öffnet die Maske Partiekunden. |
| ***Lieferanten*** | Erscheint im Menü erst, wenn im Feld Lieferantenbereich „Liste“ ausgewählt wurde. Öffnet die Maske Partielieferanten. |
