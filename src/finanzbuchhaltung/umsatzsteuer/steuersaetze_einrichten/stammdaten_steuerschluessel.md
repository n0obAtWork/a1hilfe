# Stammdaten Steuerschlüssel

<!-- source: https://amic.de/hilfe/stammdatensteuerschlssel.htm -->

Hauptmenü > Abschlussarbeiten > Umsatzsteuer > Steuern > Funktion Steuerschlüssel **F7**

Direktsprung **[STS]**.

Es ist möglich, Erlöse nach steuerlichen Gesichtspunkten zu differenzieren (Verprobung Umsatzsteuervoranmeldung). Der Steuerschlüssel wird entweder im Artikelstamm oder für Buchungen in der Finanzbuchhaltung im Sachkontenstamm hinterlegt. Der Pfleger für die Steuerschlüssel sieht lediglich die Eingabe einer Nummer und eines beschreibenden Textes vor. Eine Einrichtung der Steuerschlüssel könnte so aussehen:

| Schlüssel | Beschreibung |
| --- | --- |
| 0 | Systemsteuersatz Null |
| 1 | Voller Steuersatz z.Zt. 19 % |
| 2 | Halber Steuersatz z.Zt. 7 % |

Das Feld **Steuertyp** wird von der Finanzbuchhaltung nicht ausgewertet. Es dient der Einordnung der Steuerschlüssel in die Klassifikationen der OpenTRANS®-Standards. Das Feld wird nur angezeigt, wenn Sie OpenTRANS® verwenden und den Steuerparameter [721 – OpenTRANS®](../../../firmenstamm/steuerparameter/lizenzen/opentrans_lizenz_spa_721.md) eingeschaltet haben.
