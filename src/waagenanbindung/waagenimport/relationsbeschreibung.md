# Relationsbeschreibung

<!-- source: https://amic.de/hilfe/relationsbeschreibung.htm -->

**Relation ScriptParam**

Attribut Typ Länge, ...Defaultwert NULL PKey

ScriptPPBedKorr integer 4 0 0 N N 

ScriptPBesitzer integer 4 0 .................... N N 

ScriptPBezeich char 50 0 .................... N N 

ScriptPId char 20 0 .................... N Y 

ScriptSystem smallint 2 0 .................... N N 

**Relation ScriptParamPar**

Attribut Typ Länge, ...Defaultwert NULL PKey

ScriptPId char 20 0 .................... N Y 

ScriptPPAktiv smallint 2 0 0 N N 

ScriptPPBedKorr integer 4 0 0 N N 

ScriptPPBezeich char 50 0 .................... N N 

ScriptPPId char 30 0 .................... N Y 

ScriptPPTyp smallint 2 0 .................... N N 

ScriptPPWert1 char 50 0 N N 

ScriptPPWert2 char 50 0 N N 

ScriptPPWert3 char 50 0 N N 

ScriptSystem integer 4 0 0 N N
