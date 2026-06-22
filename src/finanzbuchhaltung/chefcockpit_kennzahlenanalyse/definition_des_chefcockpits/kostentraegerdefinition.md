# Kostenträgerdefinition

<!-- source: https://amic.de/hilfe/kostentrgerdefinition.htm -->

Hauptmenü > Abschlussarbeiten > Chefcockpit > Chefcockpit-Designer > Definitionstyp **Kostenträgerdefinition**

Direktsprung **[CCD]**

Hier legt man eine Liste von Kostenträger an, auf die man bei der Berechnung später zugreifen will. In einer Kostenträgerdefinition kann ein Kostenträger nur einmal erscheinen. Doppelt angegebene Kostenträger werden nur einmal gespeichert.

Im Normalfall werden Sollsalden positiv und Habensalden negativ dargestellt. Man müsste man also das Ergebnis der Kennzahlengruppe mit -1,00 multiplizieren um Erträge positiv und Aufwendungen negativ darzustellen. Um bereits das Ergebnis in der gewünschten Form zu erhalten kann man von vornherein einen Faktor -1,00 angeben, der für die einzelnen Salden das Vorzeichen automatisch dreht.

Will man auf die Planzahlen dieser Definition zugreifen, so muss man keine neue Liste definieren. Man stellt dem Kürzel einfach PLAN_ vorweg - also PLAN_AKZ für die Plandaten. Siehe auch [Kontendefinition](./kontendefinition.md) oder [Kostenstellendefinition](./kostenstellendefinition.md).
