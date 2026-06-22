# Hausbanken

<!-- source: https://amic.de/hilfe/hausbanken.htm -->

Hauptmenü > Finanzbuchhaltung > Stammdaten > Hausbanken

Direktsprung **[BNKH]**.

Hausbanken sind die eigenen Banken des jeweiligen Man­danten für den Zahlungsverkehr. Sie werden im Automatischen Zahlungsverkehr, von e-Clearing, der Wechselbuchhaltung und vom Zahlungsverkehr Bank verwendet. Hier werden alle Informationen hinterlegt, die für die Abwicklung der Bankgeschäfte notwendig sind:

| | Allgemein |
| --- | --- |
| | Hausbanknummer | Eine frei zu vergebende eindeutige Nummer, über die dann auf die Hausbank verwiesen wird.<br> |
| Währung | Währung, in der das Hausbankkonto geführt wird.<br> |
| | IBAN<br> | Ab dem 28.01.2008 wird für den Zahlungsverkehr SEPA (Single Euro Payments Area) eingeführt. Dieses Verfahren benötigt die IBAN (International Bank Account Number). Diese kann/muss in dem Feld IBAN eingetragen werden. Es erscheint folgender Hinweis, wenn keine IBAN eingetragen wird:<br> <br>**Wenn sie den Zahlungsverkehr ab dem 20.01.2008 auf SEPA-Basis laufen lassen, müssen sie die IBAN eintragen.**<br> <br>Hat man eine IBAN eingetragen, so wird aus dieser (für Deutschland, Österreich und Belgien) die Bank und Kontonummer generiert. Werden diese Daten nicht vorgeschlagen, so ist entweder die IBAN nicht nach dem Standardschema aufgebaut, falsch eingegeben oder die Stammdaten der Banken sind nicht korrekt gepflegt (z.B. nicht eingetragener Staat) .<br>Anschließend wird sie über ein Prüfziffernverfahren getestet. Bei fehlerhafter Nummer erscheint folgende Fehlermeldung:<br> <br>**Die Prüfziffernberechnung ergibt, dass diese IBAN falsch ist.**<br> <br>Diese Meldung ist nur eine Warnmeldung. Änderungen werden trotz Meldung abgespeichert.<br>Der Test der IBAN kann entweder für einzelne [Banken](./bankenstamm.md) oder global per [Steuerparameter](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) abgeschaltet werden.<br> |
| | Bank | Verweis auf die im [Bankenstamm](./bankenstamm.md) festgelegte Bank. Man kann direkt die Bezeichnung oder die BLZ eingeben. In der F3-Auswahl kann zusätzlich auch nach BIC oder der Banknummer gesucht werden.<br> |
| | BIC / BLZ | Hier werden der BIC(Bank Identifier Code) und die Bankleitzahl der Bank angezeigt, wie sie im Bankenstamm hinterlegt sind.<br> |
| | Bankkonto<br> | Kontonummer des Bankkontos. Hier wird die 10-stellige Kontonummer erwartet. Hat man bis keine IBAN angegeben, so wird soweit eine Kontonummer eingetragen wurde, für deutsche, österreichische und belgische Banken eine IBAN vorgeschlagen. Diese vorgeschlagene IBAN ist in jedem Fall zu überprüfen, da die IBAN ausschließlich von der kontoführenden Bank vergeben wird.<br>Die Vorbelegung der IBAN kann per [Steuerparameter](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_vorbelegung_nach_standardverfahren_spa_896.md) abgeschaltet werden.<br> |
| | Matchcode | Zusätzliche Möglichkeit ein Suchkriterium festzulegen.<br> |
| | Datenträgeraustausch |
| | SEPA-Version | Da das SEPA-Verfahren in ständiger Entwicklung ist, existieren bei unterschiedlichen Banken auch unterschiedliche Versionen. Welche Versionen implementiert sind findet man unter [SEPA-Kennzeichen im Hausbankenstamm](../sepa/sepa_kennzeichen_im_hausbankenstamm.md)<br> |
| | SEPA-Zeichensatz | 1) In den XML-Dateien darf nur ein eingeschränkter Datensatz verwendet werden. Hier kann A.ens zwischen zwei System unterscheiden:<br>• Einfacher Zeichensatz mit 0-9, a-z, A-Z sowie den Sonderzeichen ":", "?", ",", "-", " ", "(", "+", ".", ")", "/". Umlaute werde umgewandelt: Beispiel Ä=AE.<br>• Erweiterter Zeichensatz. Wie der einfache Zeichensatz, aber zusätzlich mit den Zeichen "&", "\*", "$", "%". Umlaute werden NICHT umgewandelt.<br> |
| | Sammel-/Einzelaufträge | Das SEPA-Verfahren sieht ein Kennzeichen vor, mit dem man der Bank mitteilen kann, ob man Sammel- oder Einzelaufträge im Kontoauszug der Bank erhalten möchte. In A.eins kann das Verhalten pro Hausbank festgelegt werden. Unter Umständen wird das Kennzeichen vom Institut nicht ausgewertet, da jede Bank die Verarbeitung unterschiedlich handhabt (die Deutsche Bank ignoriert z.B. dieses Kennzeichen und führt immer Sammelbuchungen aus.).<br>• laut Vereinbarung mit der Hausbank. Diese ist die Standardeinstellung.<br>• Sammelbuchung durchführen<br>• Einzelbuchung durchführen<br> |
| | Auftraggeber DTA | Beim Datenträgeraustausch wird der hier eingetragene Wert als Auftraggeber übermittelt. Es ist dabei unabhängig, ob es sich um den DTA im SEPA-Format, im alten Format oder um Auslands-DTA handelt.<br> |
| | AuslandsDTA Name<br>AuslandsDTA Str.<br>AuslandsDTA Ort | Diese Felder werden für den Auslandszahlungsverkehr und im SEPA-DTA verwendet. |
| | Ansprechpartner | Der Ansprechpartner beim Auslandszahlungsverkehr mit übertragen.<br> |
| | **Konten Finanzbuchhaltung** |
| | Finanzbuchhaltung | Dieses Konto wird beim Erstellen der Belege im auomatischen Zahlungsverkehr und beim e-Clearing verwendet.<br> |
| | Verrechnungskonto | Dieses Konto wird bei Bankbuchungen aus der Wechselbuchhaltung verwendet.<br> |
| | Besitzwechsel<br>Wechselobligo<br>Wechselobligo | Diese Konten werden von der [Wechselbuchhaltung](../../wechselbuchhaltung/index.md) verwendet. |
| | **Nummernkreis** |
| | Schecknummer | Man kann den [Scheckdruck](../zahlungen_bearbeiten/scheckdruck.md) so Einrichten, dass die Schecknummer aus einem Nummernkreis gezogen wird. Dieser muss hier hinterlegt werden.<br> |
| | Zahlungsverkehr Bank | Beim Erstellen der Fibu-Belege werden Belegnummern vergeben. Im Modul [e-Clearing](../../e_clearing/index.md) – dort muss unter Optionen die Verwendung dieses Nummernkreises aktiviert werden - und beim Erfassen von Bankzahlungen wird dieser Nummernkreis verwendet.<br> |
| | Autom.Zahlungsverkehr | Dieser Nummernkreis wird beim Erstellen der Fibubelege aus dem automatischen Zahlungsverkehr verwendet. Dort muss der Einrichterparameter „Nummernkreis der Hausbank verwenden“ auf **Ja** stehen. Ist die Option „Schecknummer als Belegnummer vergeben“ beim Übertrag in die Primanota gesetzt und eine Schecknummer vergeben, dann wird nach wie vor die Schecknummer verwendet.<br> |
| | **Obergrenze/Limits** |
| | Kredit-Limit | Wird von A.eins zurzeit nicht ausgewertet. Wird jedoch beim Erfassen von Zahlungen mit angezeigt.<br> |
| | Haben-Obergrenze | Wird von A.eins zurzeit nicht ausgewertet.<br> |
| | Wechsel-Kontingent | Wird von A.eins zurzeit nicht ausgewertet.<br> |
| | **Anschrift** |
| | Anschrift | Anschrift der Bank. Weitere Anschriftenmerkmale können über F10 gepflegt werden.<br> |
| | **e-Clearing** |
| | Abweichendes Bankkonto e-Clearing | Normalerweise sind Kontonummern auf 10 Stellen normiert. Einige Banken übermitteln in den Kontoauszügen für e-Clearing manchmal längere Nummern, in denen z.B. die Filialnummer noch enthalten ist. Diese längere Kontonummer kann hier eingetragen werden und wird dann bei der Bestimmung der Hausbank im Modul e-Clearing verwendet.<br> |
| | Auftraggeber aus Verwendungszweck verwenden (MT940) | Diese e-Clearing-Option gilt nur für das Dateiformat MT940 – Swift. Wenn eine Bank den Auftraggeber nicht in dem dafür vorgesehenen Feld liefert, so kann man hier einstellen, dass stattdessen die erste Zeile des Verwendungszwecks als Auftraggeber verwendet wird. Diese Option wird nur angewendet, wenn das Feld Auftraggeber leer ist. Zusätzlich kann im Pfleger für die Geschäftsvorfallcodes für einzelne Geschäftsvorfälle diese Option abgeschaltet werden.<br>Der Auftraggeber wird für die Bestimmung des Kontos benötigt.<br> |
| | **eRechnung** |
| | Bei eRechnungsversand verwenden | Wenn dieses Feld auf „ja“ steht, wird das entsprechende Konto bei der Erstellung einer eRechnung verwendet, andernfalls nicht.<br>Dieses Feld ist nur bei aktiver eRechnung-Lizenz freigeschaltet.<br> |
| | **Bemerkungen** |
| | Bemerkungen | Frei zu verwendendes Textfeld.<br> |
| | | |
