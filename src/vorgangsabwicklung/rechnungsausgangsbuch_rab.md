# Rechnungsausgangsbuch (RAB)

<!-- source: https://amic.de/hilfe/rechnungsausgangsbuchrab.htm -->

Im Rechnungsausgangsbuch [RBUA] können die gedruckten Verkaufsvorgänge (Rechnungen, Storno Rechnungen, Gutschriften, Storno Gutschriften) – nach Geschäftsjahr getrennt – abgelegt werden. Somit erhalten Sie eine Übersicht über die an Ihre Kunden geschickten Belege. Es können nur gedruckte Vorgänge ins RAB übernommen werden. Im RAB abgelegte Vorgänge stehen der Vorgangsbearbeitung nicht mehr zur Verfügung.

Das RAB besteht aus fortlaufend durchnummerierten Drucklisten (s. Nummernkreis). Eine RAB-Liste wird über die Funktion Erstdruck RAB erzeugt. Es erscheint ein Auswahlmenü mit weitgehenden Selektions-, Sortier- und Gruppierungsfunktionen. So ist z.B. denkbar, dass für jeden Monat eine RAB- Liste gedruckt wird.

Nach dem Erstdruck erscheint die RAB-Liste in der Auswahlliste Rechnungsausgangsbuch. Dort sind wichtige Grunddaten des Listeninhalts, wie z.B. Anzahl der enthaltenen Belege, jüngstes/ältestes Belegdatum, sowie Umsatzwerte (Summe).

Funktionen

Nummernkreis setzen

Vor Eröffnung des Rechnungsausgangsbuches sollte unter NUK ein eigener Nummernkreis eingerichtet werden. Der Nummernkreis kann unter Nummernkreis setzten F8 per F3 ausgewählt werden. Die nächste Nummer lässt sich hier setzen, sowie Text und Ober-/Untergrenze.

ACHTUNG! Wird die nächste Nummer geändert, wenn schon RAB-Listen erstellt wurden, werden u.U. die vorhandenen Listennummern automatisch angepasst!

Löschen RAB

Löschen der RAB- Listen für das in der Auswahlliste unter Bereich eingetragene Jahr.

Erstdruck RAB

Erstellung einer RAB-Liste nach Selektionskriterien. Vor der Erstellung wird zunächst geprüft, ob alle Belege im Selektionsbereich freigegeben sind. Falls noch Belege bearbeitet werden, erscheint eine Abfrage:

HINWEIS: Ein(ige) Beleg(e) im Auswahlbereich noch in Bearbeitung! RAB- Liste trotzdem starten?

Eine Bejahung der Frage erstellt eine RAB-Liste mit den freigegebenen Belegen. Auf dem Listendeckblatt ist in diesem Fall ein deutlicher Hinweis angedruckt, dass nicht alle Belege des Auswahlzeitraums in die Liste aufgenommen werden konnten.

Während der Erstellung stehen ALLE Belege des Selektionsbereiches der Vorgangsbearbeitung NICHT mehr zur Verfügung.

Alle im RAB befindlichen Belege sind ebenfalls von weiterer Vorgangsbearbeitung ausgeschlossen.

Beschreibung der Auswahlmaske und Selektionskriterien

Tabelle 1

| Auswahlbedingung | Raffg. | Sel. | Grup. | Bemerkungen |
| --- | --- | --- | --- | --- |
| !Kunde | Ja | Ja | Ja | wahlfreie Gruppierung; Rang wie in der Spalte Gruppierung angegeben.  
'\*' in der von-Spalte bedeutet: Raffung (Ausblenden der Auswahlbedingung, keine Kundenzwischensummen, Berücksichtung des Datenmaterials aller Kunden).  
'\*10000' in der von-Spalte bedeutet ebenfalls Raffung; aber unter Berücksichtigung des selektierten Bereiches.  
HINWEIS: die Funktionalität des '\*' gilt analog bei anderen gruppierbaren Selektionskriterien. Diese sind durch ein führendes '!'gekennzeichnet. |
| Kundentyp | \- | \- | \- | Auswahl des zu berücksichtigenden Kundentyps |
| Belegdatum | Nein | Ja | Nein | |
| Valutadatum | Nein | Ja | Nein | |
| Belegnummer | Nein | Ja | Nein | |
| !Fakturiergruppe | Ja | Ja | Ja | |
| !Bediener Neu | Ja | Ja | Ja | wahlfreie Gruppierung über alle Bediener, die im Selektionsbereich Vorgänge ERSTELLT haben. |
| !Bediener Korrektur | Ja | Ja | Ja | wahlfreie Gruppierung über alle Bediener, die im Selektionsbereich Vorgänge KORRIGIERT haben. |
| !Vorgangsart | Ja | Ja | Ja | wahlfreie Gruppierung über den Verkauf betreffende Vorgänge (Rechnungen und Gutschriften, sowie deren Stornierungen).  
Folgende Selektionen sind möglich:  
• Rechnungen/Gutschriften VK  
• \*Rechn./Gutschr. VK gerafft  
• nur Rechnungen VK  
• \*nur Rechn. VK gerafft  
• nur \*nur Gutschr. VK gerafft   
• nur Storno- Rechnungen VK  
• \*nur Storno- Rechn. VK gerafft   
• nur Storno- Gutschriften VK  
• \*nur Storno- Gutschr VK geraff |
| !Bank | Ja | Ja | Ja | wahlfreie Gruppierung über die im Beleg angesprochenen Kundenbanken. |
| !Steuerschlüssel | Ja | Ja | Ja | wahlfreie Gruppierung über die im Beleg angesprochenen Steuerschlüssel. |
| !Steuerklasse | Ja | Ja | Ja | wahlfreie Gruppierung über die im Beleg angesprochenen Steuerklassen. |
| Sortierung nach | \- | \- | \- | Auswahl der Sortierung für den Datenrumpf der Liste:  
• Belegnummer  
• Belegdatum  
• Valutadatum  
• Betrag |
| Rangfolge | \- | \- | \- | Rangfolge der Sortierung im Datenrumpf:  
z.Zt. nur Absteigend  
Aufsteigend  
Absteigend |
| Kundenstammdaten | \- | \- | \- | Andruck ausführlicher Kundenstammdaten (Name, Anschrift, Telefon) bei Gruppierung nach Kunden im Gruppenkopf/-fuß:  
• Nein  
• Kundendaten kurz  
• Kundendaten lang |
| Kundenjahressummen | \- | \- | \- | Andruck der Kundenjahressummen Vorjahr/Lfd.Jahr im rechten Teil der Liste:  
• Nur bei Gruppierung nach Kunden als Gruppe 1  
• Ja  
• Nein |
| Skontobetrag | \- | \- | \- | Andruck der Spalte Skontobetrag  
• Ja  
• Nein  
• Kopie (Bildschirm) |

Anklicken einer RAB Liste der Auswahlliste. Die Liste erscheint auf dem Bildschirm im Vorschau-Modus.

Sie ist eine Kopie des Erstdrucks. Ein Zähler in der Liste weist die Anzahl der Ausdrucke auf. Die Liste wird deutlich als Kopiedruck mit der Listennummer und der Drucknummer des Kopienzählers ausgewiesen.

Kopie (Drucker)

Wie oben; jedoch direkt auf den Standard-Drucker.

Kopie mit Auswahl

Die gewählte RAB-Liste kann vor dem Ausdruck in ihren Selektionskriterien noch verändert werden.

ACHTUNG!!

Die Veränderung der Kriterien wirkt sich jedoch nur auf das beim Erstdruck selektierte Datenmaterial aus!

Diese Funktion ist sinnvoll, wenn z.B. andere Gruppierungen über das gleiche Datenmaterial vorgenommen werden sollen. Als Auswahl wird diejenige des Erstdrucks angeboten.

Varianten

Rechnungsausgangsbuch

Bereich:

Anzeige der kumulierten Grunddaten der RAB Listen

Rechnungsausgangsbuch (Belege)

Bereich wie oben.

Anzeige der nicht kumulierten Daten der RAB Listen.

Rechnungsausgangsbuch (Auswahl)

Bereich wie oben.

Anzeige der zu den RAB Listen gehörenden Selektionsmasken

Neue Belege (noch nicht im RAB)

Bereich:

Belege, die noch nicht im RAB sind.
