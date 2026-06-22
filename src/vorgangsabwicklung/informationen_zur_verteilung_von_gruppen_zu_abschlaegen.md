# Informationen zur Verteilung von Gruppen-Zu-/Abschlägen

<!-- source: https://amic.de/hilfe/_vorggrpzuab_info.htm -->

Die Datenbankrelation **VorgGruppenZuAbInfo** enthält nach Abschluss eines Vorgangs Informationen über die Verteilung der Gruppen-Zu- und Abschläge inklusive Gruppen-Rabatte, Gruppen-Fachten und Gruppen-Verpackungssätzen. Sie dient der Informationsfindung, welche Beträge eines Gruppen-Zu-/Abschlags auf welche Warenpositionen mit welchem Steuerschlüssel in welcher Höhe verteilt wurden.

Die einzelnen Datenbankfelder der Relation sind:

| Feld | Beschreibung |
| --- | --- |
| V_Id | Die ID des Vorgangs.  
 |
| V_PosiZaehler_GrZuAb | Positionsnummer des zugehörigen Gruppen-Zu/Abschlagsatzes.  
 |
| V_PosiZaehler_Ware. | Positionsnummer des zugehörigen Warenpositionssatzes.  
 |
| V_GrZuAbInf_Typ | Der Typ des Zu-/Abschlags:  
 1 - Rabatt  
11 – individueller Rabatt  
 2 – Zu-/Abschlag  
12 – infividueller Zu-/Abschlag  
 3 – Fracht  
13 – individuelle Fracht  
 4 – Verpackung  
 |
| V_GrZuAbInf_Kalk | 1: kalkulatorischer Zu-/Abschlag  
Sonst: nicht kalkulatorisch  
 |
| V_GrZuABInf_Anteil | Der auf die Warenposition entfallende Nettobetrag.  
 |
| V_GrZuABInf_SAnteil l | Der auf die Warenposition entfallende Steuerbetrag.  
 |
| V_GrZuABInf_Bezug. | Die Bezugsgröße der Warenposition für diesen Zu-/Abschlag.  
 |
| V_GrZuABInf_WaehrAnteilert | Der auf die Warenposition entfallende Nettobetrag in Währung.  
 |
| V_GrZuABInf_WaehrSAnteil | Der auf die Warenposition entfallende Steuerbetrag in Währung.  
 |
| V_GrZuAbInf_WaehrBezug | Die Bezugsgröße der Warenposition für diesen Zu-/Abschlag in Währung.  
 |
| V_GrZuAbInf_StSchluessel | Der zugehörige Steuerschlüssel.  
 |
| V_GrZuAbInf_StKlasse | Die zugehörige Steuerklasse.  
 |
| V_GrZuAbInf_StSatz | Der zugehörige Steuersatz.  
 |

**Hinweis:** Für ältere Vorgänge, für die diese Einträge noch nicht erzeugt wurden, werden diese auch beim Öffnen des jeweiligen Vorgangs im *Ansehen-Modus* nachgetragen.
