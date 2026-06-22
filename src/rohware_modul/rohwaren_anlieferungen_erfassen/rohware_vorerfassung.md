# Rohware-Vorerfassung

<!-- source: https://amic.de/hilfe/rohwarevorerfassung.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohware-Vorerfassung

Direktsprung **[RWV]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohware-Vorerfassung bearbeiten

Direktsprung **[RWVB]**

Das Modul zur Vorerfassung von Rohwareanlieferungen dient der schnellen Erfassung von Rohwarebeleg-Daten wie Wiegenummer, Kunde, Artikel, Menge sowie gegebenenfalls eine Partienummer, wenn zum Beispiel noch keine rohwarespezifischen Details bekannt sind.

Die Daten von Vorerfassungsbelegen können im Rohware-Bearbeitungs-Modul bei der Erfassung einer Rohwareanlieferung im Maskenfeld **Artikel** mit der Funktion ***Vorefassungsdaten übernehmen [F8]*** zur Übernahme in den Rohwarebeleg ausgewählt werden.

Intern wird der Vorerfassungsbeleg als Vorgang der Vorgangsklasse 1100 (Bestellanfrage) mit Unterklasse 9999 gespeichert. Durch Vorerfassungsbelege werden noch keine Bestandsveränderungen vorgenommen, dieses geschieht erst bei der Erfassung der Rohwarenanlieferung.

Die **Belegnummer** und das **Belegdatum** des Vorerfassungsbeleg werden bei der Datenübernahme auch als **Liefernummer** und **Lieferdatum** interpretiert. Dieses ist bezüglich der **Nummernkreis-Zuordnung** zu beachten.

UFLD-Felder, die für die Vorerfassungsdaten-Übernahme von Bedeutung sind:

| Feld | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 1510 | Lagernummer-Vorbesetzung | Wird bei Vorerfassungs-Übernahme übernommen |
| 110 | Wiegescheinnummer | Wird bei Vorerfassungs-Übernahme übernommen |
| 1771 | Bemerktext1 | Wird als **Partienummer** bei Vorerfassungs-Übernahme übernommen |
| 1772 | Bemerktext2 | Wird als **Bemerktext1** bei Vorerfassungs-Übernahme übernommen, ist aber nur bei Druck des Zielbelegs sichtbar |
| 1773 | Bemerktext3 | Wird als **Bemerktext3** bei Vorerfassungs-Übernahme übernommen, ist aber nur bei Druck des Zielbelegs sichtbar |
| 1774 | Bemerktext4 | Wird als **Bemerktext4** bei Vorerfassungs-Übernahme übernommen, ist aber nur bei Druck des Zielbelegs sichtbar |
| 1775 | Bemerktext5 | Wird als **Bemerktext5** bei Vorerfassungs-Übernahme übernommen, ist aber nur bei Druck des Zielbelegs sichtbar |
| 1034 | VersandartId | Wird bei Vorerfassungs-Übernahme übernommen |
| <br> | | |

Die Ware wird im **Positionsteil [F5]** des Belegs mit der Funktion **Artikel [F4**] durch Angabe von **Lagernummer, Artikelnummer, Lagerplatz, Menge und Mengeneinheit** erfasst.

Es kann pro Beleg nur eine Position erfasst werden.
