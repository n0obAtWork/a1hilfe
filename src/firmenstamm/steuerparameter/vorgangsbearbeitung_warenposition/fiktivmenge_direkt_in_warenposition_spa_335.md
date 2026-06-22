# Fiktivmenge direkt in Warenposition(SPA 335)

<!-- source: https://amic.de/hilfe/_SPA_335.htm -->

Mit diesem Steuerungsparameter kann das Eingabefeld ‚Fiktive Menge‘ auf der Warenpositionsmaske aktiviert werden. Währen dieser SPA eine allgemeine Einstellung darstellt, kann unter FRZ / SPA diese Einstellung auch für jede Vorgangsunterklasse individuell eingestellt werden.

Bei Aktivierung der Fiktiven Menge wird der Warenposition mitgeteilt, dass sich diese Position auf eine größere Menge bezieht, von der nur ein Teil hier erfasst wird. Alle Mengenbezüge bezüglich Zu/Abschlägen oder Frachten beziehen sich dann auf die fiktive Menge und nicht auf die tatsächlich in dieser Warenposition erfassten Menge.

Beispiel:

Ein Kunde bekommt einen mengenbezogen Abschlag ab der Menge 1000 kg, aktuell werden aber nur 300 kg geliefert. Trägt man in der fiktiven Menge 1000 kg ein, wird der Abschlag trotz der nicht erfüllten realen Mengengrenze gewährt. Das System überprüft jedoch nicht, ob die fiktive Menge irgendwann erreicht wird. Es obliegt hier also der Verantwortung des Bedieners über die korrekte Verwendung dieser Konstruktion.

Hinweis: Diese SPA-Einstellung kann durch eine Angabe in der [Vorgangsunterkasse](../../../vorgangsabwicklung/formularzuordnung/spa.md) überschrieben werden!
