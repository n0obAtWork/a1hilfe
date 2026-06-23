# Kassenabschluss Hauptkasse, automatische Einreichung

<!-- source: https://amic.de/hilfe/kassenabschlusshauptkasseautom.htm -->

Betroffene SPA-Einstellungen:

„Aut. Buchung von Finanzvorgängen in Fibu“, wird üblicherweise nur abgeschaltet, wenn keine A.eins Fibu angeschlossen ist.

„Umbuchung der Zahlungsmittel auf Konten“= ja bewirkt, dass Zahlungen automatisch beim Kassenabschluss auf definierte Konten umgebucht werden. Barzahlungen werden immer vom Kassenkonto auf das angegebene Bargeldkonto umgebucht. Die Einstellung „nein“ bedeutet im Übrigen, dass die Zahlungsmittel vorgetragen werden und in den folgenden Sitzungen nur einzeln eingereicht werden können. Die automatische Umbuchung bezieht sich stets nur auf die Zahlungsmittel der jeweiligen Sitzung.

Folgende Fibu Konten sind beteiligt:

Kassenkonto Fibu lt. Kassenverwaltung

Kassenverrechnungskonto lt. Kassenverwaltung

Bargeldkonto lt. Kasseneinstellung

Scheckkonto lt. Kasseneinstellung

Gutscheinkonto lt. Kasseneinstellung

Kreditkartenkonto lt. Kasseneinstellung

Bankeinzugskonto lt. Kasseneinstellung

Differenzkonto Zählung lt. Kasseneinstellung

Stornokonto lt. Kasseneinstellung

Verrechnungskonten der Hausbanken (Einreichungen)

<p class="just-emphasize">Bisheriges Verfahren Kassenabschluss Unterkasse mit Abschöpfung an Hauptkasse:</p>

Bargeld:

1. Unterkasse erzeugt einen Einreichungsbeleg über die Höhe des Bargeldsolls laut Fortschreibung des Kassenbestands.

2. Bar-Soll der Unterkasse wird um Einreichung gemindert, Einreichungssumme erhöht.

3. Falls automatische Umbuchung von Zahlungsmitteln eingestellt ist, so werden bis zu 4 Fibu-Belege erstellt:

a. Bargeldeingang (Kasse an Bargeldkonto)

b. Bargeldausgang (Kasse an Bargeld)

c. Zähldifferenz (Differenzenkonto an Bargeld)

d. BV Storno Saldo (Kasse an Storno)

4. Falls keine automatische Umbuchung von Zahlungsmitteln vorgesehen ist, so erfolgt eine Umbuchung des Bargeldsaldo (Unterkasse an Hauptkasse).

5. Für die Hauptkasse wird ein Einreichungsvermerk eingerichtet. Bei der nächsten Gelegenheit (Bearbeitung einer Zahlung, Kasseneröffnung oder -abschluss) wird der Vermerk ausgewertet und maschinell ein Übernahmebeleg passend zur automatischen Einreichung erzeugt.

6. Das Kassensoll der Hauptkasse wird um die Geldübernahme erhöht.

7. Damit kann der Kassenabschluss der Unterkasse also auch erfolgen, wenn die zugehörige Hauptkasse nicht eröffnet ist. Die Übernahme würde dann in der nächsten Kassensitzung wirksam.

8. Der Übernahmebeleg ist mit keiner Fibu Buchung verbunden.

**Unbare Zahlungsmittel:**

1. Es wird kein Umbuchungsbeleg über zu entnehmende Zahlungsmittel erstellt.

2. Falls automatische Umbuchungen von Zahlungsmitteln vorgesehen sind,

a. so wird je Zahlungsart ein Fibubeleg über den einzureichenden Betrag gebildet (Kasse an Scheck/Gutschein/EC-Cash/Bankeinzug).

b. An der Kasse wird unter der Belegart „aut. Einreichung“ wahlweise (EPA) ein Sammelbeleg über die einzureichenden Zahlungsmittel oder Einzelbelege je Zahlungsmittel erzeugt.

c. Das Soll der jeweiligen Unterkasse je Zahlungsart wird gemindert.

3. Falls keine automatische Umbuchung von Zahlungsmitteln vorgesehen ist, so müssen entweder

a. die Zahlungsmittel manuell an die Hauptkasse abgeschöpft werden, oder

b. es wird per SPA dafür gesorgt, dass der Vortrag der Zahlungsmittel in der folgenden Sitzung auf 0 gestellt wird. Die Zahlungsmittel lösen sich allem Anschein nach in Luft auf.

4. In jedem Fall werden die Zahlungsmittel der Kasse entnommen.

<p class="just-emphasize">Bisheriges Verfahren Kassenabschluss Unterkasse ohne Abschöpfung oder Hauptkasse:</p>

Bargeld:

1. Es entsteht kein Kassenbeleg.

2. Falls automatische Umbuchung von Zahlungsmitteln eingestellt ist, so werden bis zu 4 Fibu-Belege erstellt:

a. Bargeldeingang (Kasse an Bargeldkonto), ohne evtl. Einreichungen von Unterkassen (dieser Betrag wurde bereits beim Abschluss der Unterkasse auf das zugehörige Bargeldkonto umgebucht).

b. Bargeldausgang (Kasse an Bargeld)

c. Zähldifferenz (Differenzenkonto an Bargeld)

d. BV Storno Saldo (Kasse an Storno)

3. Durch die automatische Umbuchung des Bargeldes mindert sich das Kassensoll nicht. Dieses erfolgt erst bei der Einreichung an die Bank. Fibu-seitig ist die Kasse aber bereits entlastet, das Kassensoll ist bereits auf dem Bargeldkonto.

4. Falls keine automatische Umbuchung von Zahlungsmitteln vorgesehen ist, so ändert sich nichts in der Kasse.

**Unbare Zahlungsmittel:**

1. Falls automatische Umbuchungen von Zahlungsmitteln vorgesehen sind,

a. so wird je Zahlungsart ein Fibubeleg über den einzureichenden Betrag gebildet (Kasse an Scheck/Gutschein/EC-Cash/Bankeinzug).

b. An der Kasse wird unter der Belegart „aut. Einreichung“ wahlweise (EPA) ein Sammelbeleg über die einzureichenden Zahlungsmittel oder Einzelbelege je Zahlungsmittel erzeugt.

c. Das Soll der jeweiligen Unterkasse je Zahlungsart wird gemindert.

2. Falls keine automatische Umbuchung von Zahlungsmitteln vorgesehen ist, so müssen die Zahlungsmittel manuell an die Bank werden.
