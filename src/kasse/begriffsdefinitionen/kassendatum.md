# Kassendatum

<!-- source: https://amic.de/hilfe/kassendatum.htm -->

Standardmäßig ist das A.eins Tagesdatum das Systemdatum. Bekanntlich kann man das aktuelle Tagesdatum mit Direktsprung **[DAT]** verstellen. Dann fungiert das verstellte Datum als das vorgeschlagene Belegdatum etwa für die Vorgangserfassung. Auf diese Weise kann man Belege zu einem anderen Datum erleichtert nach erfassen, ohne jeweils an die Anpassung des Belegdatums beachten zu müssen. Die eingestellte Datumsänderung gilt höchstens bis zum Abmelden aus A.eins. Das Systemdatum wird dadurch nicht beeinflusst.

<p class="just-emphasize">Von der Änderung des Tagesdatums auf Kassenarbeitsplätzen ist dringend ab zu raten!</p>

Grund: Das Kassenbuch und die Kassenberichte werden in strikter Abhängigkeit vom Systemdatum geführt. Hier wird nicht nur ein Datum, sondern ein Stempel aus Datum und Uhrzeit geführt, der die exakte Chronologie der Kassenbelege und ihre Einordnung in Kassensitzungen fest hält.

Würde man nun beispielsweise aber trotzdem am 7. April das Datum auf den 5. April zurück stellen, so würde ein Barverkaufsvorgang warenseitig mit dem 5. April, im Kassenbuch aber mit dem 7.April geführt werden. Würde man überdies nun auch noch einen Fibu Übertrag starten, so würden normalerweise ein Fibu Beleg per 7.April und ein Zahlungsbeleg per 7. April entstehen. Man sieht also, dass man sich die Abstimmung der Kasse sowohl mit der Ware wie auch mit der Fibu unnötig erschweren kann.

In der Kasse erfasste Finanzbelege werden meist sofort und automatisch an die Fibu übertragen (SPA Einstellung). Hier gilt jetzt: Belegdatum Kasse = Übertragsdatum Fibu. An dieser Stelle wurde die Logik geändert: bisher wurde für den Übertrag an die Fibu das eingestellte Datum gewählt.

<p class="just-emphasize">Unbedingt zu vermeiden ist die Umstellung des Systemdatums auf Kassenarbeitsplätzen!</p>

Das kann neben Problemen bei Abstimmbarkeit auch Störungen nach sich ziehen, wenn dadurch etwa der Zeitpunkt des Abschlusses einer Sitzung vor ihrer Eröffnung liegen würde.

Die Änderung des Tagesdatums kann durch die Einstellung des [Steuerparameters 838](../../firmenstamm/steuerparameter/kasse_barverkauf/aktuelles_tagesdatum_aendern_erlaubt_dat_spa_838.md) generell verboten werden.
