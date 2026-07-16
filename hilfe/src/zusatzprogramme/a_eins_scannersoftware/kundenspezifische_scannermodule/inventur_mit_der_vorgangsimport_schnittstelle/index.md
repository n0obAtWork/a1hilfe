# Inventur mit der Vorgangsimport Schnittstelle

<!-- source: https://amic.de/hilfe/_inventur_mitvimp.htm -->

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

Um eine Inventur mit dem Scanner über die [Vorgangsimport Schnittstelle](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) zu erfassen muss folgendes eingerichtet werden.

Folgende Steuerparameter müssen eingerichtet werden

1. [Steuerparameter 727](../../../../firmenstamm/steuerparameter/scanner/ean8_code_wird_als_solcher_erkannt_auch_wenn_dieser_nicht_gu.md)

2. [Steuerparameter 728](../../../../firmenstamm/steuerparameter/scanner/ean13_code_wird_als_solcher_erkannt_auch_wenn_dieser_nicht_g.md)

3. [Steuerparameter 801](../../../../firmenstamm/steuerparameter/scanner/private_scannerprozedur_spa_801.md)

4. [Steuerparameter 842](../../../../firmenstamm/steuerparameter/scanner/html_anzeige_beim_scanner_spa_842.md)

Folgende [Scancodes](./beispiel_scancodes_fuer_die_inventur.md) müssen ausgedruckt auswerden.

#### Ablauf der Inventur

Um die Daten für die Inventur aufzunehmen muss als erstes der Scancode IV mit dem Scanner erfasst werden. Der Scancode IV startet nicht die Inventur, sondern nur den Erfassungsblock. Es ist zu empfehlen, pro Regal ein Erfassungsblock zu starten. Wird der Scancode IV während eines offenen Blockes ein zweites Mal gescannt, so kommt die Meldung, dass es noch eine offene Inventurerfassung gibt. Jetzt hat man zwei Möglichkeiten, entweder es werden weitere Inventurdaten erfasst, oder mit nochmaligen Scannen des Scancodes werden alle erfassten Daten im [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) auf gelöscht gesetzt, und ein neuer Inventurblock wird gestartet.

Stürzt die Scannersoftware ab, oder der Scanner muss neugestartet werden, oder der Akku des Scanners ist leer, so kann nach dem Neustart der Software mit dem aktuellen Block weitergearbeitet werden. Dazu braucht man nur einen Artikel erfassen oder eine Menge eingeben. Fällt bei der Erfassung des Scanners das WLAN aus, so werden alle erfassten Scancodes zwischen gespeichert, bis ein rotes X angezeigt wird. Ist das WLAN wieder vorhanden, so werden durch drücken auf das X die Daten übertragen. Wird die Software auf dem Scanner neu gestartet, wenn das X Angezeigt wird so werden die erfassten Daten, die im Speicher sind nicht neu übertragen.

Nach dem erfassen des Scancodes IV können jetzt die Artikel erfasst werden. Bei der Erfassung eines Artikels wird dieser mit der Menge 1 vorbelegt. Dies gilt auch für Gebindeartikel hier wird die Gebindeanzahl mit 1 vorbelegt und die Ergebnismenge wird in die Menge geschrieben. Nach dem Scannen des Artikels wird die Menge eingegeben. Wenn von dem erfassten Artikel nur einer vorhanden, so braucht man die Menge 1 nicht eingeben, sondern kann mit dem nächsten Artikel fortfahren. Die Suche des Artikels funktioniert per EAN oder per Manuelle Eingabe der ArtikelNummer.wenn die Artikelnummer numerisch und mehr als sieben Stellen hat.

Wird ein Artikel beim Erfassen nicht gefunden, so wird dieser trotzdem mit in die [Vorgangsimport Schnittstelle](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) übernommen. Dort kann dieser vor dem Einspielen der Inventur bearbeitet werden.

Des Weiteren kann eine Partie erfasst werden. Dazu wird die Partiebezeichnung im EAN 128 Codiert mit der AI 10 erfasst.

Um eine Erfassungsblock für die Inventur abzuschließen, muss dieser mit dem Scancode IVENDE abgeschlossen wird.

Jeweils ein Erfassungsblock bekommt in der Vorgangsimport Schnittstelle eine eigenständige ÜbernahmeId, diese hat nach der Erfassung den Status 2 (Bereit).

#### Einspielen der Inventur

Wurden alle Inventurdaten mit den Scannern erfasst, so können diese jetzt in die offene Inventur eingespielt werden. Bevor die Daten eingespielt werden, muss die [Inventur](../../../../abschluesse_inventur/inventur/index.md) eingerichtet und eröffnet sein.

Es werden alle Daten mit dem Funktionsaufruf „Standardvorgang erzeugen in die Inventur eingespielt, die in der Auswahlliste im [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) ausgewählt wurden. Nach der erfolgreichen Einspielung der Daten wird der Status auf „erledigt“ gesetzt. Konnten bestimmte Positionen nicht eingespielt werden, so werden diese als Fehlerhaftmarkiert. Diese müssen dann korrigiert werden und der Status auf „Bereit“ zurück gestellt werden.

In dem Feld Zusatzfeld1 wird der Inventurbeleg gespeichert und angezeigt und in dem Feld Zusatzfeld2 wird die Position in dem Inventurbeleg angezeigt.

<p class="siehe-auch">Siehe auch:</p>

- [Beispiel Scancodes für die Inventur](./beispiel_scancodes_fuer_die_inventur.md)
