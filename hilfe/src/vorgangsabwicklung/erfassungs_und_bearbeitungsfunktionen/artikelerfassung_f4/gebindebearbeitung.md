# Gebindebearbeitung

<!-- source: https://amic.de/hilfe/_vorgang_gebindebearbeitung.htm -->

#### Änderung Gebindemengeneinheit

Das Ändern der Gebindemengeneinheit ist auf der Gebindemaske möglich. Dort steht das Feld „Gebinde“ zur Verfügung. Nachdem die Einheit geändert wurde, werden die Faktoren aus der entsprechenden [Faktorherkunft der Gebindeeinheit](../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/mengeneinheiten_mit_gebinde/index.md#Mengeneinheiten_mit_Gebinde_Kopfdaten) befüllt.

#### Preismengenbezugsübernahme

Als Voraussetzung für die Preismengenbezugsübernahme gilt, dass in der [Formularzuordnung](../../formularzuordnung/eingabe_eingabefelder.md) die „Gebinde Preismengenbezugsübernahme“ auf „Ja“ steht, in der [Gebindeeinheit](../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/mengeneinheiten_mit_gebinde/index.md#Mengeneinheiten_mit_Gebinde_Tabreiter_al) die Einheiten für die Faktoren und eine Mengeneinheit in „Mengeneinheit Preisbezug“ eingetragen sind. Des Weiteren erfolgt die Mengenübernahme nur, wenn es sich um ein Einzelgebinde handelt.

Die Mengenbezugsübernahme erfolgt immer, wenn sich das Gebinde oder die Preismengeneinheit ändert. Welche Menge übernommen wird, entscheidet die Preismengeneinheit. Es wird das „Zwischenergebnis“ übernommen, wo der Faktor die gleiche Mengeneinheit hat wie die Preismengeneinheit. Sollte keine Menge gefunden werden, wird die Menge nicht geändert.

*Beispiel:*

Es handelt sich um 5 Gebinde bestehend aus 10 „Kartons“ mal 7 „Schalen“ mal 0,750 „Kg“.

Die Preismengeneinheit ist „Kartons“. Als Menge würde in diesem Fall 50 „Kartons“ übernommen werden. Sollte die Preismengeneinheit auf „Schalen“ geändert werden, würde als Menge 350 „Schalen“ übernommen werden.
