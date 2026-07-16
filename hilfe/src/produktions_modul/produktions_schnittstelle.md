# Produktions-Schnittstelle

<!-- source: https://amic.de/hilfe/_produktionsschnittstelle.htm -->

A.eins verfügt über eine XML-Schnittstelle für Produktionsmaschinen. Darüber lassen sich verschiedene Daten von A.eins an die Maschine und zurück übermitteln. Dabei ist das Austauschformat über Makros sogar erweiterbar.

#### In Richtung Maschine:

- Export eines Rezepts für die Produktion
  - Komponenten
- Artikel
- Partien
- Mengen
  - Produkte
- Artikel
- Partien
- Mengen

  Dieser Export ist durch Makros um weitere Informationen aus A.eins an die Maschine erweiterbar.

- Export von an der Linie bereitgestellte Mengen (nur mit LVS)

**In Richtung A.eins:**

- Import von Ladeträgern mit dem Produkt (nur mit LVS)
  - NVE
  - Artikel
- Partien
  - Mengen
- Import von verbrauchten Komponenten (nur mit LVS)
  - NVE
  - Artikel
- Partien
  - Mengen
- Import von in ein Produktionssilo eingefügten Mengen (nur mit LVS)
  - NVE
  - Artikel
- Partien
  - Mengen
- Import von verbrauchten Komponenten (Summe)
  - Artikel
- Partien
  - Mengen
- Import von erzeugten Produkten (Summe)
  - Artikel
- Partien
  - Mengen

#### Sonderfall Rohwarenannahme

Eine Rohwarenannahme ich keine Produktion im eigentlichen Sinne, aber es können Daten im analogen Format empfangen werden, wenn eine Annahme, Schüttgut in Kisten, Säcke o.ä. Ladeträger verbringt.

Auch hier gibt es die Möglichkeit der Meldungen

- Produkte (nur mit LVS)
  - NVE
  - Artikel
- Partien
  - Mengen
- Produktsummen
  - Artikel
- Partien
  - Mengen
  - Proben
- NVE
- Werte
- ID
- Wert

  Um diese Produktsummen nun in eine Rohwarenanlieferung eingehen zu lassen, ist ein Makro notwendig, das die notwendigen Parameter ermittelt,

  Die Weiterberarbeitung der Daten findet dann in der OWaage statt.

  Der Informationsumfang der Maschine kann größer sein, als unser Standard und dieser kann um private Inhalte erweitert und im Makro ausgelesen werden.
