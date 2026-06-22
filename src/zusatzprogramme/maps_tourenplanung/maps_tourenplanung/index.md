# Maps Tourenplanung

<!-- source: https://amic.de/hilfe/_MapsTourenplanung2023.htm -->

In allen Auswahllisten mit einer adressid oder kundid in den Returnfeldern heraus kann eine Tourenplanung angeschlossen werden. Die Anwendung sucht zunächst nach einer AdressId und wenn diese nicht verfügbar ist nach der KundId, deren Hauptanschrift dann ermittelt wird.

Beispielhaft ist die Funktion in Kunden [KU] und Anschriften [ANSCH] eingebaut worden.

Der Controlstring lautet

```text
^jpl
MapsTouren <ProfilId>
```

Die ProfilId bestimmt sich aus dem eingestellten Profil in [MTPP]

Die maximale Anzahl der anzuzeigenden Wegpunkt ist je nach Anwendungsfall beschränkt:

• Bei Anzeige „nur Verteilung“ gibt es keinerlei Einschränkungen

• Bei Streckenanzeigen, die einen festen Startpunkt berücksichtigen, dürfen nur 9 weitere Wegpunkte aufgenommen werden.

• Bei Streckenanzeigen ohne festen Startpunkt können 10 Wegpunkte angegeben werden.

Die Beschränkungen liegen in der von Google bereitgestellten Entfernungsmatrix begründet.

<p class="siehe-auch">Siehe auch:</p>

- [Menu-Funktionen der MapsTourenplanung](./menu_funktionen_der_mapstourenplanung.md)
- [Masken-Funktionen der MapsTourenplanung](./masken_funktionen_der_mapstourenplanung.md)
