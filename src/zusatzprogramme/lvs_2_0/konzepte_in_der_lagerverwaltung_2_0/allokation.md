# Allokation

<!-- source: https://amic.de/hilfe/_lvs20_allokation.htm -->

In der Vorgangsunterklasse des Auftrags & Ladescheins muss eine Auslagerstrategie festgelegt werden. Auch die Produktion (Hier ausschließlich die Unterklasse 0) muss eine solche Prozedur bekommen.

Dies ist eine Prozedur der folgenden Signatur:

```sql
---<summary>Gibt Auslagerstrategien aus</summary>
---<returns>Auslagerstrategie und Mindest oder Maximalliefermengen</returns>
---<param name="in_listennr">ListNr aus der MatrialOrder</param>
---<param name="in_listenpos">ListenPosition aus der MatrialOrder</param>
create procedure P_BUK_Auslagerstrategie_Warenausgang
(
  in in_listennr  integer,
  in in_listenpos integer
)
result
(
  AuslagerStrategie integer,
  ueberlieferung numeric(15,4),
  unterlieferung numeric(15,4)
)
```

Die Prozedur ermittelt anhand der Vorgangsdaten der Materialorder, in welchen Prozentsätzen eine Über- bzw. Unterlieferung stattfinden darf, bevor kommissioniert werden muss. So soll verhindert werden, dass wegen geringer Mengen eigens eine Kommissionierung stattfindet.

Es empfiehlt sich bei Lagerumbuchungen und Produktionen diese Sätze so hoch anzusetzen, dass keine Kommissionierung stattfindet, sondern stets ganze Ladeträger ausgeliefert werden. (MIN=99,MAX=9900)

<p class="just-emphasize">Allokation im Regal-Lager</p>

Im regal-Lager wird ab dem Zeitpunkt der Allokation die Ware reserviert. Das bedeutet, dass nach einer Allokationsstrategie Paletten ausgewählt werden, die in voller Menge ins Ziel gebracht werden sollen und solche, von denen eine Teilmenge gebraucht wird, die also noch kommissioniert werden müssen. Je nach Auslagerstrategie werden dann Fahraufträge geschrieben.

<p class="just-emphasize">Allokation im Blocklagerallokation</p>

In einem Blocklager kann nicht gezielt auf eine bestimmte Palette zugegriffen werden, Oft stehen diese in Reihen hintereinander zuweilen sogar in mehreren Ebenen.

In diesem Fall findet eine Allokation nicht statt. Stattdessen wird dem Bediener angezeigt, dass es sich im Blocklagerware handelt.

Wird nun die erste Palette gescannt, so wird die notwendige Menge allokiert. Ist mehr Ware zum Ziel zu bewegen, als auf der Palette verfügbar, so wiederholt sich der Vorgang bei der nachfolgenden Palette.

Ist eine Kommissionierung vorgesehen, wird bei einer benötigten Teilmenge die Palette mit einem Fahrauftrag in den Kommissionierbereich versehen und dort kommissioniert.
