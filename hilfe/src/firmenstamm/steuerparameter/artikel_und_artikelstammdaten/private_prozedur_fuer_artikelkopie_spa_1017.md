# Private Prozedur für Artikelkopie (SPA 1017)

<!-- source: https://amic.de/hilfe/_SPA_1017.htm -->

In der Option dieses Steuerparameters kann eine private Prozedur für die Behandlung einer Artikelkopie hinterlegt werden. Diese kann nach der Kopie eines Artikels ggf. privatisierte Zusatzrelationen abseits des Artikeladdon versorgen.

Als Parameter muss die Prozedur die zwei Parameter „artikelalt“ und „artikelneu“ vom Typ „integer“ enthalten. Ist diese Bedingung nicht erfüllt, findet sich eine entsprechende Fehlermeldung im Fehlerprotokoll. Die Ausführung der Artikelkopie wird dadurch nicht verhindert.

Das Gleiche gilt auch für den Fall, dass die Prozedur, die im Steuerparameter definiert wurde nicht vorhanden ist.
