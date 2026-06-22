# Kostenobjekte

<!-- source: https://amic.de/hilfe/_kostenobjekte.htm -->

Hauptmenü \> Kostenrechnung \> Kostenobjektstamm \> Kostenobjekte

Direktsprung **[KSOBJ]**

Neben den [Kostenstellen](../kostenstellen.md) und [Kostenträgern](../kostentraeger.md) können Kosten einem Kostenobjekt zugeordnet werden. Diese unterscheiden sich in folgenden Punkten von den Kostenstellen und Kostenträgern:

• Es existiert nicht die Möglichkeit der Verteilung. Es gibt weder Verteilkostenobjekte für die automatische Verteilung noch existiert bei der Erfassung die Möglichkeit den Betrag einer Position auf verschiedene Kostenobjekte aufzuteilen.

• Zu Kostenobjekten existiert – im Gegensatz zur Kostenstellen und Kostenträgern - keine Tabelle, in der die Summen geführt werden. Diese können direkt aus der View AMIC_V_FIBUBELEG gelesen werden.

• Die Kostenobjekte sind für individuelle Auswertungen von A.eins Anwendern gedacht. Daher existieren keine Standard-Auswertungen.

Für Kostenobjekte wird eine Lizenz benötigt.

**Felder der Kostenobjekte**

| **Feld** | |
| --- | --- |
| Nummer | Die Nummer des Kostenobjektes.  
 |
| Matchcode | Die Kurzbezeichnung des Kostenobjektes.  
 |
| Bezeichnung | Die Bezeichnung des Kostenobjektes.  
 |
| Gesperrt | Gibt an, ob das Kostenobjekt für die Belegerfassung in der Finanzbuchhaltung gesperrt ist.  
 |

**Suchmöglichkeiten der Kostenobjekte**

| **Feld** | |
| --- | --- |
| Nummer | Von … Bis …  
 |

**Funktionen in der Auswahlliste**

| **Funktion** | |
| --- | --- |
| Ändern (F5) | Ändern des Kostenobjektes.  
 |
| Ansehen (F6) | Ansehen des Kostenobjektes.  
 |
| Löschen (F7) | Mit der Löschen\-Funktion werden Kostenobjekte nicht physikalisch gelöscht, sondern sie werden mit einem Löschkennzeichen versehen. Gelöschte Kostenobjekte sind für weitere Belegerfassungen gesperrt bis sie wiederhergestellt werden.  
Alle gelöschten Kostenobjekte werden in der 2.Variante „Gelöschte Kostenobjekte“ angezeigt.  
Bedingung: Bevor ein Kostenobjekt gelöscht werden kann, wird überprüft, ob dieses noch verwendet wird. Solange Einträge des Kostenobjektes in den folgenden Punkten vorhanden sind, kann die Löschung nicht erfolgen:  
• [Sachkontenstamm](../../stammdaten_der_fibu/sachkonten.md)  
• [Mandantenstamm](../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_FIBU) (als Fehlerkostenobjekt)  
• [Mahnsatz](../../mahnwesen/mahnsaetze_einrichten.md)/[Mahnzinsen](../../mahnwesen/mahnzinsen.md)  
• [Zinsgruppen](../../zinswesen/stammdaten_zinswesen/zinsgruppen.md)  
• [Periodische Buchungen](../../belegerfassung/periodische_buchungen.md)  
• [Kostenobjektgruppen](../kostenobjektgruppe.md)  
Bei bereits bebuchten Kostenobjekten erscheint ein Hinweis mit Abfrage, ob tatsächlich gelöscht werden soll.  
 |
| Wiederherstellen (F7) | Beim Wiederherstellen eines gelöschten Kostenobjektes wird sein Löschkennzeichen entfernt und es kann wieder in der Belegerfassung verwendet werden.  
Das wiederhergestellte Kostenobjekt wird in der 1.Variante „Kostenobjekte“ angezeigt.  
 |
| Neu (F8) | Anlage eines neuen Kostenobjektes.  
 |

<p class="siehe-auch">Siehe auch:</p>

- [Kostenobjekte: Einrichtung](./kostenobjekte_einrichtung.md)
- [Kostenobjekte: Pfleger](./kostenobjekte_pfleger.md)
