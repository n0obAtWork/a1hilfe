# Auswahllistenvariante Fibu Übertrag abgebrochen

<!-- source: https://amic.de/hilfe/_fibueb_abgebrochen.htm -->

Hauptmenü > Warenverkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag abgebrochen

Oder

Hauptmenü > Wareneinkauf > Abschluss > Fibu Übertrag aus Ware > Fibu Übertrag abgebrochen

Oder

Direktsprung **[FIB]**

In dieser Variante werden Vorgänge mit Fibu Status **‚i.B.‘**dargestellt, für die ein begonnener aber nicht beendeter Fibu-Übertrag-Auftrag im Datenstrom des Mandantenservers existiert und nicht mehr aktuell vom Mandantenserver bearbeitet wird.  
Derartige Konstellationen können neben Hardware-Problemen (z.B. Stromausfall) auch dadurch entstehen, dass der Mandantenserver eventgesteuert nach einer im Event eingestellten Zeit bei Ausbleiben einer Rückmeldung des Mandantenservers, die nach Beendigung jedes Mandantenserverauftrags erfolgt, in der Zeitspanne gestoppt und neu gestartet wird. Die Ursache ist häufig ein zu kurz eingestelltes Zeitintervall im Event, da insbesondere die Übertragung größerer Belege bei gleichzeitig starker Belastung des Datenbankservers möglicherweise in diesem Zeitintervall noch nicht beendet werden kann.

Die Funktion ‚*Fibu-Kennzeichen zurücksetzen‘* setzt, wenn möglich, den Fibu Status des Vorgangs auf **‚—‘** zurück, wenn der Beleg nicht in der Fibu gefunden wird. Ist der Beleg bereits in der Fibu, so wird der Fibu Status auf **‚Ja‘** gesetzt.

<details>
<summary>Felder der Auswahlliste</summary>

| Feld | Beschreibung |
| --- | --- |
| Belegnr. | Vorgangsnummer,  
bei Rohwaresammeldruck die Sammeldrucknummer |
| Belegdatum | Belegdatum des Vorgang,  
bei Rohwaresammeldruck die Sammeldrucknummer |
| Datum Buchungsauftrag | Datum der Erstellung des Übertragungsauftrags für den Mandantenserver |
| Kontonr. | Kunden-/Lieferantennummer = Kontonummer |
| Kunde | Bezeichnung des Kunden/Lieferanten |
| Typ | Vorgangsklassenkürzel des Belegs |
| Unterklasse | Nummer der Vorgangsunterklasse |
| Rohware | Rohware-Kennung:  
 --: kein Rohwarevorgang  
 RW-Beleg: Rohware-Einzelbeleg  
..RW-Sammeldruck: Rohware-Sammeldruck |
| RW-Stufe | Rohware-Abrechnungsstufe:  
Abschlag, F-Abschlag, Finale |
| Fib | ‚--': Beleg ist noch nicht übertragen  
i.B.: Übertragungsauftrag an Mandantenserver erteilt  
Ja: Beleg ist an Fibu übertragen  
NN: Beleg ist nicht übertragbar (z.B. Stornobeleg zu nicht übertragenem Originalbeleg) |
| Periode. | Warenwirtschaftsperiode des Vorgangs |
| Jahr | Warenwirtschaftsjahr des Vorgangs |
| PeriodeFibu | Fibuperiode des Vorgangs |
| JahrFibu | Fibujahr des Vorgangs |
| Erstbediener | Erster Bediener |
| Zustimmungsbediener | Zweiter Bediener beim Vieraugenprinzip |
| Erfasser | Erfasser/Ersteller des Vorgangs |

</details>

<details>
<summary>Bereichsauswahl</summary>

| Filter | Beschreibung |
| --- | --- |
| Belegnummer | Selektion der Belege mit Belegnummer (von/bis) |
| Datum | Selektion der Belege mit Belegdatum (von/bis) |
| Kunde | Selektion der Belege mit Kunden-/Lieferantennummer (von/bis) |
| Ersteller | Selektion der Belege mit dem Kürzel des Erfassers/Erstellers der Belege |

</details>

<details>
<summary>Funktionen der Auswahlliste</summary>

| Funktion | Beschreibung |
| --- | --- |
| Fibu-Kennzeichen zurücksetzen | Die Funktion setzt, wenn möglich, den Fibu Status des Vorgangs auf **‚—‘** (nicht übertragen) zurück, wenn der Beleg nicht in der Fibu gefunden wird. Ist der Beleg bereits in der Fibu, so wird der Fibu Status auf **‚Ja‘** (übertragen) gesetzt. In beiden Fällen wird der Übertragungsauftrag des Mandantenservers gelöscht. |

</details>
