# Belegmappen

<!-- source: https://amic.de/hilfe/belegmappen.htm -->

Man kann Belege, die in der Belegerfassung erfasst werden, sofort sogenannten Belegmappen zuordnen. Um dieses Feature zu benutzen, muss in der [Belegerfassung](./index.md) der Einrichterparameter „Belegmappe abfragen“ gesetzt werden. Dieser Einrichterparameter hat folgende Ausprägungen:

- Nicht aktiv. Belegmappe wird nicht abgefragt bzw. angezeigt. Dies ist die Standardeinstellung.
- Belegmappe einmal zentral abfragen. Die Belegmappe wird nur einmal im Periodenabfragefenster abgefragt.
- Belegmappe in der Belegerfassung abfragen. Man hat in der Belegerfassungsmaske zusätzlich die Möglichkeit die Belegmappe zu ändern.

Eine Belegmappe bildet sich automatisch neu, sobald mindestens ein Beleg in der Mappe existiert. Eine **F8** Funktion für NEU gibt es bewusst nicht.

    
Die Zuordnung zu einer Belegmappe kann zusätzlich auf folgende Art und Weise geschehen:

- durch nachträgliches zuordnen der Belege in der Einzelbeleganzeige. Dort stehen die Funktionen „Belegmappe zuordnen“ und „Belegmappe entfernen“ zur Verfügung. Es kann hier der Beleg nur einer existierenden Mappe zugeordnet werden.
- In der Anwendung „Standardvorgänge Fibu“ über die Funktion „Belegmappe zuordnen“. Dort wird in einer Maske die Belegmappe abgefragt und alle ausgewählten Belege werden dieser Mappe zugeordnet. Dies kann eine neue Mappe oder eine bestehende Mappe sein. Vor der Zuordnung wird noch geprüft, ob die Mappe existiert und man kann hier ggf. noch abbrechen. Da Belege nur einer Mappe zugeordnet werden können, wird auch geprüft, ob Belege mit einer anderen Mappenzuordnung ausgewählt wurden. Man hat dann die Möglichkeit abzubrechen oder die Belege der neuen Mappe zuzuordnen.
- Bei den [periodischen Buchungen](./periodische_buchungen.md).
- in der Anwendung Währungsabgrenzung.

Ein Beleg kann nur einer Belegmappe zugeordnet werden. In den Anwendungen „Standardvorgänge FIBU“, „Primanota“ und „Belegmappen“ kann die Auswahl zusätzlich nach diesen Mappen erfolgen. Hier kann mit **F3** eine der angelegten Mappen ausgewählt werden.

#### Anwendung Belegmappe

Hauptmenü > Finanzbuchhaltung > Buchungen / Journal > Belegmappen

Direktsprung **[FIBM]**

In der Anwendung „Belegmappen“ existieren einige Bearbeitungsfunktionen:

- **Buchungen auflösen.**  
Dort werden automatisch Belege erstellt, die bis auf das Sollhabenkennzeichen, Belegnummer, Belegdatum und Periode dem Originalbeleg entsprechen. Dies dient dazu, Auflösung von Abgrenzungsbuchungen zu automatisieren. Der Status der Belege (neu, aufgelöst, gelöscht, storniert) wird in der Belegmappe festgehalten. Es können nur Auflösungen für Belege vorgenommen werden, die laut Belegmappe weder aufgelöst, gelöscht noch storniert wurden.  
    

- **Ansehen Beleg.**  
Es wird der Beleg, soweit er nicht den Status gelöscht hat, in der [Einzelbeleganzeige](../op_verwaltung/einzelbeleganzeige.md) angezeigt.  
    

- **Belegmappe entfernen.**  
Die Belegmappe wird gelöscht. Dabei werden die eigentlichen Belege nicht gelöscht, sondern nur der Verweis zur Belegmappe. Gelöschte Belegmappen können nicht wiederhergestellt werden.
