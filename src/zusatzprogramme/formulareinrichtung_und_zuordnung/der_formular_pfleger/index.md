# Der Formular-Pfleger

<!-- source: https://amic.de/hilfe/_frmdialog.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Formularstamm – Register Formular</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>FELD</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formularnummer</p>
        </td>
        <td>
          <p>Nummer und Bezeichnung des Formulars</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FormularBezeichn.</p>
        </td>
        <td>
          <p>Formularbezeichnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formulartyp</p>
        </td>
        <td>
          <p>F3 Auswahl - <a href="../formular_importe/formulartypen.md">Typenangabe des Formulars</a></p>
          <p>Der Formulartyp legt fest, welche Bereiche für das neue Formular zur Verwendung frei geschaltet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorlage-Formular</p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>Möglichkeit ein schon vorhandenes Formular als Vorlage für ein neues zu verwenden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Drucktext</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Länge, Breite</p>
        </td>
        <td>
          <p>Zahlenwertangaben für beide Werte.</p>
          <p>Formularlänge und –breite lt. Einrichtung oder Druckerstamm benutzen.</p>
          <p>Es wird nur herangezogen, wenn das folgende Feld auf „Formularlänge lt. Einrichtung“ gestellt wird.<br>Bei Einstellung auf „Formularlänge lt. Druckerstamm“ wird die Einstellung für die Seitenlänge des Druckers im Druckerstamm <strong>[DRST]</strong> verwendet. Dies gilt aber auch nur für einen ASCII Druck. Ist in den Druckereinstellungen das Feld Windows Druck auf Ja gestellt worden, dann sind die Seitenlängenangaben insofern unwichtig, als dass die Auflösung des zu erreichenden Druckers&nbsp; ermittelt wird und die Seitenlänge bestimmt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Seitensteuerung</p>
        </td>
        <td>
          <p>Angabe ob Seitenumbruch oder Endlos ohne Seitensteuerung innerhalb des Formulars</p>
          <p><u>Seitenumbruch</u><u></u></p>
          <ul>
            <li><b>-&nbsp;&nbsp;&nbsp;&nbsp; </b>Kopf und Fuß werden immer auf jeder Seite gedruckt<br><u>Endlos ohne Seitensteuerung</u><u></u></li>
            <li><b>-&nbsp;&nbsp;&nbsp;&nbsp; </b>Kopf und Fuß werden je nur einmal gedruckt. Zwischen den beiden können mehrere Seiten mit vielen Informationen liegen.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./archivierung_aktivieren_fuer_das_formular.md">Archivierung</a></p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>0 = nicht archivieren</p>
          <p>1 = archivieren und Probleme immer melden</p>
          <p>2 = archivieren und Probleme nur einmal melden</p>
          <p>3 = archivieren und Nachricht im Fehler-/Ereignisprotokoll</p>
          <p>4 = archivieren, jedoch nicht drucken!</p>
          <p>7 = Mailversand ohne Druck (veraltet)</p>
          <p>8 = Mailversand mit Druck (veraltet)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./crw_vbs.md">Crw/Vbs</a></p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>0 = Keine Auswahl</p>
          <p>1 = CRW auf Basis Formular mit CRW Drucker</p>
          <p>2 = CRW auf Basis Formular mit DRZ/VRGD Drucker</p>
          <p>3 = VBS Ausgabe</p>
          <p>4 = AMIC Etikettendruck mit Druckerabfrage</p>
          <p>5 = AMIC Etikettendruck auf dem im ETIDR-Formular eingerichteten Drucker</p>
          <p>6 = AMIC Etikettendruck auf dem DRZ/VRGD Drucker</p>
          <p>7 = CRW mit VDCRYSTA und CRW Drucker</p>
          <p>8 = CRW mit VDCRYSTA und DRZ/VRGD Drucker</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Makro-Unit</p>
        </td>
        <td>
          <p>F3 Auswahl – Auswahl aus den in A.eins vorhandenen Makros</p>
          <p>Für den Makrodruck können in den Bereichen weitere Einstellungen gemacht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./db_fkt_num_text.md">DB.Fkt.Num Text</a></p>
        </td>
        <td>
          <p>Namen der privaten Prozedur, zum Darstellen von Zahlen als Text.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Signierung</p>
        </td>
        <td>
          <p>??</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archiv-Anhänge</p>
        </td>
        <td>
          <p>F3 Auswahl zur Auswahl einer SQL-Prozedur für Archiv-Anhänge</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorlauf Varianten</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Nachlauf Varianten</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Beleg für Vorschau</p>
        </td>
        <td>
          <p>Für die Unterstützung einer Vorab-Sicht-Kontrolle eines Druckes mit „echten“ Vorgangsdaten lassen sich hier Vorgangsklasse und Belegnummer hinterlegen. Diese „Vorschau“ ist dann über die „Formularbereiche“ und „Einrichtung(F6)“ über den Knopf „F6=Vorschau“ erreichbar.</p>
          <p><br>Hinweis: Bei Ersteingabe und auch bei evtl. späteren Änderungen muss das Formular zunächst gespeichert werden, bevor man die „Vorschau“ - wie oben geschildert - mit den aktualisierten Angaben bzgl. Vorgangsklasse und Belegnummer nutzen kann. Das betrifft auch weitere Rahmenparameter wie z.B. Einstellungen zum „Linker – und Oberer Rand“.<u></u></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorgangsklasse</p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>Vorgangsklassen in Klassennummer und Bezeichnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BelegNummer</p>
        </td>
        <td>
          <p>F3 Auswahl:</p>
          <p>Belegnummer und Bezeichnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./test_aktivieren.md">Test aktivieren</a></p>
        </td>
        <td>
          <p>JA / NEIN</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="./block_leerzeilen.md">Block Leerzeilen</a></p>
        </td>
        <td>
          <p>Angabe von Leerzeilen als Blocklänge</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Text Leerzeilen</p>
        </td>
        <td>
          <p>Angabe von Textleerzeilen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Reportname</p>
        </td>
        <td>
          <p>Name der AMIC Etikettendruck oder CRW Datei</p>
          <p>Muss passend zur Einstellung des Feldes <a href="./crw_vbs.md">Crw/Vbs</a> eingegeben werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name Scriptvorlage</p>
        </td>
        <td>
          <p>Name des Skriptes</p>
          <p>Einstellung des Feldes <a href="./crw_vbs.md">Crw/Vbs</a> muss hierfür „VBS Ausgabe“ sein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vorschau als PDF</p>
        </td>
        <td>
          <p>JA / NEIN</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Öffentlich Sichtbar</p>
        </td>
        <td>
          <p>Ja / Nein</p>
          <p>Kennzeichnet ein Formular, ob es nach dem Druck öffentlich sichtbar sein soll (dieses Kennzeichen kann von externen Programmen genutzt werden).</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p><a href="../font_tabellen_windowsdruck_f9/index.md">Windows Druck Einstellungen</a> ­- Für Formulare, die mit Hilfe des Windows-Drucks gedruckt werden, können hier besondere Einstellungen vorgenommen werden, die für ASCII-Formulare nicht gelten.<br><br>Anmerkungen:</p>
          <p>1)&nbsp;&nbsp; Das A.eins-Windows-Drucksystem und das A.eins-Archivierungssystem ermöglicht</p>
          <p>a)&nbsp;&nbsp; auch mit Schwarz-Weiß-Druckern eine Farb-Archivierung!</p>
          <p>b)&nbsp;&nbsp; einen „papierlosen“ Druck.</p>
          <p>2)&nbsp;&nbsp; Achten Sie auf die durchgängige Verwendung aktuell gültige Windows-Druckertreiber. Es gibt Beispiele bei denen die von den Windows-System installierten Drucker-Treiber nicht die vermuteten Ergebnisse abliefern. Angesichts der Vielzahl von möglichen Systemen können wir leider keine technische Beratung leisten.</p>
          <p>3)&nbsp;&nbsp; Bei Problemen bedenken Sie das über RDP-Verbindungen mit automatisch gemappten Drucker-Treibern u.U. mehrere Systeme involviert sein können.</p>
          <p>4)&nbsp;&nbsp; Prüfen Sie mit der Option „Anzeige Druckbereich“ die vom Drucker-Treiber und dem A.eins-Windows-Drucksystem erreichbaren Ergebnisse.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zeichensatz</p>
        </td>
        <td>
          <p>F3 Auswahl öffnet die Windows Schriftartauswahl</p>
          <p>Vorschaufenster der Auswahl darunter</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Windows Schacht</p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>0 = keine Schachtauswahl</p>
          <p>1 = erster Schacht</p>
          <p>2 = zweiter Schacht</p>
          <p>3 = Schacht 1-&gt;Schacht 2</p>
          <p>4 = Schacht 2-&gt;Schacht 1</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Windows Layout</p>
        </td>
        <td>
          <p><u>F3 Auswahl:</u><u></u></p>
          <p>0 = Hochformat</p>
          <p>1 = Querformat</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Randtreue</p>
        </td>
        <td>
          <p>JA / NEIN</p>
          <p>Alle physikalischen Windows-Drucker haben einen mehr oder weniger großen Randbereich in dem sie prinzipbedingt nicht drucken können. Softwaretechnisch wird dieser vom jeweiligen Drucker-Treiber bereitgestellt und vom A.eins-Windows-Drucksystem berücksichtigt.<br>Wird über etwaige Formulareinrichtungen in diese Bereiche versucht hinein zu drucken, wird der Drucker diese Anteile abschneiden. Da gibt es seitens der Drucker kein Pardon: Die Drucker können da nicht drucken!<br>Die A.eins-Archivierung ist zwar an die Druckvorlage gebunden, aber hat als reine Softwarelösung nicht derartige Beschränkungen. Ist also nicht gewollt das die etwaigen „Rand-Überdruckungen“ archiviert werden, lässt sich dies mit der Einstellung „Randtreue=JA“ erreichen. Dann wird 1:1 archiviert. In der Mehrheit der Fälle wohl erwünschtes Verhalten.</p>
          <p>Merkregel: Der Drucker schneidet immer ab, das Archiv auf Anforderung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anzeige Druckbereich</p>
        </td>
        <td>
          <p>JA / NEIN</p>
          <p>Aus der Wahl des Standard-Fontes ergibt sich zusammen mit dem druckbaren Bereich eine Anzahl von Zeilen und Spalten pro Druckerseite. Die Formulareinrichtungen nehmen auf diese resultierenden Größen Bezug durch die Angaben unter „Spalte“ und „Zeile“.<br><br></p>
          <p>Mit der Einstellung „Anzeige Druckbereich“ wird nun dieses Zeilen-Spalter-Raster visualisiert und kann der Überprüfung bzw. auch der Hilfe bei der Einrichtung eines Druck-Formulars dienen. Als zusätzliche Unterstützung wird das visualisierte Zeilen-Spalten-Raster auch archiviert.<br><br>Außerdem ist erkenntlich das es am rechten Rand prinzipbedingt eine Teil-Spalte und am unteren Rand eine Teil-Zeile gibt. Diese sind gewissermaßen der Rasterung geschuldete Schnittreste.<br><br>Für den Sonderfall das „Dokumente“ gedruckt werden sind diese farblich umrandet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../font_tabellen_windowsdruck_f9/x_skalierung.md">x-Skalierung</a></p>
        </td>
        <td>
          <p>Horizontaler Skalierungsfaktor zur Spaltenpositionierung (für Spezialfälle)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../font_tabellen_windowsdruck_f9/y_skalierung.md">y-Skalierung</a></p>
        </td>
        <td>
          <p>Vertikaler Skalierungsfaktor zur Zeilenpositionierung (für Spezialfälle)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><a href="../font_tabellen_windowsdruck_f9/reduzierung_der_fontgroesse_um.md">Font-Reduzierung</a></p>
        </td>
        <td>
          <p>Reduzierung der Schriftgröße um angegebene Punkte bei generell „compressed“ zu druckenden Bereichen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Linker Rand mm</p>
        </td>
        <td>
          <p>Wert in Millimetern</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Oberer Rand mm</p>
        </td>
        <td>
          <p>Wert in Millimetern</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bondruck-Spezial</p>
        </td>
        <td>
          <p>Standard-Einstellung: <b>NEIN</b></p>
          <p>In der Einstellung <b>NEIN</b> werden in der linken oberen und der rechten unteren Ecke nicht sichtbare Punkte gedruckt. Das geschieht deswegen, um im Falle eines Windows-Drucks eine seitengetreue Archivierung zu erhalten.</p>
          <p>Bei Nicht-Seitendruckern ist das Verhalten u.U. nicht erwünscht, und es lässt sich mit der Einstellung <b>JA</b> die Setzung der Punkte unterdrücken. Der Druck wird dann wie erwartet ausgeführt, die Archivierung ist ohne Ränder.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Druck-Größe (in Prozent)</p>
        </td>
        <td>
          <p>Standard ist 100 (Hinweis: 0 wird wie 100 behandelt), entspricht also 1:1.<br><br>Vornehmlich für Anpassungen/Skalierungen gedacht.<br><br>Beispiel: 71 bedeutet das alles in Breite und Höhe um den Faktor 0,71 verkleinert wird.<br><br>Diese Einstellung beeinflusst nur den Druck, nicht die Archivierung.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

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
