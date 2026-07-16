# Die Tammo-App

<!-- source: https://amic.de/hilfe/dietammoapp.htm -->

Wird an die E-Mail Adresse Projekt@&lt;Domäne>.de eine E-Mail gesendet, die in der Betreffzeile ein k:&lt;Kundennummer> oder k:&lt;Kundenbezeichnung> enthält, dann wird mit dem Body der Mail ein neues Projekt für diesen Kunden eröffnet, selbstverständlich muss dazu der Tammo Treiber auf Ihrem A.eins System installiert sein. Als Rückantwort wird die Projektnummer zurückgegeben, mit einem Informationsblatt zum Kunden und einem Übersichtsblatt zu den offenen Projekten dieses Kunden wie auch mit den offenen Projekten des Mitarbeiters.

Wird einfach nur eine Mail abgeschickt ohne Betreff an die Adresse Projekt@&lt;Domäne>.de, dann wird die dem Mitarbeiter zugeordnete Projektliste zurückgegeben.

Enthält die Betreffzeile ein p:&lt;Projektnummer> und wird diese Mail, ggf. auch mit Anhängen, direkt dem vorhandenen Projekt zugeordnet, ist die Projektnummer nicht existent, so wird dem Standardkunden der Projektverwaltung ein neues Projekt zugeordnet, wenn die sendende Mailadresse nicht schon einem Kunden zugeordnet ist.
