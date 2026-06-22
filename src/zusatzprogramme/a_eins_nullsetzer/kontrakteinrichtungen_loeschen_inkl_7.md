# Kontrakteinrichtungen löschen (inkl. 7)

<!-- source: https://amic.de/hilfe/kontrakteinrichtungenlschenink.htm -->

KONTRUNTER  
KontrUnterKlasse

KONTRVARIANTE

KONTRVARIBAUST

KONTRVARIPOSIT

KONTRVARITEXT

KONTRDISPOZUORD

KontrKlNumKreis

BemerkPosition unter der Bedingung where (BemerkId in (select BemerkId from BemerkStamm where (BemerkTyp in (31, 32)))

BemerkPositionWERTE unter der Bedingung where (BemerkId in (select BemerkId from BemerkStamm where (BemerkTyp in (31, 32)))

BemerkStamm unter der Bedingung where (BemerkTyp in (31, 32))

Bemerkung unter der Bedingung where (BemerkTyp = 31) or (BemerkTyp = 32)  
    
31=Kontrakt-Bemerkung

32=Kontraktdispositions-Bemerkung

Beim Löschen der Kontrakteinrichtungen werden automatisch die [Kontrakte](./kontrakte_loeschen.md) mit gelöscht.
