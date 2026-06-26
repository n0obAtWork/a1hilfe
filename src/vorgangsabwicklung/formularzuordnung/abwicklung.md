# Abwicklung

<!-- source: https://amic.de/hilfe/abwicklung1.htm -->

#### Prozess Abwicklung

| Feld | Beschreibung |
| --- | --- |
| Datenbereitstellung | Die Datenbereitstellung ist eine Prozedur, die aus der Datenbank jene Werte in die Tabelle “Datenstrom_ExternerProzess” lädt, die für die Anzeige im Baum wichtig sein können. Hierbei berücksichtigt die Prozedur die aktuelle connect-id und somit auch die vom Anwender in der Maske ausgewählten Einträge. |
| Datenprozedur | Die Datenprozedur gibt an, welche Prozedur den Quellbaum darstellt. (Hier: „SendeAuftragsDaten“) Diese Prozedur hat nur einen Parameter, nämlich die Connection-ID des rufenden A.eins-Programms mit Namen „connect_id“. Die Prozedur muss ein Feld mit dem Namen „ident“ in der Ergebnismenge beinhalten, das eindeutig den Datensatz beschreibt. |
| Beschreibungsstruktur | Die Beschreibungsstruktur beschreibt den Aufbau und die Darstellung der Daten im Programm. |
| Schriftgröße | Mit der Schriftgröße der Vorgangsklasse bestimmen Sie die Schriftgröße im Programm für die Verwendung dieser Vorgangsklasse und Vorgangsunterklasse. Die Schriftgröße bezieht sich auf die Schrift in den Frames und in verwendeten Toolboxen. |
| Vorgangsverarbeitung | Auswahl von Optionen der Vorgangsverarbeitung.<br>Möglich sind:<br>Ein Vorgang pro Aktion<br>Hinzufügen und Artikel zusammenfassen<br>Hinzufügen und Artikel trennen |
| Alternative Abarbeitung | Für die Abwicklung kann bei Bedarf ein alternatives Script ausgewählt werden das die Vorgangsbearbeitung übernimmt. Wird hier „Ja“ gewählt so kann im Feld Scriptname der Name des Skriptes eingegeben oder mit &lt;F3> ausgewählt werden. |
| Scriptname | Eingabe Namens oder Auswahl eines alternativen Skriptes. Zu beachten ist die korrekte Pfadangabe (absolut) |

#### X-Rechnung

| Feld | Beschreibung |
| --- | --- |
| Default Exportprofil | (nur Rechnungen und Gutschrift) Wird im Kunden kein X-rechnung-Exportprofil festgelegt, so wird das hier in der Unterklasse voreingestellte Formular beim X-rechnung-Export verwendet. |

#### Prozess Klammer

| Feld | Beschreibung |
| --- | --- |
| Klammervorbelegung | Hiermit kann eingestellt werden, das beim setzten der Vorgangsnummer die Klammernummer mit der Belegnummer und der Klammertyp mit dem Klammertyp der Unterklasse vorbelegt wird. |
| Klammertyp | Vorbelegung des Klammertyps, wenn die Klammervorbelegung auf „aus Belegnummer“ steht. |

#### Lagerverwaltungssystem

| Feld | Beschreibung |
| --- | --- |
| Transfer relevanter Position | Für das Lagerverwaltungssystem kann hier eingestellt werden, ob relevante Daten in eine Transfertabelle („LVS_ARTIKELTRANSFER_WABEW“) geschrieben werden sollen.<br> |
| Auslagerstrategie | Hier wird festgelegt, welche Prozedur den Typ der Auslagerstrategie entscheidet.<br>Muster: AMIC_DEMO_LVS_AUSLAGERSTRATEGIE |

#### Nachhaltigkeit

| Feld | Beschreibung |
| --- | --- |
| Kunde ungültige Nachhaltigkeit | Hier kann eingestellt werden, was passieren soll, wenn die Nachhaltigkeit des Kunden bei der Warenpositionserfassung ungültig ist. Diese Einstellung überschreibt die Einstellung des [SPA 1157](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/nachhaltigkeitspruefung_spa_1157.md). Einrichtung verhalten sich analog zur SPA-Einrichtung. |
| Kontraktabwahl | Hier kann eingestellt werden, was passieren soll, wenn die Nachhaltigkeit des Kunden bei der Kontraktauswahl ungültig ist. Diese Einstellung überschreibt die Einstellung des [SPA 1158](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/kontraktabwahl_bei_nachhaltigkeitsfehler_spa_1158.md). Einrichtung verhalten sich analog zur SPA-Einrichtung. |

#### Versand

| Feld | Beschreibung |
| --- | --- |
| Versandfunktion | Die Versandfunktion für den [Belegversand](../../zusatzprogramme/mailversand_allgemein/index.md) ist eine Datenbankfunktion zur Abwicklung des Belegversands. Mehr Information zu den Versandfunktionen bekommen Sie in der [Belegversandfunktionsdokumentation](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/pflege_der_vorgangsklassen/versand_funktionen.md). |
| Formular Belegversand Mailbody | Veraltet – Bitte nicht mehr verwenden |
| Versandprofil | Versandprofil aus dem Versandprofilstamm, das zur Versendung dieser Belege verwendet werden soll. Wird hier nichts angegeben gilt der Standard des Typs Ware-Beleg-Versand |
| Belegversand Mailtyp | Hier kann ein spezieller Mailtyp angegeben werden, welcher beim Belegmailversand verwendet werden soll. Der Mailtyp wird im Kunden bzw. dessen Anschrift hinterlegt und kann im Anwendungsformat „AF_MAILTYP“ eingerichtet werden. Beim Belegmailversand wird dann die E-Mail Adresse des Kunden verwendet, die diesem Mailtyp entspricht. |
| FA-Eintrag HTML-Body | FA-ID des Formulararchiv-Eintrags eines HTML-BODY-Templates, das mit Hilfe der Body-Funktion zu einem HTML-Body verarbeitet werden kann. Siehe auch Dokumentation [HTML-Body](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/pflege_der_vorgangsklassen/html_body.md) |
| Body-Funktion | Eine Datenbankfunktion, die FA_Id und FA_MndNr des zu versendenden Beleg übergeben bekommt und aus einem HTML-Body-Template einen HTML-Body macht.<br>Beispiel: AMIC_DEMO_HTMLBODY<br>Siehe auch [Doku HTML-Body](../../zusatzprogramme/mailversand_allgemein/einrichtung_mailversand/pflege_der_vorgangsklassen/html_body.md) |
| BelegversandMakro | Hier kann ein Makro (auch C#-Makro Typ Druck-Makro) hinterlegt werden. Dies hat zwei Einsprungpunkte: VOR oder Nach dem Belegversand.<br>Es bekommt drei Parameter übergeben:<br><ul><li>Fa_id - Die FA_ID des gedruckten Belegs</li><li>FA_MndNr – FA_MndNr des gedruckten Beleges</li><li>Before</li><li>1, wenn der Aufruf vor dem Belegversand erfolgt –</li><li>0 , wenn der Aufruf nach dem Belegversand erfolgt</li></ul> |

Hinweis: Bei Rohwaresammelbelegen können mittels eines Belegversandmakros eRechnungen erzeugt werden. Das privatisierbare Standardmakro dafür ist AMIC_Generiere_XRechnung_FRZ.

#### Behandlungsschema

| Feld | Beschreibung |
| --- | --- |
| Kundenänderung | Das angegebene Behandlungsschema wird im Fall von Kundenänderungen in einem Vorgang dieser Vorgangsklasse/Unterklasse verwendet bzw. als Vorgabe in Auswahlen eingetragen. |
| Nummernkreis neu bestimmen | Wird diese Funktion aktiviert, so wird beim Wechsel in diese Unterklasse die Belegnummer aus dem zugehörigen Nummernkreis neu belegt. |
| Lagernummernänderung | Das angegebene Behandlungsschema wird im Fall von Lagernummernänderungen in einem Vorgang dieser Vorgangsklasse/Unterklasse verwendet bzw. als Vorgabe in Auswahlen eingetragen. |
| Lageränderung bei Teildisposition automatisch aus VKONS | Vorbelegt mit Nein<br>Setzt man dieses Feld für einen Zielvorgang auf Ja, dann wir bei der Standard-Teildisposition automatisch eine Lageränderung vorgenommen, wenn das Ziellager nicht dem Lager in VKONS / Lagernummerfehl entspricht. |

#### Gefahrgut

| Feld | Beschreibung |
| --- | --- |
| Negative Menge für Gefahrgut positiv rechnen | Das Feld ist ein Ja/Nein Feld und ist standardmäßig mit Nein vorbelegt. Wenn der Wert auf Ja geändert wird, werden die Gefahrpunkte für negative Mengen analog der positiven Menge berechnet. Beispiel: Retour von Gefahrgütern. |
