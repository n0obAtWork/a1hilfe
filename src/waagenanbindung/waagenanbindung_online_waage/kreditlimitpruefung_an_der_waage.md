# Kreditlimitprüfung an der Waage

<!-- source: https://amic.de/hilfe/_waage_kreditlimit.htm -->

An der Waage kann schon während der Erfassung einer [Verkaufswiegung](./funktionen_in_der_auswahlliste/warenausgang_wiegung_rohwarenausgang_f7_cf7.md) geprüft werden, ob das Kreditlimit des Kunden überzogen ist. In Abhängigkeit des [Steuerparameters 233 (Kreditlimit-Prüfung)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_spa_233.md) wird das Sperrverhalten übernommen. Dies Bedeutet, dass ab der Stufe Sperren keine [Verkaufswiegungen](./funktionen_in_der_auswahlliste/warenausgang_wiegung_rohwarenausgang_f7_cf7.md) mehr für den [Kunden](../../kunden_und_lieferanten/uebersicht_kunden_und_lieferanten.md) durchgeführt werden können.

Die Kreditlimitprüfung kann für die Waage mit dem [Steuerparameter 667 (Waagemaske Kreditlimit)](../../firmenstamm/steuerparameter/waagensteuerung/qualitaetsverarbeitung_in_der_waage_spa_932.md#ueb_SPA667) separat an- und ausgeschaltet werden. Diese Prüfung findet zurzeit nur bei [Verkaufswiegung](./funktionen_in_der_auswahlliste/warenausgang_wiegung_rohwarenausgang_f7_cf7.md)(Normalware wie Rohware) statt.

Wenn [Steuerparameter 690(Waagenmaske Kreditlimit Dispomenge)](../../firmenstamm/steuerparameter/waagensteuerung/waagemaske_kreditlimit_disponierte_menge_spa_690.md) gesetzt worden ist und ein [Kontrakt](../../kontrakt/index.md) an der Wiegung hinterlegt wurde, dann werden alle noch nicht zu Lieferscheine gewandelten Waagenbelege des Kontraktes zusammen gezählt und vom Kreditlimit abgezogen.

Eine Prüfung der aktuellen Wiegemenge gegen den Markpreis des Wiegeartikels findet noch nicht statt. Dies bedeutet, die aktuelle Wiegung wird nicht zur Kreditlimit Funktion herangezogen, wenn der Wiegung kein [Kontrakt](../../kontrakt/index.md) zugeordnet worden ist.

Die Kreditlimitprüfung wird zurzeit nicht durchgeführt, wenn ein Rohwarenlieferschein aus der Hofliste erzeugt werden soll. Für die Normalware wird die Prüfung auch beim Erzeugen eines Beleges aus der Hofliste durchgeführt.

Die Überprüfung des Kreditlimits passiert nach den folgenden Eingaben.

1. Nach der Kundenauswahl

2. Nach der Kontraktauswahl

3. Vorm Abschließen der Wiegung

4. Vorm Erzeugen des Beleges

5. Nach der Eingabe der Menge

6. Nach Eingabe des Disponiertenmenge

Privatisierung der Kreditlimit Funktion

An dem [Steuerparameter 925(Allgemeiner Steuerparameter Waage)](../../firmenstamm/steuerparameter/waagensteuerung/allgemeiner_steuerparameter_fuer_die_waage_spa_925.md) kann eine private Kreditlimit Prozedur hinterlegt werden. Diese Prozedur wird dann anstelle der Standard-Kreditlimitberechnung an der Waage aufgerufen. Die Eingangsparameter der Prozedur müssen genauso heißen wie an dieser [Stelle](./kreditlimitpruefung_an_der_waage.md#Parameternamen) beschrieben.

Gibt die Prozedur als Fehler eine eins zurück, so wird der Fehlertext auf dem Bildschirm angezeigt. Die Erfassung wird dann in Abhängigkeit des [Steuerparameters 233 (Kreditlimit-Prüfung)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_spa_233.md) gesperrt, oder es erscheint eine Warnmeldung auf dem Bildschirm.

Folgende Parameter werden der Prozedur übergeben

| Parameter | Bedeutung |
| --- | --- |
| in_Owaage_Id | OwaageId des aktuellen Wagensatzes |
| in_KundId | KundId des aktuellen Wagensatzes |
| in_ArtikelId | ArtikelId des aktuellen Wagensatzes |
| in_KtrId | KontraktId des aktuellen Waagensatzes |

Folgende Parameter werden erwartet.

| Parameter | Bedeutung des Rückgabewertes |
| --- | --- |
| Fehler | 0. Das Kreditlimit stimmt die Wiegung kann weiter durchgeführt werden<br>1. Das Kreditlimit ist überschritten und es soll die Meldung angezeigt werden |
| Fehlertext | Meldungstext der auf dem Bildschirm ausgegeben werden soll. |
