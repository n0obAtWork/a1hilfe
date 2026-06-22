# Einrichtung

<!-- source: https://amic.de/hilfe/einrichtung4.htm -->

• Für die Verwendung dieses Moduls ist die Lizenz „Permanente Inventur ([Steuerparameter 902](../../firmenstamm/steuerparameter/lizenzen/permanente_inventur_lizenz_spa_902.md)) notwendig.

• Im [SPA 1045 – Permanente Inventur](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/permanente_inventur_mit_lvs_spa_1045.md) muss eingerichtet werden:

o Anzahl an Tagen für eine Zählung (0= aktuelles Wirtschaftsjahr)

o Anzahl von Artikelzeilen pro Inventurbeleg (Vorgangsklasse 5055)

• [SPA 1072 – Bewertungsverhalten permanente Inventur](../../firmenstamm/steuerparameter/mde_prozeduren_einzelhandel_spa_1059/permanente_inventur_bewertungsverhalten_spa_1072.md)

Wir empfehlen, diesen Wert auf 1 – keine Bewertung durch Bestandsbeleg einzustellen.

• [SPA 1118 – permanente Inventur besuchte Lagerplätze](../../firmenstamm/steuerparameter/mde_prozeduren_einzelhandel_spa_1059/permanente_inventur_besuchte_lagerplaetze_spa_1118.md) – wird für die LVS-Vollständigkeitsprüfung gebraucht.

• Die FiBu-Konten der FiBu-Buchungen müssen in der Erlöskennzifferzuordnung eingerichtet werden.

• Wird für die Erfassung von permanenten Inventuren ein Scannersystem verwendet, so muss im Lagerstamm des zu zählenden Lagers der Wert „permanente Inventur“ gesetzt werden. Mittels dieses Kennzeichens kann das LVS-Lager ermittelt werden, dessen Regalbesuche auf Vollständigkeit geprüft werden.

• Ggf. ist eine Anbindung des Scannersystems an die Makroschnittstelle zu individualisieren.

• Pflege des Kennzeichens „permanente Inventur“ im Artikel

Es wird dringend empfohlen während einer laufenden Inventur dieses Kennzeichen nicht zu verändern.

• Folgende AF-Formate sind zu ergänzen:

o VorgKlXXXS

o VorgKlXXS

o VorgKlasseTx

o VoKlasse

o VorgKlasse

o AF_Vorgang

Für die Vorgangsklassen

| Klassen-nummer | Name | Kürzel<br>(Vorschlag) | Bedeutung |
| --- | --- | --- | --- |
| 5055 | Inventurdifferenzbeleg | IVD | Unter dieser Vorgangsklasse können Inventurbelege eingegeben werden. Diese enthalten sowohl eine Mengen- als auch eine Wertkorrekturen. |
