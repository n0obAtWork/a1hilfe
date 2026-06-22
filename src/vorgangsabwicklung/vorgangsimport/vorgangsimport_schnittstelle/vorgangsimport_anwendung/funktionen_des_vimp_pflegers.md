# Funktionen des VIMP-Pflegers

<!-- source: https://amic.de/hilfe/_vimp_plausibilitaet.htm -->

In der Optionbox der Masken und der Auswahlliste existieren folgende Funktionen:

<details>
<summary>Funktionen des VIMP-Pflegers</summary>

| Funktion | Beschreibung |
| --- | --- |
| Status zurücksetzen. **(STRG+F5)** | Mit dieser Funktion wird auf der Auswahlliste der markierte Datensatz, wenn der Status 3 bis 7 oder 9 ist, auf 2 zurückgesetzt. Bei Problemen werden diese im Fehlerprotokoll angezeigt. |
| Datensatz als gelöscht markieren. **(STRG+F7)** | Mit dieser Funktion wird der Status auf 9 gesetzt. |
| Löschen **(F7)** | Öffnet die VIMP-Pfleger-Maske für den markierten Datensatz im Löschmodus. Es kann nur gelöscht werden, wenn der Status vorher auf 9 gesetzt wurde. |
| Standardvorgang erzeugen | Es wird aus den Daten des markierten Vorgangsimportes ein Vorgang erzeugt. Bei Problemen werden diese im Fehlerprotokoll angezeigt. |

Für die Funktion Status zurücksetzen gibt es einen Sonderfall. Vorgangsimport mit Vorgangsklasse 500(Ladeschein) und 1500(Eingangsladschein) werden mit der Funktion immer auf Status 5 gesetzt.

###### Standardvorgang erzeugen

Allgemein

Mit dieser Funktion kann ein Vorgang aus den Importieren Daten erzeugt werden. Es müssen die Positionen in der Auswahlliste markiert werden, aus denen dann ein Vorgang erzeugt werden soll und diese dürfen keine rotmarkierten Felder mehr in der Auswahlliste besitzen. 

Kann ein Vorgang bei der Vorgangserzeugung nicht angelegt werden, so wird der Status für den Stammsatz und allen dazugehörigen Positionen auf „Fehlerhaft“ gesetzt.

Kann eine Position bei der Vorgangserzeugung im Vorgang nicht angelegt werden, so wird der Status für diese Position auf Fehlerhaft gesetzt. Ansonsten wird nach erfolgreicher Erstellung des Vorgangs der Status für beide Kennzeichen auf „Erledigt“ gesetzt. Des Weiteren wird die Vorgangsnummer und die Vorgangsid in des Stammsatz geschrieben, so hat man den Überblick darüber, welcher Vorgang aus diesem Satz erzeugt worden ist.

Folgende Felder werden in den Vorgang übernommen.

Relation [ImportVorgStamm](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgstamm.htm)

In dieser Relation werden Kopfdaten des Vorgangs hinterlegt.

| Felderbeschreibung | ID’s Info | Datenbankfeld |
| --- | --- | --- |
| Kundenummer | Kundummer wird bei der Vorgangsanlegung gesetzt | KundNummer |
| Vorgangsnummer | NumNummer bei der Vorgangsanlegung. Die NumNummer wird nur gesetzt, wenn in diesem Feld ein Wert > 0 gestzt worden ist. Es erfolgt keine Prüfung ob die Vorgangsnummer in Kombination mit der Vorgangsklasse, Vorgansgunterklasse sowie der Jahrnummer schon vorhanden ist | V_NumNummer |
| Vorgangsklasse | Klasse wird bei der Vorgangsanlegung gesetzt | V_KlassNummer |
| Vorgangsunterklasse | UnterKlasse wird bei der Vorgangsanlegung gesetzt | V_UKlassNUmmer |

Relation [ImportVorgPosition](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgPosition.htm)

In dieser Relation werden Daten der Vorgangswarenposition gespeichert.

<table class="AMIC-Tabelle" style="WIDTH: 100%; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="100%" border="0"><tbody><tr><td style="WIDTH: 46.88%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="46%"><p class="AMIC-Ueberschrift-Tabelle" style="TEXT-ALIGN: center" align="center"><span style="COLOR: white">Felder</span></p></td><td style="WIDTH: 31.32%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="31%"><p class="AMIC-Ueberschrift-Tabelle" style="TEXT-ALIGN: center" align="center"><span style="COLOR: white">ID’s / Infos</span></p></td><td style="WIDTH: 21.8%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="21%"><p class="AMIC-Ueberschrift-Tabelle" style="TEXT-ALIGN: center" align="center"><span style="COLOR: white">Datenbankfeld</span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Artikel</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Artikelid bei der Positionsanlage</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ArtikelId</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Variante</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Variante bei der Positonsanlage</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ArtikelVariante</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Partie</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_PARTIENUMMER</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Partienummer</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Lagernummer</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_LAGERNUMMER</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Lagernummer</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Lagerplatz</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_LAGERPLATZ</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">LagerPlatzNummer</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ME_Nummer</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_ME_NUMMER</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ME</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ME_NummerPreis</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_ME_NUMMER_PREIS</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ME_Preis</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Preis</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_PREIS</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Preis</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Preiseinheit</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_PREISEINHEIT</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Preiseinheit</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Kontrakt</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_KONTRAKT_NUMMER</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">KonraktNummer</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Zusatzinfo</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_ZUSATZINFO</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ZusatzInfo</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Zusatinfo2</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ID_ZUSATZINFO2</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">ZusatzInfo2</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr style="HEIGHT: 160.55pt"><td style="BORDER-TOP: medium none; HEIGHT: 160.55pt; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Gebinde Informationen</span><span style="FONT-SIZE: 11pt"></span></p><table class="AMIC-Tabelle" style="WIDTH: 199.15pt; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="266" border="0"><tbody><tr><th style="WIDTH: 92.85pt; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="124"><span style="COLOR: white">Felder</span></th><th style="WIDTH: 106.3pt; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="142"><span style="COLOR: white">ID‘s</span></th></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 92.85pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="124"><span style="FONT-SIZE: 9pt; COLOR: black">Anzahl</span><span style="FONT-SIZE: 9pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 106.3pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="142"><span style="FONT-SIZE: 9pt; COLOR: black">ID_GEBINDE</span><span style="FONT-SIZE: 9pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 92.85pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="124"><span style="FONT-SIZE: 9pt; COLOR: black">Gebindefaktor1</span><span style="FONT-SIZE: 9pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 106.3pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="142"><span style="FONT-SIZE: 9pt; COLOR: black">ID_GEBINDEMASS_1</span><span style="FONT-SIZE: 9pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 92.85pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="124"><span style="FONT-SIZE: 9pt; COLOR: black">Gebindefaktor2</span><span style="FONT-SIZE: 9pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 106.3pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="142"><span style="FONT-SIZE: 9pt; COLOR: black">ID_GEBINDEMASS_2</span><span style="FONT-SIZE: 9pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 92.85pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="124"><span style="FONT-SIZE: 9pt; COLOR: black">Gebindefaktor3</span><span style="FONT-SIZE: 9pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 106.3pt; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="142"><span style="FONT-SIZE: 9pt; COLOR: black">ID_GEBINDEMASS_3</span><span style="FONT-SIZE: 9pt"></span></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 92.85pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="124"><span style="FONT-SIZE: 9pt; COLOR: black">Gebindefaktor4</span><span style="FONT-SIZE: 9pt"></span></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 106.3pt; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="142"><span style="FONT-SIZE: 9pt; COLOR: black">ID_GEBINDEMASS_4</span><span style="FONT-SIZE: 9pt"></span></td></tr></tbody></table></td><td style="BORDER-TOP: medium none; HEIGHT: 160.55pt; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Diese werden per Datenbankprozedur Prozedur „<a href="http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_FUNCTION_Gebinde_12_8bab589d.htm"><span style="FONT-SIZE: 10pt">Gebinde</span></a>“ bestimmt.</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; HEIGHT: 160.55pt; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">GebFaktor1</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">GebFaktor2</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">GebFaktor3</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">GebFaktor4</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 46.88%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="46%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Positionstexte Zeilenweise</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 31.32%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="31%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Per Funktion „Textneu“ und Zuordnung von ID_TEXTTEXT und der Zeilennummer.</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 21.8%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="21%"></td></tr></tbody></table>

Relation ImportVorgPositionLVS

Diese Relation beherbergt Informationen zu LVS-Ladeträgern, die zu dieser Position gehören.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | Uebernahmeid der zugehörigen Position der Relation ImportVorgPosition | UebernahmeID |
| SatzID | SatzId der zugehörigen Position der Relation ImportVorgPosition | SatzID |
| PositionID | Positions-ID der zugehörigen Position der Relation ImportVorgPosition | PositionID |
| PositionZaehler | Laufender Zähler der LVS-Informationen zu der gegebenen Position | PositionZaehler |
| LokalitaetsNr | Nummer des Ladeträgerstandorts | LokalitaetsNr |
| LadetraegerNr | Nummer des Ladeträgers | LadetraegerNr |
| LadeeinheitsNr | Nummer der Ladeeinheit | LadeeinheitsNr |
| LadeeinheitsPosition | Nummer der Ladeeinheitsposition auf dem Ladeträger | LadeeinheitsPosition |
| BewegungsId | | BewegungsId |
| LadetraegerExtNummer | Externe Nummer des Ladeträgers (z.B. eine NVE) | LadetraegerExtNummer |
| Menge | Menge auf dem Ladeträger | Menge |
| ME_Nummer | Mengeneinheit der Menge auf dem Ladeträger | ME_Nummer |
| IVP_GUID | Guid der dazugehörigen Position der Relation  
ImportVorgPosition | IVP_GUID |

Relation ImportVorgPositionPartie

In dieser Relation werden Informationen der Partie(n) einer Position abgelegt. Eine Partie, die hier eingetragen ist, jedoch im System noch nicht existiert, wird angelegt werden.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVP_GUID | Guid der dazugehörigen Position der Relation  
ImportVorgPosition | |
| Zaehler | Partiezähler | |
| PartieId | PartieId | |
| PartieNummer | Partienummer  
Ist die Partienummer gesetzt und die Partiebezeichnung wird mit der Kombination  
Partienummer und Partiebezeichnung nach der Partie gesucht. Wenn nur die Partienummer gesetzt worden ist wird nach der Partienummer gesucht  
Existiert mehr als eine Partie zu einer Partienummer wird immer die erste Partie gewählt | |
| PartieBezeichnung | Ist nur die Partiebezeichnung angegeben worden, und zu dieser Partie wurde keine aktive Partie gefunden, so wird eine neue Partie angelegt.  
Sind Partienummer und Partiebezeichnung angegeben, so wird die Partie nach dieser Kombination gesucht. | |
| PartieAbDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partieabdatum gesetzt | |
| PartieBisDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partiebisdatum gesetzt | |
| Menge | Menge der Partie | |
| ME | Mengeneinheit der Partie | |

Relation ImportVorgScannung

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | | |
| SatzID | | |
| PositionID | | |
| PositionZaehler | | |
| AiListe | | |
| SatzTyp | | |
| Scannung | | |
| MarkierIdent | | |

Relation ImportVorgStammAddOn

Aus dieser Relation werden Vorgangsaddon-Felder des Vorgangs befüllt. Der AddOnName muss dem des Feldes in der Tabelle VorgangAddOn entsprechen.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| AddonName | Name des AddOnFeldes | |
| AddonWert | Wert des AddOnFeldes | |

Relation ImportVorgStammUFLD

In dieser Relation werden die Setzungen von UFLD-Feldern für den Vorgang vorgenommen. Bitte beachten Sie, dass nur UFLD-Felder gesetzt werden können, die vom importierenden Bediener für den jeweiligen Vorgang gesetzt werden dürfen.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| UFLDId | ID des UFLD-Feldes | |
| UFLDWert | Wert des UFLD-Feldes | |

Relation ImportVorgTextPosition

In dieser Relation werden Textpositionen hinterlegt, die entweder vor oder nach einer Position in den Beleg eingefügt werden können.

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| UebernahmeID | Uebernahmeid der zugehörigen Position der Relation ImportVorgPosition | UebernahmeID |
| SatzID | SatzId der zugehörigen Position der Relation ImportVorgPosition | SatzID |
| PositionID | Positions-ID der zugehörigen Position der Relation ImportVorgPosition | PositionID |
| ZeilenZaehler | Laufende Nummer der Textposition | ZeilenZaehler |
| TextTyp | 0 = Positionstext | TextTyp |
| TextPosition | 0 = Vor der Position anzeigen  
1 = Nach der Position anzeigen | TextPosition |
| VorgText | Text der Positionszeile | VorgText |
| IVP_GUID | Guid der dazugehörigen Position der Relation  
ImportVorgPosition | IVP_GUID |

Relation ImportVorgStammZusatzTexte

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVS_GUID | GUID des korrespondierenden Eintrags in der Relation ImportVorgStamm | |
| TextTyp | Typ der Textzeile (Header/Line) | |
| LineNo | Zeilennummer | |
| TextZeile | Text dieser Zeile | |

Relation ImportVorgStammZuAb

Diese Relation beinhaltet einen Verweis auf die zu diesem Beleg anzuwendenden Zu-Abschläge/Frachten oder Rabatte. Da im Vorgangsimport keine Gruppen definiert werden können, wird dieser Zu/Abschlag stets auf die Gruppe 0 angewendet.

| *Felder* | *ID’s / Infos* | *Datenbankfeld* |
| --- | --- | --- |
| IVS_GUID | Guid der ImportVorgStamm | |
| IVZ_Zaehler | Laufender Zähler des Zu/Abschlags | |
| IVZ_GUID | Guid der Definition | |

Relation ImportVorgPositionZuAb

Diese Relation beinhaltet einen Verweis auf die zu dieser Position anzuwendenden Zu-Abschläge/Frachten oder Rabatte

| Felder | ID’s / Infos | Datenbankfeld |
| --- | --- | --- |
| IVP_GUID | Guid der ImportVorgPosition | |
| IVZ_Zaehler | Laufender Zähler des Zu/Abschlags | |
| IVZ_GUID | Guid der Definition | |

Relation [ImportVorgZuAbDef](../tabellen_des_vorgangsimports/importvorgzuabdef.md)

Diese Relation beinhaltet die Definition eines Zu/Abschlags, eines Rabattes oder einer Fracht.

Bitte beachten Sie, dass das Setzen einiger Werte abhängig von der verwendeten Berechnungsformel ist. Das Setzen eines nicht relevanten Wertes kann ggf. zum Abbruch und wird in jedem Fall jedoch zu einem Eintrag ins Fehlerprotokoll führen.

###### Positionen bearbeiten

Allgemein

In dieser Maske können Änderungen an einer vorhandenen Position durchgeführt werden.

Bei der Änderung des Kunden / Lieferanten oder der Vorgangsunterklasse wird die zu gerade bearbeitende Position als gelöscht markiert. Es wird ein neuer Stammsatz mit dem Kunden und der Unterklasse angelegt. Existiert für den Kunden/Lieferanten und der Unterklasse noch ein nicht weiterverarbeiteter Beleg, so wird diese Position an diesen Beleg angehängt.

Hierbei ist zu beachten, dass wenn mehrere Positionen markiert wurden und beim Blättern wird bei einem Datensatz der Lieferant oder die Unterklasse gewechselt, so wird beim Zurückblättern der geänderte Lieferant oder die geänderte Unterklasse nicht angezeigt, da die Kopfdaten der Aktuellen Position auf der Maske dargestellt werden.

Bei allen anderen Änderungen werden die Daten an der aktuellen Position abgeändert.

Partie-Neuanlage

Mit dieser Funktion kann eine neue Partie für den Artikel in dieser Position angelegt werden. Diese Partie wird dann Automatisch in das Partiefeld übernommen.

Positionstext

Auf dieser Registerkarte kann ein Positionstext eingetragen werden, welcher am Ende der Positionszeile eingefügt wird. Der Text darf nur 100 Zeichen pro Textzeile haben. Es sind aber mehrere Textzeilen möglich.

###### Plausbibilitätsprüfung des Vorgangsimportes

Um die Plausibilität von einem Vorgangsimport zu überprüfen, muss man auf die Variante Vorgangimport wechseln.

Auf jeder VIMP-Pfleger-Maske existiert die Funktion Plausibilität, die man mit F4 aufrufen kann.

Diese öffnet eine neue Maske und zeigt dort jeweils von der aktuellen Ebene abwärts Hinweise, Warnungen und Fehler im Import an.

Zuerst landet man auf die Maske ImportVorgStamm auf der alle Daten aus der Tabelle ImportVorgstamm enthalten sind.

Auf den Tabreiter Stammdaten und Zusätzliche Daten sind die für den Vorgangsimport relevanten Daten, die teilweise im Ändernmodus pflegbar sind. Diese lassen sich nur pflegen, wenn der Status des Vorgangsimportes 2 ist,

was auf der Maske als bereit(2) angezeigt wird.

Es sind nur die Felder Unterklasse und Kunde auf dem Tabreiter Stammdaten nachträglich veränderbar.

Auf den Tabreiter Fibu-Daten und Ungenutzte Felder befinden sich Felder, deren Inhalt nur informatorischen Inhalt besitzen und den Vorgangsimport nicht beeinflussen.

Auf dem Tabreiter Addon ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammAddon.

Auf dem Tabreiter UFLD ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammUFLD.

Auf dem Tabreiter Zusatztexte ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZusatztexte.

Von der Maske Importvorgstamm können drei weitere Maske geöffnet werden.

Durch das Drücken von F4 kann die Plausibilitätsmaske geöffnet werden, die Ereignisse, Warnungen und Fehler in einer Tabelle der öffnenden Maske anzeigt.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZuAb. Durch das Doppelklicken auf einen Eintrag in der Spalte Zähler öffnet sich die Maske ImportVorgZuAb.

Auf dem Tabreiter Position ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPosition. Durch das Doppelklicken auf einen Eintrag in der Spalte PositionId öffnet sich die Maske ImportVorgPosition.

Die Maske ImportVorgZuAb dient nur zu Informationszwecken und besitzt eine durch F4 aufrufbare Plausibilitätsprüfung. Wenn diese Maske über die ImportVorgStamm Maske aufgerufen wurde, gibt es kein Feld für PositionId.

Wenn der Typ Platzhalte Fracht ist, dann sind auch drei Felder zur Fracht einsehbar.

Auf der Maske ImportVorgPosition sind alle Daten aus der Tabelle ImportVorgPosition.

Auf dem Tabreiter Stammdaten und Zusätzliche Daten sind die für den Vorgangsimport relevanten Daten, die teilweise im Änderungsmodus pflegbar sind. Diese lassen sich nur pflegen, wenn der Status des Vorgangsimportes 2 ist,

was auf der Maske als bereit(2) angezeigt wird.

Es sind nur die Felder Artikelnummer, Lagernummer, Menge, Mengeneinheit, Preis, Preiseinheit, Mindeshaltbarkeitsdatum(MHD), Partienummer und Lagerplatznummer auf dem Tabreiter Stammdaten nachträglich veränderbar.

Auf dem Tabreiter Zusätzliche Daten sind die Felder Kontraktnummer, NVE, Zusatzinfo und Zusatzinfo 2 nachträglich veränderbar.

Auf dem Tabreiter Ungenutzte Felder befinden sich Felder, deren Inhalt nur informatorischen Inhalt besitzen und den Vorgangsimport nicht beeinflussen.

Auf dem Tabreiter Partie ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionPartie.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionZuAb.

Auf dem Tabreiter Addon ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionAddon.

Auf dem Tabreiter Textposition ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgTextPosition.

Von der Maske ImportVorgPosition können drei weitere Masken geöffnet werden.

Durch das Drücken von F4 kann die Plausibilitätsmaske geöffnet werden, die Ereignisse, Warnungen und Fehler in einer Tabelle der öffnenden Maske anzeigt.

Auf dem Tabreiter LVS ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgPositionLVS. Durch das Doppelklicken auf einen Eintrag in der Spalte Positionszähler öffnet sich die Mase ImportVorgPositionLVS.

Auf dem Tabreiter ZuAbschläge ist eine Tabelle mit dem Inhalt der Tabelle ImportVorgStammZuAb. Durch das Doppelklicken auf einen Eintrag in der Spalte Zähler öffnet sich die Maske ImportVorgZuAb.

Mit Hilfe des SPA 1131 kann man die Plausibilitätsprüfung auch vor dem Import mit useCS = 1 laufen lassen. Kommen dort Fehler auf, wird der Import auf fehlerhaft gestellt.

</details>
