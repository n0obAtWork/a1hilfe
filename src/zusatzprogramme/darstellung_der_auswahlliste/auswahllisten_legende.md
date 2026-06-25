# Auswahllisten-Legende

<!-- source: https://amic.de/hilfe/_auswahllistenLegende.htm -->

Bei der Auswahllisten-Legende handelt es sich um einen Dialog, welcher die einzelnen Felder und Farben einer Auswahlliste beschreibt. Die Funktion ist auf den Auswahllisten vorhanden, die Informationen für die Legende bereitstellen. Diese Informationen für die Auswahlliste werden in jedem SQL-Text der Auswahllistenvariante gepflegt. Dort werden diese per XML-Struktur zur Verfügung gestellt. Die Tags und Attribute werden immer klein geschrieben.

<p class="just-emphasize">XML-Tag &lt;auswahllistenbeschreibung></p>

Dieses Tag ist das Haupt-Tag der XML-Struktur. Unter diesem kann sich ein Tag für die Beschreibung befinden und mehrere Tags für die Felder.

```xml
<auswahllistenbeschreibung></auswahllistenbeschreibung>
```

<p class="just-emphasize">XML-Tag &lt;beschreibung></p>

Dieses Tag kann sich unter folgenden übergeordneten Tags befinden.

**Tag &lt;auswahllistenbeschreibung>**

Befindet sich das Tag unter diesem Tag, handelt es sich um die Beschreibung der Auswahlliste.

```xml
<auswahllistenbeschreibung>
    <beschreibung>Daten der
Variante</beschreibung>
</auswahllistenbeschreibung>
```

**Tag &lt;field>**

Befindet sich das Tag unter diesem Tag, handelt es sich um die Beschreibung des Feldes.

```xml
<auswahllistenbeschreibung>
    <beschreibung>Daten der
Variante</beschreibung>
    <field name="Feldname">
  <beschreibung>Beschreibung des
Feldes</beschreibung>
    </field>
</auswahllistenbeschreibung>
```

<p class="just-emphasize">XML-Tag &lt;field></p>

Ein oder mehrere dieser Tags befinden sich unter dem Haupt-Tag (&lt;auswahllistenbeschreibung>). Dieser dient der Beschreibung der einzelnen Spalten. Unter diesem befinden sich auch die Tags für die Beschreibungen der Farben

Das Tag hat zusätzlich zu den unter Tags auch das Attribut „name“. In diesem wird der Spaltenname der Feldbeschreibung eingetragen.

Feldbeschreibung innerhalb des SQL-Textes

```text
FIELD
Nummer,b.KtrNummer,I4,8,COLOR=(colorKontraktstatus,1=GELB/GRÜN,2=SCHWARZ/GELB,3=BLAU/GELB)
```

```xml
<auswahllistenbeschreibung>
    <beschreibung>Daten der
Variante</beschreibung>
    <field name="b.KtrNummer">
  <beschreibung>Beschreibung des Feldes</beschreibung>
    </field>
</auswahllistenbeschreibung>
```

<p class="just-emphasize">XML-Tag &lt;color></p>

Ein oder mehrere dieser Tags können sich unter einem &lt;field> - Tag befinden. Das Tag dient der Beschreibung einer Farbe. Bei der Beschreibung wird nicht die Farbe angegeben, sondern die Nummer wird als Attribut („nr“) aus dem SQL-Text FIELD verwendet.

Feldbeschreibung innerhalb des SQL-Textes

```text
FIELD
Nummer,b.KtrNummer,I4,8,COLOR=(colorKontraktstatus,1=GELB/GRÜN,2=SCHWARZ/GELB,3=BLAU/GELB)
```

```xml
<auswahllistenbeschreibung>
    <beschreibung>Beschreibung der
Variante</beschreibung>
    <field name="b.KtrNummer">
  <beschreibung>Beschreibung des Felds</beschreibung>
      <color nr="1">Beschreibung zur Farbe
1</color>
      <color nr="2">Beschreibung zur Farbe
2</color>
      <color nr="3">Beschreibung zur Farbe
3</color>
    </field>
</auswahllistenbeschreibung>
```

<p class="just-emphasize">Vollständiges Beispiel</p>

Im SQL-Text müssen die Informationen mit einem XML anfangen. Die XML-Struktur selber muss als erstes Zeichen einer Zeile immer ein Leerzeichen enthalten.

Beispiel XML:

```text
XML
 <auswahllistenbeschreibung>
   <beschreibung>Daten des
Kontraktstamms</beschreibung>
   <field name="KtrKlasse">
     <beschreibung>Klasse des
Kontrakts</beschreibung>
   </field>
   <field name="s.KtrUnterklasse">
<beschreibung>Unterklasse des Kontrakts</beschreibung>
   </field>
   <field name="minArtikelnummer">
     <beschreibung>Der erste
gefundene Artikel</beschreibung>
   </field>
   <field name="ArtikelListe">
     <beschreibung>Liste
aller Artikel mit Ausnahme des "ersten" Artikels</beschreibung>
   </field>
   <field name="Erledigt">
     <beschreibung>Gibt den
aktuellen Erledigungsstatus des Kontrakts zurück.</beschreibung>
     <color nr="1">Erledigter
Kontrakt</color>
     <color
nr="0">Unerledigter Kontrakt</color>
   </field>
   <field name="b.KtrNummer">
     <beschreibung>Nummer des
Kontrakts</beschreibung>
     <color nr="1">Es handelt
sich um einen Musterkotrakt.</color>
     <color nr="2">Der
Kontrakt ist storniert, erledigt oder gelöscht.</color>
     <color nr="3">Das
Kontrakt-bis-Datum liegt vor dem aktuellen Tagesdatum und ist damit schon
abgelaufen, aber noch nicht als erledigt gekennzeichnet</color>
     <color nr="4">Das
Kontrakt-von-Datum liegt nach dem aktuellen Tagesdatum und ist damit noch nicht
aktiv.</color>
     <color nr="5">Der
Kontrakt ist aktiv, da die vorherigen Punkte nicht zutreffen</color>
   </field>
   <field name="b.KtrBezeich">
<beschreibung>Bezeichnung des Kontrakts</beschreibung>
     <color nr="1">Es handelt
sich um einen Musterkotrakt.</color>
     <color nr="2">Der
Kontrakt ist storniert, erledigt oder gelöscht.</color>
     <color nr="3">Das
Kontrakt-bis-Datum liegt vor dem aktuellen Tagesdatum und ist damit schon
abgelaufen, aber noch nicht als erledigt gekennzeichnet</color>
     <color nr="4">Das
Kontrakt-von-Datum liegt nach dem aktuellen Tagesdatum und ist damit noch nicht
aktiv.</color>
     <color nr="5">Der
Kontrakt ist aktiv, da die vorherigen Punkte nicht zutreffen</color>
   </field>
 </auswahllistenbeschreibung>
```
