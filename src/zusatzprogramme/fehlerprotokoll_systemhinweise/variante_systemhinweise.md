# Variante Systemhinweise

<!-- source: https://amic.de/hilfe/variantesystemhinweise.htm -->

| **Felder** | |
| --- | --- |
| Wann | Zeitpunkt des Systemhinweises |
| Version | Die Aeins-Versionsnummer zum Zeitpunkt des Systemhinweises. |
| Bereich | Klassifizierung seitens des Programmes, in welchem Bereich der Hinweis erzeugt wurde. |
| Anzahl | Die Anzahl der unerledigten Vorkommen des Systemhinweises.<br>[Steuerparameter 868 - „Fehlerprotokolloptimierung aktiv?“](../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/fehlerprotokolloptimierung_aktiv_spa_868.md) steuert hier, ob bei erneutem Auftreten eines bis dato unerledigten Systemhinweises diese Anzahl erhöht oder ein „neuer“ Systemhinweis generiert werden soll. |
| Typ | Die auch „Fehlerstufe“ genannte Kategorisierung eines Eintrags.<br>30 = schwerer Fehler<br>20 = Fehler<br>10 = Warnung<br>1 = Testlauf<br>0 = Ereignis |
| Wer | Der Kurzname des A.eins-Bedieners |
| IP | Die IP-Adresse des A.eins-Client |
| Verm. | Über diesen Vermerk haben Sie die Möglichkeit einen Hinweis mit einer Anmerkung zu versehen. |
| Erl. | Hier kann ein Erledigungskennzeichen gesetzt werden. |
| Verm. | Erledigungsvermerk |
| Fehlernummer | Die meisten der internen Aeins-Systemhinweise sind mit Meldungen hinsichtlich der „Fehlernummer“ ausgestattet. Das erlaubt im Zweifel eine zusätzliche Bewertung der jeweiligen Umstände. |
| Was | Der Systemhinweistext |
| ID | Eindeutige Identifizierung des Systemhinweises. |

| **Funktionen** | |
| --- | --- |
| Pflege-Funktionen | Ändern, Löschen |
| Fehlerprotokoll Event | Pfleger zum Erzeugen von Events die das Fehlerprotokoll zyklisch löschen. |
| Fehlerprotokoll zurücksetzen | Löscht nach Rückfrage alle Fehlerprotokoll-Einträge |
| Zeitdifferenz messen | Bietet die Möglichkeit, die Zeitdifferenz zweier markierter Einträge in Millisekunden zu berechnen. |

| **Bereich/Profile** | |
| --- | --- |
| Tage zurück | Listet alle Systemhinweise innerhalb des Zeitraumes. |
| Bereich wie | Ermöglicht die Suche in den Bereichen.<br>**F3** ermöglicht die konkrete Auswahl und informiert über die Anzahl der jeweiligen Bereichs-Einträge. |
| Fehlerstufe von .. bis .. | Eingrenzung nach Typ der Systemhinweise |
| Erledigt | Erledigungskennzeichen Ja/Nein |
| Verursachender Bediener wie | Kurzname des A.eins-Bedieners |
| Bearbeitungsvermerk wie | Vermerk-Recherche |
| Fehlernummer | Ermöglicht die Eingrenzung nach Fehlernummern |
| Fehlertext wie | Systemhinweis-Recherche |
| Aeins-Version wie | Hier kann nach Hinweisen von konkreten A.eins-Versionen gesucht werden. |

| **Masken-Felder** | **Dialog „Fehlerprotokoll“ Registerkarte „Fehlerprotokoll“** |
| --- | --- |
| Wann | Zeitpunkt des Systemhinweises |
| Bereich | Klassifizierung seitens des Programmes in welchem Bereich der Hinweis erzeugt wurde. |
| Typ | Die auch „Fehlerstufe“ genannte Kategorisierung eines Eintrags. |
| Wer | Der Kurzname des A.eins-Bedieners |
| IP | Die IP-Adresse des A.eins-Client |
| Vermerk | Über diesen Vermerk haben Sie die Möglichkeit einen Hinweis mit einer Anmerkung zu versehen. |
| Erledigt | Hier können Sie ein Erledigungskennzeichen setzen. |
| Aeins-Version | Die Aeins-Versionsnummer zum Zeitpunkt des Systemhinweises. |
| „Was“ | Der Systemhinweistext<br> <br>Um das Kopieren von Systemhinweistexten in diesem Dialog zu unterstützen ist das Feld betretbar. Sie können in diesem Feld nichts eingeben. |
| „Wo“ | Die Programm-Umgebung die zum Zeitpunkt des Auftretens des Systemhinweises. Da u.U. sehr viele Informationen angezeigt werden können, sind folgende Abkürzungen vereinbart:<br>A = Anwendung<br>V = Variante<br>B = Besitzer der Variante (0=AMIC, 1=Privat)<br>M = Maske<br>O = Optionbox<br>F = auslösende Funktion (Controlstring)<br>FI = Formularid<br>H = Hostname ( Name des Computers)<br>W = Windows-Username des Aeins-Bedieners<br>S = Scriptname des beteiligten Makros |

<table class="AMIC-Tabelle" style="WIDTH: 100%; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="100%" border="0"><tbody><tr><td style="WIDTH: 23%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="23%"><p class="AMIC-Textkoerper" style="TEXT-ALIGN: center" align="center"><b><span style="FONT-SIZE: 11pt; COLOR: white">Masken-Funktionen</span></b></p></td><td style="WIDTH: 77%; BACKGROUND: #005d5b; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" width="77%"><p class="AMIC-Textkoerper" style="TEXT-ALIGN: center" align="center"><b><span style="FONT-SIZE: 11pt; COLOR: white">Dialog „Fehlerprotokoll“ Registerkarte „Fehlerprotokoll“</span></b><span style="FONT-SIZE: 11pt; COLOR: white"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="23%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Speichern</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 77%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="77%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Dient zum Speichern von „Vermerk“ und „Erledigt“.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="23%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Löschen</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 77%; BACKGROUND: #eff7f7; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="77%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Ermöglicht das direkte Löschen des Systemhinweises.</span><span style="FONT-SIZE: 11pt"></span></p></td></tr><tr><td style="BORDER-TOP: medium none; BORDER-RIGHT: white 1.5pt solid; WIDTH: 23%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="23%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Sende Systemhinweis</span><span style="FONT-SIZE: 11pt"></span></p></td><td style="BORDER-TOP: medium none; BORDER-RIGHT: medium none; WIDTH: 77%; BACKGROUND: #bad9d9; BORDER-BOTTOM: medium none; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; BORDER-LEFT: medium none; PADDING-RIGHT: 5.4pt" valign="top" width="77%"><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">Versendet Systemhinweis an die im <a class="topic-link" href="../../firmenstamm/firmenkonstanten/mandantenstamm.md">Mandantstamm</a> eingerichteten Empfänger. (siehe dort Empfänger, Empfängerprozedur, Selektionsprozedur )</span><span style="FONT-SIZE: 11pt"></span></p><p class="AMIC-Textkoerper"><span style="FONT-SIZE: 11pt; COLOR: black">In der Mail werden in tabellarischer Form folgende Zusatzinformationen beigefügt:</span><span style="FONT-SIZE: 11pt"></span></p><table class="MsoNormalTable" cellpadding="0" border="1"><tbody><tr><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Anwendung</span></b><b><span style="FONT-SIZE: 8pt; FONT-FAMILY: &quot;Times New Roman&quot;,serif"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Variante</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Maske</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Optionbox</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Hostname</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Windowsuser</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th><th style="BACKGROUND: #454545; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"><b><span style="FONT-SIZE: 8pt; COLOR: white">Makro</span></b><b><span style="FONT-SIZE: 8pt"></span></b></th></tr><tr><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td><td style="BACKGROUND: #d4f4a0; PADDING-BOTTOM: 3.75pt; PADDING-TOP: 3.75pt; PADDING-LEFT: 3.75pt; PADDING-RIGHT: 3.75pt"></td></tr></tbody></table></td></tr></tbody></table>
