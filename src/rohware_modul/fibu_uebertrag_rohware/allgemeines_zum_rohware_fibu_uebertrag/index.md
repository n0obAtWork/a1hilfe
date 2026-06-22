# Allgemeines zum Rohware-Fibu-Übertrag

<!-- source: https://amic.de/hilfe/_rwb_fib_allgemein.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung

Direktsprung **[RWBV]**

Die Übertragung der Rohware-Belege an die Finanzbuchhaltung erfolgt in den Modulen ***EK-Rohwarenbearbeitung*** für den Bereich **Einkauf** beziehungsweise ***VK-Rohwarenbearbeitung*** für den Bereich **Verkauf** in den Auswahllisten-Varianten ***Fibu-Übertrag Rohware Einkauf/Verkauf*** und ***Fibu-Übertrag Sammeldruck Einkauf/Verkauf***. Dabei regelt die Einstellung des Rohwareparameters [Sammelbuchungen bei Sammeldruck](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_122), in welcher Form per Sammeldruck zusammengefasste Belege an die Finanzbuchhaltung zu übergeben sind: Ist die Einstellung des Parametes ‚**Nein**‘, so werden alle Rohware-belege als Einzelbelege an die Finanzbuchhaltung übergeben, die betreffende Auswahllisten-Variante ***Fibu-Übertrag Sammeldruck*** ist dann nicht verfügbar. Bei der Einstellung ‚**Ja**‘ des Parameterwertes sind die per Sammeldruck verbundenen Belege nur in der jeweiligen Auswahllisten-Variante ***Fibu-Übertrag Sammeldruck*** berücksichtigt.

Voraussetzung für die Belegübergabe an die Finanzbuchhaltung ist immer, dass deren warenwirtschaftliche Verarbeitung durch den [Mandantenserver](../../../vorgangsabwicklung/verbuchung_im_warenwirtschaftssystem.md) abgeschlossen ist. Der Steuerparameter [Fibu-Übertragung auch ungedruckt](../../../firmenstamm/steuerparameter/fibu_uebertrag_warenwirtschaft/fibu_uebertragung_auch_ungedruckt_spa_149.md) legt die Voraussetzung für die Übertragung bezüglich des Drucks und der Archivierung fest. 

Grundsätzlich können nur Rohware-Belege der Rechnungsstufen (**Abschlag**, **Folgeabschlag**, **Finale**) gebucht werden, wenn diese den Bearbeitungsstatus ‚**abgerechnet**‘ haben und nicht bereits ein Folgebeleg erzeugt wurde, wie zum Beispiel ein Stornobeleg oder eine Finale zum Abschlag. Wird ein existierender Folgebeleg jedoch wieder storniert oder per Stornobeleg egalisiert, so kann der Fibu-Übertrag für den Ursprungsbeleg wieder erfolgen. In den Auswahllisten zum Fibuübertrag sind die nicht buchbaren Belege in der Spalte **Fib** mit dem Kennzeichen ‚**nn**‘ versehen.  
Um zu verhindern, dass Belege, die noch nicht an die Finanzbuchhaltung übertragen wurden, weiterverarbeitet werden können, gibt es den Rohwareparameter [nächste Stufe nur nach Fibuübertrag](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_113), der bei der Einstellung ‚**Ja**‘ die Erzeugung eines Belegs der nächsten Abrechnungsstufe verhindert, wenn der Fibuübertrag des Quellbelegs nicht erfolgt ist. Die Rohwareparameter [Stornobeleg nur nach Fibuübertrag](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_114) und [Sammel-Storno nur nach Fibuübertrag](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_145) erledigen dieses entsprechend bei der Erstellung von Stornobelegen. Die Generierung des jeweiligen Buchungstextes steuern die Rohwareparameter [Buchungstext bei Einzelbuchung](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_136) und [Buchungstext bei Sammelbuchung](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_135).  
Rohware-Finalabrechnungen, denen im Rahmen einer [jahresübergreifenden Abrechnung](../../weiterverarbeitung/jahresuebergreifende_abrechnung.md) im Erfassungsfeld **Finale nach Jahreswechsel** der Wert ‚**Ja ohne Fibu-Übertrag**‘ zugewiesen wurde, werden nicht an die Finanzbuchhaltung übergeben. Sie können aber unabhängig der Einstellung des Parameters [Stornobeleg nur nach Fibuübertrag](../../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_114) per Stornobelegerzeugung mit Erzeugung einer Kopie weiterverarbeitet werden.

<p class="siehe-auch">Siehe auch:</p>

- [Fibu-Übertrag von Rohware-Einzelbelegen](./fibu_uebertrag_von_rohware_einzelbelegen.md)
- [Fibu-Übertrag von Rohware-Sammeldruck-Belegen](./fibu_uebertrag_von_rohware_sammeldruck_belegen.md)
