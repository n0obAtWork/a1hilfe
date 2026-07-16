# Unerwartetes Verhalten beim Beleg-Mailversand

<!-- source: https://amic.de/hilfe/unerwartetesverhaltenbeimbeleg.htm -->

Die Rückmeldungen des Empfängermailservers über die Unzustellbarkeit einer Mail muss ebenso wie die Bearbeitung der Einträge im Fehlerprotokoll durch betriebsinterne Abläufe sichergestellt werden. Der Beleg gilt als gedruckt, wenn er an das Mailsystem abgegeben wurde.

- Ist die Mailadresse des Rechnungsempfängers nicht im Anschriftenstamm eingetragen, so wird ein Eintrag ins Fehlerprotokoll geschrieben. Das Druckkennzeichen wird dennoch gesetzt, das Mailkennzeichen wird nicht gesetzt.
- Ist die Mailadresse des Rechnungsempfängers ungültig oder die Mail nicht zustellbar, so gilt die Rechnung als versandt. Eine Verarbeitung einer Rückmeldung vom Mailserver findet an dieser Stelle nicht statt. Der Mailserver wird jedoch an die Absender-Mailadresse eine Unzustellbarkeitsmitteilung geben, der dann nachgegangen werden muss.
