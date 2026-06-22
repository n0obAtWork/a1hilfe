# Lagernummernänderung bei Standard-Teildisposition

<!-- source: https://amic.de/hilfe/_LageraendTeildispo.htm -->

Für die Teildisposition kann man eine automatische Lageränderung unter [FRZ] im Register Abwicklung im Feld [‚Lageränderung bei Teildisposition automatisch aus Vorgangslagernummer‘](../../formularzuordnung/abwicklung.md#Abwicklung_Behandlungsschema) aktivieren.  
Setzt man dieses für den Zielvorgang auf Ja, dann wir bei der Standard-Teildisposition automatisch eine Lageränderung vorgenommen, wenn das Quelllager nicht dem Lager des Vorgangs entspricht.  
[SPA 316 ‚Teildisposition nur aus aktivem Lager‘](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/teildisposition_nur_aus_aktivem_lager_spa_316.md) sollte auf Nein gesetzt sein, damit das Auswahlfenster bei der Standard-Teildisposition alles Verfügbare anzeigt.  
Das zugehörige [Behandlungsschema](./behandlungsschema_lagernummernaenderung.md) wird unter [FRZ] im Register Abwicklung eingegeben. Wurde dort keines ausgewählt, dann wird als Standard das ausgelieferte Behandlungsschema Lagernummernwechsel verwendet.
