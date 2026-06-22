# Frachttabellen

<!-- source: https://amic.de/hilfe/_Frachten_Tabellen.htm -->

Nebenbuchhaltungen > Frachtverwaltung > Frachttabellen **[FRA]**

In einer Frachttabelle werden für Frachtberechnungen notwendige Festlegungen getroffen.

Bezeichnung

Bezeichnung für die Fracht

Frachttext

[Frachttext](../fracht_texte.md)

Fracht Formel

| Formel | Bedeutung |
| --- | --- |
| % vom Warenwert | Prozentualer Anteil vom Warenwert |
| Pausch. / Lieferung | Pauschale pro Lieferung |
| Pausch. / Position | Pauschale pro Warenposition auf der Lieferung |
| Pausch. / km | Pauschale pro Kilometer Entfernung |
| Satz / Mengeneinheit | Es wird ein fester Betrag pro Mengeneinheit berechnet. Wird z.B. in Tonnen fakturiert, so wird die Fracht pro Tonne berechnet. |
| Satz / ME + km | Wie oben jedoch unter zusätzlicher Berechnung mit dem entfernungsabhängigen Formeltyp |
| Satz / Gewichtseinheiten | Es wird ein fester Betrag pro Gewichtseinheit des Artikels berechnet |
| Satz / GE + km | Wie oben jedoch unter zusätzlicher Berechnung mit dem entfernungsabhängigen Formeltyp |
| Satz / Bruttogewichtseinheiten | Es wird ein fester Betrag pro Brutto-Gewichtseinheit des Artikels berechnet |
| Satz / BruttoGE + km | Wie oben jedoch unter zusätzlicher Berechnung mit dem entfernungsabhängigen Formeltyp |
| Satz / Verpackungseinheiten | Es wird ein fester Betrag pro Verpackungseinheit des Artikels berechnet |
| Satz / VE + km | Wie oben jedoch unter zusätzlicher Berechnung mit dem entfernungsabhängigen Formeltyp |

Offene Frachtberechnung

Gibt an, ob die Fracht auf dem Beleg sichtbar ausgewiesen werden soll (nicht bei Gruppenfracht).

Gruppenfracht im Belegdruck

Gibt bei einer Gruppenfracht an, ob bei der Belegerfassung das Kennzeichen zur Druckbarkeit der Gruppenfrachtposition gesetzt werden soll oder nicht. Diese Einstellmöglichkeit ist nur verfügbar, wenn es sich nicht um eine kalkulatorische Frachtermittlung handelt (diese wird nie gedruckt) und der Steuerparameter [SPA 980](../../../../firmenstamm/steuerparameter/frachten_und_frachtwesen/druckunterdrueckung_bei_nicht_kalkulatorischer_gruppenfracht.md) die Unterdrückung der Druckausgabe erlaubt. Im Standard werden nicht kalkulatorische Gruppenfracht-Positionen immer als druckbar gekennzeichnet.  
ACHTUNG: Bei Nutzung der Druckunterdrückung von Gruppenfrachten ist zu beachten, dass der Einfluss auf die Berechnung von druckbaren Preisen und Beträgen der zugehörigen Warenpositionen sowie Zwischensummen und Endbeträgen wie bei offenen Gruppenfrachten erfolgt. Es ist von daher sehr genau auf eine passende Druckpositionswahl bei der Formulareinrichtung zu achten.

Kalkulatorische Ermittlung

Diese Option ist nur als Alternative zur offenen Frachtberechnung möglich !

Ja/Nein-Entscheidung, ob diese Fracht eine kalkulatorische Fracht sein soll, die als Teil des Preises berechnet und nicht gesondert ausgewiesen werden soll.

Eine kalkulatorische Fracht wird wie ein InZeile- Zu-Abschlag nicht auf dem Beleg ausgegeben. Eine kalkulatorische Fracht wird jedoch nicht zur Berechnung des Betrages berechnet. Die kalkulatorische Fracht sorgt dafür, dass dieser „Fracht-Anteil“ abweichend von der Erlöskennziffer des Artikels auf eine andere Erlöskennziffer gebucht wird.

Zeile/Gruppe/Preis

| Einstellung | Bedeutung |
| --- | --- |
| Zeile | Zeilenrabatt – wirkt auf eine Warenposition |
| Gruppe | Gruppenrabatt – wirkt auf alle Artikel dieser Warengruppe |
| Preis | Dieser Rabatt wirkt zunächst auf den Einzelpreis, bevor dieser mit der Menge multipliziert wird. |

Steuerschlüssel

Steuerschlüssel für diese Fracht

Kostenstelle

Hier kann eine von der Warenposition abweichende Kostenstellennummer für die Fracht angegeben werden.  
0 = es wird die Kostenstellennummer der Warenposition übernommen  
Dieses Erfassungsfeld steht nur zur Verfügung, wenn der Steuerparameter **Kostenstellen-Lizenz** aktiviert ist.

Kostenträger

Hier kann eine von der Warenposition abweichende Kostenträgernummer für die Fracht angegeben werden.  
0 = es wird die Kostenträgernummer der Warenposition übernommen  
Dieses Erfassungsfeld steht nur zur Verfügung, wenn der Steuerparameter **Kostenträgerrechnung angeschlossen** aktiviert ist.

Kostenobjekt

Hier kann eine von der Warenposition abweichende Kostenobjektnummer für die Fracht angegeben werden.  
0 = es wird die Kostenobjektnummer der Warenposition übernommen  
Dieses Erfassungsfeld steht nur zur Verfügung, wenn der Steuerparameter **Kostenobjekt-Lizenz** aktiviert ist.  
Die Bezeichnung dieses Feldes ist in der **OPTION Kostenobjekt_Label** einrichtbar!

Formeltyp nur bei entfernungsbezogener Frachtermittlung

Bei entfernungsbezogenen Frachten werden mit Hilfe des unten angegebenen Frachttyps Frachten mit der Entfernung multipliziert. Es ist möglich je km, oder auch z.B. je 100km zu berechnen.

Der Formeltyp gibt die anschließende Rundung an.

| Formeltyp | Bedeutung |
| --- | --- |
| Frachtsatz \* Entfernung / Entfernungseinheit, einfache Rundung | Hier wird der Frachtsatz mit der Entfernung multipliziert und dann einfach gerundet |
| Frachtsatz je volle Entfernungseinheit | Hier wird der Frachtsatz mit der vollen Entfernungseinheit (Entfernung / Faktor) multipliziert |
| Frachtsatz je angefangene Entfernungseinheit | Hier wird der Frachtsatz mit der angefangenen Entfernungseinheit (Entfernung / Faktor) multipliziert |

Beispiel:

• Menge 10 kg,

• Frachtsatz 10€ / Kg,

• Entfernung 150 km,

• Berechnung pro 100km,

• Frachtsatz je volle Entfernungseinheit:

10kg / 10€/kg è 100 €

150 km / 100 km = 1,5 è 1 (nur volle Entfernungseinheiten)

100€ \* 1 = 100€

<p class="siehe-auch">Siehe auch:</p>

- [Frachtsätze](./frachtsaetze.md)
- [Frachttabellenzuordnungen](./frachttabellenzuordnungen.md)
