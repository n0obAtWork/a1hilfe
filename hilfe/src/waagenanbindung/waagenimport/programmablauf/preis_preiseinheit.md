# Preis, Preiseinheit

<!-- source: https://amic.de/hilfe/preispreiseinheit.htm -->

Kann der Preis nicht gelesen werden, so wird er auf 0 gesetzt.

Konnte keine Preismengeneinheit gelesen werden, so wird sie mit der Mengeneinheit der default-Preismengeneinheit aus PRM_DEFAULT belegt. Eine ggf. nicht lesbare Preiseinheit wird standardmäßig auf den Wert voreingestellt, der aus dem Parameter PRE_DEFAULT gelesen wurde. Wenn nicht lesbar, wird 1.0 eingestellt.

(Zugehörige Positionsparameter: PR_SAx, PRE_SAx, PRM_SAx)

*Preismengeneinheit*

Die Preisengeneinheit wird nach dem Einlesen in gleicher Weise wie die Mengeneinheit konvertiert. Falls keine Mengeneinheit eingelesen werden kann oder eine Konvertierung durch die Parameter MEM_1 bis MEM_5 nicht möglich ist, z. B. wegen Fehlwert oder Inaktivschaltung des betreffenden Parameters, wird diejenige Aeins-Mengeneinheit vorgegeben, die im Parameter PRM_DEFAULT abgelegt ist.

Eine Validierung findet nicht statt.

(Zugehörige Positionsparameter: PRM_SAx)
