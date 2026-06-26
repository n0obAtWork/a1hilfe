# Einrichtung Buchstellen

<!-- source: https://amic.de/hilfe/einrichtungbuchstellen.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Buchstellen Firmenstamm

Direktspring **[BSFS]**

Bevor die XML-Daten erzeugt und versendet werden, bedarf es noch einiger Einstellungen im A.eins.

#### Buchstelle Firmenstamm

Bevor die Daten übermittelt werden können, bedarf es einiger Einstellungen im „Buchstellen Firmenstamm“ (BSFS). Dort kann die XML-Kopfstruktur und die Sendeeinstellungen hinterlegt werden.

### Allgemein

| Feldname | Beschreibung |
| --- | --- |
| Nummer | Identifizierende Nummer die der Buchstellenstamm haben soll. |
| Bezeichnung | Bezeichnung für den Buchstellenstamm |
| Nummernkreis | Hier muss ein Nummernkreis eingetragen werden, der **nur** von diesem Buchstellenstamm aus verwendet wird.<br>Der Grund dafür ist, dass beim Versenden der Daten eine fortlaufende Nummer gesendet wird. Anhand dieser kann erkannt werden, ob beim übertragen Daten verloren gegangen sind. |
| Verschlüsselungscode | Der Verschlüsselungscode wird zum verschlüsseln der Daten verwendet. Dieser Code muss bei AMIC bekannt sein, damit die Daten wieder entschlüsselt werden. |

### Sendeeinstellungen

| Feldname | Beschreibung |
| --- | --- |
| Ausgabepfad | In diesem Feld muss ein Pfad hinterlegt werden, in den die Dateien exportiert werden. Von dort aus müssen die exportierten Daten an die Buchstelle z.B. per FTP übermittelt werden. (siehe „[Export der Daten](./export_und_verarbeitung.md#bsfs_exportderdaten)“) |

### XML-Struktur

Im Bereich XML-Struktur lassen sich XML – spezifische Daten eintragen, welche später im XML – Dokument verwendet werden.

| Feldname | Beschreibung |
| --- | --- |
| Mandant | Name der Firma/Betriebs |
| Empfängername | Name der Buchstellenfirma |
| Nachrichtentyp | Typ der Nachricht (Standard „invoice“) |
| Testübertragung | Handelt es sich um eine Testübertragung |
| Externe Referenz | Hier kann eine externe Referenz eingetragen werden |
| Bez. AMIC Kundennr. | TAG – Name des XML – Tags, bei keinem Eintrag wird „*Buchstellennummer*“ verwendet |
| AMIC Kundennummer | Kundennummer der Firma (*wird von AMIC vergeben*) |

Die Kopfstruktur einer XML-Datei könnte wie folgt aussehen:

```xml
<Empfaengername>Buchstelle</Empfaengername>
<Erstellungsdatum>18.09.07</Erstellungsdatum>
<Erstellungszeit>07:30</Erstellungszeit>
<Nachrichtentyp>accounting</Nachrichtentyp>
<Testuebertragung>1</Testuebertragung>
<externe-Referenz>AMIC-13652314315</externe-Referenz>
<AnzahlBelege>1</AnzahlBelege>
<LaufendeNr>36</LaufendeNr>
```

#### Personenkonten

Die Erstellung von XML-Daten erfolgt nicht automatisch für jedes Personenkonto. Damit die Daten für ein Personenkonto erstellt werden, hinterlegen Sie in diesem den Buchstellenstamm.

Von der Kundenmaske kommt man auf die weitere Eingabemaske „Kennzeichen“ (F6), dort kann dann der Buchstellenstamm eingetragen werden.

Zu beachten ist hierbei, dass die XML-Daten nicht nachträglich für bereits gebuchte Belege erzeugt werden.
