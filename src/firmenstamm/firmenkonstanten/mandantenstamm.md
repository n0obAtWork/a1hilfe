# Mandantenstamm

<!-- source: https://amic.de/hilfe/_mandantenstammmnd.htm -->

Hauptmenü > Administration > Firmenkonstanten > Mandantenstamm

oder Direktsprung **[MND]**

Hier erfolgen die Benennung des Mandanten (Bezeichnung der Firma), seine datenbanktechnische Anbindung sowie die Zuordnung von Nummernkreisen zu wesentlichen Datenbereichen.

<p class="just-emphasize">Felder:</p>

| Feld | Bedeutung |
| --- | --- |
| Kurztext | Kurzbezeichnung des Mandanten (erscheint in der Windowskopfleiste) in der Datenbank = Basis |
| Nummer | Laufende Nummer des Mandanten (1) |
| Aktiv | 0: Nein<br>1: Ja |
| Name | Bezeichnung der Firma |
| Buchwährung | Zeigt die aktive Buchwährung des Mandanten an. Solange noch keine Belege erfasst wurden, wird im Menü eine Funktion zum Setzen der Buchwährung angeboten. |
| Testmandant | Zeigt an, ob der aktuelle Datensatz ein Testmandant ist. |

<p class="just-emphasize">Register:</p>

<details>
<summary>Registerkarte Allgemein</summary>

Für die allgemeinen Informationen stehen folgende Felder zur Verfügung

| Feld | Bedeutung |
| --- | --- |
| Technische Version | (Wird in Aeins nicht mehr benutzt) |
| Daten Version | Gibt Version der Daten an, mehr Einsicht über Direktsprung **[sysin]** |
| Versionsdatum | Gibt Datum der Version an, mehr Einsicht über Direktsprung **[sysin]** |
| Nachlaufprozedur | Optionale private parameterlose Daten-Prozedure, welche bei Update vom A.eins automatisch nachgezogen (zum Ende des Updates) aufgerufen wird. |
| Empfänger | Semikolon getrennte Liste der Empfänger-E-Mail-Adressen, die im Fehlerprotokollierungsfall eine E-Mail erhalten sollen. |
| Empfängerprozedur | Name der privaten Prozedur für die Auswahl von Empfängern in speziellen Fällen. Standard-SQL-Funktion ist hier „FehlerprotokollAbweichendeEmpfaenger“. Private Ableitungen bitte von dieser Funktion! ([SQLP]) |
| Selektionsprozedur | Name der privaten Prozedur zur Eingrenzung der zu meldenden Fehlernummern. Standard-SQL-Funktion ist hier „FehlerprotokollMailselektion“. Private Ableitungen bitte von dieser Funktion! ([SQLP]) |
| Sende ILN | Nummer die beim EDIFACT Datenaustausch eingesetzt wird |
| Terres Bestellnummer | Nummer die bei der Bestellung von Artikeln bei Terres angegeben werden muss. Diese Nummer wird von Terres vergeben. |
| Kasse/DSFin_V-K<br>Taxonomie-Version | Die in A.eins implementierte Version der DSFinV-K |
| Kasse/DSFin_V-K<br>Taxonomie-Dtd | Die in A.eins implementierte Version der DSFinV-K benötigt diese mit A.eins bereitgestellte Dokument-Type-Definition-Datei. |
| Programm läuft in | Sprache in der das A.eins grundsätzlich läuft. Diese Sprache wird von AMIC gepflegt und man kann sie mit **F3** auswählen. Die Sprachen Englisch, Dänisch, Niederländisch und Französisch sind Lizenzpflichtig. Wenn eine dieser Sprachen das erste Mal ausgewählt wird, so muss man die Aktivierung bestätigen. Es wird erst dann die aktuelle Sprache eingespielt und der Benutzer kann ohne Lizenz für 60 Tage diese Sprache nutzen. Danach muss die Lizenz erworben werden.<br>Ohne Aktivierung wird die Spracheinstellung ignoriert. |
| Systemkundennummer | Kunde des Mandanten<br>[Nachhaltigkeit](../../vorgangsabwicklung/nachhaltigkeit/index.md)<br>Um für einen Mandanten die Nachhaltigkeitsinformationen zu hinterlegen, muss hier der Systemkunde hinterlegt werden. An diesem können dann die [Zertifikate](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/zertifikate.md) vom Typ 5 gepflegt werden. |
| Handelsregister | Nummer des Eintrags im Handelsregister. |
| Handelsname | Vom Firmennamen abweichender Handelsname. |
| Electronic Address | Die für die elektronische Zustellung von eRechnung verwendete Absendeadresse |
| Zusätzliche rechtliche Informationen | Zusätzliche rechtliche Textinformationen, die für die gesamte eRechnung von Bedeutung sind, wie beispielsweise Strafdrohungen, Liefersperrandrohungen und dergleichen. |

</details>

<details>
<summary>Registerkarte Finanzbuchhaltung</summary>

Hier werden für die Finanzbuchhaltung benötigen Informationen festgelegt.

| Feld | Bedeutung |
| --- | --- |
| Eröffnungsbilanzkonto | Dieses Konto wird beim Jahreswechsel als Konto für die Eröffnungsbuchung vorgeschlagen. |
| Schlussbilanzkonto | Dieses Konto wird beim Jahreswechsel als Konto für die Abschlussbuchungen vorgeschlagen. |
| Um-/Nullbuchungskonto | Dies ist ein technisches Konto. Es hat immer den Saldo 0! |
| Konto Bilanzgewinn | Dies ist ein fiktives Konto für den Ausweis des Jahresüberschusses/Fehlbetrages auf der Bilanz. Es muss zwar im Sachkontenstamm angelegt sein, darf jedoch nicht bebucht werden. Die Daten werden aus dem aus dem GuV-Ergebnis bestimmt.<br>Ist hier kein Konto eingetragen, wird die Differenz zwischen Aktiva und Passiva am Ende der Bilanz als Jahresüberschuss/Fehlbetrag dargestellt. |
| Fehlerkostenstelle | Wurde in einem Beleg keine [Kostenstelle](../../finanzbuchhaltung/kostenrechnung/kostenstellen.md) angegeben, so wird bei GuV-Konten automatisch diese Kostenstelle verwendet. |
| Fehlerkostenträger | Wurde in einem Beleg kein [Kostenträger](../../finanzbuchhaltung/kostenrechnung/kostentraeger.md) angegeben, so wird bei GuV-Konten automatisch dieser Kostenträger verwendet. |
| Fehlerkostenobjekt | Wurde in einem Beleg kein [Kostenobjekt](../../finanzbuchhaltung/kostenrechnung/kostenobjekte/index.md) angegeben, so wird bei GuV-Konten automatisch dieses Kostenobjekt verwendet. |
| Zinsbasis | Hier kann man angeben, wie bei der Zinsabrechnung gerechnet werden soll. Möglicher Werte sind:<br>• Monat / 365<br>• 30 / 360<br>• Monat / 360 |
| Bundesland | Das Bundesland wird für die Umsatzsteuervoranmeldung / Elster gebraucht. |
| Nummer des Finanzamtes | Ist mit **F3** auswählbar. Wird für die Umsatzsteuervoranmeldung / Elster benötigt. |
| Name | Wird für die Umsatzsteuervoranmeldung / Elster benötigt |
| Straße | Wird für die Umsatzsteuervoranmeldung / Elster benötigt |
| PLZ/Ort | Wird für die Umsatzsteuervoranmeldung / Elster benötigt |
| Steuernummer | Wird für die Umsatzsteuervoranmeldung / Elster benötigt |
| Voranmeldezeitraum | Wird für die Umsatzsteuervoranmeldung / Elster benötigt |
| Ust-IdNr. | Wird für die Zusammenfassende Meldung benötigt |
| Gläubiger-ID | Die [Gläubiger-ID](./mandantenstamm.md) wird für das SEPA-Lastschriftverfahren benötigt. |
| Unternehmensform | Diese Informationen werden für die [e-Bilanz](../../finanzbuchhaltung/ebilanz_online/index.md) benötigt. Eine Auswahl der laut Taxonomie möglichen Werte ist mit **F3** möglich.<br>(Wird zurzeit in A.eins nicht genutzt) |
| Rechtsform | Diese Informationen werden für die [e-Bilanz](../../finanzbuchhaltung/ebilanz_online/index.md) benötigt. Eine Auswahl der laut Taxonomie möglichen Werte ist mit **F3** möglich.<br>(Wird zurzeit in A.eins nicht genutzt) |
| SEPA Information | Siehe [SEPA-Kennzeichen im Mandantenstamm](../../finanzbuchhaltung/zahlungsverkehr/sepa/sepa_kennzeichen_im_mandantenstamm.md) |

</details>

<details>
<summary>Registerkarte Replikation</summary>

Hier werden die Angaben für die in einer Replikation wichtigen Informationen hinterlegt.

Im Bereich Filialwesen wird festgelegt, um welche Betriebsstätte/Filiale es sich bei dieser Datenbank handelt und ob es sich um eine Filiale oder Zentrale handelt.

Die unter dem Bereich **optionale „dbremote“-Parameter** aufgeführten Felder werden von dem SQL-Remote Nachrichtenagenten „**dbremote**“ verwendet.

Der Mandantenstamm-Eintrag einer WebPortal-Datenbank muss im Feld ‚*Datenbank ist WebPortal-Datenbank?*‘ mit dem Wert ‚**Ja**‘ versorgt werden.

| Felder | Bedeutung |
| --- | --- |
| Filialnummer | Nummer der Filiale |
| Betrieb ist | Auswahl ob der Betrieb Filiale oder Zentrale ist |
| Maximale Dateigröße der Transaktionslog-Datei? | Nummerische Größenangabe mit Auswahl der Speichereinheitsgröße |
| Maximale Dateigröße der Ausgabelog-Datei? | Nummerische Größenangabe mit Auswahl der Speichereinheitsgröße |
| Ausführliche Ausgabe im Log | Ausführliche Ausgabe im Log |

| Signal | Bedeutung |
| --- | --- |
| Frequenz (in Minuten) | Frequenz in Minuten innerhalb der das Signal erzeugt für die Remote-User erzeugt wird.<br>Beispiel: 5 Minuten |
| Schwellenwert | Schwellenwert in Minuten ab dem ein Signal als nicht bestätigt von den Remote-User(n) angesehen wird.<br>Beispiel: 10 Minuten<br>D.h. wenn ein Replikationspartner innerhalb von 10 Minuten nicht auf ein angefordertes Signal geantwortet hat, dann hat die Replikation in der Übertragungsstrecke bzw. dem DBRemote-Wesen ein Problem und muss genauer begutachtet werden. |

| WebPortal | Bedeutung |
| --- | --- |
| Datenbank ist WebPortal-Datenbank? | Ja: Es handelt sich bei der Datenbank dieses Mandanten um eine WebPortal-Datenbank.<br>Nein: Diese Datenbank ist nicht die Datenbank des WebPortals.<br>Dieses Feld ist nur bei einer Replikations-Datenbank sichtbar, die keine konsolidierte Datenbank ist. |

</details>

<details>
<summary>Registerkarte Zollausfuhr</summary>

In der Registerkarte Zollausfuhr werden wichtige Informationen zur Abwicklung der Zollausfuhr und der Datenverbindung zum externen Datendienstleister AEB eingestellt.

| Parameter | Vorgabewert für Tests | Bedeutung |
| --- | --- | --- |
| Teilnehmeridentifikation: | 0 | Teilnehmeridentifikation (TIN) des Empfängers der Ausfuhr. |
| Deutsche Identifikation: | Nein | Angabe ob es sich bei der angegeben TIN um eine deutsche TIN handelt. |
| Sofortsendung | Nein | Angabe, ob eine Ausfuhr nach Versendung an den Webanbieter (genauer definiert im Feld „Servicename“) direkt an die Zollverwaltung weiter versendet wird. Um weitere Arbeiten auf der Webseite des Anbieters zu verhindern sollte hier „Ja“ gewählt sein. |
| Schnittstelle | 2.3 | Schnittstellenversion des Webanbieters. Stand 01.01.20 ist die Version 2.3. Dieses Feld muss nur angepasst werden, wenn sich die Versionsnummer beim Webanbieter ändert. |
| InstallationsID | 1 | ID der installierten A.eins Version. Sollten mehrere Installationen von A.eins im Hause vorliegen, kann dieses Feld genutzt werden um festzustellen von welcher Version aus die Ausfuhr abgeschickt wurde. Standardbelegung = 1 |
| Mandantname | AMIC | Mandantname beim Webanbieter. Dies ist der erste von drei Teilen der Zugangsdaten beim Webanbieter. |
| Benutzer | test | Benutzername beim Webanbieter. Dies ist der zweite von drei Teilen der Zugangsdaten beim Webanbieter. |
| Passwort | hallo2008 | Passwort für den Zugang beim Webanbieter. Dies ist der dritte von drei Teilen der Zugangsdaten beim Webanbieter. |
| Mail Ankunft Dokumente | a@b.de | Mailadresse, an der mit Prozeduren automatisch abgeholte Ausfuhrbegleitdokumente signalisiert werden |
| Mail Ablehnung | a@b.de | Mailadresse, an der mit Prozeduren automatisch empfangene Ablehnungen signalisiert werden |
| Zugangsart | 1.) Testzugang<br>2.) Operativer Zugang | Operativer Zugang NUR mit Zugangsdaten von AEB! |
| Webanbieter | 1.) Test: https://rz3.aeb.de/test2aae<br>2.) Operativ: https://rz3.aeb.de/prod2aae | Hier kann die Webadresse für den Zugriff über die vom Anbieter zur Verfügung gestellte Oberfläche eingegeben werden. Beim Betätigen des Buttons Anzeigen wird ein neues Fenster geöffnet, in dem ein Browser-Element enthalten ist, welches die Anzeige ermöglicht. |
| Servicename | AUSFUHR||XPRESS plus | Default-Wert |
| Zertifikat | C:\\AEBZertifikat.cer | Festplattenpfad, unter dem das Zertifikat des Webservice-Anbieters abgespeichert ist. Das Zertifikat muss immer auf dem gleichen Rechner abgelegt sein, auf dem auch die Datenbank läuft. |

Zusätzlich finden Sie in der Optionbox eine Funktion „Zollanmelder Adresse“. Legen Sie hier die Adressdaten für die Zollanmeldung fest. Dieser Datensatz wird beim ersten Aufruf automatisch mit der Mandanten-Anschrift vorbelegt.

</details>

<details>
<summary>Registerkarte Verbotsliste</summary>

In der Registerkarte Verbotsliste werden wichtige Informationen zur Datenverbindung zum externen Datendienstleister AEB eingestellt, der Anschriften gegen Verbotslisten prüft.

| Feld | Bedeutung |
| --- | --- |
| Mandantname | Name des Mandanten (Auf Seiten des Dienstanbieters) |
| Benutzer | Username des Zugangs |
| Passwort | Passwort des Zugangs |
| Profil | Profil von AEB |

</details>

<details>
<summary>Registerkarte Internet</summary>

Auf der Registerkarte „Internet“ werden Informationen hinterlegt, die für externe Systeme im Internet oder Intranet gebraucht werden.

<p class="just-emphasize">Proxyeinstellungen</p>

| Feld | Bedeutung |
| --- | --- |
| Proxy | Name des Proxyservers für eine Verbindung von A.eins ins Internet |
| ProxyPort | Port des Proxyservers für eine Verbindung von A.eins ins Internet |
| User | Username für den Zugriff von A.eins aufs Internet via Proxy |
| Passwort | Passwort für den Zugriff von A.eins aufs Internet via Proxy |

<p class="just-emphasize">Standard SMTP Informationen</p>

| Feld | Bedeutung |
| --- | --- |
| Versand Profil | Über F3 wird hier ein Profil aus dem Versandprofilstamm ausgewählt. Anschließend werden alle folgenden für SMTP-Informationen bestimmten Felder mit den Daten aus dem Versandprofilstamm gefüllt.<br>Bleibt das Feld leer werden die Daten aus den Feldern des Mandantenstamm angezeigt. |
| Absender-E-Mailadresse | Absender-E-Mailadresse für den Versand |
| SMTP-Server | IP-Adresse oder Servername des SMTP-Servers. |
| SMTP-Port | Port des SMTP-Servers |
| Sendername | Alias der anstatt der Absender-E-Mailadresse angezeigt werden kann |
| Benutzer | Benutzername für den Zugriff auf den SMTP-Server (falls erforderlich) |
| Passwort | Passwort für den Zugriff auf den SMTP-Server (falls erforderlich) |

<p class="just-emphasize">Signierung</p>

| Feld | Bedeutung |
| --- | --- |
| Signatur-Datei | Dateiname der Datei mit einem PK12-Schlüssel zur Signierung von PDF-Dateien. |

<p class="just-emphasize">Fax-Einstellungen</p>

| Feld | Bedeutung |
| --- | --- |
| Versand Profil | Hier kann mit F3 Auswahl das Versandprofil aus dem Versandprofilstamm ausgewählt werden, welches zum Versenden von Faxnachrichten verwendet werden soll. |
| Faxstring | Hier wird der Provider (zum Beispiel „faxmaker.com“) oder das Verzeichnis ( zum Beispiel bei Tobit ) aus dem gewählten Versandprofil angezeigt. |

</details>

<details>
<summary>Registerkarte Zugangsdaten</summary>

Auf der Registerkarte „Zugangsdaten“ werden Zugangsdaten hinterlegt, die für externe gebraucht werden.

| e-Mailgruppen |
| --- |
| Bereich | Für welchen Bereich soll die Mailadresse gelten. Die Bereiche können mit F3 ausgewählt werden. Der F3-Auswahl liegt das Anwendungsformat **AF_MANDMAIL** zugrunde. Es können ab der Nummer 100 eigene Bereiche definiert werden.<br> |
| Mailadresse | Hier kann eine Mailadresse bzw. mehrere durch Semikolon getrennte Mailadressen hinterlegt werden.<br> |

Für den Zugriff auf die Mailgruppen existiert die Funktion MandMailAdresse(Bereich). Um also z.B. auf die Mailadresse unter „e-Mailgruppe 3“ zuzugreifen schreib man:

```sql
Select MandMailAdresse(3)
```

</details>

<details>
<summary>Registerkarte GeoDaten</summary>

Auf der Registerkarte „GeoDaten“ werden Einstellungen für Geodatendienste gemacht. Für die Verwendung benötigen Sie eine separate Lizenz.

| GeoDaten |
| --- |
| Adressprozedur | Diese Prozedur liefert für eine regelmäßige Abfrage von Anschriften ohne GeoDaten die AdressId zurück. Als Beispiel wurde die Prozedur „AMIC_DEMO_GeoDataAnschriften“ ausgeliefert. |
| Google API Key | Dies ist der API-Key für die Ermittlung von Entfernungen zwischen Wegpunkten sowie den Geodaten bei der MapsTourenplanung. [Zum Diensteanbieter.](../../zusatzprogramme/maps_tourenplanung/diensteanbieter/entfernungmatrix.md) |

</details>

<details>
<summary>Registerkarte Webdienst</summary>

Auf der Registerkarte „Webdienst“ sind Informationen für den Webdienst hinterlegt. Außerdem gibt es die Möglichkeit den Webdienst mit einem einzelnen Knopfdruck anzulegen, sollte er noch nicht existieren.

Wenn das Feld „Adresse“ leer ist, dann liegt das daran, dass der Datenbankserver nicht mit der Option „-xs“ (akzeptierbare Webprotokolle) gestartet wurde.

| Webdienst | |
| --- | --- |
| Adresse | Mit dieser Adresse ist der Webservice im lokalen Netzwerk ansprechbar. |
| Name | Dies ist der Name des Webdienstes |
| Status | Zeigt den momentanen Status des Webdienstes an. |

</details>

<details>
<summary>Registerkarte Firmenlogo</summary>

Hier kann man sein Firmenlogo hinterlegen. Klickt man auf die Registerkarte und hat noch kein Logo hinterlegt, so öffnet sich ein Dateiauswahldialog, in dem man die Grafikdatei auswählen kann. Erlaubte Formate sind \*.bmp und \*.jpg. Das Logo lässt sich mit der Funktion ***Bild entfernen*** **F7** wieder löschen.

Das Logo kann in den Finanzbuchhaltungs-Reporten angezeigt werden. Es wird exakt so groß dargestellt, wie es im Mandantenstamm hinterlegt ist. Dazu muss man jedoch in den [Crystal Report Optionen](../../zusatzprogramme/crystal_report/crystal_report_optionen.md) die Option ***Anzeige des Firmenlogos*** aktivieren.

</details>

<p class="just-emphasize">Funktionen:</p>

<details>
<summary>Funktionen Mandantenstamm</summary>

Die Funktionen innerhalb des Mandantenstamms sind folgende:

| Feld | Beschreibung |
| --- | --- |
| DSGVO-Liste | Zeigt den DSGO Datensatz zu dem Systemkunden an: [DSGVO](../../dokumentenverwaltung/eu_dsgvo_datenschutz_grundverordnung/index.md) |
| Mandanten Nummernkreise | Ruf die Maske für die Mandantenstamm [Nummernkreise](<../nummernkreise_fuer_ware_und_fibu/index.md#Einrichtung von Nummernkreisen>) auf. |
| Kunde | Ruft den Pfleger für den Systemkunden auf. |
| Adresse | Ruft den [Anschriftenpfleger](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/anschriften/registerkarten_in_anschriften/allgemein.md) auf um die Adresse des Systemkunden zu editieren. |
| Zollmelder Adresse | Ruft den [Anschriftenpfleger](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/anschriften/registerkarten_in_anschriften/allgemein.md) auf um die Adresse des Zollmelders zu editieren. |
| Zollmelder Ansprechpartner | Ruft den [Anschriftenpfleger](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/anschriften/registerkarten_in_anschriften/allgemein.md) auf um den Ansprechpartner des Zollmelders zu editieren. |
| USt-IdNR bearbeiten | Ruft die [Umsatzsteuer Identifikationsnummer](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/umsatzsteuer_identifikationsnummern.md) Maske auf |
| Ansprechpartner ZMDO **(F10)** | Hier wird die Anschrift gepflegt, welche für die Zusammenfassende Meldung verwendet wird. |
| Ansprechpartner UVA | Hier wird die Anschrift gepflegt, welche für die Umsatzsteuervoranmeldung verwendet wird. |
| Verpostungstamm **(Shift + F5)** | Öffnet den [Versandprofilstamm](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/versandprofilstamm.md) |
| Bild entfernen **(F7)** | Entfernt das Bild, welches in der registerkarte [Firmenlogo](./mandantenstamm.md#Registerkasrte_Firmenlogo) hinterlegt wurde. |
| Druckfelder | Öffnet eine die Formularansicht in der Variante [Druckfelder](../../zusatzprogramme/formulareinrichtung_und_zuordnung/druckfelder.md). |
| Archiv anzeigen **(Strg + F12)** | Zeigt das Mandanten spezifische Archiv an. |

</details>
