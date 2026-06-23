# Änderungen im Ablauf des Zahlungsverkehrs

<!-- source: https://amic.de/hilfe/nderungenimablaufdeszahlungsve.htm -->

Der grundsätzlich Ablauf (Zahlungsvorschläge erstellen, Zahlungsvorschläge freigeben, ...) bleibt bestehen wie er ist. Für SEPA werden nur die Kunden und Lieferanten herangezogen, deren Bank zu einem der 31 teilnehmenden Länder zählt. Dafür muss das Kennzeichen „SEPA-Teilnahmestaat“ im Staatstamm gesetzt sein und der Bank muss das korrekte Land zugewiesen sein. Zum SEPA-Lastschriftverfahren werden nur die Kunden herangezogen, bei denen ein Mandat hinterlegt ist. In der Auswahlliste „[Zahlungsvorschläge bearbeiten](../zahlungsvorschlaege_bearbeiten.md)“, gibt ein Hinweistext genauere Auskunft darüber, warum die Belege ggf. nicht zum SEPA-Verfahren herangezogen werden.

Zusätzlich kommt bei SEPA-Lastschriften hinzu, dass Erstlastschriften fünf Bankarbeitstage - sogenannte TARGET-Arbeitstage –, Folgelastschriften zwei Bankarbeitstage vor Fälligkeit und Firmenlastschriften einen Tag vor Fälligkeit bei der Bank des Debitors eintreffen müssen. Diese Tage werden beim Erstellen der Zahlungsvorschläge für SEPA-Lastschriften über „[Zahlvorschläge erstellen](../zahlungsvorschlaege_erstellen.md)“ (Direktsprung **[ZHVE]**) automatisch berücksichtigt. Die Anzahl der Tage lässt sich dort in den Einrichterparametern festlegen. Sie stehen standardmäßig auf 5, 2 bzw. 1 Tage. Sollen auch die Laufzeiten automatisch mitberücksichtigt werden, so können man hier die Tage auf z.B. 6 und 3 Tage geändert werden. Bei der Berechnung der Bankarbeitstage werden neben Samstagen und Sonntagen auch Neujahr, Karfreitag, Ostermontag, der 1.Mai sowie der 1. Und 2. Weihnachtsfeiertag nicht mitgezählt.  
    

Aus diesem Grund kann man das Ausführdatum für SEPA nicht mehr für die gesamte Datei setzen, wie man es vom DTA her kennt. Das Ändern des Ausführdatums erfolgt jetzt für jeden Zahlungsbeleg in der Anwendung „Zahlungen Bearbeiten“ mit der Funktion ***Formularänderung*** **F5.** Dort wird bei der Erfassung des Datums die Prüfung der Bankarbeitstage noch einmal vorgenommen und das eingegebene Datum ggf. automatisch angepasst. Das Ausführungsdatum kann bis zu 15 Tage in die Zukunft vordatiert werden. Diese Frist kann in „[Zahlvorschläge erstellen](../zahlungsvorschlaege_erstellen.md)“ (Direktsprung **[ZHVE]**) mit dem Einrichterparameter „SEPA Maximale Vordatierung des Ausführdatums(Kalendertage)“ geändert werden.

Man muss hier nicht für jeden Datensatz das Datum neu eingeben. Man **markiert** einfach alle Datensätze, für die man das Datum setzen möchte und wählt die Funktion ***Formularänderung*** **F5.** Dort muss man dann nur einmal das Datum eingeben und kann dann mit der Funktion ***Alle ändern*** **F5** das Datum für alle markierten Datensätzen übernehmen. Die Funktion „Alle ändern“ steht nur dann zur Verfügung, wenn man die Datensätze wirklich markiert hat.

Hinsichtlich der Dateinamenskonventionen gelten nachfolgende Festlegungen:

SCL_&lt;BIC>&lt;Zusatz>&lt;JJJJMMTT>&lt;Lfd.Nr>

Dabei ist:

- „SCL_“ eine 4-stellige alphanumerisch Konstante
- &lt;BIC> der 8-stellige alphanumerische Live-BankIdenitfierCode der Kommunikationsstelle des Teilnehmers
- &lt;Zusatz> 3-stelliger alphanumerischer Branch-Code. Ergibt zusammen mit der BIC den im Bankenstamm eingetragenen Swift-Code.
- JJJJMMTT 8-stelliges numerisches Erstellungsdatum der Datei (Geschäftstag)
- Lfd.Nr. 6 stellige laufende Datei-Nr.

Der Dateiname muss geschäftstäglich eindeutig sein. Beispiel eines Dateinamens:

SCL_ BHYPDEB1XXX20071210000001.XML.

Dieser Dateiname wird automatisch generiert! Der Dateiname kann jedoch überschrieben werden, wenn in dem Einrichterparameter „[Prozedur zur Anpassung des Dateinamens](../zahlungen_bearbeiten/dta.md#ProzedurzurAnpassungdesDateinamens):“ eine Prozedur hinterlegt ist.

Steht der Einrichterparameter „[Ausgabedatei im Explorer anzeigen:“](../zahlungen_bearbeiten/dta.md#AusgabeImExplorerAnzeigen) auf **Ja**, dann wird der Explorer in dem Verzeichnis geöffnet und die Datei ist automatisch markiert.
