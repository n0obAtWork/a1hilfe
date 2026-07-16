# Kostenstellendefinition

<!-- source: https://amic.de/hilfe/kostenstellendefinition.htm -->

Hauptmenü > Abschlussarbeiten > Chefcockpit > Chefcockpit-Designer > Definitionstyp **Kostenstellendefinition**

Direktsprung **[CCD]**

Hier legt man eine Liste von Kostenstellen an, auf die man bei der Berechnung später zugreifen will. In einer Kostenstellendefinition kann eine Kostenstelle nur einmal erscheinen. Doppelt angegebene Kostenstellen werden nur einmal gespeichert.

Im Normalfall werden Sollsalden positiv und Habensalden negativ dargestellt. Man müsste man also das Ergebnis der Kennzahlengruppe mit -1,00 multiplizieren um Erträge positiv und Aufwendungen negativ darzustellen. Um bereits das Ergebnis in der gewünschten Form zu erhalten kann man von vornherein einen Faktor -1,00 angeben, der für die einzelnen Salden das Vorzeichen automatisch dreht.

Will man auf die Planzahlen dieser Definition zugreifen, so muss man keine neue Liste definieren. Man stellt dem Kürzel einfach PLAN_ vorweg - also PLAN_AKZ für die Plandaten. Siehe auch [Kontendefinition](./kontendefinition.md) oder [Kostenträgerdefinition](./kostentraegerdefinition.md).
