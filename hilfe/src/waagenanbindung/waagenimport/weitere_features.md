# Weitere Features

<!-- source: https://amic.de/hilfe/weiterefeatures.htm -->

Nach Markieren einer Zeile kann über Direktsprung [LIB], [REB] etc. in die betreffende Vorgangsliste gesprungen werden, wobei der neue Beleg vorselektiert ist. (An nachfolgende Auswahllisten wird die Kundennummer und die Belegnummer übergeben, so kann mit automatischer Vorselektion in jede Auswahlliste gesprungen werden, die eine der beiden Vorbelegungen akzeptiert, also z.B. auch Kundenlisten, OP-Listen etc).

Belegerzeugung aus importierten Vorgangsdaten

Die Daten aus einem Datenträger werden in 2 Relationen abgelegt: Rohwarenbelege erscheinen in der Relation RohwareHauptsatz_Waage und können über die Direktsprünge [RWWE] und [RWWV] angesehen und weiterverarbeitet werden.

Belege aus der Faktura, Umbuchungen und Produktionsbelege erscheinen in der Relation VorgangUebergabe und können über den Direktsprung [VUEB] angesehen und weiterverarbeitet werden.

Die Funktionen in der Option-Box dieser Auswahlliste sind weitgehend dieselben wie in den Anwendungen [RWWE] und [RWWV]. D. h. die Funktionen heißen zwar anders, haben aber denselben Inhalt und werden ohne Rohwaren-Steuerparameter aufgerufen.

Im einzelnen sind folgende Funktionen bis auf den SPA identisch:

CEREA_WAAGE_Import (mit SPA) = Vorgang_Import (ohne SPA)

CEREA_WAAGE_Rueck4 (mit SPA) = Vorgang_Import_Rueck4 (ohne SPA)

CEREA_WAAGE_Rueck2 (mit SPA) = Vorgang_Import_Rueck2 (ohne SPA)
