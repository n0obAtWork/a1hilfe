# TSE-Austausch Schritt-für-Schritt-Anleitung

<!-- source: https://amic.de/hilfe/_kassenSichVsfsB.htm -->

Ziel der Kassensicherungsverordnung ist, nachträgliche Manipulationen an Umsatzdaten herausfinden zu können. Die Überprüfung erfolgt in einem exportierbaren Journal, das durch das Finanzamt mit einer Prüfsoftware auf Veränderungen und Lücken geprüft werden kann.

Jede Kassenbuchung wird mit einer elektronischen Signatur versehen. Die Signatur funktioniert nach dem Blockchain Prinzip.

Bei der Generierung der Signatur werden nicht nur Bestandteile des aktuellen Verkaufsbelegs herangezogen, sondern auch die Signatur des vorherigen Belegs.

Weiterhin ist die externe, durch die Kassensoftware nicht manipulierbare, Sicherheitseinrichtung in die Signaturerstellung eingebunden.

Die Signatur wird verschlüsselt im Journal gespeichert.

Wenn Transaktionen im Journal manipuliert werden, ist die Kette der Signaturen nicht mehr konsistent. Es kann mit einer Prüfsoftware auf Knopfdruck herausgefunden werden, an welcher Stelle die Manipulation stattgefunden hat.

<p class="just-emphasize">**Voraussetzungen:**</p>

- Der neue TSE-Stick liegt vor.
- Die Lizenzdatei liegt vor.

<p class="just-emphasize">Tipp!</p>

Um zu jederzeit sicherzustellen, dass der Betrieb gewährleistet ist, empfiehlt es sich mehr als einen TSE-Stick vorrätig zu haben.

Wenn es zu Ausfällen kommen sollte, können Sie so schnell agieren. Dies schafft insbesondere beim Betrieb von mehreren Kassen Sicherheit.

<p class="just-emphasize">Hinweis!</p>

Voraussetzung für die Installation über das Netzwerk ist, dass der TSE-Stick von Ihrem IT-Betreuer im Netzwerk freigegeben wird sowie einem Laufwerk zugeordnet wird.

Der Tausch besteht aus den folgenden Schritten:

- Schritt 1 [Kassenabschluss durchführen](./tse_austausch_schritte_1_bis_4.md).
- Schritt 2 [Daten sichern per Export](./tse_austausch_schritte_1_bis_4.md#KassenSichV_sfs2B)
- Schritt 3 [Neue A.eins Lizenz (ahoi2.xml) einspielen](./tse_austausch_schritte_1_bis_4.md#KassenSichV_sfs3B)
- Schritt 4 [TSE-Sticks tauschen und Kassenarbeitsplatz A.eins® starten](./tse_austausch_schritte_1_bis_4.md#KassenSichV_sfs4B)
- Schritt 5 [TSE aktivieren](./tse_austausch_schritt_5_7.md).
- Schritt 6 [TSE der Kasse zuweisen](./tse_austausch_schritt_5_7.md#KassenSichV_sfs6B).
- Schritt 7 [Kasse eröffnen](./tse_austausch_schritt_5_7.md#KassenSichV_sfs7B).

<p class="siehe-auch">Siehe auch:</p>

- [TSE-Austausch Schritte 1 bis 4](./tse_austausch_schritte_1_bis_4.md)
- [TSE-Austausch Schritt 5-7](./tse_austausch_schritt_5_7.md)
