# Druckerstamm

Hauptmenü > Administration > Drucker > Druckerstamm

oder Direktsprung **[DRST]**

In dieser Variante können A.eins-Drucker definiert werden.

<details>
  <summary>Felder des Druckerstamm:</summary>

  | Felder | Beschreibung |
  | :--- | :--- |
  | D | <p>Es kann einen Drucker mit „*“-Kennzeichnung geben. Es handelt sich dann um den als Default-Drucker bezeichneten Standard-A.eins-Drucker.</p><p>Dieser Drucker wird auch im Hauptmenü angezeigt.</p> |
  | Druckernummer | Laufende Druckernummer |
  | Bezeichnung | Druckerbezeichnung |
  | Queue / Datei | <p>Kann entweder die verfügbare LPTx Schnittstelle sein, welche im Capture zugeordnet wurde oder die Direktansprache für eine Queue in der Syntax `\\{Druckserver}\{Druckername}\`</p><p>Bei Windows-Druck die Bezeichnung der Druckerwarteschlange.</p><p>Im Windows-System zu finden unter **Systemsteuerung\Hardware und Sound\Geräte und Drucker** bzw. **Funktion „Druckerdialog“**</p> |
  | Druckertyp | <p>A.eins-Druckertyp</p><p>Einrichtung erfolgt über Direktsprung **[DRT]**</p> |
  | Kurzname | Alphanumerischer Code zur Identifizierung eines Druckers |
  | Windows (Druck) | <p>Ja: Windows Drucker</p><p>Nein: ASCII-Drucker</p> |
  | Senden An | <p>Ja: Drucker unterstützt direkt „Senden An“-Funktionalität des Archives.</p><p>Nein: Keine Wirkung.</p> |
  | Senden An-Funktion | Senden-An und Nulldrucker zur Laufzeit per Funktion bestätigen |
  | Archiv aus | <p>Ja: Auch wenn eine Archivierung durchgeführt werden würde findet sie nicht statt!</p><p>Nein: Keine Wirkung</p> |
  | Nulldrucker | <p>Ja: Der Drucker wird zur Druckaufbereitung herangezogen aber es erfolgt kein Abschluss der Druckerwarteschlange, d.h. in der Praxis „es erfolgt kein Druck“</p><p>Nein: Keine Wirkung</p> |
  | Bemerkung | Ergänzende Informationen. |

</details>

<details>
  <summary>Suchmöglichkeiten des Druckerstamm</summary>

  | Suchen | Beschreibung |
  | :--- | :--- |
  | Druckernummer ab | Bereich von Druckernummern |
  | Bezeichnung wie | Filtert nach Kriterium |
  | Bemerkung wie | Filtert nach Kriterium |
</details>

<details>
  <summary>Funktionen des Druckerstamm</summary>

  Es stehen neben Pflege-Funktionen folgende weitere Funktionen zur Verfügung:

  | Funktionen | Beschreibung |
  | :--- | :--- |
  | Standarddrucker setzen | Der ausgewählte Drucker wird A.eins-Standard-Drucker. |
  | Druckerstamm entladen | <p>Die Druckerkonfiguration wird entladen.</p><p>Das kann z.B. in der Absicht geschehen, eine erstellte Druckerkonfiguration in weitere Mandanten zu übernehmen.</p><p>Involvierte Tabellen: Druckerstamm, Druckertyp, Druckersteuerung.</p> |
  | Druckstamm laden | Lädt eine Druckerkonfiguration. |
  | Druckertypen | Pflege Druckertypen |
  | Druckerdialog | Öffnet den allgemeinen Windows-Dialog für die Druckeinstellungen |
</details>

<p class="siehe-auch">Siehe auch:</p>

- [Druckerstamm: Pfleger](pfleger.md)