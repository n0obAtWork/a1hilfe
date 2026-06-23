# Folgeartikel verwalten

<!-- source: https://amic.de/hilfe/_folgeartikelverwalte.htm -->

Mit der Folgeartikelverwaltung lassen sich verschiedene inhaltliche Fragestellungen lösen und über die Steuerparametergruppe „[Folgeartikel](../../../firmenstamm/steuerparameter/folgeartikel/index.md)“ individualisieren.

Die Möglichkeiten sollen an zwei Beispielen beschrieben werden.

- Verkauf zusammenhängender Artikel
- [Leergutverwaltung](./leergutverwaltung/index.md)

<p class="just-emphasize">Verkauf zusammenhängender Artikel</p>

Häufig hängen an einem Produkt Folgeprodukte, die mit verkauft oder angeboten werden. (z.B. Regenrinne mit zwei Halterungen und Schrauben)

Für diese Fragestellung werden an den Hauptartikel Folgeartikel gehängt und über den Folgetyp festgelegt, ob alle Artikel der Liste, mehrere oder nur einer zulässig ist.

| Feld | Beschreibung |
| --- | --- |
| Alle | Alle Folgeartikel werden angeboten |
| Einen | Nur ein Folgeartikel ist zulässig |
| Mehrere | Mehrere Folgeartikel sind zulässig |

Hierfür stehen folgende Felder auf der Folgeartikelmaske zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Folgezähler | Laufende Nummer |
| Artikelstammnummer | Artikelstamm des Folgeartikels |
| Gültig ab | Folgeartikel werden nur dann gezogen, wenn das Abgrenzdatum (in der Regel das Lieferdatum) in der Vorgangserfassung innerhalb des durch *gültig ab* und *gültig bis* bestimmten Zeitraums liegt. |
| Gültig bis | Folgeartikel werden nur dann gezogen, wenn das Abgrenzdatum (in der Regel das Lieferdatum) in der Vorgangserfassung innerhalb des durch *gültig ab* und *gültig bis* bestimmten Zeitraums liegt. |
| EK / VK | Hier kann festgelegt werden, ob der Folgeartikel in den Bereichen Einkauf / Verkauf / Lagerumbuchung heranzuziehen ist. |
| Mengenkennzeichen | Das Mengenkennzeichen bestimmt, ob sich anschließend die Menge des Folgeartikels mittels eines Faktors oder Divisors aus der Menge des Hauptartikels ergibt oder aber der Wert hier festgelegt wird.<br>*Beispiel:*<br>*Faktor, Wert = 2* ⇨ *Menge* *des Hauptartikels = 1 so ist die Menge des Folgeartikels = 2*<br>*Divisor, Wert = 2* ⇨ *Menge des Hauptartikels = 1 so ist die Menge des Folgeartikels = 0,5*<br>*Menge, Wert = 4* ⇨ *Unabhängig vom Hauptartikel: Die Menge des Folgeartikels = 4* |
| Mengenbezug | |
| Wert | Wert oder Faktor / Divisor des Folgeartikels |
| Sperre | Hier kann z.B. vorübergehend die Verwendung des Folgeartikels gesperrt werden. |
| Wertartikel | Die Mengenbuchung des Folgeartikels kann durch die Kennzeichnung als Wertartikels an dieser Stelle unterdrückt werden. |
| Hauptartikelbezug | Die Verwendung des Folgeartikels kann hinsichtlich der im Hauptartikel verwendeten Mengeneinheit bestimmt werden:<br>*Immer* ⇨ Keine Einschränkung hinsichtlich der Hauptartikelmengeneinheit<br>*Nur bei Gebinde* ⇨ Der Folgeartikel wird nur gezogen, wenn es sich bei der Mengeneinheit des Hauptartikels um ein Gebinde handelt<br>*Nicht bei Gebinde* ⇨ Der Folgeartikel wird nur gezogen, wenn es sich bei der Mengeneinheit des Hauptartikels nicht um ein Gebinde handelt<br>*Nur bei Mengeneinheit* ⇨ Der Folgeartikel wird nur gezogen, wenn es sich bei der Mengeneinheit des Hauptartikels um die in der Spalte *bei ME* angegebene Einheit handelt |
| bei ME | Hier ist die Mengeneinheit zur Einstellung ‚*Nur bei Mengeneinheit‘* in der Spalte *Hauptartikelbezug* anzugeben |
| [Kundenbezug](./kundenbezug_einrichten.md) | Soll ein Kundenbezug zu diesem Artikel hergestellt werden, muss hier ein ja eingetragen werden. |

**Wichtiger Hinweis:** Die Auswahl der zu berücksichtigenden Folgeartikel zu einer Warenposition erfolgt in der Vorgangsbearbeitung nur bei der Neuerfassung einer Warenposition. Bei Korrekturen dieser Position hinsichtlich Menge oder Mengeneinheit werden zwar die Mengen der Folgeartikel bei entsprechender Einstellung der Erfassungsparameter neu berechnet, es werden aber weder nun passende Folgeartikel hinzugefügt noch nicht mehr passende gelöscht. Nicht mehr passende Folgeartikel aufgrund von Änderungen der Mengeneinheit des Hauptartikels werden bei der Korrektur mit automatischer Kalkulation der Folgeartikel mit der Menge = 0 versehen.

<p class="siehe-auch">Siehe auch:</p>

- [Kundenbezug einrichten](./kundenbezug_einrichten.md)
- [Leergutverwaltung](./leergutverwaltung/index.md)
