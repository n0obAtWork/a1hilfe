# Gesamtbarverkaufssystem (BVS) eröffnen/abschließen

<!-- source: https://amic.de/hilfe/gesamtbarverkaufssystembvserff.htm -->

Die Funktionen für das Barverkaufsystem sind u.a. in der Anwendung Gesamtbarverkauf (Pulldown-Menu: Vorgang / Barvorgänge / Gesamtbarverkauf, Hauptauswahlmenu: Warenwirtschaftssystem / Barvorgänge / Gesamtbarverkauf) zu finden.

Bevor Kassenfunktionen ausgeführt werden können, muss zuerst das Gesamtbarverkaufssystem eröffnet werden (F8).

Das Gesamtbarverkaufssystem selbst ist allerdings nur zu eröffnen, wenn es noch kein geöffnetes Gesamtbarverkaufssystem gibt, d.h., es gibt höchstens ein aktives System (BVS) pro Filiale.

Ebenso ist es nur möglich das Gesamtsystem abzuschließen, wenn dieses System bereits eröffnet ist und es keine noch offenen Kassensitzungen gibt. (F9)

Das Recht, das Gesamtbarverkaufssystem zu eröffnen, liegt bei dem Benutzer, der in der Ahoi.ini unter [Acash2] den Eintrag BVManager=Ja aufweisen kann.

Um zu ermitteln, welche Kassen noch offen sind, wird eine Tabelle angezeigt, die über den Zustand der einzelnen Kassen informiert.

Jede Barverkaufssystemeröffnung bekommt eine eigene fortlaufende Sitzungsnummer.

Beim Abschluss des Barverkaufs werden die Werte aller innerhalb der im Gesamtbarverkaufssystem durchgeführten Kassensitzungen mit der Sitzungsnummer dieses Barverkaufs in die Zeile der Datenbank verprobt, die als noch aktives Barverkaufssystem gekennzeichnet ist.

Weiter besteht die Möglichkeit, Zahlungsmittel (Scheck, Kreditkarte, Bargeld, ...) automatisch beim Tagesabschluss auf Kostenkonten zu verteilen.

Hierzu erforderliche Einstellungen sind:

Der [SPA 355 - Buchung der Zahlungsmittel auf Kostenkto](../../firmenstamm/steuerparameter/kasse_barverkauf/umbuchung_der_zahlungsmittel_auf_konten_spa_355.md) in der Gruppe Kasse/Barverkauf ist auf Ja zu setzen.

In den [Kasseneinstellungen](../kassen_einrichtung/kasseneinstellungen.md) in der Gruppe Konten sind für die Zahlungsmittel Kostenkonten aus der FiBu zuzuweisen.

Der Einrichterparameter Einzelbuchung pro Zahlungsmittel auf der Maske des Kassenabschlusses steuert, ob jedes Zahlungsmittel (z.B. jede EC-Karte) einzeln gebucht werden soll (Ja) oder nur die Summe der Zahlungsmittel einer Zahlungsart gebucht werden soll (Nein).

<p class="just-emphasize">Bemerkung:</p>

Der [Steuerparameter 333 - aut.Buchung von Finanzvorg. in FiBu](../../firmenstamm/steuerparameter/kasse_barverkauf/aut_buchung_von_finanzvorg_in_fibu_spa_333.md) steuert, ob an der Kasse FiBu Buchungen erzeugt werden sollen.

Dieser Steuerparameter übersteuert den Steuerparameter Buchung der Zahlungsmittel auf Kostenkto, d.h. wenn dieser SPA auf Nein steht, werden trotz Spa-Einstellung von Buchung der Zahlungsmittel auf Kostenkto auf Ja keine Buchungen erzeugt.  
    

Bei einer Neueröffnung werden die Vortragswerte der Bargeldbestände aus der letzten Barverkaufssitzung übernommen.

Für die Barverkaufssystemeröffnung / Abschluss ist unter Vorgang / Barvorgänge / Barverkaufssystem Eröffnung/Abschluss ein Pulldown-Menu eingerichtet.  
Über das Hauptauswahlmenu gelangt man über Warenwirtschaft / Barvorgänge / Barverkaufssystem Eröffnung in diese Funktionalität.
