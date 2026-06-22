# Kassenverwaltung (logische Kasse)

<!-- source: https://amic.de/hilfe/_kass_einr_logkas.htm -->

| Funktionen |
| --- |
| Kasse deaktivieren | ACHTUNG! Bitte verwenden Sie diese Funktion nur mit Bedacht. Eine einmal vergebene Kassenseriennummer sollte zugewiesen bleiben.<br>Wenn diese Seriennummer ausnahmsweise aus guten Gründen einer anderen Kasse zugewiesen werden soll, so wird diese Kasse fortan nicht wieder verwendet werden können und auch nicht mit einer neuen Seriennummer versehen werden dürfen.<br>Die Zuweisung der Seriennummer an eine neue Kasse müssen Sie in jedem Fall an die Finanzbehörden melden. |
| Kassensystem bearbeiten | ruft die [Bearbeitungsmaske des verbundenen Kassensystems](./kassensystemverwaltung_hardware/index.md) auf. |
| Standard Anzeigeschema einrichten | richtet ein Anzeigeschema für ein [externes Display](./einrichtung_der_marktkasse/einrichtung_des_externen_kassendisplays.md) nach AMIC-Vorlage ein.<br> |
| Ext. Display testen | Führt Sie durch den Test eines [externen Kassendisplay](./einrichtung_der_marktkasse/einrichtung_des_externen_kassendisplays.md) |
| AnyBill einrichten | Öffnet die Profil-Liste für [AnyBill-Profile](./einrichtung_der_marktkasse/qr_code.md), in der diese eingerichtet werden können |

| Kassenstamminformationen Kopfdaten |
| --- |
| Kassennummer | Nummer der Kasse (des Kassenstandorts) |
| Bezeichnung | Bezeichnung der Kasse |
| Anlagedatum | Ebendies |
| Anmeldedatum | Ebendies |

| Registerkarte Allgemein |
| --- |
| Hauptkasse | Gibt an, ob diese Kasse die Hauptkasse ist. |
| Kassensystem | Kassensystem (Hardware-Typ) |
| Hauptkassennr | Nummer der Hauptkasse |
| Sitzungsnummer | Anzeige der Nummer Kassensitzung |
| Belegnummer | Anzeige der Nummer des aktuellen Kassenbeleges |
| Kassenkonto FiBu | FiBu-Konto der Kasse |
| Ver konto FiBu | FiBu-Konto Verrechnungen |
| Nummer Hausbank | Angabe der Hausbank |
| Akt. Bediener | Anzeige aktueller Bediener |
| Wechselgeld | Anzeige Wechselgeldmenge |
| Vorlage | Einstellungen der Kasse aus den Kasseneinstellungen |
| TSE-ID | Die Id der [TSE](../kassensicherungsverordnung/kassensicherungsverordnung_einrichtung/tse_auswahlliste/index.md) |
| Kassenseriennummer | Eine vom Hersteller (AMIC) vergebene Seriennummer der Kasse.<br>Dieses Feld ist nur editierbar, solange noch keine Seriennummer zu dieser Kasse zugewiesen wurde. Die Kassenseriennummern werden automatisch aus der Lizenz bereitgestellt. |
| Inaktiv seit | Angabe eines Datums ab dem diese Seriennummer dieser Kasse nicht mehr zugewiesen ist |
| Ks-Betriebsstätte | In diesem Feld wird die Kassenbetriebstätte hinterlegt. Die Kassenbetriebsstätte wird nur für den DSFinV-K Export benötigt. |

| Registerkarte Formulare<br>Wird hier kein Formular eingerichtet, (Wert 0), so wird hier das nebenstehende Standardvorlageformular (mit neg. Nummer) genommen. |
| --- |
| Barverkauf | |
| Barverkauf Gutschrift | |
| Bareinkauf | |
| Storno Barverkauf | |
| Storno BV Gutschrift | |
| Storno Bareinkauf | |
| Scheckformular | |
| EC-Lastschrift | |
| Zahlungsmeldung | |
| Einzahlung | |
| Geldübernahme | |
| Auszahlung | |
| Entnahme | |
| Geldübergabe | |
| Einreichung | |
| Sortenwechsel | |
| Kassensturz | |
| Zami-Umwandlung | |

| Registerkarte Geräte |
| --- |
| Kassensystem | Informationen aus der zugewiesenen Kassensystemverwaltung |
| Kassenschublade | Informationen aus der zugewiesenen Kassensystemverwaltung |
| Kassendisplay | Informationen aus der zugewiesenen Kassensystemverwaltung |
| Magnetstreifenleser | Informationen aus der zugewiesenen Kassensystemverwaltung |
| (Bon-)Drucker | Informationen aus der zugewiesenen Kassensystemverwaltung |
| Hardware externes Display | Siehe Tabelle |
| Serielles Display | Siehe Tabelle |

| Hardware externes Display |
| --- |
| Ausprägung | In diesem Feld geben Sie an, ob das externe Display als Bildschirmdisplay, als Bildschirmdisplay mit zusätzlichem Zeilendisplay oder nur zur Ansteuerung eines Zeilendisplays verwendet wird.<br>• Kein – Es gibt keine Kundendarstellung<br>• Nur Zeilendisplay – Es wird ausschließlich die Darstellung an das Zeilendisplay gesendet<br>• Nur Bildschirm – Es wird ausschließlich die Darstellung an das Bildschirmdisplay gesendet<br>• Bildschirm und Zeilendisplay – Es werden beide Darstellungen gesendet |
| Server | Pfad auf dem der Server Daten zur Anzeige auf dem externen Display ablegt<br>Siehe auch [Einrichtung externes Kassendisplay](./einrichtung_der_marktkasse/einrichtung_des_externen_kassendisplays.md) |
| Client | Pfad auf dem der Client Daten zur Anzeige auf dem externen Display liest.<br> |
| AnyBill | Das AnyBill-Profil, das für diese Kasse gelten soll. Siehe auch [Einrichtung externes Kassendisplay](./einrichtung_der_marktkasse/einrichtung_des_externen_kassendisplays.md) |

| Serielles Display |
| --- |
| Typ | • \--- = Nichts<br>• EPSON = Epson Zeilendisplay |
| Zeilenlänge | Länge der Zeilen im Display |

| Registerkarte Anzeige<br>Wird ein externes Kassendisplay verwendet, kann dies hier eingerichtet werden.<br>[Siehe Einrichtung externes Kassendisplay](./einrichtung_der_marktkasse/einrichtung_des_externen_kassendisplays.md) |
| --- |
