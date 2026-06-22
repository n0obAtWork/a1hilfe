# LVS

<!-- source: https://amic.de/hilfe/lvs.htm -->

In Abhängigkeit von Unterklassen wird im Vorgangsimport der Vorgangsklasse 5150 (LVS) eine Buchung im LVS vorgenommen.

Der Vorgangsimport von LVS-Einträgen erfolgte früher über ein Vorgangsimportmakro für LVS. Dieses wird nicht mehr gepflegt.

Es muss für die Verarbeitung im ImportVorgstamm das Feld „useCS“ auf „1“ gesetzt sein.

| Unterklasse | Bedeutung | Bemerkungen | Pflichtfelder |
| --- | --- | --- | --- |
| 10 | Ladeträgerlokalität | Hier wird ein Ladeträger erstellt, oder wenn er bereits vorhanden ist, wird seine Lokalität verändert. | LadetraegerNr o.ext.Nr. LokalitaetsNr<br>Bediener |
| 20 | Beladung | Hier wird ein Ladeträger mit der gegebenen Ware beladen. | LadetraegerNr o.ext.Nr.<br>LokalitaetsNr<br>Menge<br>Mengeneinheit<br>Bediener |
| 21 | Beladen mit Inventur | Hier wird ein Ladeträger mit der gegebenen Ware beladen.<br>Eintrag ins Bewegungsprotokoll als geplante Inventur<br>Diese Art der Beladung erfolgt an Maschinen oder geeichten Waagen. | LadetraegerNr o.ext.Nr.<br>LokalitaetsNr<br>Menge<br>Mengeneinheit<br>Bediener |
| 30 | Umpacken | Hier wird von einem Ladeträger auf einen anderen umgepackt. Es müssen also zwei korrespondierende Sätze in der Tabelle ImportVorgPositionLVS vorhanden sein. Die jeweils erste Zeile ist die Abgangs-, die zweite die Zugangszeile. | Bediener<br>Artikelid<br>Quell-Ladetraegernummer bzw. ext.Nr<br>Ziel-Ladetraegernummer bzw. ext.Nr<br>Quell-Ladeeinheitsnummer<br>Quell-Ladeeinheitsposition<br>Menge<br>Mengeneinheit |
| 40 | Umbuchung | Hier wird eine Artikelumbuchung vorgenommen. Dazu sind zwei korrespondierende Zeilen notwendig. Die Abgangszeile wird mit TypZuAbgang = 1, die Zugangszeile mit TypZuAbgang=2 gekennzeichnet.<br>Der Ladeträger bleibt erhalten. | Bediener<br>Quell- und Ziel-ArtikelId<br>Quell-Ladeeinheitsnummer<br>Quell-Ladeeinheitsposition<br>Menge<br>Mengeneinheit |
| 50 | Fahrauftrag | Hier wird ein Fahrauftrag für einen Ladeträger generiert | Bediener<br>LadetraegerNr o. ext.Nr<br>LokalitaetsNr<br>(Optional) ListenNr |
| 60 | Geplante Inventur | Menge wird korrigiert – Eintrag ins Bewegungsprotokoll als geplante Inventur auch bei Übereinstimmung | LadetraegerNr o. ext.Nr<br>LadeeinheitsNummer<br>LadeeinheitsPosition<br>ArtikelId<br>Bediener<br>Menge<br>Mengeneinheit |
| 61 | Ungeplante Inventur | Menge wird korrigiert – Eintrag ins Bewegungsprotokoll als ungeplante Inventur | LadetraegerNr o. ext.Nr<br>LadeeinheitsNummer<br>LadeeinheitsPosition<br>ArtikelId<br>Bediener<br>Menge<br>Mengeneinheit |
| 90 | Leeren | Die gegebene Ladeeinheit wird um die gegebene Menge erleichtert. Ist die Menge identisch mit der Menge der Ladeeinheit, so wird die Ladeeinheit entfernt. Ist es die letzte Ladeeinheit, so wird der Ladeträger deaktiviert. | LadetraegerNr o. ext.Nr<br>LadeeinheitsNummer<br>LadeeinheitsPosition<br>Bediener |
