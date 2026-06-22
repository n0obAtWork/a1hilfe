# EK-Waage-RWLieferungen/VK-Waage-RWLieferungen

<!-- source: https://amic.de/hilfe/_rwwev_anlieferung.htm -->

Hauptmenü > Rohwarenabrechnung > EK-Waage-RWLieferungen

Direktsprung [RWWE]

Hauptmenü > Rohwarenabrechnung > VK-Waage-RWLieferungen

Direktsprung [RWWV]

Alle aus der Waage oder mit der Offline-Waage erzeugten Rohwarenbelege werden in diese Schnittstelle übertragen. Sofern in dem [Wiegeprozess](../waagenanbindung_online_waage/prozess_einrichten/index.md) auf der [Registerkarte Rohware](../waagenanbindung_online_waage/prozess_einrichten/registerkarte_rohware.md) für die Online-Waage nicht der Punkt Rohwarenbelege sofort erzeugen auf „Ja“ steht können hier noch die erfassten Daten für den Rohwarenbeleg korrigiert werden.

Ausprägungen des Feldes Status

| Bezeichnung | Bedeutung |
| --- | --- |
| Übernahme läuft | Dies bedeutet, dass zu Importierende Daten in die Schnittstelle übernommen werden |
| Fehl: ÜB! | Fehler bei der Übernahme |
| \-- | Einspielung hat funktioniert. Datensatz kann weiterverarbeitet werden |
| Belerz. Läuft | |
| FEHL Belerz! | Fehler bei der Belegerzeugung. Die Daten müssen mit Ändern F5 korrigiert werden. |
| erledigt! | Es wurde ein Rohwarenbeleg erzeugt. Dieser ist unter [RWE] für Rohwareneinkauf oder [RWB] Rohwarenverkauf zu finden. |

Funktionen der Waagenimportschnittstelle

| Funktion | Bedeutung |
| --- | --- |
| RW-Waage Import Shift + F12  
 | Mit dieser Funktion werden Waagenbelege in die Schnittstelle importiert. Einrichtung der Scriptparameter finden Sie [hier](../waagenimport/index.md). |
| Waage Qualitäten Shift +F7  
 | Mit dieser Funktion können Qualitäten zu einem Satz nach erfasst werden. |
| FEHL: Belerz Rücksetzten Shift +F8  
 | Mit dieser Funktion kann der Status auf – zurückgesetzt, wenn bei der Belegerzeugung ein Fehler aufgetreten ist. |
| Ganze Überna. Löschen F7  
 | Mit dieser Funktion werden markierte Belege komplett aus der Schnittstelle gelöscht. Erzeugte Belege und Wiegungen belieben aber bestehen. |
| Aufräumen  
 | Mit dieser Funktion werden alle Belege aus der Schnittstelle gelöschte erzeugte Beleg bleiben aber bestehen. |
| Ändern F5  
 | Mit dieser Funktion kann ein Satz vorm Erzeugen einer Lieferung noch korrigiert werden. |
| Ansehen F6  
 | Mit dieser Funktion kann ein Satz zur Ansicht geöffnet werden. |
| Lieferungen erzeugen F9  
 | Diese Funktion erzeugt aus einem markierten Datensatz einen Rohwarenbeleg. |
| Analysewerte korrigieren F11 | Mit dieser Funktion können die Analysewerte korrigiert werden. |

<p class="siehe-auch">Siehe auch:</p>

- [Funktion Ändern](./funktion_aendern.md)
- [Funktion Beleg löschen](./funktion_beleg_loeschen.md)
