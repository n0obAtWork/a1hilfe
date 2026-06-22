# Vorsteuerabzug

<!-- source: https://amic.de/hilfe/_Vorsteuerabzug.htm -->

Hauptmenü > Abschlussarbeiten > Umsatzsteuer > Abzugsfähige Vorsteuer

Ein Vorsteuerabzug ist dann möglich, wenn entweder

• eine Rechnung vorliegt und die Leistung empfangen worden ist oder

• eine Rechnung vorliegt und die Zahlung bereits erfolgt ist.

Für den Vorsteuerabzug ist also in jedem Fall der Eingang der Rechnung zwingend erforderlich, was wiederum bedeutet, dass die Vorsteuer erst der Periode zugeordnet werden darf, in der die Rechnung vorliegt.

Wird z.B. eine Rechnung am Jahresende erfasst, so kann es vorkommen, dass Rechnung erst im darauffolgenden Jahr eingeht.

Um diesen Sachverhalt gerecht zu werden, kann in A.eins für Eingangsrechnungen, Eingangsgutschriften sowie für Sonstige Belege ein Eingangsdatum erfasst werden. Die Erfassung eines Eingangsdatums ist nur dann notwendig, wenn die Periode des Eingangsdatums sich von der Periode des Beleges unterscheidet. Bei der Erfassung wird geprüft, ob das Eingangsdatum hinter dem Belegdatum liegt. Diese Prüfung lässt sich mit dem **SPA 1130** „Eingangsdatum muss hinter dem Belegdatum liegen“ abstellen.

Vor dem Erstellen einer Umsatzsteuervoranmeldung sind folgende Punkte zu beachten:

In der Anwendung „Abzugsfähige Vorsteuer“ werden alle Belege aufsummiert, bei denen sich die Periode des Eingangsdatums hinter der Periode des Belegs liegt. In dieser Anwendung sind folgende Varianten vorhanden:

• Variante „Nach Kennziffer Steuer“: Die Steuer wird nach der Kennziffer auf dem Umsatzsteueranmeldeformular gruppiert.

• Variante „Nach Klasse/Schlüssel/Gruppe“: Die Steuer wird nach den Steuersätzen gruppiert.

In beiden Varianten kann man sich mithilfe der Funktion ***Einzelbelege*** **F6** die entsprechenden Belege ansehen.

Für die in der Anwendung „Abzugsfähige Vorsteuer“ aufgeführten Beträge/Steuerpositionen müssen jeweils zwei Steuer-Direktbuchungen erstellt werden:

1. Eine Buchung in die Periode des Beleges, die die Steuer auf ein Konto "abweichende Periode" umbucht.

2. Eine Buchung, die die Steuer vom Konto „abweichende Periode“ in die Periode des Eingangsdatums überträgt.

*Hinweis: Man beachte, dass in der Anwendung „Abzugsfähige Vorsteuer“ auch Beträge/Steuerpositionen aufgelistet werden, für die bereits die entsprechenden Steuer-Direktbuchungen erstellt wurden.*

| | Periode/Jahr | Belegdatum | Eingangsdatum | Steuer | |
| --- | --- | --- | --- | --- | --- |
| Eingangsrechnung | 12/2020  
 | 29.12.2020 | 04.01.2020 | 1.200,00 S | |
| Steuer ausbuchen (SO-Beleg) | 12/2020  
 | 29.12.2020 | | 1.200,00 H | Steuersatz mit der Steuerformel 100% verwenden. |
| Steuer in die Periode des Rechnungseingangs buchen | 01/2021 | 04.01.2020 | | 1.200,00 S | Steuersatz mit der Steuerformel 100% verwenden. |

Die Steuersätze 100% müssen über dieselben Kennziffern verfügen wie die Steuersätze der betroffenen Eingangsrechnungen.
