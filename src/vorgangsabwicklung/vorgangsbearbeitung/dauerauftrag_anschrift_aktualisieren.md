# Dauerauftrag: Anschrift aktualisieren

<!-- source: https://amic.de/hilfe/_DauerauftragAnschrift.htm -->

Hauptmenü > Warenverkauf > Auftrag > Dauerauftrag bearbeiten

oder Direktsprung [DAB]

Steht der Steuerparameter [„Anschriften archivieren?“](../../firmenstamm/steuerparameter/kundenstammdaten/anschriften_archivieren_spa_574.md) auf „Ja“, so werden in Vorgängen die Anschriften zum Zeitpunkt der Erfassung festgehalten. Wird die Kundenhauptanschrift nach der Erfassung des Dauerauftrags geändert, so wird die Anschrift im Dauerauftrag nicht aktualisiert. Die alte Anschrift bleibt im Dauerauftrag bestehen. Das gleiche gilt auch für Rechnungen, die aus dem Dauerauftrag erstellt werden. Sie enthalten auch die alte Anschrift.

In dem Dialog „Dauerauftrag: Anschriften aktualisieren“ kann die Hauptanschrift einzelner oder mehrerer Daueraufträge aktualisiert werden. In der Datentabelle werden alle Daueraufträge aufgelistet, die über eine veraltete Hauptanschrift oder eine manuelle Vorgangsanschrift verfügen. In dem Feld „Aktualisieren?“ können die jeweiligen Daueraufträge ausgewählt und mithilfe der Funktion Anschriften aktualisieren aktualisiert werden.

*Hinweis: Die Anschrift eines Dauerauftrags kann auch während seiner Korrektur aktualisiert werden (siehe* [*Hauptanschrift aktualisieren*](../vorgangskopf/funktionen_vorgangserfassung_kopf.md#HauptAnschriftAktualisieren)*). Außerdem besteht für Daueraufträge die Möglichkeit, dass die Hauptanschriften automatisch aktualisiert werden (siehe Steuerparameter* [*Anschrift im Dauerauftrag automatisch aktualisieren*](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/anschrift_im_dauerauftrag_automatisch_aktualisieren_spa_1082.md)*).*

| Feld | Beschreibung |
| --- | --- |
| Kundennummer | Der Kunde des Dauerauftrags.  
 |
| Belegnummer | Die Belegnummer des Dauerauftrags.  
 |
| AdressId des Vorgangs | Die ID der Hauptanschrift, die im Dauerauftrag hinterlegt ist.  
 |
| Manuell | Kennzeichen, ob es sich bei der Hauptanschrift des Dauerauftrags um eine manuelle Vorgangsanschrift handelt.  
• „Ja“: Hauptanschrift des Vorgangs ist eine manuelle Vorgangsanschrift.  
• „Nein“: Hauptanschrift des Vorgangs ist keine manuelle Vorgangsanschrift.  
 |
| Name | Name und Vorname, die in der Hauptanschrift des Dauerauftrags angegeben sind.  
 |
| AdressId des Kunden | Die ID der Kundenhauptanschrift.  
 |
| Name | Name und Vorname, die in der Kundenhauptanschrift angegeben sind.  
 |
| Aktualisieren? | Hier kann mithilfe der F3\-Taste bestimmt werden, ob die Anschrift des jeweiligen Dauerauftrags beim Aufruf der Funktion Anschriften aktualisieren aktualisiert werden soll:  
• „Ja“: Die Anschrift des Dauerauftrags soll aktualisiert werden.  
• „Nein“: Die Anschrift des Dauerauftrags soll nicht geändert werden.  
Die Vorbelegung ist „Nein“.  
 |

| Funktion | Bedeutung |
| --- | --- |
| Anschriften aktualisieren F9 | Beim Aufruf der Funktion werden die Hauptanschriften aller Daueraufträge aktualisiert, die zuvor ausgewählt wurden. Dabei wird die Hauptanschrift des Dauerauftrags mit der Hauptanschrift des Kunden aktualisiert.  
Versandanschriften werden nicht aktualisiert. Die Anschrift des Rechnungsempfängers und die Anschrift des Zahlungspflichtigen werden nur dann aktualisiert, wenn sie mit der Hauptanschrift des Dauerauftrags übereinstimmen. Ansonsten werden sie nicht geändert.  
 |
| Alte Anschrift ansehen | Funktion zum Ansehen der Hauptanschrift, die im Dauerauftrag hinterlegt ist.  
 |
| Neue Anschrift ansehen | Funktion zum Ansehen der Kundenhauptanschrift.  
 |
| Alle auswählen | Wählt alle Daueraufträge in der Datentabelle zum Aktualisieren aus.  
 |
| Alle abwählen | Alle bereits ausgewählten Daueraufträge können mithilfe dieser Funktion abgewählt werden.  
 |
