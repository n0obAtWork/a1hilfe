# Setup Erstinbetriebnahme Schritt-für-Schritt-Anleitung

<!-- source: https://amic.de/hilfe/_kassenSichVsfs.htm -->

Ziel der Kassensicherungsverordnung ist, nachträgliche Manipulationen an Umsatzdaten herausfinden zu können. Die Überprüfung erfolgt in einem exportierbaren Journal, das durch das Finanzamt mit einer Prüfsoftware auf Veränderungen und Lücken geprüft werden kann.

Jede Kassenbuchung wird mit einer elektronischen Signatur versehen. Die Signatur funktioniert nach dem Blockchain Prinzip. Bei der Generierung der Signatur werden nicht nur Bestandteile des aktuellen Verkaufsbelegs herangezogen, sondern auch die Signatur des vorherigen Belegs. Weiterhin ist die externe, durch die Kassensoftware nicht manipulierbare, Sicherheitseinrichtung in die Signaturerstellung eingebunden. Die Signatur wird verschlüsselt im Journal gespeichert.

Wenn Transaktionen im Journal manipuliert werden, ist die Kette der Signaturen nicht mehr konsistent. Es kann mit einer Prüfsoftware auf Knopfdruck herausgefunden werden, an welcher Stelle die Manipulation stattgefunden hat.

<p class="siehe-auch">Siehe auch:</p>

- [TSE-Setup Schritt 1 Setup](./tse_setup_schritt_1_setup.md)
- [TSE-Setup Schritt 2 Konfiguration](./tse_setup_schritt_2_konfiguration.md)
- [TSE-Setup Schritt 3 Abschluss](./tse_setup_schritt_3_abschluss.md)
