# Versandprofilstamm

<!-- source: https://amic.de/hilfe/_versandprofilstamm.htm -->

Administration > Firmenkonstanten > Versandprofilstamm

Direktsprung **[VPST]**

Der Versandprofilstamm dient zum Hinterlegen von Profilen für den Versand von zum Beispiel E-Mail oder Fax zu verschiedensten Zwecken in A.eins. Das Profil, welches im Mandantenstamm hinterlegt wurde, wird hier mit grüner Farbe markiert und kann an dieser Stelle auch nicht einfach gelöscht werden.

Im „Startmenü“ die Anwendung „Dienste“ aufrufen. Dort den A.eins.Mailservice selektieren und „Eigenschaften“ aufrufen. Startyp auf „Automatisch“ festlegen und den Dienst starten.

### Eingabefelder

| Feld | Beschreibung |
| --- | --- |
| Profil-Bezeichnung | Name des Profils |
| Typ | Dieser [Typ](./versandprofilstamm.md#Versandprofiltypen) gibt an, für welche Verwendung dieser Eintrag zuständig ist |
| Standard | Kennzeichnet den Versandprofilstammeintrag als Standard |
| Standard dieses Typs | Kennzeichnet diesen Eintrag ggf. als Standard des o.a. Typs |

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Sendeeinstellungen</strong></p>
          <p>Die Sendeeinstellungen enthalten die Informationen, welche zum Versenden der Daten erforderlich sind. Die mit einem <b>*</b> versehenen Felder sind hierbei die Angaben, welche mindestens zur Verfügung stehen müssen!</p>
          <p>Registerkarten und Felder passen sich je nach Typ-Auswahl den entsprechenden Gegebenheiten an (E-Mail oder Fax).</p>
          <p>Über den Knopf „Sendeeinstellungen testen“ wird versucht eine Verbindung zum SMTP-Server aufzubauen, je nach Status wird eine Meldung ausgegeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung *</p>
        </td>
        <td>
          <p>Bezeichnung des Versandprofilstamms</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versandart *</p>
        </td>
        <td>
          <p><a href="./versandprofilstamm.md#Versandart">Versandart</a> setzt die Technologie, mit der E-Mails versendet werden können.</p>
          <p><b>Bitte verwenden Sie zur Neueinrichtung ausschließlich die Option 7 – Vermailung ! ! !</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versende-Server *</p>
        </td>
        <td>
          <p>Der Name oder die IP-Adresse des verwendeten SMTP-Servers (smtp.gmail.com oder 74.125.136.108)</p>
          <p><b>Wichtig</b>: Wenn Microsoft Graph zur Versendung verwendet wird, müssen in die Felder Benutzername und Absender die bei Microsoft Graph eingerichtete Mailadresse eingetragen werden. Außerdem müssen die Felder Client Id, Client Secret und Tenant Id mit den, bei Microsoft Identity Plattform hinterlegten Werten belegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SMTP Port *</p>
        </td>
        <td>
          <p>Port der Verbindung</p>
          <p>Über F3 wird ein Anwenderformat aufgerufen, welches im Auslieferzustand folgende Port vorgibt:</p>
          <p>1.&nbsp;&nbsp; 25 = Standard SMTP-Port (ohne Authentifizierung)</p>
          <p>2.&nbsp;&nbsp; 465 = SSL (wird nur bei Versandart Vermailung unterstützt)</p>
          <p>3.&nbsp;&nbsp; 587 = TLS (mit Authentifizierung)</p>
          <p>In dem Anwenderformat „af_SMTPPort“ können auch weitere Ports eingerichtet werden. Damit ist man in der Lage auch eigene Einrichtungen bedienen zu können.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Benutzername</p>
        </td>
        <td>
          <p>Wird zusammen mit dem Kennwort, für die Authentifizierung benötigt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kennwort</p>
        </td>
        <td>
          <p>Wird, zusammen mit dem Benutzernamen, für die Authentifizierung benötigt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zertifikat</p>
        </td>
        <td>
          <p>Zertifikat des Servers.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Authentifizierung</p>
        </td>
        <td>
          <ul>
            <li>Username/Passwort (sofern gegeben)</li>
            <li>SASLPlain</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>SSL.Verhandlung</p>
        </td>
        <td>
          <ul>
            <li>None – keine Verschlüsselung</li>
            <li>Implicit (SSL) Port 465</li>
            <li>Explicit (STARTTLS) Port 587</li>
            <li>Automatic Hier wird die Verschlüsselung beim verbindungsaufbau ausgehandelt.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Absender *</p>
        </td>
        <td>
          <p>Wird zwingend benötigt, um eine Verbindung zu einem SMTP-Server herzustellen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Absender-Alias</p>
        </td>
        <td>
          <p>Der Alias zu der im Feld „Absender“ stehenden E-Mail Adresse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Antwortadresse</p>
        </td>
        <td>
          <p>Vom Absender abweichende Adresse, an die die Antworten auf diese E-Mail gehen sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Provider</p>
        </td>
        <td>
          <p>Hier kann der Provider für den Fax-Versand hinterlegt werden. Das Feld ist aktiv, wenn als Typ „<b>Fax-Server Versand</b>“ ausgewählt wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verzeichnis für Fax</p>
        </td>
        <td>
          <p>Hier kann ein Pfad auch via F3 Auswahl angegeben werden, in dem die zu versendenden Dateien zum Fax-Versand abgelegt werden sollen. Das Feld ist aktiv, wenn als Typ „<b>Fax Versand per Verzeichnis</b>“ ausgewählt wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verzeichnis des Mail-Clients</p>
        </td>
        <td>
          <p>Pfadangabe zum Verzeichnis des .Net-Mail-Clients „AeinsSendMail.exe“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Odbc DSN</p>
        </td>
        <td>
          <p>Nur in veralteten Versandarten wie .NetMail</p>
          <p>Verbindungsbezeichnung von Odbc, welche der Mail-Client für die Herstellung der Datenbankverbindung verwenden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Client Id</p>
        </td>
        <td>
          <p>Ermöglicht die eindeutige Identifizierung der Anwendung in Microsoft Identity Platform. Sie wird bei der Überprüfung der von Identity Platform empfangenen Sicherheitstoken herangezogen.</p>
          <p>Weitere Informationen findet man unter: <a href="https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app">https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Client Secret</p>
        </td>
        <td>
          <p>Bei einem geheimen Clientschlüssel handelt es sich um einen Zeichenfolgenwert, der anstelle eines Zertifikats von Ihrer App für die Identifizierung verwendet werden kann.</p>
          <p>Weitere Informationen findet man unter: <a href="https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app">https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tenant Id</p>
        </td>
        <td>
          <p>Ein Mandant stellt eine Organisation dar. Hierbei handelt es sich um eine dedizierte Instanz von Azure&nbsp;AD, die eine Organisation oder ein App-Entwickler zu Beginn einer Beziehung mit Microsoft erhält. Diese Beziehung kann beispielsweise mit der Registrierung für Azure, Microsoft Intune oder Microsoft&nbsp;365 beginnen.</p>
          <p>Weitere Informationen findet man unter: <a href="https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant">https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant</a></p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>E-Mail Einstellungen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Empfänger Cc</p>
        </td>
        <td>
          <p>Empfänger einer sichtbaren Kopie der Mails, die mit diesem Profil versendet werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Empfänger Bcc</p>
        </td>
        <td>
          <p>Empfänger einer unsichtbaren Kopie der Mails, die mit diesem Profil versendet werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Testmail</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Empfänger</p>
        </td>
        <td>
          <p>Empfänger der Testmail</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Betreff</p>
        </td>
        <td>
          <p>Hier kann ein Betreff für die Testmail angegeben werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Text:</p>
        </td>
        <td>
          <p>Hier muss ein kleiner Text als E-Mail Inhalt für die Testmail angegeben werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anlage aus Archiv:</p>
        </td>
        <td>
          <p>Über F3 lassen sich hier Anhänge folgender Dateitypen auswählen:</p>
          <ul>
            <li>PDF</li>
            <li>BMP</li>
            <li>JPEG</li>
            <li>PNG</li>
            <li>GIF<br>Sollte keine passende Datei angezeigt werden, kann hier z. Bsp. über Drag&amp;Drop ein entsprechendes Format eingefügt werden.<br>Nachdem die Datei im Archiv ausgewählt wurde (Zeile markieren) werden die benötigten Daten mit ESC vom Versandprofilstammpfleger aufgenommen. Im Feld erscheint dann die Belegreferenz der ausgewählten Datei.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sende Test-E-Mail</p>
        </td>
        <td>
          <p>Sendet die Testmail an den angegebenen Empfänger und meldet das Ergebnis dem Benutzer. Ist der Versand nicht erfolgreich, so gibt es hier einen Hinweis, sich im Fehlerprotokoll über das Ereignis zu informieren</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Versandart</strong></p>
          <p>Versandarten stellen Technologien zum Aufbau der Verbindung zum Mailserver dar. In unterschiedlichen Umgebungen sind u.U. unterschiedliche Technologien notwendig.</p>
          <p><b>Wir empfehlen ausschließlich die Option 7 – Vermailung zu verwenden</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1 - Datenbankversand</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
          <p>Versand über die üblichen SQL-Prozeduren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2 - Outlook-Versand</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
          <p>Versand mit Outlook-Anbindung durch „smtp_outlook.sql“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3 - AeinsSendMail</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
          <p>Versand über den .Net Mail-Client AeinsSendMail.exe</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4 - SPA999 Qutlook ohne SPA-Setzung</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>5 - Per SPA999 .Net ohne SPA-Setzung</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>6 - SPA überschreiben und per Standard</p>
        </td>
        <td>
          <p>(veraltet – bitte nicht mehr verwenden)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>7 - Vermailung</p>
        </td>
        <td>
          <p>Empfohlene Versandtechnologie über die Datenbank bzw. einen externen Dienst</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Typ</strong></p>
          <p>Typen des Versandprofilstamms. Hier können den Versandprofilen unterschiedliche Verwendungen zugeordnet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>---</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Beleg E-Mail</p>
        </td>
        <td>
          <p>Versand von Belegen per E-Mail über das Modul Beleg-Mailversand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anschreiben Mail</p>
        </td>
        <td>
          <p>Anschreiben via E-Mail-Versand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anschreiben FAX</p>
        </td>
        <td>
          <p>Anschrieben via FAX-Versand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Infomails Zollabwicklung</p>
        </td>
        <td>
          <p>Benachrichtigungen für Zollabwicklung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formulararchiv Mail</p>
        </td>
        <td>
          <p>Versand aus dem Formulararchiv</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Hotline Automat</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Standard aus Mandantenstamm</p>
        </td>
        <td>
          <p>Hier finden sich die Versandinformationen, welche aus dem Mandantenstamm übernommen wurden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### Funktionen

Die Funktion „Testmail versenden“ bietet die Möglichkeit, eine Mail an den eingestellten Empfänger zu senden. Diese Funktion wird ebenfalls von der Schaltfläche „Sende Test-E-Mail“ verwendet
