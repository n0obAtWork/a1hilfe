# AIS im Vorgang

<!-- source: https://amic.de/hilfe/_aisrefreshvorgang.htm -->

Mit diesem Modul können bestimmte oder alle AIS-Felder auf den Vorgangserfassungsmasken in Abhängigkeit von bestimmten Feldern und Events mittels eines Makros aktualisiert werden. Das aktualisieren der einzelnen AIS Felder wird generell über ein Makro gesteuert, welches in **[FRZ]** auf der Registerkarte AIS einzutragen ist.

Es gibt aber einige Ausnahmen, hier sind die Aktualisierungspunkte fest vergeben und aktualisieren das komplette AIS auf der Maske. Die Ausnahmen werden in den Einrichtungshilfen zu den jeweiligen Masken erklärt.

**Folgende Vorgangsmasken unterstützen bislang das dynamische Aktualisieren von AIS Feldern:**

**1. SVMAIN**

**2. SVPOSI**

**3. SVWARE**

**4. SVUMMAIN**

**5. SVUMWARE**

**6. SVPOSBAR2**

#### Einrichtung

1. Einrichtung des [AIS](../../zusatzprogramme/ais_a_eins_informationssystem/index.md)

   Hauptmenü > Administration > Werkzeuge > Informationssystem

   oder Direktsprung **[AIS]**

2. Erstellen eines Makro

 Hauptmenü > Administration > Makroverarbeitung > Makro-Programme

oder Direktsprung **[MAKRO]**

3. Einrichtung des AIS für die Vorgangsmasken auf der [Registerkarte AIS](../formularzuordnung/ais.md)

 Hauptmenü > Administration > Formulare / Abläufe > Formularzuordnung/Vorgangsunterklasse

oder Direktsprung **[FRZ]**

Es empfiehlt sich für jede Vorgangsmaske eine eigene Funktion in dem Makro anzulegen. In der Vorgangsunterklassen Zuordnung wird die gewünschte Funktion des Makros der AIS-Gruppe zugeordnet. Der Makro-Name in dem Feld „Screen-Makro“ kommt aus der jeweiligen AIS-Gruppe. Für jede Vorgangsmaske können mehrere AIS-Gruppen in FRZ hinterlegt werden. Dabei ist zu beachten, dass alle Gruppen, die in FRZ einer Maske zugeordnet worden sind, nacheinander aufgerufen werden.

#### Hinweis zu dem Makro

Das Makro, welches das Aktualisieren des AIS steuert, darf **nicht zur Wertveränderung** im Vorgang benutzt werden. Da nicht sichergestellt werden kann, dass die Änderungen mit in den Vorgang übernommen werden. Um Änderungen am Vorgang vorzunehmen, ist dies weiterhin per Kontrollmakro zu realisieren.

Außerdem ist darauf zu achten, dass das Zusammenstellen der Daten für das AIS nicht zu viel Zeit in Anspruch nimmt, da dadurch das Erfassen oder Bearbeiten von Belegen mehr Zeit in Anspruch nimmt.

#### Datenübergabe in das Makro

Die Übergabe von Daten an das MAKRO erfolgt per JVARS und nicht per Paramater, da die Übergabe mit den JVARS flexibler ist, denn die Parameteranzahl ist für das Makro auf vier beschränkt.

Die JVARS unterscheiden sich in Abhängigkeit der Maske. Welche Werte mit den JVARS übergeben werden, ist in den Einrichtungshilfen zu den Masken erklärt.

#### Ablauf

Nach jeder Eingabe oder jedem Ereignis, das von Systemseite definiert worden sind. Wird das Makro aufgerufen kann dann anhand der Übergabeparameter entschieden werden, ob das AIS komplett, einzelne AIS-Felder oder gar nicht aktualisiert werden soll. Es können pro Vorgangsmaske mehrere AIS-Gruppen hinterlegt werden. Beim Aktualisieren werden alle Gruppen, die in diesem Grid hinterlegt sind, nacheinander aufgerufen. Deswegen ist darauf zu achten, dass keine zeitintensiven SQL-Statements ausgeführt werden, da diese den Ablauf massiv stören könnten.

### Funktionen, die im Makro verwendet werden

Damit das AIS Aktualisiert wird, müssen bestimmte Funktionalitäten im Makro angesprochen werden.

#### Abfragen und setzten der JVARS(BAGS) im Makro

Für das Abfragen der JVARS(BAG) in dem Makro wird kein spezieller owner benötigt. Der owner der JVARS wird automatisch vorbelegt und ist immer nur solange gültig, wie die aufrufende Maske des Makros offen ist. Die benutzten JVARS(BAGs) werden nach dem Aufruf des Makros vom System wieder abgeräumt.

1. Mit BAGGET(“JAVRNAME“,“ZIELVARIABLE“,länge) wird eine JVAR(BAG) geholt

2. Mit BAGSET(“JVARNAME“, Wert) wird eine JVAR(BAG) gesetzt.

Das Setzten einer JVAR(BAG) aus dem Makro heraus wird bislang nur für eine spezielle JAVR(BAG) in der Maske SVMAIN benötigt. Alle anderen JVARS(BAGS) werden nur im Makro gelesen.

#### Funktion zur Aktualisierung der AIS Felder

1. Mit der Funktion dbx_io ("AISREFRESH","Zeit$", "", "") wird das Feld Zeit$ aktualisiert. Wird der zweite Parameter leer gelassen, so wird das ganze AIS auf der Maske aktualisiert.

<p class="siehe-auch">Siehe auch:</p>

- [SVMAIN](./svmain.md)
- [SVPOSI](./svposi.md)
- [SVWARE](./svware.md)
- [SVUMWARE](./svumware.md)
- [SVPOSBAR2](./svposbar2.md)
