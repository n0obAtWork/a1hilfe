# Relation KundenTankKarte

<!-- source: https://amic.de/hilfe/relationkundentankkarte.htm -->

In der Relation KundenTankKarte können zu einer KundId Tankkartennummern mit einem max. Gültigkeitsdatum hinterlegt werden. Diese Relation kann auch für Beliebige Umschlüsselungen der Kundennummer verwendet werden.

Kundid integer 4 0 .................... N N 

KundTKartBisDatum date 4 0 today(\*)+365 Y N 

KundTKartNr integer 4 0 .................... N Y 

**KundId:** Die Verknüpfung mit dem Kundenstamm

**KundTKartBisDatum:** Letztes Gültigkeitsdatum (hat nur Informations-Charakter, da eine Validierung nicht stattfindet.

**KundTKartNr:** Nummer der Tankkarte, muss absolut eindeutig sein.
