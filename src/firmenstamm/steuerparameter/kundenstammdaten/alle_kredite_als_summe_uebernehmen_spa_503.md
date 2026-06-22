# Alle Kredite als Summe übernehmen?(SPA 503)

<!-- source: https://amic.de/hilfe/_SPA_503.htm -->

Im Kundenstamm gibt es die Funktion Kreditvergabe, in der mehrere Kredite vergeben werden. Die genehmigten Kredite schlagen sich wie folgt auf das Kreditlimit im Kundenstamm nieder:

| Typ | Wert |
| --- | --- |
| Nein | Nur der am frühesten genehmigte Kredit (Die erste gültige Zeile) bildet das Kreditlimit. Das Kreditlimit kann zusätzlich direkt im Kunden-/Lieferantenstamm erfasst werden. |
| Ja | Die Summe aller genehmigten Kredite bildet das Kreditlimit. Es werden nur die Kredite berücksichtigt, die nicht gelöscht und noch gültig sind. Dabei wird die Einstellung der Summierung des Kredittyps nicht berücksichtigt. In dieser Einstellung kann man das Kreditlimit nicht mehr im Kunden-/Lieferantenstamm pflegen, da nicht geklärt ist, wie sich dieses neue Limit auf die einzelnen genehmigten Kredite niederschlagen soll. Im Kundenstamm [KU] steht die Funktion „Kreditsummenübertrag“ zur Verfügung.  
Wenn dieser Wert ausgewählt wird, werden einmalig die manuell gepflegten Kreditlimite aus dem Kundenstamm in die Krediterfassung übernommen. |
| Mit individueller Datenbankprozedur | Es muss eine Prozedur mit dem Namen P_IndivKreditLimit zur Bestimmung des gesamten Limits existieren. Diese hat als Übergabeparameter die Kundid und muss einen Wert vom Typ Numeric zurückliefern. In dieser Ausprägung ist das Kreditlimit nicht im Kunden-/Lieferantenstamm änderbar.  
    
Zu beachten: Wenn diese Ausprägung ausgewählt wurde, wird die individuelle Prozedur auch in den folgenden Funktionen von AMIC verwendet:  
• amic_func_KundKredit  
• amic_func_Update_KundKredit  
• AMIC_Kreditlimit  
   
Es gibt zusätzlich den SPA 594: „Erm. Kreditlimit mit P_IndivKreditLimit“. Ist dieser auf Ja gesetzt, wird im Vorgang ebenfalls diese individuelle Datenbankprozedur zur Ermittlung des Kreditlimits verwendet. Möchten Sie beim SPA 503 ein abweichendes Verhalten gegenüber dem Vorgang mit SPA 594 implementieren, dann müssen Sie die alternative individuelle Datenbankprozedur verwenden. |
| Mit alternativer individueller Datenbankprozedur | Es muss eine Prozedur mit dem Namen P_IndivKreditLimitAlternativ zur Bestimmung des gesamten Limits existieren. Diese hat als Übergabeparameter die KundId und muss einen Wert vom Typ Numeric zurückliefern. In dieser Ausprägung ist das Kreditlimit nicht im Kunden-/Lieferantenstamm änderbar.  
Zu beachten: Wenn diese Ausprägung ausgewählt wurde, wird die individuelle Prozedur auch in den folgenden Funktionen von AMIC verwendet:  
• amic_func_KundKredit  
• amic_func_Update_KundKredit  
• AMIC_Kreditlimit  
   
Mit dieser Einstellung kann ein abweichendes Verhalten zwischen dem SPA 594, der im Vorgang ausgewertet wird, realisiert werden. |
| Ja, unter Berücksichtigung des Kredittyps | Es werden nur die Kredite summiert, die kein Löschkennzeichen haben und aktuell gültig sind. Außerdem muss der Kredittyp des jeweiligen Kredits im Feld „In Summierung berücksichtigen“ den Wert „Ja“ haben. Auch in dieser Ausprägung kann das Kreditlimit nicht mehr im Kunden-/Lieferantenstamm gepflegt werden. |
