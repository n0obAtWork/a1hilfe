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

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><b>Masken-Funktionen</b></p>
        </td>
        <td>
          <p><b>Dialog „Fehlerprotokoll“ Registerkarte „Fehlerprotokoll“</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Speichern</p>
        </td>
        <td>
          <p>Dient zum Speichern von „Vermerk“ und „Erledigt“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Löschen</p>
        </td>
        <td>
          <p>Ermöglicht das direkte Löschen des Systemhinweises.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sende Systemhinweis</p>
        </td>
        <td>
          <p>Versendet Systemhinweis an die im <a href="../../firmenstamm/firmenkonstanten/mandantenstamm.md">Mandantstamm</a> eingerichteten Empfänger. (siehe dort Empfänger, Empfängerprozedur, Selektionsprozedur )</p>
          <p>In der Mail werden in tabellarischer Form folgende Zusatzinformationen beigefügt:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Anwendung</b></th>
                <th><b>Variante</b></th>
                <th><b>Maske</b></th>
                <th><b>Optionbox</b></th>
                <th><b>Hostname</b></th>
                <th><b>Windowsuser</b></th>
                <th><b>Makro</b></th>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>
