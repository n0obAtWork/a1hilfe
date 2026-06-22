# Ladescheinbearbeitung

<!-- source: https://amic.de/hilfe/_ladescheinbearbeitung_scanner.htm -->

Als erstes müssen die [Scancodes](../anwendung_scanner_in_aeins/status_der_scanner.md) „LAD“ und „LADENDE“ eingerichtet werden. Diese können bequem auch mit der Funktion [Standard Einstellung Scancodes](../anwendung_scanner_in_aeins/scanner_scancodes_bearbeiten_modus/standard_einstellungen_scancodes.md) eingerichtet werden. Danach können die [Scancodes](../anwendung_scanner_in_aeins/status_der_scanner.md) noch bearbeitet werden.

Um eine Ladescheinliste auszudrucken wurde in der Anwendung AMIC Etikettendruck in der Variante „Vorlagen AMIC Etikettendruck Reporte“ eine Vorlage erstellt. Anhand dieser Vorlage kann dann eine Pickliste erstellt werden. Auf dieser Liste können dann auch die erforderlichen Scancodes für das Ladescheinstarten angedruckt werden.

Ablauf

Mit diesem Modul können [Aufträge](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md), die zu einem Ladeschein umgewandelt worden sind, bearbeitet werden. Aus diesen Ladeschein wird dann ein [Lieferschein](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lieferschein/index.md) erzeugt, und die Lieferscheinmenge wird dann per Teildisposition vom Auftrag abgebucht.

Dabei ist zu beachten, dass der Scanner keine Partieverteilung kennt. Dies bedeutet, dass es immer nur eine 1 zu1 Beziehung geben kann. Sollen mehrere Partien eines Artikels in dem Auftrag verarbeitet werden, so muss für jede Partie eine neue Position im Auftrag angelegt werden.

Als erstes muss ein Auftrag erfasst werden. Aus diesem Auftrag kann in der Anwendung Aufträge in der Variante „Aufträge mit Positionen“ mit der Funktion „Ladeschein zusammenstellen“ ein Ladeschein über alle markierten Positionen erstellt werden. In der Erfassungsmaske kann dann ein Ladeschein mit einem Dummykunden angelegt werden, wenn der Ladeschein aus mehreren Aufträgen mit unterschiedlichen Kunden zusammengestellt worden ist.

Als Vorbelegung kann auch im Einrichterparameter eine Kundennummer hinterlegt werden. Als zweite Möglichkeit kann in der Anwendung „Ladeschein“ ein neuer Ladeschein angelegt werden. In diesem Ladeschein kann auf der Positionsebene über die Funktion „Schnelle Teildisposition“ ein Artikel aus einem Auftrag hinzugefügt werden.

Ist der Ladeschein erstellt worden, so kann dieser jetzt per Scanner abgearbeitet werden. Dazu wird der Start Befehl „**LAD Ladescheinnummer**“ gescannt. Es ist dabei zu beachten, dass zwischen dem „**LAD**“ und der Vorgangsnummer des Ladescheins ein **Leerzeichen** ist.

Jetzt können die einzelnen Positionen des Ladescheins erfasst werden. Dabei ist zu beachten, dass die Suche des Artikels immer erst per EAN Code passiert, wird zu der Position noch eine Partie erfasst, so wird im Ladeschein nach der Kombination Artikel Partie gesucht. Existiert ein Artikel mehrmals ohne Partie in diesem Ladeschein so wird die erste Artikelposition bebucht. Wird die Ladeschein Verarbeitung mit dem Lagerverwaltungssystem gekoppelt, so wird in Leeren- oder Füllen Funktion die Menge an die erfasste Position geschrieben. Wird zum Beispiel Ware von einem Ladeträger entnommen und auf einen Mischladeträger gefüllt, so muss der Schalter „Füllen ohne Änderung“ im [Scancode](../anwendung_scanner_in_aeins/status_der_scanner.md) auf Ja gestellt werden.

Mit dem Befehl „**STORNO**“ können einzelne Position wieder rückwärts gelöscht werden.

Ist ein Ladeschein abgearbeitet worden, so wird mit dem Befehl „**LADENDE**“ die Abarbeitung des Ladescheins beendet.

Nach der Erfolgreichen Abarbeitung des Ladescheins werden automatisch Lieferscheine zu den dazugehörigen Aufträgen erstellt. Diese können dann per Automatik ausgedruckt werden, dazu ist ein Bedieneranzulegen „LALIDRU“ diesem Bediener kann dann ein Standarddrucker zugewiesen werden, auf diesem Drucker werden dann die Lieferschein ausgedruckt werden.

Die erfassten Daten werden in der Anwendung [Vorgangsimport](../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) [**VIMP**] gespeichert.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

Lagerveraltungssystem

Dieses Modul kann mit dem Lagerverwaltungssystem gekoppelt werden. Dazu müssen im Scancode LAD die Lagerverwaltungsspezifischen Funktionen wie z.B. FUELLEN, LEEREN zugelassen werden. Des Weiteren muss nach der Ladescheinnummer im Scancode die Lokalität angedruckt werden.

Beispiel

LAD 4711 4712. Dies bedeutet der Ladeschein 4711 wird auf der Lokalität 4712 bearbeitet.

Gebinde Informationen

Die Gebinde Informationen werden aus dem Ladeschein genommen und dann mit in ImportVorgPosition gespeichert.
