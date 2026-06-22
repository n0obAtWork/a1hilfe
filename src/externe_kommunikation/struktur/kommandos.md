# Kommandos

<!-- source: https://amic.de/hilfe/_ prodI_command.htm -->

Das XML kennt folgende Kommandos im „COMMAND“-Tag:

| COMMAND |
| --- |
| Kommando | Richtung | LVS | Bedeutung |
| BEGIN | IN A.eins | Nein | Hier wird in die Tabelle „ProduktionsInfo“ die Linne eingetragen, auf der diese Produktion jetzt laufen soll. In die Tabelle „ProduktionsInfo“ wird eingetragen, in welchem Status sich die Produktion befindet. |
| MATERIAL | IN A.eins | Ja | Hier wird von der Produktion Material als Fertigware an das LVS gemeldet. Der angegebene Ladeträger wird in der Lokalität der Fertigware der angegebenen Linie mit Hilfe eines Vorgangsimports (LVS) erzeugt. |
| END | IN A.eins | Nein | Mit den Verbrauchsdaten und den Produktionsdaten wird die Produktion in A.eins korrigiert. |
| MATERIALREQUEST | IN A.eins | Ja | Hier wird eine Materialanforderung gegeben. Diese kann, muss aber nicht einer Produktion zugeordnet sein. Wichtig ist die Angabe der Linie, da diese im LVS die Bereitstellungszone bestimmt.<br>Die Materialanforderung wird in die LVS_Materialorder geschrieben |
| PARTS | AUS A.eins | NEIN | Dies ist die Stückliste der Produktion |
