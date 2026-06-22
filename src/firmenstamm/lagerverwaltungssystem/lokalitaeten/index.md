# Lokalitäten

<!-- source: https://amic.de/hilfe/_lokalitten.htm -->

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Lokalitäten

Direktsprung [LVSLK]

Eine Lokalität ist ein Ort, auf dem sich ein oder mehrere Ladeträger befinden können. Ein Lokalität kann ein Lager, ein Lagerplatz, eine Maschine oder eine Waage sein. Die Lokalität ist an ein Lager gebunden.

In der Auswahlliste werden die Daten der Lokalitäten angezeigt. Zusätzlich werden in der Auswahlliste die aktuelle Anzahl von Ladeträgern und eine Liste der Ladeträger auf der Lokalität angezeigt.

Die Bearbeitungsmaske ist in folgende Bereiche aufgeteilt.

Kopfdaten

| Feldname | Bedeutung |
| --- | --- |
| Lokalitätsnummer | Nummer der Lokalität |
| Bezeichnung | Bezeichnung der Lokalität |
| [Lokalitätstyp](./lokalitaetstyp.md) | Typ der Lokalität |
| Lager | Lagernummer der Lokalität |

Registerkarte Dimension

Die Dimensionsfelder können per Einrichterparameter vorbelegt werden. Sollte für eine/mehrere Dimensionen eine Vorbelegung vorhanden sein, so kann diese auf der Maske nicht geändert werden.

| Feldname | Bedeutung |
| --- | --- |
| [Dimension 1 / Wert](./lokalitaetsdimensionen.md) | Die Dimension wird entweder durch den EPA vorbelegt oder kann eingegeben werde.  
Als Wert für die Dimension kann ein Text eingegeben werden. |
| Dimension 2 / Wert | s. Dimension 1 |
| Dimension 3 / Wert | s. Dimension 1 |
| Dimension 4 / Wert | s. Dimension 1 |
| Dimension 5 / Wert | s. Dimension 1 |
| Wertigkeit | |
| Arbeitsregelnummer | |
| Koordinate X | |
| Koordinate Y | |
| Koordinate Z | |
| Volumen in l/kg | |
| Länge / Mengeneinheit | |
| Breite / Mengeneinheit | |
| Höhe / Mengeneinheit | |

Registerkarte Definition

Die Einstellungen auf dieser Registerkarte sind nur für Spezialanwendungen im Lagerverwaltungssystem und werden im Standard nicht ausgewertet.

| Feldnamen | Bedeutung |
| --- | --- |
| Drucker Nummer | In diesem Feld kann ein Drucker hinterlegt werden, welcher der Lokalität zugeordnet werden soll. |
| Tara Berechnung | In diesem Feld kann eingestellt werden, ob an der Lokalität eine Brutto oder Netto Wiegung durchgeführt werden soll. |
| Mehrfach LPP | Hier kann eingestellt werden, ob ein Ladeträger mehr als eine Position haben darf. |
| Rohwarenbeleg erzeugen | Ist die Lokalität eine Waage, so kann hier eingestellt werden, dass beim Abschluss des Waagenvorgangs ein Rohwarenbeleg erzeugt werden soll. |
| Automatische Boxenanlage | Hier kann eingestellt werden, ob an der Lokalität Ladeträger automatisch angelegt werden dürfen. |
| Waagenvorlage | Hier kann die Waagenvorlage hinterlegt werden, die beim Erzeugen des Rohwarenbeleges verwendet werden soll. |

Registerkarte Maschine

Die Einstellungen auf dieser Registerkarte sind nur für Spezialanwendungen im Lagerverwaltungssystem und werden im Standard nicht ausgewertet.

| Feldnamen | Bedeutung |
| --- | --- |
| Startcode | Wird nicht mehr ausgewertet |
| Endcode | Wird nicht mehr ausgewertet |
| Rezeptvariante | Wird an der Maschine ein Artikel, gereinigt oder Abgesackt, so kann hier die Rezeptvariante für den Produktartikel eingetragen werden. |
| Unterklasse | Wird an der Maschine eine Produktion erfasst, z.B. eine Reinigung, so kann hier die Unterklasse für die Produktion hinterlegt werden |
| Buchungsautomat | Buchungsautomatik für Maschinen im Lagerverwaltungssystem bezüglich der Produktionsbuchung |
| Maschine | Ist die Lokalität eine Maschine, so kann hier der Typ hinterlegt werden.  
0. Keine Maschine  
1. Reinigung  
2. Mischung  
3. Trocknung  
4. Absackung |

Wird der Maschine ein Typ zugeordnet, so können in der Datentabelle bislang zwei Artikel hinzugefügt werden. Es stehen zwei Typen zur Auswahl.

Es ist dabei zu beachten, dass die Artikel sich auf dem gleichen Lager befinden, in dem sich auch die Maschine befindet.

1. Wertloserabgang

2. Maschinelaufzeit

Als Wertlosenabgang kann hier ein Artikel hinterlegt werden, auf dem dann der Mengenunterschied bei einer Reinigung gebucht wird. Dies bedeutet, es soll eine Menge von 5000 kg von einem Artikel X gereinigt werden und nach dem Reinigen beträgt die Menge des Artikels nur noch 4700 kg. Dann wird die Differenz von 300 kg auf den Wertlosenabgang gebucht. Der Wertlose Artikel wird dann dem Produktionsbeleg als neue Komponente hinzugefügt.

Wird eine Maschinelaufzeit mit angegeben und die Mengeneinheit des Artikel ist Sekunden, Minuten oder Stunden, so wird die Laufzeit der Maschine vom Starten bis zum Ende der Reinigung als auch in dieser Mengeneinheit in dem Produktionsbeleg als Komponente angelegt.

<p class="siehe-auch">Siehe auch:</p>

- [Lokalitätstyp](./lokalitaetstyp.md)
- [Lokalitätsdimensionen](./lokalitaetsdimensionen.md)
- [Gruppe](./gruppe.md)
- [Gruppenindex](./gruppenindex.md)
