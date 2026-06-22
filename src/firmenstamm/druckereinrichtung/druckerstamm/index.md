# Druckerstamm

<!-- source: https://amic.de/hilfe/_druckerstammdrst.htm -->

Hauptmenü > Administration > Drucker > Druckerstamm

oder Direktsprung **[DRST]**

In dieser Variante können A.eins-Drucker definiert werden.

<details>
<summary>Felder des Druckerstamm:</summary>

| Felder | Beschreibung |
| --- | --- |
| D | Es kann einen Drucker mit „\*“-Kennzeichnung geben. Es handelt sich dann um den als Default-Drucker bezeichneten Standard-A.eins-Drucker.  
Dieser Drucker wird auch im Hauptmenü angezeigt. |
| Druckernummer | Laufende Druckernummer |
| Bezeichnung | Druckerbezeichnung |
| Queue / Datei | Kann entweder die verfügbare LPTx Schnittstelle sein, welche im Capture zugeordnet wurde oder die Direktansprache für eine Queue in der Syntax \\\\{Druckserver}\\{Druckername}\\  
   
Bei Windows-Druck die Bezeichnung der Druckerwarteschlange.  
Im Windows-System zu finden unter *Systemsteuerung\\Hardware und Sound\\Geräte und Drucker **bzw. Funktion „Druckerdialog“*** |
| Druckertyp | A.eins-Druckertyp  
Einrichtung erfolgt über Direktsprung **[DRT]** |
| Kurzname | Alphanumerischer Code zur Identifizierung eines Druckers |
| Windows (Druck) | Ja: Windows Drucker  
Nein: ASCII-Drucker |
| Senden An | Ja: Drucker unterstützt direkt „Senden An“-Funktionalität des Archives.  
Nein: Keine Wirkung. |
| Senden An-Funktion | Senden-An und Nulldrucker zur Laufzeit per Funktion bestätigen |
| Archiv aus | Ja: Auch wenn eine Archivierung durchgeführt werden würde findet sie nicht statt!  
Nein: Keine Wirkung |
| Nulldrucker | Ja: Der Drucker wird zur Druckaufbereitung herangezogen aber es erfolgt kein Abschluss der Druckerwarteschlange, d.h. in der Praxis „es erfolgt kein Druck“  
Nein: Keine Wirkung |
| Bemerkung | Ergänzende Informationen. |

</details>

<details>
<summary>Suchmöglichkeiten des Druckerstamm</summary>

| Suchen | Beschreibung |
| --- | --- |
| Druckernummer ab | Bereich von Druckernummern |
| Bezeichnung wie | Filtert nach Kriterium |
| Bemerkung wie | Filtert nach Kriterium |

</details>

<details>
<summary>Funktionen des Druckerstamm</summary>

Es stehen neben Pflege-Funktionen folgende weitere Funktionen zur Verfügung:

| Funktionen | Beschreibung |
| --- | --- |
| Standarddrucker setzen | Der ausgewählte Drucker wird A.eins-Standard-Drucker. |
| Druckerstamm entladen | Die Druckerkonfiguration wird entladen.  
Das kann z.B. in der Absicht geschehen, eine erstellte Druckerkonfiguration in weitere Mandanten zu übernehmen.  
Involvierte Tabellen: Druckerstamm, Druckertyp, Druckersteuerung. |
| Druckstamm laden | Lädt eine Druckerkonfiguration. |
| Druckertypen | Pflege Druckertypen |
| Druckerdialog | Öffnet den allgemeinen Windows-Dialog für die Druckeinstellungen |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Druckerstamm: Pfleger](./druckerstamm_pfleger.md)
