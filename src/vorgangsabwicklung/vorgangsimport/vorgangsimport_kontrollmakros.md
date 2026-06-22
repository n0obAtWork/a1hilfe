# Vorgangsimport Kontrollmakros

<!-- source: https://amic.de/hilfe/_vorgimportctrlmakro.htm -->

Während des laufenden Vorgangsimports können an verschiedenen Stellen Kontrollmakros angeschlossen werden, die einen Eingriff in den laufenden Import ermöglichen.

Dazu kann ein Vorgangsimportkontrollmakro als Makro 2.0 [CSM] oder als Pascal Makro erstellt werden.

In der Vorgangsunterklassendefinition der zu importierenden Vorgangs(unter)klasse kann nun dieses Makro als „Vorgangsimport Kontrollmakro“ eingetragen werden.

Es gibt folgende Einsprungpunkte im C# Makro:

Vorgangs-Methoden:

vimp_Vorgang_vor_Neu

Wird vor der Erstellung eines neuen Vorgangs aufgerufen. Als Parameter wird hier nur die IVS_GUID aus der Tabelle ImportVorgStamm gegeben.

vimp_Vorgang_vor_Speichern

Wird direkt vor dem Speichern des Vorgangs aufgerufen. Als Parameter wird die ivs_guid aus der Tabelle Importvorgstamm und das Handle des instanziierten Vorgangs gegeben.

Zusätzlich wird angegeben, ob es sich um eine Neuanlage oder eine Änderung handelt.

vimp_Vorgang_nach_Speichern

Wird direkt vor dem Speichern des Vorgangs aufgerufen. Als Parameter wird die ivs_guid aus der Tabelle Importvorgstamm gegeben.

Positions-Methoden:

vimp_Position_vor_Neu

Wird direkt vor der Erzeugung einer Position aufgerufen. Als Parameter werden die IVS_GUID aus der Tabelle ImportVorgStamm und die IVP_GUID aus der Tabelle ImportVorgPosition sowie das Handle auf den instanziierten Vorgang und der Modus (Neuanlage/Änderung) angegeben.

vimp_Position_erstellt

Wird direkt nach der Erzeugung einer Position aufgerufen. Als Parameter werden die IVS_GUID aus der Tabelle ImportVorgStamm und die IVP_GUID aus der Tabelle ImportVorgPosition sowie das Handle auf den instanziierten Vorgang, das Handle der erzeugten (aber noch nicht eingefügten) Position und der Modus (Neuanlage/Änderung) angegeben.

vimp_Position_vor_Speichern

Wird nach der Erzeugung und Zuweisung von Eigenschaften und vor dem Speichern einer Position aufgerufen. Als Parameter werden die IVS_GUID aus der Tabelle ImportVorgStamm und die IVP_GUID aus der Tabelle ImportVorgPosition sowie das Handle auf den instanziierten Vorgang, das Handle der erzeugten (aber noch nicht eingefügten) Position und der Modus (Neuanlage/Änderung) angegeben.

vimp_Position_nach_Speichern

Wird nach dem Speichern einer Position aufgerufen. Als Parameter werden die IVS_GUID aus der Tabelle ImportVorgStamm und die IVP_GUID aus der Tabelle ImportVorgPosition sowie das Handle auf den instanziierten Vorgang, das Handle der eingefügten Position und der Modus (Neuanlage/Änderung) angegeben.

Pascal Makro:

Im Pascal Makro gibt es die gleichen Einsprungspunkte wie im C# Makro, diese müssen aber anders angesprochen werden. Die Einstiegspunkte werden mittels JVAR aus dem Vorgangsimport übergeben. Als Beispiel Pascal Makro dient das Makro „**VIMP_BeispielMakro**“. In diesem Makro sind die Übergabeparameter und der Inhalt der JAVRS erklärt.

LVS-Methoden:

Beim Vorgangsimport der LVS-Klasse (5150) werden andere Makroeinsprungpunkte genutzt.

Diese werden jeweils vor und nach der Ausführung des Imports aufgerufen. Als Parameter werden jeweils die Objekte ImportVorgStamm und zwei ImportVorgPositionen übergeben. Die zweite ImportVorgPosition ist jedoch ausschließlich im Fall „Umbuchen“ gesetzt.

| Methode | IVP1 | IVP2 | Unterklasse | Bemerkung |
| --- | --- | --- | --- | --- |
| Before_Import | Nein | Nein | \-- | Vor dem gesamten Import |
| After_Import | Nein | Nein | \-- | Nach dem gesamten Import |
| Before_Create | Ja | Nein | [10](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_10) | Nur wenn der Ladeträger generiert werden muss |
| After_Create | Ja | Nein | [10](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_10) | Nur wenn der Ladeträger generiert werden muss |
| Before_Lokalitaet | Ja | Nein | [10](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_10) | |
| After_ Lokalitaet | Ja | Nein | [10](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_10) | |
| Before_Beladen | Ja | Nein | [20](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_20) | |
| After_ Beladen | Ja | Nein | [20](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_20) | |
| Before_Umpacken | Ja | Nein | [30](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_30) | Hier gibt es zwei ImportVorgPositionLVS-Einträge (Quelle und Ziel) |
| After_Umpacken | Ja | Nein | [30](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_30) | Hier gibt es zwei ImportVorgPositionLVS-Einträge (Quelle und Ziel) |
| Before_Umbuchen | Ja | Ja | [40](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_40) | Zweite Position enthält den Zielartikel |
| After_Umbuchen | Ja | Ja | [40](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_40) | Zweite Position enthält den Zielartikel |
| Before_Fahrauftrag | Ja | Nein | [50](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_50) | |
| After_Fahrauftrag | Ja | Nein | [50](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_50) | |
| Before_planned_Inventur | Ja | Nein | [60](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_60) | |
| After_planned_Inventur | Ja | Nein | [60](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_60) | |
| Before_unplanned_Inventur | Ja | Nein | [61](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_61) | |
| After_unplanned_Inventur | Ja | Nein | [61](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_61) | |
| Before_Leeren | Ja | Nein | [90](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_90) | |
| After_Leeren | Ja | Nein | [90](./vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md#LVS_VUK_90) | |
