# Fahraufträge

<!-- source: https://amic.de/hilfe/_fahrauftraege.htm -->

Hauptmenü > Stammdatenpflege > Lagerverwaltungssystem > Fahraufträge

Direktsprung **[LVSFA]**

Es ist möglich, programmatisch Fahraufträge für einen Ladeträger in die Tabelle „LVS_Ladetraeger“ einzutragen. Diese können dann die Notwendigkeit eines Transports eines Ladeträgers zu einem bestimmten Ort dokumentieren. Die Erledigung von Fahraufträgen geschieht durch eine Datenbankfunktion im Rahmen einer Ladeträgerbewegung.

Die Datenbankfunktion kann im [Steuerparameter 947](../steuerparameter/allgemeine_programmsteuerung/lvs_fahrauftrag_verwenden_spa_947.md) als Option für einen leeren Schlüssel angegeben werden. Eine Vorlage für diese Funktion finden Sie unter dem Namen „AMIC_DEMO_ErledigeLVSFahrauftrag“.

Als Eingangsparameter erhält die Funktion die Nummern des Ladeträgers und der Lokalität auf den der Ladeträger soeben bewegt wurde. Es ist nun an der Datenbankfunktion zu entscheiden, ob eine exakte Übereinstimmung oder ein anderes Regalfach des gleichen Regals o.ä. dem Anspruch genügt, den Fahrauftrag als beendet zu kennzeichnen.

In regelmäßigen Abständen sollten Sie per Event die Tabelle „LVS_Ladetraeger“ aufräumen lassen, um alte erledigte Aufträge nicht unnötig lange zu sammeln. Als Vorlage für eine Aufräumenprozedur können Sie „AMIC_DEMO_LVS_CLEANUPFAHRAUFTRAG“ ansehen.

Mit der Hilfe der Auswahliste können Sie sich die anstehenden Fahraufträge ansehen und ggf. auch manuell auf erledigt setzen.
