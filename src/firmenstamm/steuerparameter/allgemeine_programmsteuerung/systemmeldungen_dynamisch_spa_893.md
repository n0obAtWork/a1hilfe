# Systemmeldungen dynamisch(SPA 893)

<!-- source: https://amic.de/hilfe/_SPA_893.htm -->

Komplexer Steuerparameter.

Steuert die dynamische Wiederholung der Systemmeldungsprüfung.

Der Schlüssel wird aus dem Format „SYSMSGdyn“ geholt.

| Aktiv | Schlüssel / Parameter | Option / Zeitangaben |
| --- | --- | --- |
| **JA / NEIN**  
Entscheidet ob der folgende Schlüssel ausgewertet werden soll oder nicht. | **AnAus**  
Entscheidet, ob die fortwährende dynamische Prüfung der Systemmeldungen überhaupt stattfinden soll.  
Entscheidungskriterium ist hier der Wert unter „**Aktiv**“. | . |
| **JA / NEIN**  
Entscheidet ob der folgende Schlüssel ausgewertet werden soll oder nicht. | **Wartezeit**  
Diese Zeit wird zwischen den Prüfungen der Systemmeldungen gewartet | Wert in Sekunden angeben.  
Voreinstellung sind 60 Sekunden. |
| **JA / NEIN**  
Entscheidet ob der folgende Schlüssel ausgewertet werden soll oder nicht. | **Verfallszeit**  
Nach Ablauf dieser Zeitdauer verfällt eine auf vormals gut erkannte Systemmeldung und wird erneut geprüft | Wert in Sekunden angeben.  
Voreinstellung sind 180 Sekunden. |
