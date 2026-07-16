# Profil Verwaltung: Pfleger

<!-- source: https://amic.de/hilfe/_profil_verwaltung_pfleger.htm -->

<details>
<summary>E-Mail-Profil</summary>

| Feld | Beschreibung |
| --- | --- |
| Id | Die Id des Profils, readonly |
| E-Mail-Adresse | Die E-Mail-Adresse des Postfachs |
| Passwort | Das Passwort für den Zugriff auf das Postfach |
| Client Id | Ermöglicht die eindeutige Identifizierung der Anwendung in Microsoft Identity Platform. Sie wird bei der Überprüfung der von Identity Platform empfangenen Sicherheitstoken herangezogen.<br>Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app) |
| Client Secret | Bei einem geheimen Clientschlüssel handelt es sich um einen Zeichenfolgenwert, der anstelle eines Zertifikats von Ihrer App für die Identifizierung verwendet werden kann.<br>Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app) |
| Tenant Id | Ein Mandant stellt eine Organisation dar. Hierbei handelt es sich um eine dedizierte Instanz von Azure AD, die eine Organisation oder ein App-Entwickler zu Beginn einer Beziehung mit Microsoft erhält. Diese Beziehung kann beispielsweise mit der Registrierung für Azure, Microsoft Intune oder Microsoft 365 beginnen.<br>Weitere Informationen findet man unter: [https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant](https://learn.microsoft.com/en-us/azure/active-directory/develop/quickstart-create-new-tenant) |
| Domäne | Die Domäne des Dienstes<br>**Wichtig**: Bei Verwendung von Microsoft Graph zur Versendung von Mails muss hier der Wert graph.microsoft.com eingetragen werden. |
| Port | Der Port für die Kommunikation mit dem Dienst |
| SSL verwenden | Ja/Nein |
| Excel als PDF/A | Ja/Nein – Sollen angehängte Excel-Dokumente in das PDF/A-Format für Langzeitarchivierung konvertiert werden? |
| Plugin Name | Es kann zwischen IMAP, Webservice und Graph gewählt werden.<br>**Wichtig**: Bei Auswahl von Graph muss in das Feld Domäne der Wert graph.microsoft.com eingetragen werden.<br>Außerdem muss in das Feld E-Mail-Adresse, die bei der Microsoft Identity Plattform registrierte Mailadresse eingetragen werden.<br>Des Weiteren müssen die Felder ClientId, Client Secret und Tenant Id mit den dort hinterlegten Werten gefüllt werden. |
| Nachlauf-Prozedur | Name der Prozedur die nach dem Zugriff ausgeführt werden soll.<br>Übergabeparameter: fa_Id, fa_mndNr.<br>Musterprozeduren in den Auslieferungen fangen mit der Bezeichnung „eMailConnector_“ an. |
| Exchange Version | Die Version des verwendeten Exchange-Servers, sofern dieser vom Dienst eingesetzt wird. |
| Webservice URL | Die vollständige Adresse des Webservices |
| Excel als Xml | Ja/Nein – Sollen angehängte Excel-Dokumente in Xml konvertiert werden? Die Anzeige des Feldes hängt von einer gültigen Excel-Archivimport-Lizenz ab. |
| eRechnung import | Ja/Nein - Steht dieser Schalter auf „Ja“, so werden angehängte PDF und XML-Dateien daraufhin geprüft, ob sie eine eRechnung im Format UBL oder CII enthalten und ggf. werden diese Dateien automatisch importiert. |

</details>

<details>
<summary>Funktionen der Profil Verwaltung</summary>

| Funktion | Beschreibung |
| --- | --- |
| Verbindung testen | Im Ansehen und Editieren-Modus wird diese Funktion angeboten. Sie testet den Zugriff auf das ausgewählte Postfach und gibt im Erfolgsfall die Anzahl der ungelesenen E-Mails aus. Im Fehlerfall wird die Fehlermeldung angezeigt. |

</details>
