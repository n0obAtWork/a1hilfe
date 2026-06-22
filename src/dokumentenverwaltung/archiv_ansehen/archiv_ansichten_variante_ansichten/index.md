# Archiv-Ansichten-Variante: Ansichten

<!-- source: https://amic.de/hilfe/_archivansichtenvaria.htm -->

Hauptmenü > Administration > Archiv > Zugriffssteuerung > Ansichten

Direktsprung **[FAA]**

| Feld |
| --- |
| Name | Bezeichnung der Archiv-Ansicht |
| Bedienerklasse | Zugeordnete Bedienerklasse.  
Sind mehrere Ansichten gleichen Namens vorhanden, entscheidet die Bedienerklasse darüber, welche Archiv-Ansicht zur Verfügung gestellt wird.  
Somit ist es möglich, jeweils verschiedenen Bedienerklassen auch bestimmte Archiv-Ansichten zukommen zu lassen.  
Die Bedienerklasse -1 steht stellvertretend für alle Bedienerklassen.  
Die Bezeichnung dieser „Bedienerklasse“ ist „***Defaultklasse Kunden***“ |
| Bedienerklassenbezeichnung | Bezeichnung der Bedienerklasse |
| Besitzer | ***AMIC***:  
Auf Kundendatenbanken handelt es sich dabei um eine „Auslieferung“.  
***Privat***:  
Eine privatisierte Auslieferung oder eine neu erstellte Ansicht, deren Ansichts-Name keiner Auslieferung zugeordnet ist. |
| Grundlage | Versucht über das Einsatzgebiet der Archiv-Ansichten zu informieren.  
Mögliche Identifizierungen sind:  
0 : Frei  
1 : Auswahlliste  
2 : Dialog  
3 : Extern  
4: Auswahl |
| Ansichts-Status | ***Auslieferung****:*  
Auslieferungen sind Ansichten, die mit „*AMIC_*“ beginnen und deren Besitzer „*AMIC*“ ist.  
***Privatisierte Auslieferung:***  
Privatisierte Auslieferungen sind Auslieferungen die in aller Regel durch die Funktion „Ansicht duplizieren“ erzeugt wurden.  
Sie lassen sich aber ebenso komplett neu erstellen. Das wichtige Erkennungsmerkmal ist, dass eine solche Ansicht den gleichen Namen wie eine „Auslieferung“ hat.  
***Privat:***  
Eine private Ansicht ist weder eine Auslieferung noch eine privatisierte Auslieferung.  
***Ableitung:***  
Private Ansichten, also solche mit Ansichts-Status „Private Auslieferung“ oder „Privat“, können weiter abgeleitet werden.   
***Egal:***  
Einer der obigen Ansichts-Stati. |
| Einsatz | Beschreibung über den Einsatz bzw. Verwendungen der Ansichts-Definition. |
| Ansichts-Id | Bildet zusammen mit dem „Besitzer“ den Schlüsselbegriff der Archiv-Ansichten. |
| Ausliefer-Id | Im Falle von „privatisierten Auslieferungen“ stellt die Ausliefer-Id mit dem Ausliefer-Besitzer den Rückverweis auf die dazugehörige Auslieferung da. |
| Ausliefer-Besitzer | Im Falle von „privatisierten Auslieferungen“ stellt die Ausliefer-Id mit dem Ausliefer-Besitzer den Rückverweis auf die dazugehörige Auslieferung da. |
| Different | Natürlicherweise darf eine privatisierte Auslieferung Unterschiede zu der Auslieferung aufweisen, das ist das Wesen einer privatisierten Auslieferung.  
Anhand dieser Kennzeichnung kann nun aber rückwirkend entschieden werden, ob eine privatisierte Auslieferung sich von einer Auslieferung „unterscheidet“.  
Unterschiedlich sind zwei beteiligte Archiv-Ansichten, wenn sie unter Berücksichtigung der aktiven Richtlinien und der SPA-Einstellung „*Archiv-Richtlinien in privaten Ansichten berücksichtigen*“ (782) noch weitere Unterschiede aufweisen.  
Nach Einführung der Archiv-Richtlinien kann aber folgende Fragestellung interessant sein: Wann darf man denn eine „privatisierte Auslieferung“ gefahrlos löschen? Zur Beantwortung dieser Frage stellt eben diese Kennzeichnung eine interessante Hilfestellung zur Verfügung. |

<p class="siehe-auch">Siehe auch:</p>

- [Archiv-Ansichten: Technische Unterstützung](./archiv_ansichten_technische_unterstuetzung.md)
- [Archiv-Ansicht Standard-Auslieferung: Kunden, Vorgang](./archiv_ansicht_standard_auslieferung_kunden_vorgang.md)
