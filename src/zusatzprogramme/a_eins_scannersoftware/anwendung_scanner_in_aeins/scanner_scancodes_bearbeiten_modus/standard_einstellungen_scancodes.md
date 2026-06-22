# Standard Einstellungen Scancodes

<!-- source: https://amic.de/hilfe/_cestdladeschein.htm -->

Über den Scanner Direktsprung [SCTCP], die Auswahl der ersten Variante (Scanner Scancode) und der Funktion Standard Scancodes (SF8), können Sie ganz einfach die benötigten Scancodes für die Vorgangserfassung so wie die dazugehörige AI-Zuordnung (Application Identifier) anlegen. Sie wählen aus, welche Erfassungsvorgänge mit dem Scanner erfasst werden sollen.

Bei den Erfassungsvorgängen von Aufträgen, Bestellungen, Inventur und Eingangslieferscheinen können Sie noch optional auswählen, ob Sie mit Partie arbeiten wollen oder nicht.

Danach klicken Sie auf Standard Einspielen (F9) und die Scancodes werden samt der dazugehörigen AI-Zuordnung eingespielt.

Anhand des Scancodes weiß der Scanner welchen Erfassungsmodus er starten soll.

Die Scancodes werden als EAN 128 Code verschlüsselt eingescannt.

###### Standard System Scancodes

Die Standard System Scancodes sind für die Navigation in der Anzeige auf dem MDE (**M**obile **D**aten**e**rfassung) Gerät.

Es werden folgende Standard System Scancodes angelegt:

| Scancode | Bedeutung |
| --- | --- |
| ZENTRAL | Verbindung mit der zentralen Datenbank |
| KEYLEFT | Ermöglicht das Ansehen einer alternativen Itembox. Im Falle der Auftragsbearbeitung und Bestellung werden hier schon alle abgearbeiteten Positionen angezeigt. |
| KEYRIGHT | Schaltet die eigentliche Itembox wieder in den Vordergrund |
| KEYDOWN | Blättern in der Itembox nach unten wenn mehr als eine Position erfasst wurden |
| KEYUP | Blättern in der Itembox nach oben wenn mehr als eine Position erfasst wurden |
| STORNO | Mit diesem Befehl können rückwärts die letzten Positionen gelöscht werden. Beim ersten STORNO Scannen wird eine Itembox mit den zu stornierenden Befehlen angezeigt. Beim zweiten Scannen von STORNO wird die letzte Position gelöscht, mit jedem weiteren Scannen von STORNO wird eine Position gelöscht. Wurden alle falschen Positionen gelöscht so kann mit der Erfassung fortgefahren werden.  
Soll erneut storniert werden so beginnt das Prozedere wieder von vorne. |
| | | |

###### Standard Einrichtung Inventur / Inventur LVS

Bei Standard Einrichtung für die Inventur / Inventur LVS werden die unten genannten Scancodes angelegt.

Die Standard AI-Zuordnung ist in beiden Erfassungsvorgängen gleich.

| Scancodes Inventur |
| --- |
| IV |
| IVENDE |

| Scancodes Inventur LVS |
| --- |
| LVSIV |
| LVSIVENDE |

AI-Zuordnung für die Inventur / Inventur LVS

| AI-Nr. | Application Identifier | Gruppe | Typ | Optional |
| --- | --- | --- | --- | --- |
| \-30 | Mengeneingabe per Hand | 2 | Inventur-Start / LVS Inventur | Nein |
| \-6 | UPC-A Code | 1 | Inventur-Start / LVS Inventur | Nein |
| \-5 | EAN-Code 8 | 1 | Inventur-Start / LVS Inventur | Nein |
| \-4 | EAN-Code 13 | 1 | Inventur-Start / LVS Inventur | Nein |
| 1 | EAN Nummer der Handelseinheit | 1 | Inventur-Start / LVS Inventur | Nein |
| 2 | EAN der Verpackung | 1 | Inventur-Start / LVS Inventur | Nein |
| 10 | Charge/Partie | 3 | Inventur-Start / LVS Inventur | Nein |
| 30 | Menge in Stück  
(EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |
| 37 | Menge in Stück  
(EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |
| 3101 | Nettogewicht in Kilogramm (EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |
| 3102 | Nettogewicht in Kilogramm (EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |
| 3103 | Nettogewicht in Kilogramm (EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |
| 3104 | Nettogewicht in Kilogramm (EAN128) | 2 | Inventur-Start / LVS Inventur | Nein |

###### Standard Einrichtung Eingangslieferscheine / Bestellungserfassung

Bei der Standard Einrichtung für die Eingangslieferscheine / Bestellungserfassung werden die unten genannten Scancodes angelegt. Die Standard AI-Zuordnung ist in beiden Erfassungsvorgängen gleich.

| Scancodes Eingangslieferscheine |
| --- |
| EL |
| ELENDE |

| Scancodes Bestellungserfassung |
| --- |
| EBS |
| EBSENDE |

| Scancodes Lieferschein |
| --- |
| LI |
| LIENDE |

###### Standard Einrichtung Auftrag / Bestellung / Teildisponierung / Retoure / Lagerumbuchung

Bei der Standard Einrichtung für Auftrag / Bestellung / Teildisponierung / Lagerumbuchung werden die unten genannten Scancodes angelegt.

Wobei an den Scancodes noch die V_Numnummer mit angedruckt werden muss.

Beispiel: AU 55 oder BS 55 oder LGU 55

Die Standard AI-Zuordnung ist in allen Erfassungsvorgängen gleich.

| Scancodes Auftrag |
| --- |
| AU |
| AUENDE |

| Scancodes Bestellung |
| --- |
| BS |
| BSENDE |

| Scancodes Teildisponierung |
| --- |
| AT |
| ATENDE |
| ATSTORNO |
| ATPRINT |

| Scancodes Lagerumbuchung |
| --- |
| LGU |
| LGUENDE |

| Scancodes Auftrag Retoure |
| --- |
| AUR |
| AURENDE |

AI-Zuordnung Auftrag / Bestellung / Teildisponierung / Lagerumbuchung

| AI-Nr. | Application Identifier | Gruppe | Typ | Optional |
| --- | --- | --- | --- | --- |
| \-30 | Mengeneingabe per Hand | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| \-6 | UPC-A Code | 1 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| \-5 | EAN-Code 8 | 1 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| \-4 | EAN-Code 13 | 1 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 1 | EAN Nummer der Handelseinheit | 1 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 2 | EAN der Verpackung | 1 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 10 | Charge/Partie | 3 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 30 | Menge in Stück(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 37 | Menge in Stück(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 3101 | Menge in Kg(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 3102 | Menge in Kg(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 3103 | Menge in Kg(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |
| 3104 | Menge in Kg(EAN128) | 2 | Kommission-Start/ Bestellung Start/ Strecke/Teildispo Start/ Lagerumbuchung Start | Nein |

###### Standard Einstellung Produktion

Bei der Standard Einrichtung für die Produktion werden die unten genannten Scancodes angelegt.

Bei der Produktion gibt es keine AI-Zuordnung.

| Scancodes Produktion |
| --- |
| ABSACKENS |
| ABSACKENE |
| TROSTARTR |
| TROENDE |
| PRODSTARTR |
| PRODSTARTM |
| PRODNR |
| PRODENDE |
| STOP |

###### Standard Einrichtung Lagerverwaltungssystem

Bei der Standard Einrichtung für das Lagerverwaltungssystem werden die unten genannten Scancodes angelegt.

Bei dem Lagerverwaltungssystem gibt es keine AI-Zuordnung.

| Scancodes Lagerverwaltungssystem |
| --- |
| WAAGE |
| WAAGEENDE |
| LP |
| LPENDE |
| LPL |
| LPLENDE |
| FUELLEN |
| FUELLENENDE |
| LEEREN |
| LEERENENDE |
| WIEGEN |
| WIEGENENDE |
| UMPACK |
| UMPACKENDE |
| LOI |
| LOIP |
| BOI |
| BOIP |
| PA |
| PAP |
| MAI |
| MAIP |
| AUSWIEGEN |

###### Standard Einrichtung Labor

Bei der Standard Einrichtung für das Labor werden die unten genannten Scancodes angelegt. 

Bei dem Labor gibt es keine AI-Zuordnung.

| Scancodes Labor |
| --- |
| LABW |
| LABWE |
| LABWSTOP |
| LABP |
| PKG |
| PKU |
| PTKM |
| PRE |

###### Standard Einrichtung Ladescheinbearbeitung

Bei der Standard Einrichtung für Ladeschein wird der unten genannten Scancode angelegt.

Wobei an den Scancode noch die V_Numnummer mit angedruckt werden muss.

Beispiel: LAB 55

Die Standard AI-Zuordnung ist in allen Erfassungsvorgängen gleich.

| Scancodes Auftrag |
| --- |
| LAB |
| LABENDE |

AI-Zuordnung Auftrag / Bestellung / Teildisponierung / Lagerumbuchung

| AI-Nr. | Application Identifier | Gruppe | Typ | Optional |
| --- | --- | --- | --- | --- |
| \-30 | Mengeneingabe per Hand | 2 | Ladeschein Start | Nein |
| \-6 | UPC-A Code | 1 | Ladeschein Start | Nein |
| \-5 | EAN-Code 8 | 1 | Ladeschein Start | Nein |
| \-4 | EAN-Code 13 | 1 | Ladeschein Start | Nein |
| 1 | EAN Nummer der Handelseinheit | 1 | Ladeschein Start | Nein |
| 2 | EAN der Verpackung | 1 | Ladeschein Start | Nein |
| 10 | Charge/Partie | 3 | Ladeschein Start | Nein |
| 30 | Menge in Stück(EAN128) | 2 | Ladeschein Start | Nein |
| 37 | Menge in Stück(EAN128) | 2 | Ladeschein Start | Nein |
| 3101 | Menge in Kg(EAN128) | 2 | Ladeschein Start | Nein |
| 3102 | Menge in Kg(EAN128) | 2 | Ladeschein Start | Nein |
| 3103 | Menge in Kg(EAN128) | 2 | Ladeschein Start | Nein |
| 3104 | Menge in Kg(EAN128) | 2 | Ladeschein Start | Nein |
