# Versandprofilstamm

<!-- source: https://amic.de/hilfe/_versandprofilstamm.htm -->

Administration > Firmenkonstanten > Versandprofilstamm

Direktsprung **[VPST]**

Der Versandprofilstamm dient zum Hinterlegen von Profilen für den Versand von zum Beispiel E-Mail oder Fax zu verschiedensten Zwecken in A.eins. Das Profil, welches im Mandantenstamm hinterlegt wurde, wird hier mit grüner Farbe markiert und kann an dieser Stelle auch nicht einfach gelöscht werden.

Im „Startmenü“ die Anwendung „Dienste“ aufrufen. Dort den A.eins.Mailservice selektieren und „Eigenschaften“ aufrufen. Startyp auf „Automatisch“ festlegen und den Dienst starten.

Eingabefelder

| Feld | Beschreibung |
| --- | --- |
| Profil-Bezeichnung | Name des Profils |
| Typ | Dieser [Typ](./versandprofilstamm.md#Versandprofiltypen) gibt an, für welche Verwendung dieser Eintrag zuständig ist |
| Standard | Kennzeichnet den Versandprofilstammeintrag als Standard |
| Standard dieses Typs | Kennzeichnet diesen Eintrag ggf. als Standard des o.a. Typs |

| Sendeeinstellungen  
Die Sendeeinstellungen enthalten die Informationen, welche zum Versenden der Daten erforderlich sind. Die mit einem **\*** versehenen Felder sind hierbei die Angaben, welche mindestens zur Verfügung stehen müssen!  
Registerkarten und Felder passen sich je nach Typ-Auswahl den entsprechenden Gegebenheiten an (E-Mail oder Fax).  
Über den Knopf „Sendeeinstellungen testen“ wird versucht eine Verbindung zum SMTP-Server aufzubauen, je nach Status wird eine Meldung ausgegeben. |
| --- |
| Bezeichnung \* | Bezeichnung des Versandprofilstamms |
| Versandart \* | [Versandart](./versandprofilstamm.md#Versandart) setzt die Technologie, mit der E-Mails versendet werden können.  
**Bitte verwenden Sie zur Neueinrichtung ausschließlich die Option 7 – Vermailung ! ! !** |
| Versende-Server \* | Der Name oder die IP-Adresse des verwendeten SMTP-Servers (smtp.gmail.com oder 74.125.136.108)  
**Wichtig**: Wenn Microsoft Graph zur Versendung verwendet wird, müssen in die Felder Benutzername und Absender die bei Microsoft Graph eingerichtete Mailadresse eingetragen werden. Außerdem müssen die Felder Client Id, Client Secret und Tenant Id mit den, bei Microsoft Identity Plattform hinterlegten Werten belegt werden. |
| SMTP Port \* | Port der Verbindung  
Über F3 wird ein Anwenderformat aufgerufen, welches im Auslieferzustand folgende Port vorgibt:  
1. 25 = Standard SMTP-Port (ohne Authentifizierung)  
2. 465 = SSL (wird nur bei Versandart Vermailung unterstützt)  
3. 587 = TLS (mit Authentifizierung)  
In dem Anwenderformat „af_SMTPPort“ können auch weitere Ports eingerichtet werden. Damit ist man in der Lage auch eigene Einrichtungen bedienen zu können. |
| Benutzername | Wird zusammen mit dem Kennwort, für die Authentifizierung benötigt |
| Kennwort | Wird, zusammen mit dem Benutzernamen, für die Authentifizierung benötigt |
| Zertifikat | Zertifikat des Servers. |
| Authentifizierung | • Username/Passwort (sofern gegeben)  
• SASLPlain |
| SSL.Verhandlung | • None – keine Verschlüsselung  
• Implicit (SSL) Port 465  
• Explicit (STARTTLS) Port 587  
• Automatic Hier wird die Verschlüsselung beim verbindungsaufbau ausgehandelt. |
| Absender \* | Wird zwingend benötigt, um eine Verbindung zu einem SMTP-Server herzustellen |
| Absender-Alias | Der Alias zu der im Feld „Absender“ stehenden E-Mail Adresse |
| Antwortadresse | Vom Absender abweichende Adresse, an die die Antworten auf diese E-Mail gehen sollen. |
| Provider | Hier kann der Provider für den Fax-Versand hinterlegt werden. Das Feld ist aktiv, wenn als Typ „**Fax-Server Versand**“ ausgewählt wurde. |
| Verzeichnis für Fax | Hier kann ein Pfad auch via F3 Auswahl angegeben werden, in dem die zu versendenden Dateien zum Fax-Versand abgelegt werden sollen. Das Feld ist aktiv, wenn als Typ „**Fax Versand per Verzeichnis**“ ausgewählt wurde. |
| Verzeichnis des Mail-Clients | Pfadangabe zum Verzeichnis des .Net-Mail-Clients „AeinsSendMail.exe“. |
| Odbc DSN | Nur in veralteten Versandarten wie .NetMail  
Verbindungsbezeichnung von Odbc, welche der Mail-Client für die Herstellung der Datenbankverbindung verwenden soll. |
| Client Id | Ermöglicht die eindeutige Identifizierung der Anwendung in Microsoft Identity Platform. Sie wird bei der Überprüfung der von Identity Platform empfangenen Sicherheitstoken herangezogen.  
Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app) |
| Client Secret | Bei einem geheimen Clientschlüssel handelt es sich um einen Zeichenfolgenwert, der anstelle eines Zertifikats von Ihrer App für die Identifizierung verwendet werden kann.  
Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app) |
| Tenant Id | Ein Mandant stellt eine Organisation dar. Hierbei handelt es sich um eine dedizierte Instanz von Azure AD, die eine Organisation oder ein App-Entwickler zu Beginn einer Beziehung mit Microsoft erhält. Diese Beziehung kann beispielsweise mit der Registrierung für Azure, Microsoft Intune oder Microsoft 365 beginnen.  
Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant) |

| E-Mail Einstellungen |
| --- |
| Empfänger Cc | Empfänger einer sichtbaren Kopie der Mails, die mit diesem Profil versendet werden |
| Empfänger Bcc | Empfänger einer unsichtbaren Kopie der Mails, die mit diesem Profil versendet werden. |

| Testmail |
| --- |
| Empfänger | Empfänger der Testmail |
| Betreff | Hier kann ein Betreff für die Testmail angegeben werden |
| Text: | Hier muss ein kleiner Text als E-Mail Inhalt für die Testmail angegeben werden |
| Anlage aus Archiv: | Über F3 lassen sich hier Anhänge folgender Dateitypen auswählen:  
• PDF  
• BMP  
• JPEG  
• PNG  
• GIF  
Sollte keine passende Datei angezeigt werden, kann hier z. Bsp. über Drag&Drop ein entsprechendes Format eingefügt werden.  
Nachdem die Datei im Archiv ausgewählt wurde (Zeile markieren) werden die benötigten Daten mit ESC vom Versandprofilstammpfleger aufgenommen. Im Feld erscheint dann die Belegreferenz der ausgewählten Datei. |
| Sende Test-E-Mail | Sendet die Testmail an den angegebenen Empfänger und meldet das Ergebnis dem Benutzer. Ist der Versand nicht erfolgreich, so gibt es hier einen Hinweis, sich im Fehlerprotokoll über das Ereignis zu informieren |

| Versandart  
Versandarten stellen Technologien zum Aufbau der Verbindung zum Mailserver dar. In unterschiedlichen Umgebungen sind u.U. unterschiedliche Technologien notwendig.  
**Wir empfehlen ausschließlich die Option 7 – Vermailung zu verwenden** |
| --- |
| 1 - Datenbankversand | (veraltet – bitte nicht mehr verwenden)  
Versand über die üblichen SQL-Prozeduren. |
| 2 - Outlook-Versand | (veraltet – bitte nicht mehr verwenden)  
Versand mit Outlook-Anbindung durch „smtp_outlook.sql“. |
| 3 - AeinsSendMail | (veraltet – bitte nicht mehr verwenden)  
Versand über den .Net Mail-Client AeinsSendMail.exe |
| 4 - SPA999 Qutlook ohne SPA-Setzung | (veraltet – bitte nicht mehr verwenden) |
| 5 - Per SPA999 .Net ohne SPA-Setzung | (veraltet – bitte nicht mehr verwenden) |
| 6 - SPA überschreiben und per Standard | (veraltet – bitte nicht mehr verwenden) |
| 7 - Vermailung | Empfohlene Versandtechnologie über die Datenbank bzw. einen externen Dienst |

| Typ  
Typen des Versandprofilstamms. Hier können den Versandprofilen unterschiedliche Verwendungen zugeordnet werden. |
| --- |
| \--- | |
| Beleg E-Mail | Versand von Belegen per E-Mail über das Modul Beleg-Mailversand |
| Anschreiben Mail | Anschreiben via E-Mail-Versand |
| Anschreiben FAX | Anschrieben via FAX-Versand |
| Infomails Zollabwicklung | Benachrichtigungen für Zollabwicklung |
| Formulararchiv Mail | Versand aus dem Formulararchiv |
| Hotline Automat | |
| Standard aus Mandantenstamm | Hier finden sich die Versandinformationen, welche aus dem Mandantenstamm übernommen wurden. |

Funktionen

Die Funktion „Testmail versenden“ bietet die Möglichkeit, eine Mail an den eingestellten Empfänger zu senden. Diese Funktion wird ebenfalls von der Schaltfläche „Sende Test-E-Mail“ verwendet
