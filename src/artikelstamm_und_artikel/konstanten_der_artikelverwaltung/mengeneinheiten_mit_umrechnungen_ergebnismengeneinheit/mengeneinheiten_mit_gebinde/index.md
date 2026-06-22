# Mengeneinheiten mit Gebinde

<!-- source: https://amic.de/hilfe/_mengeneinheitenmitge.htm -->

Wenn Kartoffeln im 50 kg Sack verkauft werden und die Grundmengeneinheit "kg" ist, die Bestände in "kg" geführt werden, der Preis sich jedoch auf den "Sack" bezieht, dann muss die Gebindeumrechnung aktiviert werden. Hierzu werden in der Eingabemaske folgende Daten erfasst.

Die Maske ist in folgende Bereiche aufgeteilt:

[Kopfdaten](./index.md#Mengeneinheiten_mit_Gebinde_Kopfdaten)

[Tabreiter – Allgemein](./index.md#Mengeneinheiten_mit_Gebinde_Tabreiter_al)

Tabreiter – Zusatz

| Kopfdaten |
| --- |
| Nummer | Nummer der zu definierenden Mengeneinheit. |
| Kurztext | Kurztext der Mengeneinheit (z. B. für Ausdrucke), z.B. "Sack". |
| Langtext | Langtext der Mengeneinheit (z.B. für Ausdrucke) |
| Bezeichnung | Ausführliche Bezeichnung der Mengeneinheit, z. B. für Auswahllisten. In diesem Fall z.B. "Sack 50 kg". |
| Grundmengeneinheit | Nummer der Grundeinheit, auf die zurückgerechnet werden soll, z.B. „kg“. |
| Ergebniseinheit | Diejenige Mengeneinheit, in der das Ergebnis der Gebindeberechnung zurückgegeben wird.  
Beispiel:  
Es wird eine Palette mit Dosen à x Liter bearbeitet, dann ist das Ergebnis der Gebindeberechnung "Liter";  
Handelt es sich um eine Palette mit Säcken à x kg, so kommen kg dabei heraus.  
Das Ergebnis eines Volumengebindes sind dann z.B. Liter oder m³ sein |
| [Gebindetyp](./index.md) | Hier ist eine Angabe erforderlich, wenn ein Gebinde abgerechnet werden soll:  
1 lineares Gebinde (Anzahl)  
2 Gebinde 2. Stufe (Fläche)  
3 Gebinde 3. Stufe (Volumen)  
4 Addition (Gebi1 + Gebi2)  
5 Subtraktion (Geb1 - Geb2)  
6 Faktor1 \* Faktor2 / Faktor3  
7 Faktor1 \* Faktor2 \* Faktor3 \* Faktor4 |
| [Faktorherkunft](./gebindetypen.md) | Kennzeichnung, woher die Gebinde-Faktoren für die Berechnung kommen. Es ist hier ein dreistufiges System implementiert, es können bei den Artikeln, beim Artikelstamm aber auch beim Gebinde selbst die Faktoren hinterlegt werden. |

Tabreiter

Hier ist eine Auflistung der einzelnen Felder auf den Tabreitern der Maske.

| Allgemeine Informationen |
| --- |
| Gebinde auf Stufe 2 eingebbar | Dieses Kennzeichen steuert, ob es bei der (Gebinde-) Mengeneinheit zulässig sein soll, die Gebindeanzahl statt auf der obersten auf eingebbar der zweiten Stufe einzugeben.  
Beispiel:  
"Karton à 8 Kanister à 5 Liter"  
Eingabe auf Ebene 1: Anzahl Kartons  
Eingabe auf Ebene 2: Anzahl Kanister  
Eingabe auf unterster Ebene: Anzahl Liter |
| Effektive Menge eingebbar | Dieses Kennzeichen steuert, ob es bei der (Gebinde-) Mengeneinheit zulässig sein soll, die effektive Menge statt der Gebindeanzahl einzugeben  
Beispiel:  
Karton à 8 Kanister à 5 Liter  
Eingabe auf Ebene 1: Anzahl Kartons  
Eingabe auf Ebene 2: Anzahl Kanister  
Eingabe auf unterster Ebene: Anzahl Liter  
Die Eingabe der "Litermenge" ergibt dann als Ergebnis die Anzahl der Kartons. |
| Anbruch Gebinde Behandlung | F3 Auswahl  
• Normal  
• Anbruch  
• Abrunden  
• Aufrunden  
• Aufrunden St.2  
 |
| Anzahlermittlung angebrochene Gebinde | F3 Auswahl  
• Mitzählen  
• Nicht mitzählen |
| Preis unabhängig von Faktoren | F3 Auswahl  
• Ja  
• Nein  
JA = Preis pro Gebindeanzahl  
NEIN = Preis pro Ergebnismenge |
| Bei Mengeneingabe ersten Faktor errechnen | F3 Auswahl  
• Ja  
• Nein  
Bei der Angabe von „Ja“ wird nach der Ergebnismengeneingabe der Faktor 1 neu bestimmt. |
| Bei Gebinde-Typ 8 Menge mal Faktor 3 | Spezialeinstellung.  
F3 Auswahl  
• Ja  
• Nein  
Bei der Angabe von „Ja“ wird der Faktor 3 bei der Ergebnismengeneingabe ignoriert. |
| Zwischenergebnis auf Gebinde-Maske | F3 Auswahl  
• Nein  
• Anzeigen  
• Eingebbar  
Steuert die Anzeige / Eingabe von Zwischenergebnissen auf der Gebinde-Maske. |
| Rundungsstellen bei Umrechnung Menge/Gebinde | Anzahl der Nachkommastellen die beim Runden berücksichtigt werden sollen. |
| Ohne Autofenster | F3 Auswahl  
• Ja  
• Nein  
Steuert die Anzeige des Automatikfensters in der Vorgangsverarbeitung. |
| Mengeneinheit Preisbezug | Es kann vorkommen, dass bei der Fakturierung die im Gebinde berechnete Endmenge nicht mit der tatsächlichen Menge des Gebindes übereinstimmt. Soll nun der Preis auf die tatsächlich gemessene Menge bezogen werden, so kann zusätzlich zum Mengenfeld noch ein weiteres Feld auf der Warenerfassungsmaske geöffnet werden, welches dann für die Preisberechnung herangezogen wird. Wird an dieser Stelle nun eine Mengeneinheit bestimmt, die so einen Preisbezug festlegt, dann wird das zusätzliche Feld abgefragt. |

| Gebindefaktoren |
| --- |
| Gebinde Faktor 1- 4 | Hier sind die Gebinde Faktoren einzutragen |
| Einheit | Hier kann eine Mengeneinheit eingetragen werden. Diese steht im Zusammenhang mit der [Preismengenbezugsübernahme](../../../../vorgangsabwicklung/erfassungs_und_bearbeitungsfunktionen/artikelerfassung_f4/gebindebearbeitung.md#GebindebearbPreismengenbezug) bei der Erfassung eines Artikels mit Gebinde. |
| Gebinde Text | Kurzbezeichnung der Gebindeeinheit für Ausdrucke etc. in Standardsprache.  
Im Beispiel soll nach Auflösung des Gebindes der Text "kg" erscheinen; er ist hier einzutragen. |
| Gebinde änderbar | |
| Statistikkennzeichen | |
| [LVS Relevant](./faktorherkunft.md) | Dieses Kennzeichen steuert ob die Mengeneinheit LVS-relevant ist |
| LVS Default Lokalität | Hier wird die default LVS Lokalität eingetragen, wenn das Gebinde LVS-relevant ist, wird diese Lokalität standardmäßig verwendet |

 

<p class="siehe-auch">Siehe auch:</p>

- [Gebindetypen](./gebindetypen.md)
- [Faktorherkunft](./faktorherkunft.md)
- [LVS-relevant](./lvs_relevant.md)
