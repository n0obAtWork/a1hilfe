# Informationen zur Verteilung von Gruppen-Zu-/Abschlägen

<!-- source: https://amic.de/hilfe/_vorggrpzuab_info.htm -->

Die Datenbankrelation **VorgGruppenZuAbInfo** enthält nach Abschluss eines Vorgangs Informationen über die Verteilung der Gruppen-Zu- und Abschläge inklusive Gruppen-Rabatte, Gruppen-Fachten und Gruppen-Verpackungssätzen. Sie dient der Informationsfindung, welche Beträge eines Gruppen-Zu-/Abschlags auf welche Warenpositionen mit welchem Steuerschlüssel in welcher Höhe verteilt wurden.

Die einzelnen Datenbankfelder der Relation sind:

| Feld | Beschreibung |
| --- | --- |
| V_Id | Die ID des Vorgangs.<br> |
| V_PosiZaehler_GrZuAb | Positionsnummer des zugehörigen Gruppen-Zu/Abschlagsatzes.<br> |
| V_PosiZaehler_Ware. | Positionsnummer des zugehörigen Warenpositionssatzes.<br> |
| V_GrZuAbInf_Typ | Der Typ des Zu-/Abschlags:<br> 1 - Rabatt<br>11 – individueller Rabatt<br> 2 – Zu-/Abschlag<br>12 – infividueller Zu-/Abschlag<br> 3 – Fracht<br>13 – individuelle Fracht<br> 4 – Verpackung<br> |
| V_GrZuAbInf_Kalk | 1: kalkulatorischer Zu-/Abschlag<br>Sonst: nicht kalkulatorisch<br> |
| V_GrZuABInf_Anteil | Der auf die Warenposition entfallende Nettobetrag.<br> |
| V_GrZuABInf_SAnteil l | Der auf die Warenposition entfallende Steuerbetrag.<br> |
| V_GrZuABInf_Bezug. | Die Bezugsgröße der Warenposition für diesen Zu-/Abschlag.<br> |
| V_GrZuABInf_WaehrAnteilert | Der auf die Warenposition entfallende Nettobetrag in Währung.<br> |
| V_GrZuABInf_WaehrSAnteil | Der auf die Warenposition entfallende Steuerbetrag in Währung.<br> |
| V_GrZuAbInf_WaehrBezug | Die Bezugsgröße der Warenposition für diesen Zu-/Abschlag in Währung.<br> |
| V_GrZuAbInf_StSchluessel | Der zugehörige Steuerschlüssel.<br> |
| V_GrZuAbInf_StKlasse | Die zugehörige Steuerklasse.<br> |
| V_GrZuAbInf_StSatz | Der zugehörige Steuersatz.<br> |

**Hinweis:** Für ältere Vorgänge, für die diese Einträge noch nicht erzeugt wurden, werden diese auch beim Öffnen des jeweiligen Vorgangs im *Ansehen-Modus* nachgetragen.
