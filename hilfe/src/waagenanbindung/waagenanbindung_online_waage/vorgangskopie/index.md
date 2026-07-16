# Vorgangskopie

<!-- source: https://amic.de/hilfe/_waage_vorgangskopie.htm -->

Mit der Funktionalität der Vorgangskopie wird gegen einen bestehenden Vorgang wie z.B.Auftrag oder Bestellung gewogen. Die Vorgangskopie kann an zwei verschiedenen Stellen Aktiviert werden.

1. Mit dem Einrichterparameter [Teildisposition/Vorgangskopie aus Auftrag](../../../firmenstamm/einrichterparameter/online_waage_epa_owaage.md) der Waage

2. In den Waage-[Prozessen](../prozess_einrichten/index.md) auf der [Registerkarte Vorgang](../prozess_einrichten/registerkarte_vorgang.md)

    
Folgende weitere [Einrichterparameter](../../../firmenstamm/einrichterparameter/online_waage_epa_owaage.md) sind für die Vorgangskopie entscheidend:

1. Itembox für Teildispo aus Auftrag

2. Abfrage, ob Auftrag storniert werden soll

3. Vorgangskopie: Prozentzahl der Menge bei deren Unterschreiten Auftrag Storno

4. Vorgang erzeugen: Belegdatum als Lieferdatum anstatt des Tagesdatums

Hat man sich im [Waagenprozess](../prozess_einrichten/index.md) auf der [Registerkarte Vorgang](../prozess_einrichten/registerkarte_vorgang.md) bei „Art der Vorgangserzeugung“ für „nicht aktiv / Einrichterparameter entscheidet“ entschieden und trotzdem Itemboxes angegeben, dann werden diese aktiv, wenn der Einrichterparameter [Teildisposition/Vorgangskopie aus Auftrag](../funktionen_auf_der_waagenmaske/einrichterparameter_in_der_waage.md) auf Vorgangskopie oder Teildispo steht. Die Itembox aus der Vorlage wird der Itembox aus dem Einrichterparameter [Itembox für Teildispo aus Auftrag](../funktionen_auf_der_waagenmaske/einrichterparameter_in_der_waage.md) dann vorgezogen.

Bei der Vorgangskopie wird z. B. ein Auftrag in einen Lieferschein kopiert. Die gewogene Menge und andere wichtige Daten (wie z.B. Datum, Versandart, LKW) aus der Waage werden in den Lieferschein übernommen.  
Welcher Vorgang erzeugt wird ist abhängig von der Einstellung der Vorgangsklasse auf der Waagenmaske.

Die gewogene Menge wird vom Auftrag abgebucht. Der Auftrag kann nach der Vorgangskopie storniert werden. Dieses lässt sich über die Einrichterparameter steuern.

Bei einer Vorgangskopie wird die durch den Einrichterparameter eingestellte IB auf dem Feld Kunde aktiv. Diese sollte eine V_Id z. B. für einen Auftrag zurückliefern, damit klar ist, aus welchem Vorgang eine Vorgangskopie erstellt werden soll. Wählt man z.B. einen Auftrag aus, dann werden u.a. Artikelnummer und Versandart aus dem Auftrag übernommen und die Felder auf der Maske deaktiviert.

Verwendet man die **F3**\-Auswahl auf dem Feld Kunde nicht, sondern gibt manuell eine Kundennummer und einen Artikel an (und wählt somit keinen Auftrag aus), dann wird keine Vorgangskopie erzeugt, sondern die normale Vorgangserzeugung angestoßen.

Folgende Dinge sind noch zu beachten:

Der Quellbeleg und der Zielbeleg sollten aus dem gleichen Bereich stammen („beide Einkauf“ oder „beide Verkauf“).  
Beispiel: Ist der Quellbeleg ein Auftrag, die Wiegung aber eine Wareneingangswiegung mit einer entsprechenden Vorgangsklasse, dann erhält man eine Fehlermeldung.

Der Quellbeleg sollte kein Rohwarebeleg sein.  
Beispiel: Hat man sich in dem [Waagenprozess](../prozess_einrichten/index.md) auf der [Registekarte F3-Auswahlen](../prozess_einrichten/registerkarte_f3_auswahlen.md) eine IB für den Einkauf eingerichtet, die die Auswahl von Einkaufslieferscheinen auf dem Feld Kunde ermöglicht, dann sollte der ausgewählte Einkaufslieferschein nicht die Unterklasse 9999 haben. Wählt man doch so einen Einkaufslieferschein aus, erhält man eine Fehlermeldung beim Versuch eine Vorgangskopie zu erstellen.

In der Waage gibt es die Möglichkeit, die Originalmenge eines Ursprungsauftrags auch nach mehrfachem Wiegen von diesem Auftrag jederzeit nachzuvollziehen. Dafür wird sich die Originalmenge des Auftrages nun im Feld „wabeworimenge“ (Originalmenge in den Positionen) gemerkt.

Beim Erzeugen einer Vorgangskopie wird die Belegnummer des Vorganges (Auftrages) in die Referenznummer des erzeugten Vorganges (Lieferscheines) gespeichert, wenn sie nicht bereits mit einem anderen Inhalt gefüllt ist.

<p class="siehe-auch">Siehe auch:</p>

- [F3 Auswahlen/ Itemboxen für die Vorgangskopie](./f3_auswahlen_itemboxen_fuer_die_vorgangskopie.md)
