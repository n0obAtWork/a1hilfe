# Individuelle Frachten bei Kontrakten berücksichtigen (SPA 1174)

<!-- source: https://amic.de/hilfe/_SPA_1174.htm -->

Dieser Steuerparameter bestimmt, welche Frachtgruppenlogik innerhalb eines Kontrakts angewendet wird. Er entfaltet seine Wirkung nur dann, wenn am Kontrakt eine Frachtgruppe hinterlegt ist und die Option „Individuelle Frachtgruppe“ auf „wie SPA“ eingestellt ist.

In diesem Fall wird über den Steuerparameter global festgelegt, ob die individuelle Frachtgruppe aus dem Artikel oder die Frachtgruppe aus dem Kontrakt für die Berechnung herangezogen werden.

Verhalten des Steuerparameters:

| Einstellung | Bedeutung |
| --- | --- |
| Ja (Defautl) | Es wird die am Artikel hinterlegte individuelle Frachtgruppe verwendet. |
| Nein | Auch wenn am Artikel eine Individuelle Frachtgruppe hinterlegt worden ist, so wird diese nicht berücksichtigt. Stattdessen wird die im Kontrakt hinterlegte Frachtgruppe angewendet |
