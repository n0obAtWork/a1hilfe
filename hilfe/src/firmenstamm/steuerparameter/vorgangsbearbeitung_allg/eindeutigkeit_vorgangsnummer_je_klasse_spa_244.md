# Eindeutigkeit Vorgangsnummer je Klasse(SPA 244)

<!-- source: https://amic.de/hilfe/_SPA_244.htm -->

Dieser Steuerparameter entscheidet, in welcher Form Nummern im Reservierungssatz des Vorgangs abgespeichert werden. Ist der Steuerparameter auf Gesamt eingestellt, wird in das Feld JahrNummer eine 0 eingetragen und dann die nächste freie Belegnummer gesucht - gemäß Nummernkreiszuordnung dieser Vorgangsklasse. Ist der SPA auf Jahr eingestellt, wird in das Feld JahrNummer das aktuelle Jahr eingetragen und dann die nächste freie Belegnummer gesucht- gemäß Nummernkreiszuordnung dieser Vorgangsklasse; dabei kann im folgenden Jahr wieder mit der ersten Nummer gemäß Nummernkreis begonnen werden. Daher sollte dieser Steuerparameter möglichst nicht geändert werden, nachdem man sich für eine Vorgehensweise entschieden hat.

*ACHTUNG: Innerhalb einer Vorgangsklasse wird jeweils die nächste freie Nummer gesucht. Es ist also nicht möglich, in einer Vorgangsklasse für Vorgänge in verschiedenen Unterklassen dieselbe Belegnummer zu vergeben. Man sollte durch Anlegen disjunkter Zählergrenzen sauber zwischen Belegnummern verschiedener Vorgänge unterschiedlicher Vorgangsunterklassen innerhalb einer Vorgangsklasse trennen.*
