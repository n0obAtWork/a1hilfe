# App Steuerung(SPA 1039)

<!-- source: https://amic.de/hilfe/_SPA_1039.htm -->

Hiermit werden verschiedene Voreinstellungen für die App vorgenommen. Nach vorgenommenen Änderungen empfiehlt sich ein Neustart des Mandantenservers, sonst kann es passieren, dass Änderungen nicht direkt übernommen werden.

Folgende Einstellungen können getroffen werden (Format: AppSteuerung):

| Aktiv | Schlüssel | Option |
| --- | --- | --- |
| Ja/Nein | Order-Vorgangsklasse | Vorgangsklasse für Aufträge welche mit der A.eins-App erstellt wurden. Wenn nichts eingetragen ist, wird 400 genutzt. |
| Ja/Nein | Order-Vorgangsunterklasse | Vorgangsunterklasse für Aufträge welche mit der A.eins-App erstellt wurden. Wenn nichts eingetragen ist, wird 0 genutzt. |
| Ja/Nein | Lagernummer | Hier steht die Lagernummer des Lagers, auf welches gebucht wird, wenn ein Vorgang über die A.eins-App angelegt wird. (Dieses Lager wird für alle Vorgangsklassen genutzt. Wenn nichts eingetragen ist, wird 0 genutzt. |
| Ja/Nein | Import-Funktion | Prozedur, welche den Vorgangsimport übernimmt. Als Vorlage und als Default-Wert dient die Prozedur „amic_AeinsAppOrder2VIMP“. |
| Ja/Nein | Importierte Vorgänge direkt erstellen | Hier muss der Wert „1“ oder „Ja“ eingetragen werden, damit Vorgänge direkt erzeugt werden. Ansonsten wird ein Eintrag im VIMP angelegt, welcher händisch importiert werden kann. |
