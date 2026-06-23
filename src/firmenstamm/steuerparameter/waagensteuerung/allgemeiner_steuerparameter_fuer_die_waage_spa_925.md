# Allgemeiner Steuerparameter für die Waage(SPA 925)

<!-- source: https://amic.de/hilfe/_SPA_925.htm -->

An diesem Steuerparameter werden Optionen für die Waage eingetragen.

<p class="just-emphasize">Folgende Optionen sind verfügbar:</p>

| Option | Wert |
| --- | --- |
| KREDITLIMITFUNKTION | Hier kann eine Prozedur hinterlegt werden, die die Standard Kreditlimitfunktion an der Waage überschreibt. Mit dem [Steuerparameter 667(Waagenmaske Kreditlimit)](./qualitaetsverarbeitung_in_der_waage_spa_932.md#ueb_SPA667) wird die Kreditlimitprüfung an der Waage an- und ausgestellt. |
| ARTIKELBYQUALITÄTEN | Hier kann eine Prozedur hinterlegt werden, die vor dem Abschließen einer Wiegung aufgerufen wird. Der Prozedur wird die OwaageId als Eingangsparameter übergeben. In dieser Prozedur können dann Daten an dem Owaagesatz auf Relationsebene geändert werden. Nach dem Aufruf der Prozedur wird die Waagenmaske mit den neuen Werten geladen. Beim Abändern der Daten muss sorgfältig vorgegangen werden, damit keine Inkonsistenten Daten erzeugt werden. |
| FLICHTFELDERWAAGENPROZESS | Hier kann eine private Prozedur hinterlegt werden, die bestimmt welche Felder auf der Waagenmaske als [Pflichtfelder](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_drucken.md) in Abhängigkeit des [Prozesses](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/index.md) markiert werden sollen. |
| SILOPOSLOESCHENBEILOESCHEWIEGUNG | Mit dieser Option wird eingestellt, ob beim Löschen von Wiegungen die dazu gehörigen Positionen aus dem Lagerverwaltungssystem / Silo ausgebucht werden soll |
| KONTRAKTABWAHLAUFTRAGSUEBERBUCHUNG | Mit dieser Option wird eingestellt, ob im Auftrag der Kontrakt automatisch abgewählt werden soll, wenn im Auftrag und in der Waage die Menge größer ist als die Kontraktrestmenge, damit die gesamte Kontraktrestmenge in dem daraus resultierenden Lieferschein / Rechnung verfügbar ist.<br>0 = deaktiviert<br>1 = aktiviert |
| WIEGETYPAENDERNBEIROHWARE | Wenn diese Einstellung aktiviert wird, dann wird der Wiegetyp automatisch auf Rohware geändert, wenn bei der normalen Warenausgangs bzw. –eingangs Wiegung ein Rohwarekontrakt zugewiesen wird.<br> <br>0 = deaktiviert<br>1 = aktiviert |
| Qualitätswerte für Nichtüberschreibung (>0) | Nicht aktiv.<br>Hier kann einen mit Komma getrennte Liste von Qualitätswerten angegeben werden, welche bei einem Nullwert nicht überschrieben werden sollen.<br>z.B. „16,17“ |
