# Bestandsbuchungen Vorgangsklasse 5055

<!-- source: https://amic.de/hilfe/bestandsbuchungenvorgangsklass.htm -->

Es ist sicherzustellen, dass nach dem Zeitpunkt der Erstellung dieses Beleges keine weiteren Zu- oder Abgangsbuchungen an dem aktuellen Tag erstellt werden.

Hinweis:

Voraussetzung für die Anwendung dieser Differenzkorrektur ist, dass alle offenen Belege zum Zeitpunkt der Inventur gebucht wurden. Eine spätere Erfassung oder Veränderung von Lieferungen vor dem Inventurzeitpunkt ist nicht erlaubt.

Ausnahmen bedeuten eine definierte Handlungsanweisung:

1. Ware wird zurückgeliefert – Beleg darf storniert / die Position darf gelöscht werden, wenn die Menge der Rücklieferung mit der Belegmenge identisch ist.

2. Ware wird teilweise zurückgeliefert – Die Position darf korrigiert werden, wenn die Menge der Rücklieferung mit der Änderung der Belegmenge identisch ist.

3. Es wurde eine falsche Menge im Beleg eingetragen – Die Position darf nur korrigiert werden, wenn zugleich die zusammenhängende Zählung in der Inventur korrigiert wurde. Alternativ muss die Ware nach Korrektur des Beleges neu gezählt werden.

4. Es muss ein Beleg nach der Zählung erfasst werden – Die zum erfassten Beleg gehörenden Artikel müssen in der Inventurzählung entsprechend korrigiert oder erneut gezählt werden.

Hinweis:

Es ist dringend empfohlen, dass Artikel nur mit Partie oder komplett ohne Partie geführt werden. Differenzmengenerfassungen für Artikel ohne Partieangabe werden zu Fehlinterpretationen des Systems führen, wenn es zu dem Artikel parallel Partiebestände gibt!

Nähere Informationen zum technischen Ablauf von Buchungen von Artikeln und Partien entnehmen Sie dem Abschnitt [techn. Informationen Buchungen](./techn_informationen_buchungen.md)

Mengenbuchung

Die erfasste Menge wird dem zum Zeitpunkt der Erfassung festgestellten Soll-Bestand gegenübergestellt.

Wertbuchung

In Abhängigkeit des [SPA 1072 – Bewertungsverhalten permanente Inventur](../../../firmenstamm/steuerparameter/mde_prozeduren_einzelhandel_spa_1059/permanente_inventur_bewertungsverhalten_spa_1072.md) wird die Inventur an dieser Stelle bewertet (0) oder die Bewertung wird erst am Jahresende (1) vorgenommen werden.

Die Ware wird hier mit dem aktuellen Bewertungspreis ausgebucht und mit dem eingegebenen (0) bzw. aktuellen Bewertungspreis (1) eingebucht.

FiBu-Buchung

Vor der Einstellung in die FiBu muss sichergestellt sein, dass ein gültiger Einbuchpreis angegeben wurde. Ein Preis 0 muss, wenn er gewollt ist als NULLPREIS gekennzeichnet sein.

Buchung nach Buchungsschluss

Es kann vorkommen, dass am Ende des Geschäftsjahres die Buchungsperiode zur Sicherheit abgeschlossen wird, um versehentliche weitere Buchungen in dieser Periode zu vermeiden. In diesem Fall würde auch der Bestandsbuchungsbeleg nicht vom LVS aus erzeugt werden können.

Um diese Belege zu erstellen, muss der Ersteller in die Liste der Buchungsadministratoren in der Periode eingetragen sein [[PERBA]](../../../firmenstamm/wirtschaftsjahre_und_perioden/buchungsadministratoren/index.md).

<p class="siehe-auch">Siehe auch:</p>

- [techn. Informationen Buchungen](./techn_informationen_buchungen.md)
- [techn. Informationen für Makro-Implementationen](./techn_informationen_fuer_makro_implementationen.md)
- [Automatisierung der Erstellung von Bestandsbuchungen](./automatisierung_der_erstellung_von_bestandsbuchungen.md)
