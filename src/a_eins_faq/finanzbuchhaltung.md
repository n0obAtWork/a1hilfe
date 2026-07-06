# Finanzbuchhaltung

<!-- source: https://amic.de/hilfe/_faq_fibu.htm -->

<details>
<summary>Frage: Der Steuersatz hat sich geändert, was muss ich berücksichtigen/einstellen?</summary>

#### Antwort:

Bei einer Steuersatzänderung müssen mehrere Faktoren berücksichtigt werden. Dazu stehen 2 Anleitungen bereit:

- [Steuersatz (ändern)](../finanzbuchhaltung/umsatzsteuer/steuersaetze_einrichten/steuersatzaenderung.md)
- [Erlöskennziffer Kontozuordnung](../firmenstamm/erloeskennziffern/erloeskennziffer_kontozuordnung/erloeskennziffer_kontozuordnung_bei_steuersatzaenderung.md)

Information für alle A.eins-Nutzer, die ihre Daten über die Datevschnittstelle exportieren: [https://apps.datev.de/dnlexka/document/1018040](https://apps.datev.de/dnlexka/document/1018040)

Nach den Änderungen der Steuersätze muss der Mandantenserver neugestartet werden. Dafür mit dem Direktsprung **[MSI]** in die Maske des Mandantenservers. Hier die Funktion ***Stop Mandantenserver*** und danach die Funktion ***Normale Bearbeitung*** ausführen.

### Eine grobe Übersicht der wichtigsten Konten:

SKR04

| Steuerkonten | Beschreibung |
| --- | --- |
| 1403 | Abziehbare Vorsteuer 5% |
| 1405 | Abziehbare Vorsteuer 16% |
| 3803 | Umsatzsteuer 5 % |
| 3805 | Umsatzsteuer 16 % |

| Skontikonten | Beschreibung |
| --- | --- |
| 4732 | Gewährte Skonti 5% |
| 4735 | Gewährte Skonti 16% |
| 5732 | Erhaltene Skonti 5% |
| 5737 | Erhaltene Skonti 16% |
| 5747 | Erhaltene Skonti innnergem. Erwerb 5% |
| 5749 | Erhaltene Skonti innergem. Erwerbe 16% |

| Erlös-/Aufwandskonten | Beschreibung |
| --- | --- |
| 4300 | Erlöse 7/5% |
| 4400 | Erlöse 19/16 % |
| 5300 | Wareneingang 7/5% |
| 5400 | Wareneingang 19/16% |
| 5420 | Innergem. Erwerb 7/5% |
| 5425 | Innergem Erwerb 19/16% |

SKR03

| Steuerkonten | Beschreibung |
| --- | --- |
| 1568 | Abziehbare Vorsteuer 5% |
| 1575 | Abziehbare Vorsteuer 16% |
| 1589 | Vorsteuer EU 16% |
| 1773 | Umsatzsteuer 5 % |
| 1775 | Umsatzsteuer 16% |
| 1786 | Umsatzsteuer EU 16% |

| Skontikonten | Beschreibung |
| --- | --- |
| 8732 | Gewährte Skonti 5% |
| 8735 | Gewährte Skonti 16% |
| 3732 | Erhaltene Skonti 5% |
| 3737 | Erhaltene Skonti 16% |
| 3747 | Erhaltene Skonti innnergem. Erwerb 5% |
| 3749 | Erhaltene Skonti innergem. Erwerbe 16% |

| Erlös-/Aufwandskonten | Beschreibung |
| --- | --- |
| 8300 | Erlöse 7/5% |
| 8400 | Erlöse 19/16 % |
| 3300 | Wareneingang 7/5% |
| 3400 | Wareneingang 19/16% |
| 3420 | Innergem. Erwerb 7/5% |
| 3425 | Innergem Erwerb 19/16% |

</details>

<details>
<summary>Frage: Das Programm meldet „ungebuchte Belege“. Wo kann ich die finden?</summary>

#### Antwort:

Eine Möglichkeit der Anzeige ist im Menü unter „Finanzbuchhaltung“ – „Standardvorgänge Fibu“ [FISV] in der Variante „ungebuchte Belege“. Hier kann man über den F2-Filter auch auf die Vorjahre abgrenzen.

Hintergrund: An verschiedenen Stellen in A.eins wird beim Zusammenstellen der Daten getestet, ob noch relevante Belege in der Primanota stehen, so z. B. beim Jahreswechsel, bei der Umsatzsteuer-Voranmeldung, beim DATEV-Export oder verschiedenen Reports. I.d.R. geht es um Belege vor oder in den durch Filter eingegrenzten Perioden.

</details>

<details>
<summary>Frage: In der Saldenliste Debitoren steht ein anderer Saldo als auf dem Kontoblatt des Kontos Forderungen. Woran liegt das?</summary>

#### Antwort:

Die Saldenlisten und die Kontoinformation [KOI] zeigen die Salden in Bezug auf die Buchungsperioden an. An anderen Stellen ist das Buchungsdatum zusätzlich anzugeben wie bspw.bei der Erstellung des DATEV-Exports oder der Kontenblätter. In besonderen Fällen kann das Buchungsdatum nicht in der Buchungsperiode liegen, wodurch z. B. ein Beleg vom 15.07.17 mit Periode 6/17 in der Saldenliste bis 6/17 auftaucht, nicht aber auf dem Kontoblatt, wenn man hier als Eingrenzung neben der Periode auch das Datum auf 30.06.17 gesetzt hat.

</details>

<details>
<summary>Frage: In der OP-Liste historisch steht zum Stichtag ein anderer Saldo als auf dem Personenkonto in der OPV. Warum?</summary>

#### Antwort:

Bei der Abgrenzung von Offenen Posten zu einem Stichtag ist im Zusammenhang mit der Verrechnung von Belegen das Datum der Auszifferung maßgeblich.

In der OP-Liste wird das OP-Kennzeichen zum Stichtag ausgewertet. Wurde auf ein Personenkonto bspw. eine Rechnung mit Belegdatum 13.12.2016 gebucht und eine Zahlung statt mit 20.12.2017 versehentlich mit 20.12.2170 eingegeben und wurden diese Belege gegeneinander ausgeziffert, dann ist in der Offene Posten Verwaltung [OPV] kein Offener Posten zu sehen, in der Offene Posten Liste historisch [OPH] zum 31.12.2016 steht allerdings die Rechnung als Offener Beleg, denn die Zahlung (und Auszifferung) mit Datum 20.12.2170 ist ja nach dem Stichtag.

</details>

<details>
<summary>Frage: Eine Eingangsrechnung / Ausgangsrechnung geht nicht in die Fibu. Nach dem Fibu-Übertrag arbeitet das Programm erst und es steht „i. B.“ in der Spalte „Fib“, wenn ich die Auswahlliste neu aufrufe sind dort wieder Striche. Warum klappt die Übertragung nicht?</summary>

#### Antwort:

Der Grund für die fehlende Übernahme in die Fibu ist nachzulesen unter „Finanzbuchhaltung“ – „Protokoll Fibuübertrag“ [FIBF].

Hintergrund: Meistens ist die Erlöskennziffern-Zuordnung [EKZZ] unvollständig. Zum Beispiel hat man eine neue Erlöskennziffer [EKZ] angelegt und in einem neuen Artikel hinterlegt, aber noch keine Verknüpfung zu einem GuV-Konto unter EKZZ eingetragen. Ähnliches kann bei Anlage einer neuen Steuergruppe passieren oder auch einfach auf eine noch nie benötigte Kombination zutreffen. Alternativ sollte man sich fragen, ob die im Beleg stehenden Angaben z. B. auch zu den Steuersätzen korrekt sind.

</details>

<details>
<summary>Frage: Wie buche ich einen Eigenverbrauch?</summary>

#### Antwort:

Ware, die man vom Lager nimmt für den Eigenverbrauch, kann über eine Eingangsrechnung mit negativer Menge erfasst werden. Als Lieferant setzt man den eigenen Betrieb ein bzw. legt ihn unter Lieferanten zuvor an.

Dieser Lieferant sollte einer Steuergruppe „Eigenverbrauch“ (STS „normale Steuer“, Steuersatz 0) angehören. Die Steuergruppe ist hinterlegt in den Kunden-/Lieferantenstammdaten auf dem Reiter ‚Allgemein‘ und kann – abhängig von der Einrichtung – im Vorgang selbst gewechselt werden.

</details>

<details>
<summary>Frage: Wir haben einen Vorsteuerbetrag, der überwiesen wurde und zu 100% direkt auf Vorsteuer gebucht werden muss. In der Belegerfassung der Fibu gibt es aber eine Meldung: „Dieses Konto ist für die Direkterfassung gesperrt”</summary>

#### Antwort:

Als Steuerkonten definierte Konten sind für die Direkterfassung in A.eins gesperrt, damit Sie die zusätzlichen Auswertungsanforderungen erfüllen. [Sachkonten](../finanzbuchhaltung/stammdaten_der_fibu/sachkonten.md)

Muss Steuer tatsächlich direkt erfasst werden, z. B. Einfuhrumsatzsteuer oder im Zusammenhang mit einem Versicherungsfall, dann ist ein spezieller Steuerschlüssel für die Buchung zu verwenden.

Meist richtet man hierfür – sofern noch nicht vorhanden – den Steuerschlüssel 99 ein und hinterlegt in STS im Zusammenhang mit der Steuerklasse 102 die Steuer Formel: 100 % und den Steuersatz: 100

Als Gegenbuchung wird im Beleg dann nicht das unter STS angegebene Steuerkonto eingetragen, sondern ein Hilfskonto. Dieses ist nur aus technischen Gründen vorhanden. Der volle Betrag wird auf das in STS angegebene Steuerkonto gebucht.

Weitere Hilfe:

- [Steuerkonten bebuchen](../finanzbuchhaltung/umsatzsteuer/steuerkonten_bebuchen.md)

</details>
