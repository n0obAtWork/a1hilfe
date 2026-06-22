# Kasseneinstellungen

<!-- source: https://amic.de/hilfe/_kass_einr_einst.htm -->

Die Kasseneinstellungen werden in Vorlagen gegliedert, die wiederum in der Kassenverwaltung den einzelnen Kassenstandorten zugeordnet werden können.

| Aeins |
| --- |
| Lagernummer Markt | Lagernummer des Marktes bei Barverkauf |
| Abgleich mit Preisliste | Bei Preisänderung wird dieser mit dieser EK-Liste verprobt |

| Allgemein |
| --- |
| EC-Karte manuell erfassen | Nein=EC-Kartenleser ist aktiv, nur dort Eingabe möglich  
Ja=Keine Steuerung über EC Kartenleser. Siehe auch [Zahlungen mit Karte](./zahlungen_mit_karte.md) |
| BV-Abschluss erzwingen | |
| Tagesabschluss erzwingen | Eine Erfassung kann bei Einstellung „Ja“ nur erfolgen, wenn die Kassensitzung am gleichen Tag eröffnet wurde. Ggf. Muss die laufende Sitzung erst geschlossen und dann neu eröffnet werden. |

| Barverkauf  
Nicht in POS-Kasse |
| --- |
| Max Skontosatz | Maximaler Skontosatz in Prozent, der in der Kasse mit dieser Vorlage verwendet werden kann. |

| Displaytext |
| --- |
| 01:Displaytext Eröffnung | Text, der bei Eröffnung der Kasse angezeigt wird. |
| 02:Displaytext Abschluß | Text, der bei Abschluss der Kasse angezeigt wird. |
| 03:Displaytext Unterbrechung | Text, der bei Pause der Kasse angezeigt wird. |
| 04:Displaytext nach Vorgang | Text, der bei Abschluss der Abschluss eines Vorgangs angezeigt wird. |
| 05:Displaytext nach Parken | Text, der nach Entparken eines Vorgangs angezeigt wird. |
| 06:Displaytext Kunde zahlt noch | Text, der auf dem Display bei Verkauf Zame angezeigt wird. |
| 07:Displaytext Kasse zahlt noch | Text, der auf dem Display bei Einkauf/Retoure angezeigt wird. |
| 08:Displaytext Kasse zahlt zurück | Text, der auf dem Display bei Rückgeld bei Verkauf/Zame angezeigt wird. |
| 09:Displaytext Kasse zahlt zurück | Text, der auf dem Display bei Rückgeld beim Sortenwechsel angezeigt wird. |
| 10:Displaytext Passend gezahlt | Text, der auf dem Display bei Passend gezahlt angezeigt wird. |
| 11:Displaytext Einzahlungssumme | Text, der auf dem Display bei Einzahlung angezeigt wird. |
| 12:Displaytext Einzahlungsstorno | Text, der auf dem Display bei Einzahlungsstorno angezeigt wird. |
| 13:Displaytext Auszahlungsstorno | Text, der auf dem Display bei Auszahlungsstorno angezeigt wird. |
| 14:Displaytext Auszahlungssumme | Text, der auf dem Display bei Auszahlung angezeigt wird. |
| 15:Displaytext Sortenwechsel | Text, der auf dem Display bei Sortenwechsel angezeigt wird. |
| 16:Displaytext Entnahmesumme | Text, der auf dem Display bei Entnahme angezeigt wird. |
| 17:Displaytext Einreichungssumme | Text, der auf dem Display bei Einreichung angezeigt wird. |
| 18:Displaytext Übergabesumme | Text, der auf dem Display bei Geldübergabe angezeigt wird. |
| 19:Displaytext Übernahmesumme | Text, der auf dem Display bei Geldübernahme angezeigt wird. |
| 20:Displaytext Zählungssumme | Text, der auf dem Display bei Zählung angezeigt wird. |
| 21:Displaytext Kunde zahlt | Text, der in der Marktkasse bei Einleitung der Zahlung angezeigt wird. |

| Formulare |
| --- |
| Formularnummer Scheckdruck | Wird nicht mehr verwendet! Einstellung wird in der Kassenverwaltung vorgenommen. |
| Formularnummer EC-Lastschrift | Wird nicht mehr verwendet! Einstellung wird in der Kassenverwaltung vorgenommen. |

| Geld |
| --- |
| Rolleninhalt 1 | Anzahl der Münzen in Rollen der Währungseinheit 1 (siehe Währung) |
| Rolleninhalt 2 | Anzahl der Münzen in Rollen der Währungseinheit 2 (siehe Währung) |
| Rolleninhalt 3 | Anzahl der Münzen in Rollen der Währungseinheit 3 (siehe Währung) |
| Rolleninhalt 4 | Anzahl der Münzen in Rollen der Währungseinheit 4 (siehe Währung) |
| Rolleninhalt 5 | Anzahl der Münzen in Rollen der Währungseinheit 5 (siehe Währung) |
| Rolleninhalt 6 | Anzahl der Münzen in Rollen der Währungseinheit 6 (siehe Währung) |
| Rolleninhalt 7 | Anzahl der Münzen in Rollen der Währungseinheit 7 (siehe Währung) |
| Rolleninhalt 8 | Anzahl der Münzen in Rollen der Währungseinheit 8 (siehe Währung) |

| Kasse |
| --- |
| Kassenwährung | Nummer der Kassenwährung in der die Kasse geführt wird (in der Regel meist die Buchwährung). In dieser Währung wird das Wechselgeld ausgegeben. |

| Konten |
| --- |
| Bargeldkonto | FiBu-Konto für die Buchung des Bargeldbestands |
| Scheckkonto | FiBu-Konto für die Buchung von Schecks (kaum noch verwendet) |
| Gutscheinkonto | FiBu-Konto für die Buchung von Gutscheinen |
| Kreditkartenkonto | FiBu-Konto für die Buchung von Kreditkartenzahlungen |
| Bankeinzugkonto | FiBu-Konto für die Buchung von Bankeinzügen |
| Differenzkonto Zählung | FiBu-Konto für die Buchung von Zählungsdifferenzen |
| Stornokonto | FiBu-Konto für die Buchung von Stornierungen |

| Kunden |
| --- |
| Standard-Kundennummer | Voreinstellung für den Kunden beim Barverkauf |
| Standard-Lieferantenkontonummer | Voreinstellung für den Kunden beim Bareinkauf |
| Kundennummer POS-Kasse | Voreinstellung für den Kunden beim Barverkauf in der POS-Kasse |

| Währung |
| --- |
| Waehrungseinheit 1 | Wert der kleinsten Währungsstückelung z.B. 0.01€ |
| Waehrungseinheit 2 | Wert der zweitkleinsten Währungsstückelung z.B. 0.02€ |
| Waehrungseinheit 3 | Ebendies z.B. 0.05€ |
| Waehrungseinheit 4 | Ebendies z.B. 0.10€ |
| Waehrungseinheit 5 | Ebendies z.B. 0.20€ |
| Waehrungseinheit 6 | Ebendies z.B. 0.50€ |
| Waehrungseinheit 7 | Ebendies z.B. 1.00€ |
| Waehrungseinheit 8 | Ebendies z.B. 2.00€ |
| Waehrungseinheit 9 | Ebendies z.B. 5.00€ |
| Waehrungseinheit 10 | Ebendies z.B.10.00€ |
| Waehrungseinheit 11 | Ebendies z.B.20.00€ |
| Waehrungseinheit 12 | Ebendies z.B.50.00€ |
| Waehrungseinheit 13 | Ebendies z.B.100.00€ |
| Waehrungseinheit 14 | Ebendies z.B.200.00€ |
| Waehrungseinheit 15 | Ebendies z.B.500.00€ |

| Zahlungsmeldung |
| --- |
| Wie oft soll die Zahlungsmeldung gedruckt werden | Anzahl der Drucke einer Zahlungsmeldung |
