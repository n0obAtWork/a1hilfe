# Outlook Terminplanung

<!-- source: https://amic.de/hilfe/_tammo_OutlookTerminplanung.htm -->

Die im Outlook gepflegten Terminelemente können mit dem Tammo Modul bequem mit dem Auftragsmodul (ggf. auch anderen Vorgängen) verbunden werden. Es sind hierbei 4 Einrichtungselemente zu beachten:

Einrichtung eines e-Mail Accounts der immer als Teilnehmer in dem Termin eingetragen wird, damit Tammo dem entsprechenden Terminplanungseintrag zugeordnet ist

Im Tammo Einrichtungsbereich muss angegeben werden, welche Vorgangsklasse / Unterklasse für diese Funktionalität genutzt werden soll.

Im Anschriftenpfleger muss hinterlegt sein, dass der eintragende Terminplaner auch die Erlaubnis hat, das Tammo Modul nutzen zu dürfen.

Im Tammo Einrichtungsbereich ist festzulegen, welche Artikel / Lager Kombination im Vorgang die Zeitspanne abrechnen soll.

Im Tammo Einrichtungsbereich ist festzulegen, ob die Berechnung nach Stunden oder Minuten erfolgen soll.

Wird nun ein Terminelement im Outlook erfasst, und wird diesem der Tammo – Teilnehmer zugeordnet (am besten [TammoTermin@domain.de](mailto:TammoTermin@domain.de) ), so wird auf Basis der im Termin eingetragenen Werte ein Vorgang erzeugt. Dieser Vorgang enthält in den Zeitstempeln der Warenposition die Terminelement (Startzeit/Endzeit und Dauer) und es wird eine Zuordnung des Termins zu dem Initiator dieses Termins über die Vertretergruppenzuordnung hergestellt (Es ist dabei darauf zu achten, dass die Vertreter in Ihren Adresszuordnungen korrekt die passenden E-Mail Einträge enthalten).

Die Kundenzuordnung wird auf verschiedene Weisen abgearbeitet:

Ist im Termin ein Terminteilnehmer eingetragen, der eine Kundenadresse repräsentiert, so wird dieser Kundeneintrag als Kunden des Vorgangs zugeordnet.

Ist in der Betreffzeile die E-Mail-Adresse des Terminteilnehmers der eine Kundenadresse repräsentiert eingetragen, so wird dieser Kundeneintrag als Kunden des Vorgangs zugeordnet (wenn nicht sofort der Kunde über die Terminplanung informiert werden soll).

Ist in der Betreffzeile ein Vermerk der Form K:nr oder K:&lt;Name>,&lt;Ort> eingetragen, so wird dieser Schlüssel analysiert und es wird eine entsprechende Kundenzuordnung vorgenommen (Leertaste zwischen Kundennummer und K: ist nicht erlaubt) .

Ist kein Partner in der Mail enthalten und ist auch Kein Kunde in der Betreffzeile zugeordnet, so wird der Kunde zur Verarbeitung herangezogen, der der Tammo-Email zugeordnet ist.

Im Fall-Back Fall (nichts der obigen Argumente trifft zu) wird der erste gültige Kunde genommen.

Es gelten beim Vorgang folgende Regeln:

Terminüberschneidungen führen zum Storno des Vorgängers (FiFo), bis auf die Ausnahme, dass der Vorgang weiterverarbeitet ist.

Sind mehrere Vertretergruppen am Termin beteiligt (weitere Teilnehmer), so werden für jeden Vertreter einzelne Vorgänge angelegt.

Zeitänderungen im Termin führen zur Stornierung des Auftrages und Neuanlage eines Auftrages mit den neuen Terminen.

Als Rückmeldung zu dem Termin wird eine E-Mail mit einem PDF-Anhang gesendet, der:

das Kundenstammblatt enthält

eine Terminübersicht dieser Vertretergruppe darstellt.

Bei Erreichen des Termins wird eine Stunde vor Beginn (ggf. auch im Intervall der einen Stunde, wenn der Termin kurzfristig geplant wird)

Eine E-Mail-Benachrichtigung mit einem Excel Anhang versendet.

Dieser Excel-Anhang kann dann also Protokoll des Termins genutzt werden, um die vorgenommenen Maßnahmen festzuhalten.

Wird dann diese Excel Datei direkt zurückgesandt, werden alle eingetragenen Werte in den Auftrag übernommen, und der Auftrag wird (bei gesetztem Kennzeichen) direkt in eine Rechnung umgewandelt.
