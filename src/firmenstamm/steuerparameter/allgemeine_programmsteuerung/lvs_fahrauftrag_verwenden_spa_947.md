# LVS Fahrauftrag verwenden (SPA 947)

<!-- source: https://amic.de/hilfe/_SPA_947.htm -->

In diesem Steuerparameter kann in der Option mit leerem Schlüssel eine Datenbankfunktion hinterlegt werden, die beim Speichern einer Ladeträgerbewegung aufgerufen wird.

Als Eingangsparameter erhält die Funktion die Nummern des Ladeträgers und der Lokalität auf den der Ladeträger soeben bewegt wurde. Es ist nun an der Datenbankfunktion zu entscheiden, ob eine exakte Übereinstimmung oder ein anderes Regalfach des gleichen Regals o.ä. dem Anspruch genügt, den Fahrauftrag als beendet zu kennzeichnen.

Eine Vorlage für diese Funktion finden Sie unter dem Namen „AMIC_DEMO_ErledigeLVSFahrauftrag“.

Der Rückgabewert der Datenbankfunktion ist 1, wenn die Bewegung gültig ist. Dies sollte auch der Standard-Fall sein. Wird 0 zurückgegeben, so wird die eingegebene Bewegung nicht gespeichert!
