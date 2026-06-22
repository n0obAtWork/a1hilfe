# Funktionen Vorgangserfassung Kopf

<!-- source: https://amic.de/hilfe/funktionenvorgangserfassungkop.htm -->

<p class="just-emphasize">Abbruch (F10)</p>

Abbruch der Erfassung, zurück zum Ausgangsmenü.

<p class="just-emphasize">Abschluss</p>

Vorgang wird gespeichert. Zusätzlich besteht die Möglichkeit zur Durchführung eines Sofortdrucks.

<p class="just-emphasize">Abschluss mit Signatur (UMSCHALT+STRG+F8)</p>

Vorgang wird unter Verwendung einer integrierten Signatur abgeschlossen. Zusätzlich besteht die Möglichkeit zur Durchführung eines Sofortdrucks

<p class="just-emphasize">Abschluss / Lieblingsdruckerdruck (STRG+F5)</p>

Diese Funktion ermöglicht es, ausgewählten Vorgängen für den Druck einen anderen als den Standarddrucker zuzuordnen. Zusätzlich kann zum Ausdruck ein anderes Formular durch Markieren der Unterklasse gewählt werden.

<p class="just-emphasize">Abschluss / Nächster Beleg (F6)</p>

Vorgang wird zunächst gespeichert. Zusätzlich besteht die Möglichkeit zur Durchführung eines Sofortdrucks. Waren in der übergeordneten Auswahlliste weitere Belege markiert, so erfolgt ein Wechsel zum nächsten Beleg in der Auswahl.

<p class="just-emphasize">Allgemeine Zuordnung (F9)</p>

Abfrage von Informationen zu diesem Bereich (siehe unten).

![Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_204.jpg "Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.")

Hier werden sowohl generelle, nicht mehr änderbare, sowie änderbare Einstellungen angezeigt. Da Veränderung von Einstellungen auf dieser Maske möglicherweise Preis- und Wertänderungen auslösen können, müssen Änderungen hier mit Bedacht durchgeführt werden.

<p class="just-emphasize">Andere Unterklasse / Andere Vorgangsklasse (UMSCHALT+F11)</p>

Hier kann auf eine andere als die gerade verwendete Vorgangsklasse / Vorgangsunterklasse umgeschaltet werden: Mit einem Vorgangsklassenwechsel von Rechnung zu Angebot wird aus einer Rechnungserfassung ein erfasstes Angebot unter Mitnahme aller Werte; dabei wird der Nummernkreis des Angebotes gezogen. Nach dem Vorgangsklassenwechsel befindet man sich in der Zielvorgangsklasse. Will man weiter Rechnungen erfassen, muss man dahin zurückkehren.

Mit dem Unterklassenwechsel kann z.B. ein anderes Formular gezogen werden. Mit dem Wechsel von Rechnung auf Barverkauf werden allerdings auch Funktionen und Buchungsabläufe, wie Zahlungsverkehr aufgerufen.

<p class="just-emphasize">Anschriften aktualisieren</p>

Steht der Steuerparameter „[Anschriften archivieren?](../../firmenstamm/steuerparameter/kundenstammdaten/anschriften_archivieren_spa_574.md)“ auf „Ja“, so werden in den Vorgängen die zum Zeitpunkt der Vorgangserfassung gültigen Anschriften gespeichert. Wird die Kundenhauptanschrift nach der Erfassung des Vorgangs geändert, so wird die Anschrift im Vorgang nicht automatisch aktualisiert. Die ursprüngliche Anschrift bleibt zunächst im Vorgang bestehen.

Beim Aufruf der Funktion ***Anschriften aktualisieren*** werden die Hauptanschrift, die Anschrift des Rechnungsempfängers, sowie die Anschrift des Zahlungspflichtigen im Vorgang aktualisiert. Der Anwender muss diesen Schreibvorgang gesondert bestätigen. Es besteht zudem die Möglichkeit, speziell für den aktuellen Vorgang erfasste, manuelle Anschriften mit der Kundenhauptanschrift zu überschreiben. Gesondert erfasste Versandanschriften für den aktuellen Vorgang können auf diese Weise allerdings nicht aktualisiert werden.

**Hinweise:**

Ab der Stufe „Rechnung“ kann die Anschrift des Vorgangs nicht aktualisiert werden, wenn der Vorgang über eine abweichende Rechnungsempfänger-Adresse verfügt. In diesem Fall erscheint die Meldung „Hauptanschrift kann nicht aktualisiert werden!“.

Verfügt der Vorgang über eine manuelle Vorgangsanschrift, so erscheint eine Abfrage, ob die manuelle Vorgangsanschrift mit der Kundenhauptanschrift überschrieben werden soll:

![Ein Bild, das Text, Screenshot, Schrift, Reihe enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_205.png "Ein Bild, das Text, Screenshot, Schrift, Reihe enthält. KI-generierte Inhalte können fehlerhaft sein.")

Wird die Frage mit „Nein“ beantwortet, so werden die Anschrift des Rechnungsempfängers und/oder die Anschrift des Zahlungspflichtigen trotzdem aktualisiert, wenn diese nicht mit der manuellen Vorgangsanschrift übereinstimmen. Am Ende erscheint eine Meldung, welche Anschriften insgesamt aktualisiert wurden. Im Beispiel:

![Ein Bild, das Text, Schrift, Screenshot, Software enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_206.png "Ein Bild, das Text, Schrift, Screenshot, Software enthält. KI-generierte Inhalte können fehlerhaft sein.")

Die Änderungen werden erst nach dem Speichern des Vorgangs wirksam.

<p class="just-emphasize">Daten neu anzeigen (STRG+F6)</p>

Die Daten der UFLD-Felder, des Infofeldes und des Adressfeldes werden neu angezeigt.

<p class="just-emphasize">Etiketten (Formulardruck)</p>

Es kann ein Formular mit Informationen des Vorgangsstamms gedruckt werden. Hierzu muss im Formulareinrichter ein Formular des Typs 4 (Kundenetikett) eingerichtet sein.

<p class="just-emphasize">Gesamtsummen (UMSCHALT+F10)</p>

Anzeige der Endsummen des aktuellen Beleges, vorrangig Netto, Warenwert, Zu-/Abschläge und Steuern. Ferner werden Informationen zu Mengeneinheiten und Gebinden zusammengestellt.

<p class="just-emphasize">Kunden anzeigen (F4)</p>

Von hier kann in den Kundenstamm verzweigt werden, um dort ggf. eine Anschrift zu ändern oder einen Kunden neu zu erfassen. Ebenso ist es möglich, über die Menüleiste Kunden, Artikel etc. während der laufenden Erfassung einzugeben.

<p class="just-emphasize">Kunden – Etikett (STRG+F11)</p>

Für einen einzelnen oder mehrere ausgewählte Kunden können Etiketten gedruckt werden.

<p class="just-emphasize">Kunden – Verkaufsauswertung (UMSCHALT+F12)</p>

Es wird in die Verkaufsauswertung (VKA) mit der Nummer des bearbeiteten Kunden verzweigt.

<p class="just-emphasize">Kundeninfo KUI (STRG+F10)</p>

Anzeige der Informationen aus dem individuellen Kundeninformationssystem für den ausgewählten Kunden.

<p class="just-emphasize">Kundennummer wechseln</p>

Möglichkeit zur Bestimmung eines anderen Kunden für die Vorgangserfassung.

<p class="just-emphasize">Lagerwechsel</p>

Wechsel von Lagernummer und/oder Lagerplatznummer.

<p class="just-emphasize">Manuelle Adresse (UMSCHALT+F6)</p>

Hier kann eine vom Kundenstammsatz abweichende Anschrift, nur für den aktuellen Vorgang, hinterlegt werden.

<p class="just-emphasize">Manuelle Versandadresse</p>

Hier kann eine vom Kundenstammsatz abweichende Versandanschrift, nur für den aktuellen Vorgang, hinterlegt werden.

<p class="just-emphasize">Positionsteil (F5)</p>

Wechsel in den Positionsteil.

<p class="just-emphasize">Schnellerfassung (STRG+F7)</p>

In der Schnellerfassung können für den aktuell gewählten Kunden Positionen, bestehend aus Menge, Artikelnummer und Preisinformationen in Tabellenform schnell erfasst und gespeichert werden.

<p class="just-emphasize">Stapelende</p>

Aktueller Vorgang wird zunächst gespeichert. Zusätzlich besteht die Möglichkeit zur Durchführung eines Sofortdrucks. Waren in der übergeordneten Auswahlliste weitere Belege markiert, so wird deren Verarbeitung abgebrochen und dann zur Auswahlliste zurückgekehrt.

<p class="just-emphasize">Steuer (F11)</p>

Aufstellung wesentlicher Steuerinformationen des Beleges, vorrangig Nettobetrag, Steuerbetrag und anzuwendende Steuersätze.

<p class="just-emphasize">Texte / Kommentare (STRG+F8)</p>

Hier werden Texte und Kommentare für den Rechnungskopf und -abschluss automatisch oder manuell abgerufen: Im Feld “Eingabe” werden nacheinander Informationen für die Stopppositionen abgefragt (siehe dazu “[Textbausteine](../../zusatzprogramme/vorgangstexte/anwendung_vorgangstexte/vorgangstext_pfleger/textbaustein_pfleger.md)”). Für manche Vorgangstypen sind Texte/Kommentare allerdings nicht vorgesehen.

<p class="just-emphasize">Tourzuordnung</p>

Der Vorgang wird einer Tour zugeordnet. Siehe dazu Tourverwaltung.

<p class="just-emphasize">Vorschau Druck (UMSCHALT+F5)</p>

Die spätere Druckausgabe des aktuellen Vorgangs wird angezeigt.

<p class="just-emphasize">Warengruppen</p>

Übersicht der vorhandenen Warengruppen.

<p class="just-emphasize">Zahlungsbedingung (F8)</p>

Anzeige und Bearbeitungsmöglichkeit der Zahlungsbedingung beim Vorgangsabschluss.
