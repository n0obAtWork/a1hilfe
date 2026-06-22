# Systemmeldungs-Vorlage-Pfleger

<!-- source: https://amic.de/hilfe/systemmeldungsvorlagepfleger.htm -->

| **Felder** | |
| --- | --- |
| Name | Eindeutiger technischer Name der Systemmeldung. |
| Aktiv | Ja/Nein  
Bestimmt, ob die Systemmeldung überhaupt aktiv sein soll, d.h. ob die Bedingungen für eine Anzeige beim Programmstart überhaupt geprüft werden sollen. |
| Funktion | Die Funktion, die ausgeführt werden soll, wenn ein User die Systemmeldung klickt.  
Sie können hier private Funktionen anbinden. |
| Beschriftung | Die explizite Beschriftung der Systemmeldung.  
Hinweis: Es handelt sich hierbei nicht um die Beschriftung der Funktion. |
| Statement | Auf Basis dieses Statements wird entschieden, ob eine Systemmeldung durchgeführt werden soll oder nicht.  
Das Statement ist so zu formulieren, dass der Ziel-Alias „wert“ heißt.  
Dann entscheidet der Wert:  
1 bedeutet Systemmeldung anzeigen.  
0 bedeutet keinen Systemmeldung anzeigen. |
| Typ | SQL (Standard)  
JPP ist möglich, aber nicht allgemein! |
