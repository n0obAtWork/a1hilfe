# Der Formular-Pfleger

<!-- source: https://amic.de/hilfe/_frmdialog.htm -->

| Formularstamm – Register Formular |
| --- |
| FELD | Beschreibung |
| Formularnummer | Nummer und Bezeichnung des Formulars |
| FormularBezeichn. | Formularbezeichnung |
| Formulartyp | F3 Auswahl - [Typenangabe des Formulars](../formular_importe/formulartypen.md)<br>Der Formulartyp legt fest, welche Bereiche für das neue Formular zur Verwendung frei geschaltet werden. |
| Vorlage-Formular | F3 Auswahl:<br>Möglichkeit ein schon vorhandenes Formular als Vorlage für ein neues zu verwenden |
| Drucktext | |
| Länge, Breite | Zahlenwertangaben für beide Werte.<br>Formularlänge und –breite lt. Einrichtung oder Druckerstamm benutzen.<br>Es wird nur herangezogen, wenn das folgende Feld auf „Formularlänge lt. Einrichtung“ gestellt wird.<br>Bei Einstellung auf „Formularlänge lt. Druckerstamm“ wird die Einstellung für die Seitenlänge des Druckers im Druckerstamm **[DRST]** verwendet. Dies gilt aber auch nur für einen ASCII Druck. Ist in den Druckereinstellungen das Feld Windows Druck auf Ja gestellt worden, dann sind die Seitenlängenangaben insofern unwichtig, als dass die Auflösung des zu erreichenden Druckers ermittelt wird und die Seitenlänge bestimmt. |
| Seitensteuerung | Angabe ob Seitenumbruch oder Endlos ohne Seitensteuerung innerhalb des Formulars<br>Seitenumbruch<br>**\-** Kopf und Fuß werden immer auf jeder Seite gedruckt<br>Endlos ohne Seitensteuerung<br>**\-** Kopf und Fuß werden je nur einmal gedruckt. Zwischen den beiden können mehrere Seiten mit vielen Informationen liegen.<br> <br> |
| [Archivierung](./archivierung_aktivieren_fuer_das_formular.md) | F3 Auswahl:<br>0 = nicht archivieren<br>1 = archivieren und Probleme immer melden<br>2 = archivieren und Probleme nur einmal melden<br>3 = archivieren und Nachricht im Fehler-/Ereignisprotokoll<br>4 = archivieren, jedoch nicht drucken!<br>7 = Mailversand ohne Druck (veraltet)<br>8 = Mailversand mit Druck (veraltet) |
| [Crw/Vbs](./crw_vbs.md) | F3 Auswahl:<br>0 = Keine Auswahl<br>1 = CRW auf Basis Formular mit CRW Drucker<br>2 = CRW auf Basis Formular mit DRZ/VRGD Drucker<br>3 = VBS Ausgabe<br>4 = AMIC Etikettendruck mit Druckerabfrage<br>5 = AMIC Etikettendruck auf dem im ETIDR-Formular eingerichteten Drucker<br>6 = AMIC Etikettendruck auf dem DRZ/VRGD Drucker<br>7 = CRW mit VDCRYSTA und CRW Drucker<br>8 = CRW mit VDCRYSTA und DRZ/VRGD Drucker |
| Makro-Unit | F3 Auswahl – Auswahl aus den in A.eins vorhandenen Makros<br>Für den Makrodruck können in den Bereichen weitere Einstellungen gemacht werden. |
| [DB.Fkt.Num Text](./db_fkt_num_text.md) | Namen der privaten Prozedur, zum Darstellen von Zahlen als Text. |
| Signierung | ?? |
| Archiv-Anhänge | F3 Auswahl zur Auswahl einer SQL-Prozedur für Archiv-Anhänge |
| Vorlauf Varianten | |
| Nachlauf Varianten | |
| Beleg für Vorschau | Für die Unterstützung einer Vorab-Sicht-Kontrolle eines Druckes mit „echten“ Vorgangsdaten lassen sich hier Vorgangsklasse und Belegnummer hinterlegen. Diese „Vorschau“ ist dann über die „Formularbereiche“ und „Einrichtung(F6)“ über den Knopf „F6=Vorschau“ erreichbar.<br><br>Hinweis: Bei Ersteingabe und auch bei evtl. späteren Änderungen muss das Formular zunächst gespeichert werden, bevor man die „Vorschau“ - wie oben geschildert - mit den aktualisierten Angaben bzgl. Vorgangsklasse und Belegnummer nutzen kann. Das betrifft auch weitere Rahmenparameter wie z.B. Einstellungen zum „Linker – und Oberer Rand“. |
| Vorgangsklasse | F3 Auswahl:<br>Vorgangsklassen in Klassennummer und Bezeichnung |
| BelegNummer | F3 Auswahl:<br>Belegnummer und Bezeichnung |
| [Test aktivieren](./test_aktivieren.md) | JA / NEIN |
| [Block Leerzeilen](./block_leerzeilen.md) | Angabe von Leerzeilen als Blocklänge |
| Text Leerzeilen | Angabe von Textleerzeilen |
| Reportname | Name der AMIC Etikettendruck oder CRW Datei<br>Muss passend zur Einstellung des Feldes [Crw/Vbs](./crw_vbs.md) eingegeben werden. |
| Name Scriptvorlage | Name des Skriptes<br>Einstellung des Feldes [Crw/Vbs](./crw_vbs.md) muss hierfür „VBS Ausgabe“ sein |
| Vorschau als PDF | JA / NEIN |
| Öffentlich Sichtbar | Ja / Nein<br>Kennzeichnet ein Formular, ob es nach dem Druck öffentlich sichtbar sein soll (dieses Kennzeichen kann von externen Programmen genutzt werden). |
| [Windows Druck Einstellungen](../font_tabellen_windowsdruck_f9/index.md) ­- Für Formulare, die mit Hilfe des Windows-Drucks gedruckt werden, können hier besondere Einstellungen vorgenommen werden, die für ASCII-Formulare nicht gelten.<br><br>Anmerkungen:<br>1) Das A.eins-Windows-Drucksystem und das A.eins-Archivierungssystem ermöglicht<br>a) auch mit Schwarz-Weiß-Druckern eine Farb-Archivierung!<br>b) einen „papierlosen“ Druck.<br>2) Achten Sie auf die durchgängige Verwendung aktuell gültige Windows-Druckertreiber. Es gibt Beispiele bei denen die von den Windows-System installierten Drucker-Treiber nicht die vermuteten Ergebnisse abliefern. Angesichts der Vielzahl von möglichen Systemen können wir leider keine technische Beratung leisten.<br>3) Bei Problemen bedenken Sie das über RDP-Verbindungen mit automatisch gemappten Drucker-Treibern u.U. mehrere Systeme involviert sein können.<br>4) Prüfen Sie mit der Option „Anzeige Druckbereich“ die vom Drucker-Treiber und dem A.eins-Windows-Drucksystem erreichbaren Ergebnisse. |
| Zeichensatz | F3 Auswahl öffnet die Windows Schriftartauswahl<br>Vorschaufenster der Auswahl darunter |
| Windows Schacht | F3 Auswahl:<br>0 = keine Schachtauswahl<br>1 = erster Schacht<br>2 = zweiter Schacht<br>3 = Schacht 1->Schacht 2<br>4 = Schacht 2->Schacht 1 |
| Windows Layout | F3 Auswahl:<br>0 = Hochformat<br>1 = Querformat |
| Randtreue | JA / NEIN<br>Alle physikalischen Windows-Drucker haben einen mehr oder weniger großen Randbereich in dem sie prinzipbedingt nicht drucken können. Softwaretechnisch wird dieser vom jeweiligen Drucker-Treiber bereitgestellt und vom A.eins-Windows-Drucksystem berücksichtigt.<br>Wird über etwaige Formulareinrichtungen in diese Bereiche versucht hinein zu drucken, wird der Drucker diese Anteile abschneiden. Da gibt es seitens der Drucker kein Pardon: Die Drucker können da nicht drucken!<br>Die A.eins-Archivierung ist zwar an die Druckvorlage gebunden, aber hat als reine Softwarelösung nicht derartige Beschränkungen. Ist also nicht gewollt das die etwaigen „Rand-Überdruckungen“ archiviert werden, lässt sich dies mit der Einstellung „Randtreue=JA“ erreichen. Dann wird 1:1 archiviert. In der Mehrheit der Fälle wohl erwünschtes Verhalten.<br> <br>Merkregel: Der Drucker schneidet immer ab, das Archiv auf Anforderung. |
| Anzeige Druckbereich | JA / NEIN<br>Aus der Wahl des Standard-Fontes ergibt sich zusammen mit dem druckbaren Bereich eine Anzahl von Zeilen und Spalten pro Druckerseite. Die Formulareinrichtungen nehmen auf diese resultierenden Größen Bezug durch die Angaben unter „Spalte“ und „Zeile“.<br><br><br>Mit der Einstellung „Anzeige Druckbereich“ wird nun dieses Zeilen-Spalter-Raster visualisiert und kann der Überprüfung bzw. auch der Hilfe bei der Einrichtung eines Druck-Formulars dienen. Als zusätzliche Unterstützung wird das visualisierte Zeilen-Spalten-Raster auch archiviert.<br><br>Außerdem ist erkenntlich das es am rechten Rand prinzipbedingt eine Teil-Spalte und am unteren Rand eine Teil-Zeile gibt. Diese sind gewissermaßen der Rasterung geschuldete Schnittreste.<br><br>Für den Sonderfall das „Dokumente“ gedruckt werden sind diese farblich umrandet. |
| [x-Skalierung](../font_tabellen_windowsdruck_f9/x_skalierung.md) | Horizontaler Skalierungsfaktor zur Spaltenpositionierung (für Spezialfälle) |
| [y-Skalierung](../font_tabellen_windowsdruck_f9/y_skalierung.md) | Vertikaler Skalierungsfaktor zur Zeilenpositionierung (für Spezialfälle) |
| [Font-Reduzierung](../font_tabellen_windowsdruck_f9/reduzierung_der_fontgroesse_um.md) | Reduzierung der Schriftgröße um angegebene Punkte bei generell „compressed“ zu druckenden Bereichen |
| Linker Rand mm | Wert in Millimetern |
| Oberer Rand mm | Wert in Millimetern |
| Bondruck-Spezial | Standard-Einstellung: **NEIN**<br> <br>In der Einstellung **NEIN** werden in der linken oberen und der rechten unteren Ecke nicht sichtbare Punkte gedruckt. Das geschieht deswegen, um im Falle eines Windows-Drucks eine seitengetreue Archivierung zu erhalten.<br>Bei Nicht-Seitendruckern ist das Verhalten u.U. nicht erwünscht, und es lässt sich mit der Einstellung **JA** die Setzung der Punkte unterdrücken. Der Druck wird dann wie erwartet ausgeführt, die Archivierung ist ohne Ränder. |
| Druck-Größe (in Prozent) | Standard ist 100 (Hinweis: 0 wird wie 100 behandelt), entspricht also 1:1.<br><br>Vornehmlich für Anpassungen/Skalierungen gedacht.<br><br>Beispiel: 71 bedeutet das alles in Breite und Höhe um den Faktor 0,71 verkleinert wird.<br><br>Diese Einstellung beeinflusst nur den Druck, nicht die Archivierung. |

| Funktionen - Register Formular | |
| --- | --- |
| Funktion | Beschreibung |
| | |

<p class="siehe-auch">Siehe auch:</p>

- [Archivierung aktivieren für das Formular](./archivierung_aktivieren_fuer_das_formular.md)
- [DB.Fkt.Num Text](./db_fkt_num_text.md)
- [Font Zuordnung zum Formular](./font_zuordnung_zum_formular.md)
- [Test aktivieren](./test_aktivieren.md)
- [Block Leerzeilen](./block_leerzeilen.md)
- [Crw/Vbs](./crw_vbs.md)
- [Beispiel zum Formulardruck im AMIC Etikettendruck](./beispiel_zum_formulardruck_im_amic_etikettendruck.md)
- [Betrag in Worten drucken](./betrag_in_worten_drucken.md)
- [Parameter der Datenbankfunktion](./parameter_der_datenbankfunktion.md)
- [Hinweise zur Entwicklung dieser Funktion](./hinweise_zur_entwicklung_dieser_funktion.md)
- [Spezielle deutsche Version](./spezielle_deutsche_version.md)
- [Beispiel Betrag in Worten im Formular](./beispiel_betrag_in_worten_im_formular.md)
