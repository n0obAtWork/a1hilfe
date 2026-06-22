# Globale Einstellungen

<!-- source: https://amic.de/hilfe/globaleeinstellungen.htm -->

Da die Umwandlung von Belegen (intern) eine komplizierte Angelegenheit ist, kommt man auch hier nicht ohne globale Einstellungen aus. Es handelt sich hierbei um die ehemaligen SPA Einstellungen: Trennen nach Geschäftsjahr und Trennen nach Perioden.

Die SPA Einstellungen behandeln Geschäftsjahr und Periode als eine Einheit, da eine getrennte Einstellung kaum Sinn macht. Ferner wird unterschieden nach Einzelumwandlung und Sammelumwandlung, da bei der Sammelumwandlung etwas unterschiedliche Bedingungen vorliegen als im Einzelumwandlungsmodus:

Für den SPA ‚Periode / Jahr bei Einzelumwandlung’ gibt es folgende Einstellung:

**0 = Original übernehmen**

Diese Einstellung haben wir lediglich aus Gründen der Kompatibilität beibehalten – wir empfehlen die Einstellung ausdrücklich **NICHT**. Bei der Umwandlung wird die Originalperiode beibehalten (bei den Umwandelarten Storno / Kopie / Gutschrift aus Rechnung ist das Beibehalten inhaltlich bedingt mit dem Häkchen ‚1 zu 1’ einstellbar).

**1 = neu laut Belegdatum**

Die Periode des Zielbeleges wird aus dem Belegdatum abgeleitet.

Für die Sammelumwandlung wird im dem SPA ‚Periode / Jahr bei Sammelumwandlung’ sowohl die Trennung der Belege als auch die Bestimmung der Periode geregelt.

Achtung:

Ein Sammelbeleg kann nur Bewegungen einer Periode enthalten (aber durchaus unterschiedliche Lieferdaten). Da die Periodenzugehörigkeit für diverse Abstimmungen herangezogen wird, sind mehrere Perioden innerhalb eines Beleges sehr komplex zu behandeln (die Periodenzuordnung wird nicht nur im Vorgang-Stamm, sondern auch in allen Warenbewegungen festgehalten).

***0 = Original nehmen, nach Quelle trennen***

Die Periodenzuordnung der Lieferscheine wird übernommen, aber es wird nach Perioden getrennt (siehe ACHTUNG weiter oben)

***1 = Neu, aber nach Quelle trennen***

Die Lieferscheine werden nach ihrer Periode getrennt, der Zielbeleg erhält aber eine neue Periode. (z.B.: Jeweils 5 Lieferscheine im Mai und Juni, Umwandlung im Juli, ergibt zwei Belege im Juli mit jeweils Periode Juli)

***2 = keine Trennung + neue Periode***

Die Perioden spielen keine Rolle bei der Zusammenfassung von Belegen, der Zielbeleg erhält eine neue Periode.

Eine weitere Einstellung bezüglich Lieferdaten und Periode ist in den Vorgangunterklassen (FRZ) angesiedelt. Das Lieferdatum spielt eine zentrale Rolle zur Ermittlung des echten (physischen) Buchbestands zu einem bestimmten Zeitpunkt (ab Lieferschein). In den fakturierten Umsätzen werden jedoch nur Bewegungen ab Stufe Rechnung summiert. Bei einem Geschäftsjahreswechsel als auch bei Inventurabschlüssen muss zwingend eine Gleichheit des Buchbestandes laut Lieferdatum und der fakturierten Summen hergestellt werden.

Hier gibt es unter Periodenbehandlung folgende Einstellung:

**0 = Jahresgrenzen / Inventurgrenzen**

Mit dieser Einstellung (von uns als Standard vorgesehen) wird sichergestellt, dass das Lieferdatum und die Periode niemals durch einen Inventurstichtag oder einen Geschäftsjahreswechsel getrennt werden.

Hinweis:

Da A.eins eine Verletzung der eben beschriebenen Bedingung durch Inventuren nur feststellen kann, wenn auch tatsächlich eine Inventur eröffnet wurde, empfehlen wir, Inventuren rechtzeitig anzulegen! So können die meisten Fehlzuordnungen schon während der Erfassung festgestellt werden – der Nachbearbeitungsaufwand bei der Inventureinspielung kann hierdurch deutlich reduziert werden.

***1 = Lieferdatum nie in geschlossene Periode***

Wenn diese Einstellung gewählt wird, lässt A.eins bei der Neuerzeugung von Belegen kein Lieferdatum zu, welches in geschlossene Perioden (auch Buchungsschluss!) verweist.

Da dieses Situation unterjährig nicht immer einfach zu umgehen ist, wurde diese Einstellung auch bewusst von der Vorgangsklasse / Unterklasse abhängig gemacht.

Hinweis:

Durch kurzfristiges Wiederaufheben des Buchungsschlusses kann man auch eine zu starke Einschränkung lockern!
