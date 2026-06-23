# Schachtverwendung beim Druck

<!-- source: https://amic.de/hilfe/_druckerschachtverw.htm -->

Moderne Drucker haben unter Umständen mehrere Papierzufuhreinrichtungen mit Vorratsbehältern für Papier. Diese sogenannten Schächte können dafür verwendet werden, Papier mit unterschiedlichen Maßen, Farben oder Vordrucken aufzunehmen.

Das kann hilfreich sein, wenn z.B. für das erste Blatt ein Papier mit vorgedrucktem Briefkopf verwendet werden soll oder der Zweitdruck mit einem andersfarbigen Papier gekennzeichnet werden soll.

Hardwarevoraussetzungen

Für die Verwendung von Schächten ist es zunächst notwendig, dass der Drucker mehrere Schächte hat und diese sich über den Druckertreiber gezielt ansteuern lassen.

Einrichtung im Druckerstamm

Im [Druckerstamm [DRST]](./druckerstamm/druckerstamm_pfleger.md#DRST_Einrichtung) kann eingestellt werden, dass und welche Schächte verwendet werden sollen und welcher Schacht verwendet werden soll, wenn keine der Steuerungen dies angibt.

Schachtdefinition im Formular

Im [Formular [FRM]](../../zusatzprogramme/formulareinrichtung_und_zuordnung/der_formular_pfleger/index.md#Windows_Druck_Einstellungen) kann festgelegt werden, ob ein bestimmter (von der Druckerstammeinstellung abweichender) Schacht verwendet werden soll.

Es kann sogar definiert werden, dass die erste Seite auf einem von den folgenden Seiten abweichenden Schacht gedruckt werden soll.

Vorgangsdruckklassen-Einstellungen

In der [Vorgangsdruckklasse [VRGD]](./vorgangsdruckklassen.md) kann festgelegt werden, ob ein bestimmter (von der Druckerstammeinstellung abweichender) Schacht verwendet werden soll.

Es kann sogar definiert werden, dass die erste Seite auf einem von den folgenden Seiten abweichenden Schacht gedruckt werden soll. Das kann hilfreich sein, wenn für das erste Blatt ein Papier mit vorgedrucktem Briefkopf verwendet werden soll.

Reihenfolge der Entscheidungsfindung bei widersprüchlichen Einstellungen

Zunächst ist zwingend notwendig, dass im Druckerstamm [DRST] Schächte definiert sind und ein Standardschacht angegeben wurde. Ohne diese Einstellung gibt es keine Schachtverwendung!

- Beim Druck wird zunächst die Einstellung im Formular [FRM] gelesen.
- Anschließend wird - wenn anwendbar – die Schachteinstellung der verwendeten Vorgangsdruckklasse [VRGD] berücksichtigt und diese überschreibt - wenn vorhanden – die bisherigen Einstellungen.
- Ist bisher keine Einstellung erfolgt, wird der Standardschacht aus dem Druckerstamm [DRST] verwendet.
