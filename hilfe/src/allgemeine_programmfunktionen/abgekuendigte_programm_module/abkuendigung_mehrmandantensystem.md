# Abkündigung: Mehrmandantensystem

<!-- source: https://amic.de/hilfe/_ab32045.htm -->

Das Mehrmandantensystem wurde vor vielen Jahren im System integriert. Zwischenzeitlich gibt es zuverlässige Methoden, mehrere Datenbanken auf einem gleichen Stammdatenstand zu halten. Inhaltlich entwickeln wir das Mehrmandantensystem nicht mehr weiter. Um auf die neuen Transfer umzustellen, müssen leider die eindeutigen Schlüssel in den Datenbanktabellen in allen Datenbanken identisch sein. Das ist vom Mehrmandantensystem nicht gewährleistet. Als Voraussetzung für die Integration der Replikationslösung müssen Sie einen ihrer bisherigen Mandanten als „Hauptmandanten“ definieren. Alle angeschlossen Mandanten müssen mit einem neuen Wirtschaftsjahr quasi bei 0 mit der Vorgangserfassung beginnen. Es sind Vorkehrungen zu treffen, dass gewisse Systemstammdaten unabhängig voneinander geführt werden können. Im Einzelfall muss hier ein Gespräch stattfinden und ein Lösungsansatz für die Umstellung der System erstellt werden.

#### Tags:

Abkündigung
