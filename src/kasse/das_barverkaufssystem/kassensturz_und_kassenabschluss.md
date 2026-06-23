# Kassensturz und Kassenabschluss

<!-- source: https://amic.de/hilfe/_kass_sturz_abschluss.htm -->

Hauptmenü > Barvorgäng > Kasseneröffnung/Abschluss

<p class="just-emphasize">Relevante SPA-Einstellungen</p>

**Aut. Buchung Finanzvorgänge in Fibu**: Falls nein, so erfolgen überhaupt keine Übergaben an Fibu. Die Nein-Einstellung ist normalerweise nur gedacht für Anschluss Fremdfibu.

**Aut. Abschöpfung Unterkasse** an Hauptkasse, im Folgenden genannt SPA Abschöpfung: Unterkasse kann in diesem Fall Bargeldbestand im Zuge des Kassenabschluss an die Hauptkasse übergeben. Übergabe der Zahlungsmittel je nach SPA Zami

**Umbuchung Zahlungsmittel auf Konten**: Umbuchung Zahlungsmittel auf die in den Kasseneinstellungen vorgesehenen Konten. Umbuchung des Bargeldes auf das Bargeldkonto, falls eingerichtet.

**Unterkasse mit Abschöpfung ohne Zählung abschließen**: Im Standardfall (ja) wird an der Unterkasse keine Zählung durchgeführt, es wird also immer das Kassensoll an die Hauptkasse transferiert und nur dort werden Kassendifferenzen festgestellt. Per SPA Freischaltung kann man auch eine Zählung an der Unterkasse aktivieren.

<p class="just-emphasize">Kasseneinstellungen „Konten“:</p>

**Bargeld**: Auf dieses Konto wird der Bargeldbestand bei eingeschaltetem SPA Zami umgebucht. Auf das Bargeldkonto werden alle Barzahlungssummen ohne Berücksichtigung von BV-Stornobelegen der Sitzung unterschieden nach Bargeldeingang und Bargeldausgang umgebucht. Dadurch Fibu-seitig automatische Entlastung des Kassenkontos beim Kassenabschluss. Kassenseitig ist durch diese Umbuchung keine Entnahme des Bargeldes verbunden!

**Umbuchungskonten für unbare Zahlungsmittel** im Rahmen des Kassenabschluss bei Verwendung SPA Zami (Scheck, Gutschein, EC Cash, Bankeinzug).

**Differenzkonto**: Umbuchung einer evtl. Zähldifferenz beim Kassenabschluss.

**Stornokonto**: Separate Umbuchung von Stornobelegen saldiert je Sitzung auf dieses Konto. Nur im Zusammenhang mit Bargeldumbuchung zu behandeln: Wenn das Bargeldkonto nicht oder identisch dem Kassenkonto eingerichtet ist, erfolgt keine Stornoumbuchung. Wenn ein Bargeldkonto eingerichtet wurde, so sollte auch immer ein Stornokonto eingerichtet werden (im Zweifelsfall identisch dem Bargeldkonto).

Der Eintrag 0 bedeutet, dass keine Umbuchung erfolgt. In der Wirkung ist die 0 gleichbedeutend mit der Eingabe des Kassenkontos.

<p class="just-emphasize">Unterkassen und Hauptkassen</p>

Die Festlegung einer Unterkasse macht nur Sinn, wenn die Bedienung laut SPA Abschöpfung erfolgen soll. Ist der SPA nicht geschaltet, wird die Unterkasse im Rahmen des Kassenabschlusses wie eine Hauptkasse behandelt. Ansonsten zieht die Unterscheidung Unterkasse / Hauptkasse nur bei Einreichungen / Abschöpfungen, die dann je nach dem als Abschöpfung an die Hauptkasse oder als Einreichung bei der Bank ausgeführt werden.

<p class="just-emphasize">Automatische Einreichungsbelege</p>

Im Zuge des Kassenabschlusses werden passend zu den in der Fibu zu erzielenden Transaktionen automatische Einreichungen erzeugt. Dies gilt auch für evtl. auszuführende Umbuchungen auf das Bargeldkonto. Im Gegensatz zu den anderen Einreichungen sind diese Belege auf den Kassenbestand unwirksam. Es handelt sich um reine Platzhalter zur Fibu-Abstimmung.

Bei der automatischen Einreichungen von Kassenbelegen in der Kassenabschlussfunktion wird ein Kontrollprotokoll erzeugt. Dieses wird bei erfolgreichem Abschluss des Verfahrens wieder entfernt. Andernfalls kann dieses Protokoll zur Diagnose von Abbrüchen zur internen Auswertung herangezogen werden.Das Kontrollprotokoll „EinreichungProtokoll.txt“ ist im lokalen temporären Verzeichnis zu finden.

<p class="just-emphasize">Buchungen in Kassenbericht und Fibu</p>

Kontoabkürzungen:

UK=Kassenkonto der Unterkasse

HK=Kassenkonto der Hauptkasse

Diff=FibuKonto zur Umbuchung von Zähldifferenzen

Bar= FibuKonto zur Umbuchung des Bargeldes ohne Stornobelege

Sto= FibuKonto zur Umbuchung des Bargeldes aus Stornobelegen

ZK=FibuKonto zur Umbuchung unbarer Zahlungsmittel (je Zahlungsmittelart)

| UK/<br>HK | SPA<br>Zami | SPA<br>Abschöpfung | Kassensturz | Aut. Einreichungen | Kassenbericht<br>Vortrag |
| --- | --- | --- | --- | --- | --- |
| UK | Nein | Nein | UK->Diff | \-- keine -- | Bar= Barsoll<br>Zami=ZamiSoll |
| UK | Nein | Ja | UK->Diff | UK->HK(Barsoll)<br>UK->HK(Zami) | Bar=0<br>Zami=0 |
| UK | Ja | Nein | Bar->Diff | UK->Bar(Barzugang)<br>UK->Bar(Barabgang)<br>UK->Sto(Sto-Belege)<br>UK->ZK(Zami) | Bar= Barsoll<br>Zami=0 |
| UK | Ja | Ja | UK->Diff | UK->HK (Barsoll)<br>UK->ZK(Zami) | Bar=0<br>Zami=0 |
| HK | Nein | Nein | HK->Diff | \-- keine -- | Bar= Barsoll<br>Zami=ZamiSoll |
| HK | Nein | Ja | HK->Diff | \-- keine -- | Bar= Barsoll<br>Zami=ZamiSoll |
| HK | Ja | Nein | Bar->Diff | HK->Bar(Barzugang)<br>HK->Bar(Barabgang)<br>HK->Sto(Sto-Belege)<br>HK->ZK(Zami) | Bar= Barsoll<br>Zami=0 |
| HK | Ja | Ja | Bar->Diff | HK->Bar(Barzugang)<br>HK->Bar(Barabgang)<br>HK->Sto(Sto-Belege)<br>HK->ZK(Zami) | Bar= Barsoll<br>Zami=0 |

<p class="just-emphasize">Hinweise zum Verständnis:</p>

Hauptkasse, SPA Zami=an besagt dann auch, dass mit dem Kassenabschluss das Bargeld umgebucht werden soll. Allerdings wird da nicht einfach das aktuelle Kassensoll umgebucht, sondern die Bargeldzuflüsse und -abflüsse getrennt voneinander, wobei die Belegart „Kassensturz“ nicht enthalten ist. Dann wird so gebucht:

1. beim Kassensturz: gar nichts!

2. beim Abschluss Kassenzugänge als Kasse an Bargeld

3. beim Abschluss Kassenabgänge als Kasse an Bargeld

4. beim Abschluss Kassendifferenz als Bargeld an Differenzenkonto

Würde man schon beim Kassensturz Kasse an DiffKonto buchen, so müsste man beim Abschluss eine Ausgleichsbuchung DiffKonto an Bar vornehmen. Also eine Fibu Buchung gespart.

Ist der SPA Zami aus, so bucht nur der Kassensturz die Differenz als Kasse an DiffKonto.

Die Unterkasse wird der Kassenbestand saldiert abgeschöpft an die Hauptkasse. Deswegen muss die Unterkasse beim Kassensturz Kasse an DiffKonto und die Abschöpfung des aktuellen Solls (also schon um Diff vermindert) als UK und HK buchen.

<p class="just-emphasize">Empfehlenswerte Einstellung für möglichst einfache Handhabung und Abstimmung</p>

- Spa Zami=ja
- jede Kasse eigenes Kassenkonto und eigenes Diff.konto
- Bargeldkonto und Stornokonto =0 oder =Kassenkonto
- Zami Konten für die verwendeten Zami, müssen nicht notwendig je Kasse eingerichtet sein
- SPA Abschöpfung und Einrichtung Unterkasse beliebig

<p class="just-emphasize">Erfassungsparameter beim Kassenabschluss</p>

1. **Abschluss ohne Zählung möglich**: Bei Einstellung ja wird abgefragt, ob eine Zählung erfolgen soll. Bei nein immer Zählung, es sei denn durch SPA abgewählt.

2. **Einzelbuchung je Zahlungsmittel**: Bei Einstellung ja wird je Zahlungsmittel ein Einreichungsbeleg erzeugt. Das kann für Abstimmungszwecke ganz sinnvoll sein, erhöht aber das Belegvolumen. Bei Einstellung nein wird ein Sammelbeleg je Zahlungsart erstellt.
