# Kontraktverteilung in der Waage

<!-- source: https://amic.de/hilfe/_waagekontraktverteilung.htm -->

Das Waagemodul wurde um die Kontraktverteilung erweitert. Die Kontraktverteilung kann auf der Registerkarte LVS/Silo/Kontrakt vorgenommen werden. Soll der Wiegung nur ein Kontrakt zugeordnet werden, so kann die Kontraktzuordnung wie gewohnt über das Kontraktfeld auf der Registerkarte Wiegungen vorgenommen werden. Ist in der Datentabelle Kontraktzuordnung mehr als ein Eintrag vorhanden, so wird das Kontraktfeld auf der Registerkarte Wiegung gesperrt.

#### Besonderheit

Bei der Vorgangserzeugung wird immer pro Zeile in der Datentabelle Kontraktzuordnung ein neuer Waagensatz erzeugt. Es wird bei der Vorgangserzeugung kein Vorgang erzeugt, der eine Warenposition und N Kontraktzeilen hat.

***Wiegebelege, die eine Kontraktzuordnung mit mehreren Kontrakten haben, können nur aus der Hofliste erzeugt werden***.

#### Datentabellenbeschreibung Kontraktaufteilung

| Feldname | Bedeutung |
| --- | --- |
| Kontraktnummer | In diesem Feld wird die Kontraktnummer eingetragen. |
| Menge | In diesem Feld wird die Menge eingetragen. Sie wird immer in die Mengeneinheit des Kontraktes umgerechnet. Ist die eingegeben Menge kleiner als die Wiegemenge, so wird automatisch eine Position mit der Restmenge angefügt. Die Restmenge wird in der Mengeneinheit der Wiegung dargestellt.<br>Falls die automatische Kontraktaufteilung aktiviert ist und ein zweiter aktiver Kontrakt existiert, welcher die Übermenge fassen kann, wird die Übermenge automatisch auf diesen gebucht.<br>Sollte kein Kontrakt existieren, welcher die komplette Übermenge aufnehmen kann, aber zumindest noch ein Kontrakt der einen Teil hiervon aufnehmen kann, so wird dieser bebucht. Der dann noch verbliebene Rest wird auf den Tagespreis gebucht (Kontrakt 0).<br>Bei einer abweichenden Einheit von Kontrakt 1 zu Kontrakt 2 kann es zu Rundungsfehlern kommen. |
| ME-Bezeich | Mengeneinheit des Kontraktes |
| Laufzeitab | Begin der Laufzeit des Kontraktes |
| Laufzeitbis | Ende der Laufzeit des Kontraktes |
| Kontraktmenge | Kontraktmenge |
| Kontraktrestmenge | In diesem Feld wird die Restmenge des Kontraktes angezeigt. Mit einberechnet werden auch alle Wiegungen, die diesem Kontrakt zugeordnet worden sind, aber aus denen noch kein Vorgang erzeugt worden ist, sowie die eingegebene Menge.<br>Insbesondere wird hier die gewogene Bruttomenge vor Abzug der Qualitäten genutzt. Dies sorgt bei der automatischen Kontraktaufteilung für einen negativen Wert. |
| Kontraktmengeneinheit | Mengeneinheit des Kontraktes |
| Kontraktbezeichnung | Bezeichnung des Kontraktes |

#### Ablauf

Als erstes wird eine neue Wiegung erfasst. Es können [Eingangs](../funktionen_in_der_auswahlliste/wareneingang_wiegung_rohwareneingang_f6_sf6.md) oder [Ausgangswiegungen](../funktionen_in_der_auswahlliste/warenausgang_wiegung_rohwarenausgang_f7_cf7.md) mit Rohware oder Normalware durchgeführt werden.

Nach dem sich die Wiegemaske geöffnet hat, wird ein Kontrakt dem aktuellen Datensatz auf der Registerkarte „Wiegungen“ zugeordnet. Anhand des Kontraktes werden Felder wie Kunde, Artikel vorbelegt.

Dieser ausgewählte Kontrakt wird dann auch schon in die Datentabelle Kontraktaufteilung auf der Registerkarte LVS/Silo/Kontrakt übernommen.

Sobald eine Menge gewogen worden ist, wird die Menge in die erste Zeile in der Kontraktverteilung übernommen. Als Mengeneinheit wird immer die Mengeneinheit des Terminals genommen. Wird ein Kontrakt ausgewählt, der eine andere Mengeneinheit als das Terminal hat, so wird die Menge in die Mengeneinheit des Kontraktes umgerechnet.

Wird eine kleinere Menge als die Wiegemenge eingegeben, so wird in der Datentabelle eine neue Zeile mit der Restmenge eingetragen. Dieser Zeile kann dann wiederum ein Kontrakt zugeordnet werden. Dabei zu beachten gilt, dass nur Kontrakte mit dem gleichen Kunden und dem gleichen Artikel ausgewählt werden können. Es ist auch möglich eine Zeile ohne Kontraktzuordnung zu haben. Es ist jedoch nicht möglich die Wiegemenge auf mehrere Positionen ohne Kontraktzuordnung zu verteilen.

Nach dem die Wiegung abgeschlossen worden ist, kann über die Funktion „[Vorgang erzeugen](../funktionen_auf_der_waagenmaske/vorgang_erzeugen_f6.md)“, „[Rohwarenbeleg erzeugen](../prozess_einrichten/registerkarte_silo.md)“ aus der Hofliste heraus die Wiegung in einen Vorgang gewandelt werden. Es wird für jede Kontraktzuordnung in dieser Datentabelle eine Kopie des Wiegebeleges erzeugt. Die Wiegemenge ist dann die Menge aus der Kontraktzuordnung. Aus diesem neuen Wiegebeleg wird dann per Automatik ein Vorgang erzeugt. Die Eigentliche Wiegung bekommt dann den Status „geteilt gelöscht“.

Um diesen Waagebelege wieder zu aktivieren, müssen erst alle Vorgänge und die dazugehörigen Waagebelege gelöscht werden.

#### Rückabwicklung

Um die Originalwiegung wieder zu aktivieren, muss wie folgt vorgegangen werden.

1. Alle Vorgänge die aus dem Originalbeleg erzeugt werden, müssen storniert werden

2. Der Original Wiegebeleg kann in der Variante „gelöschte Wiegungen“ wieder reaktiviert werden. Dabei werden alle Unterbelege der Wiegung gelöscht. Die Originalwiegung bekommt dann wieder den Status abgeschlossen.
