# Weiterverarbeitung

<!-- source: https://amic.de/hilfe/weiterverarbeitung.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung

Direktsprung **[RWBV]**

Erfasste Rohwarelieferungen können in einer oder mehreren Stufen abgerechnet werden. Die einzelnen Stufen sind:

Abschlagabrechnung

Folgeabschlagabrechnung

Finalabrechnung

Im Einkauf können zu bestehenden Finalabrechnungen gegebenenfalls zu einem späteren Zeitpunkt noch **Nachvergütungen** erstellt werden.

Dazu ist es zunächst erforderlich, die betreffenden Belege in einen Beleg der gewünschten Abrechnungsstufe umzuwandeln. Dieses geschieht durch Aufruf einer der Funktionen

Abschlag vorbereiten 

F-Abschlag vorbereiten 

Finale vorbereiten 

Dabei wird jeweils ein neuer Beleg erzeugt, der aber die Daten des Ursprungsbelegs enthält. Grundsätzlich kann ein Beleg nur dann in die nächste Stufe übertragen werden, wenn das entsprechende Statuskennzeichen

Status Abschlag

Status Folgeabschlag

Status Finale

dieses erlaubt:

**ohne** (gibt es nicht bei **Status Finale**):  
Abschlag bzw. Folgeabschlag ist nicht vorgesehen, es kann zur Lieferung nur ein Finalbeleg erzeugt werden.

**gesperrt:  
**Der Beleg wird für diese Stufe erzeugt, kann aber erst abgerechnet werden, wenn er dafür per Korrektur freigegeben wird.

**freigegeben:  
** Der Beleg wird für diese Stufe erzeugt und kann auch ohne Korrektur abgerechnet werden.

Es wird dabei immer sichergestellt, dass keine vorgesehene Abrechnungsstufe ausgelassen wird.  
Soll zu einer Lieferung direkt ein Finalbeleg erstellt werden, so muss das Kennzeichen **Status Abschlag** mit dem Wert **ohne** belegt sein. Entsprechend muss das Kennzeichen **Status Folgeabschlag** mit dem Wert **ohne** belegt sein, wenn der Folgebeleg zur Abschlagabrechnung der Finalbeleg sein soll. Folgeabschlag-Belege können nur aus Abschlagbelegen erzeugt werden.

Ist die Einstellung des Rohwareparameters *nächste* [*Stufe nur nach Fibuübertrag*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_113) für das Abrechnungsschema des Quellbelegs mit dem Wert ‚<em>ja</em><strong>‘</strong> belegt, so kann die Umwandlung erst nach erfolgtem Fibuübetrag durchgeführt werden. Gleiches gilt, wenn der Steuerparameter ‚<em>Mengenbuchung bei Fibuübertrag‘</em> mit ‚<em>ja</em><strong>‘</strong> eingestellt ist. Außerdem müssen die Quellbelege durch den Mandantenserver abgearbeitet sein.  
Das Abrechnen eines derartig erzeugten Belegs erfolgt in einem gesonderten Schritt. Dieser kann, zum Beispiel nach erfolgten Ergänzungen und Korrekturen, mit der Funktion ***Abrechnen*** ausgelöst werden. Bei entsprechender Einstellung des Rohwareparameters [*Abrechnung nach Belegkorrektur*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_172) kann dieses für freigegebene Belege auch automatisch nach Ergänzungen und Korrekturen erfolgen.

Grundsätzlich ist ein frisch erzeugter Abrechnungsbeleg noch nicht abgerechnet, da per Belegkorrektur in der Regel noch für die neue Abrechnungsstufe ergänzende Angaben zu machen sind. Es ist jedoch möglich, durch entsprechende Einstellung der Zusatzfunktion auf der vor der Ausführung der Umwandlungsfunktion erscheinenden Vorbereitungs-Maske das automatische Abrechnen nach erfolgter Umwandlung zu erzwingen. Die möglichen Einstellungen für das Feld **Zusatzfunktion** sind:

keine automatische Weiterverarbeitung

Abrechnen

Abrechnen und Druck

Abrechnen, Druck und Fibuübertrag

Abrechnen und Fibuübertrag

Auf der Vorbereitungs-Maske kann auch das mit dem aktuellen Tagesdatum vorbelegte neue Belegdatum sowie gegebenenfalls die zugeordneten Buchungsperioden für Warenwirtschaft und Finanzbuchhaltung vorgegeben werden. Bei aktivierter Abrechnungs-Zusatzfunktion kann hier ebenfalls ein Valutadatum angegeben werden, welches in den Belegen mit Zahlungsbedingungen, deren Valutadatum nicht berechnet wird, bei der Abrechnung herangezogen wird. In diesem Fall muss der Quellbeleg unabhängig von allen weiteren Einstellungen bereits an die Fibu übertragen sein.

<p class="siehe-auch">Siehe auch:</p>

- [Jahresübergreifende Abrechnung](./jahresuebergreifende_abrechnung.md)
