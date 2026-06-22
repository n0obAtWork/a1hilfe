# Druckbereich 81: Sammelformulareinrichtung QUER

<!-- source: https://amic.de/hilfe/druckbereich81sammelformularei.htm -->

Hauptmenü > Administration > Formulare/Abläufe > Formulare > Formulareinrichtung

Direktsprung **[FRM]**

Mittels geschickt parametrisierter spezieller Druckeinrichtungspositionen kann ein Rohware-Sammeldruckformular auch ‚quer‘ eingerichtet werden, d.h. die relevanten Informationen pro Einzelbeleg werden in einer Druckzeile dargestellt. Hierzu wird von den unterschiedlichen Einzelbeleg-Druckbereichen lediglich der Druckbereich 81 (Rohware-Sammeldr.-Einzelfussinfo) im Formular eingerichtet, der beim Druck des Formulars genau einmal pro Einzelbeleg ausgegeben wird.

Der Bezug zur jeweils heranzuziehenden Waren-/Qualitäts- oder Kostenposition wird durch die Angabe eines bestimmten Wertes im Feld ‚Parameter‘ im Detail-Bereich der Positionseinrichtung hergestellt. Dabei handelt es sich, je nach Gruppe der Druckpositionen, um die feste Referenznummer (Rohwarengruppendefinition, nicht änderbar, in Belegposition gespeichert), der variablen Ref2 ( Rohwarengruppendefinition, immer änderbar, nicht in Belegposition gespeichert) oder der ebenfalls variable Ref3 (Abrechnungsschemadefinition, immer änderbar, nicht in Belegposition gespeichert). 

Die dafür zur Verfügung stehenden Druckpositionen und die Bedeutung des jeweils unter Detail angegebenen Wertes für Parameter sind nachfolgend aufgelistet.

ID_MASSNUMMER

Parameter: Referenznummer der Waren-/Qualitäts- oder Kostenposition

Masseinheitsnummer der Waren-/Qualitäts- oder Kostenposition zur Referenznummer.

ID_MASSEINHEIT

Parameter: Referenznummer der Waren-/Qualitäts- oder Kostenposition

Masseinheit (Text) der Waren-/Qualitäts- oder Kostenposition zur Referenznummer.

ID_ARTIKELNUMMER

Parameter: Referenznummer der Waren- oder Kostenposition

Artikelnummer der Waren- oder Kostenposition zur Referenznummer.

ID_ARTISTAMMNUMMER

Parameter: Referenznummer der Waren- oder Kostenposition

Artikelstammnummer der Waren- oder Kostenposition zur Referenznummer.

ID_ME_BEZEICHNUNG

Parameter: Referenznummer der Waren- oder Kostenposition

Mengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer.

ID_ME_BEZEICHNUNGPREIS

Parameter: Referenznummer der Waren- oder Kostenposition

Preismengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer.

ID_ME_LANGTEXT

Parameter: Referenznummer der Waren- oder Kostenposition

Mengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer.

ID_ME_LANGTEXTPREIS

Parameter: Referenznummer der Waren- oder Kostenposition

Preismengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer.

ID_ME_TEXT

Parameter: Referenznummer der Waren- oder Kostenposition

Mengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer.

ID_ME_TEXTPREIS

Parameter: Referenznummer der Waren- oder Kostenposition

Preismengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer.

ID_ME_NUMMER

Parameter: Referenznummer der Waren- oder Kostenposition

Mengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer.

ID_ME_NUMMERPREIS

Parameter: Referenznummer der Waren- oder Kostenposition

Preismengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer.

ID_BRUTTOMENGE

Parameter: Referenznummer der Warenposition

Liefer-/Anfangsmenge der Warenposition zur Referenznummer.

**ID_NETTOMENGE**

Parameter: Referenznummer der Warenposition

Liefer-/Endmenge der Warenposition zur Referenznummer.

**ID_RWAPPREIS**

Parameter: Referenznummer der Warenposition

Anfangspreis der Warenposition zur Referenznummer.

**ID_RWEPPREIS**

Parameter: Referenznummer der Warenposition

Endpreis der Warenposition zur Referenznummer.

**ID_RWAAPREIS**

Parameter: Referenznummer der Warenposition

Anfangsabschlagpreis der Warenposition zur Referenznummer.

(Nicht bei prozentualer Abschlagermittlung!)

**ID_RWEAPREIS**

Parameter: Referenznummer der Warenposition

Endabschlagpreis der Warenposition zur Referenznummer.

(Nicht bei prozentualer Abschlagermittlung!)

**ID_RWAWPREIS**

Parameter: Referenznummer der Warenposition

Anfangsweltmarktpreis der Warenposition zur Referenznummer.

**ID_RWEWPREIS**

Parameter: Referenznummer der Warenposition

Endweltmarktpreis der Warenposition zur Referenznummer.

**ID_RWAMPREIS**

Parameter: Referenznummer der Warenposition

Anfangsmindestpreis der Warenposition zur Referenznummer.

**ID_RWEMPREIS**

Parameter: Referenznummer der Warenposition

Endmindestpreis der Warenposition zur Referenznummer.

**ID_RW_KTRMENGE**

Parameter: Referenznummer der Warenposition

Kontraktmenge der Warenposition zur Referenznummer.

**ID_RWKST_SATZ**

Parameter: Referenznummer der Kostenposition

Kostensatz der Kostenposition zur Referenznummer.

**ID_RWKST_MENGE**

Parameter: Referenznummer der Kostenposition

Kostenmenge der Kostenposition zur Referenznummer.

**ID_RWKST_BETRAG**

Parameter: Referenznummer der Kostenposition

Nettobetrag der Kostenposition zur Referenznummer.

**ID_ANALYSE_WERT**

Parameter: Referenznummer der Qualitätsposition

Analysewert der Qualitätsposition zur Referenznummer.

**ID_ANALYSE_WERT_KORREKTUR**

Parameter: Referenznummer der Qualitätsposition

Korrigierter Analysewert der Qualitätsposition zur Referenznummer.

**ID_UNTER_BASIS**

Parameter: Referenznummer der Qualitätsposition

Unterer Basiswert der Qualitätsposition zur Referenznummer.

**ID_OBER_BASIS**

Parameter: Referenznummer der Qualitätsposition

Oberer Basiswert der Qualitätsposition zur Referenznummer.

**ID_BEZUGSMENGE**

Parameter: Referenznummer der Qualitätsposition

Bezugsmenge der Qualitätsposition zur Referenznummer bei relativen Mengenzu-/abschlägen.

**ID_BEZUGSPREIS**

Parameter: Referenznummer der Qualitätsposition

Bezugspreis der Qualitätsposition zur Referenznummer bei relativen Preiszu-/abschlägen.

**ID_ZUABMENGE**

Parameter: Referenznummer der Qualitätsposition

Zuschlags-/Abzugsmenge der mengenrelevanten Qualitätsposition zur Referenznummer.

Ist der Parameterwert = 0: Summe aller Mengenzu-/abschläge der Qualitäten des Einzelbelegs.

**ID_ZUABPREIS**

Parameter: Referenznummer der Qualitätsposition

Zuschlags-/Abzugspreis der preisrelevanten Qualitätsposition zur Referenznummer.

Ist der Parameterwert = 0: Summe aller Preiszu-/abschläge der Qualitäten des Einzelbelegs.

**ID_ZUABSCHLAG**

Parameter: Referenznummer der Qualitätsposition

Zuschlags-/Abzugspreis oder -Menge der Qualitätsposition zur Referenznummer.

**ID_NEU_MENGE**

Parameter: Referenznummer der Qualitätsposition

Menge der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer.

**ID_NEU_PREIS**

Parameter: Referenznummer der Qualitätsposition

Preis der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer.

ID_RWREF2_MASSNUMMER

Parameter: Referenznummer 2 (Ref2) der Waren-/Qualitäts- oder Kostenposition

Masseinheitsnummer der Waren-/Qualitäts- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_MASSEINHEIT

Parameter: Referenznummer 2 (Ref2) der Waren-/Qualitäts- oder Kostenposition

Masseinheit (Text) der Waren-/Qualitäts- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ARTIKELNUMMER

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Artikelnummer der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ARTISTAMMNUMMER

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Artikelstammnummer der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_BEZEICHNUNG

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Mengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_BEZEICHNUNGPREIS

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Preismengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_LANGTEXT

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Mengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_LANGTEXTPREIS

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Preismengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_TEXT

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Mengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_TEXTPREIS

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Preismengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_NUMMER

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Mengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

ID_ RWREF2_ME_NUMMERPREIS

Parameter: Referenznummer 2 (Ref2) der Waren- oder Kostenposition

Preismengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_BRUTTOMENGE**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Liefer-/Anfangsmenge der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_NETTOMENGE**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Liefer-/Endmenge der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_APPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Anfangspreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_EPPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Endpreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_AAPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Anfangsabschlagpreis der Warenposition zur Referenznummer 2 (Ref2).

(Nicht bei prozentualer Abschlagermittlung!)

**ID_ RWREF2_EAPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Endabschlagpreis der Warenposition zur Referenznummer 2 (Ref2).

(Nicht bei prozentualer Abschlagermittlung!)

**ID_ RWREF2_AWPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Anfangsweltmarktpreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_EWPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Endweltmarktpreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_AMPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Anfangsmindestpreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_EMPREIS**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Endmindestpreis der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2__KTRMENGE**

Parameter: Referenznummer 2 (Ref2) der Warenposition

Kontraktmenge der Warenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_KST_SATZ**

Parameter: Referenznummer 2 (Ref2) der Kostenposition

Kostensatz der Kostenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_KST_MENGE**

Parameter: Referenznummer 2 (Ref2) der Kostenposition

Kostenmenge der Kostenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_KST_BETRAG**

Parameter: Referenznummer 2 (Ref2) der Kostenposition

Nettobetrag der Kostenposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_ANALYSE_WERT**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Analysewert der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_ANALYSE_WERT_KORREKTUR**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Korrigierter Analysewert der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_UNTER_BASIS**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Unterer Basiswert der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_OBER_BASIS**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Oberer Basiswert der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_BEZUGSMENGE**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Bezugsmenge der Qualitätsposition zur Referenznummer 2 (Ref2) bei relativen Mengenzu-/abschlägen.

**ID_ RWREF2_BEZUGSPREIS**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Bezugspreis der Qualitätsposition zur Referenznummer 2 (Ref2) bei relativen Preiszu-/abschlägen.

**ID_ RWREF2_ZUABMENGE**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Zuschlags-/Abzugsmenge der mengenrelevanten Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_ZUABPREIS**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Zuschlags-/Abzugspreis der preisrelevanten Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_ZUABSCHLAG**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Zuschlags-/Abzugspreis oder -Menge der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_NEU_MENGE**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Menge der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer 2 (Ref2).

**ID_ RWREF2_NEU_PREIS**

Parameter: Referenznummer 2 (Ref2) der Qualitätsposition

Preis der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer 2 (Ref2).

ID_RWREF3_MASSNUMMER

Parameter: Referenznummer 3 (Ref3) der Waren-/Qualitäts- oder Kostenposition

Masseinheitsnummer der Waren-/Qualitäts- oder Kostenposition zur Referenznummer 3 (Re32).

ID_ RWREF3_MASSEINHEIT

Parameter: Referenznummer 3 (Ref3) der Waren-/Qualitäts- oder Kostenposition

Masseinheit (Text) der Waren-/Qualitäts- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ARTIKELNUMMER

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Artikelnummer der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ARTISTAMMNUMMER

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Artikelstammnummer der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_BEZEICHNUNG

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Mengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_BEZEICHNUNGPREIS

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Preismengeneinheitsbezeichnung der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_LANGTEXT

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Mengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_LANGTEXTPREIS

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Preismengeneinheitslangtext der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_TEXT

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Mengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_TEXTPREIS

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Preismengeneinheitskurztext der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_NUMMER

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Mengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

ID_ RWREF3_ME_NUMMERPREIS

Parameter: Referenznummer 3 (Ref3) der Waren- oder Kostenposition

Preismengeneinheitsnummer der Waren- oder Kostenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_BRUTTOMENGE**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Liefer-/Anfangsmenge der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_NETTOMENGE**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Liefer-/Endmenge der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_APPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Anfangspreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_EPPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Endpreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_AAPREIS**

Parameter: Referenznummer 3 (Re32) der Warenposition

Anfangsabschlagpreis der Warenposition zur Referenznummer 3 (Ref3).

(Nicht bei prozentualer Abschlagermittlung!)

**ID_ RWREF3_EAPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Endabschlagpreis der Warenposition zur Referenznummer 3 (Ref3).

(Nicht bei prozentualer Abschlagermittlung!)

**ID_ RWREF3_AWPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Anfangsweltmarktpreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_EWPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Endweltmarktpreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_AMPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Anfangsmindestpreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_EMPREIS**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Endmindestpreis der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3__KTRMENGE**

Parameter: Referenznummer 3 (Ref3) der Warenposition

Kontraktmenge der Warenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_KST_SATZ**

Parameter: Referenznummer 3 (Ref3) der Kostenposition

Kostensatz der Kostenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_KST_MENGE**

Parameter: Referenznummer 3 (Ref3) der Kostenposition

Kostenmenge der Kostenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_KST_BETRAG**

Parameter: Referenznummer 3 (Ref3) der Kostenposition

Nettobetrag der Kostenposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_ANALYSE_WERT**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Analysewert der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_ANALYSE_WERT_KORREKTUR**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Korrigierter Analysewert der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_UNTER_BASIS**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Unterer Basiswert der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_OBER_BASIS**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Oberer Basiswert der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_BEZUGSMENGE**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Bezugsmenge der Qualitätsposition zur Referenznummer 3 (Ref3) bei relativen Mengenzu-/abschlägen.

**ID_ RWREF3_BEZUGSPREIS**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Bezugspreis der Qualitätsposition zur Referenznummer 3 (Ref3) bei relativen Preiszu-/abschlägen.

**ID_ RWREF3_ZUABMENGE**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Zuschlags-/Abzugsmenge der mengenrelevanten Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_ZUABPREIS**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Zuschlags-/Abzugspreis der preisrelevanten Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_** RWREF3_**ZUABSCHLAG**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Zuschlags-/Abzugspreis oder -Menge der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_ RWREF3_NEU_MENGE**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Menge der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer 3 (Ref3).

**ID_** RWREF3_**NEU_PREIS**

Parameter: Referenznummer 3 (Ref3) der Qualitätsposition

Preis der bezogenen Warenposition nach Anwendung (Abrechnung) der Qualitätsposition zur Referenznummer 3 (Ref3).
