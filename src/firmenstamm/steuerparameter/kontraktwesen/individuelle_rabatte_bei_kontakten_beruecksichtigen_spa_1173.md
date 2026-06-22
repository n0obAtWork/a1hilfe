# Individuelle Rabatte bei Kontakten berücksichtigen (SPA 1173)

<!-- source: https://amic.de/hilfe/_SPA_1173.htm -->

Dieser Steuerparameter bestimmt, welche Rabattlogik innerhalb eines Kontrakts angewendet wird. Er entfaltet seine Wirkung nur dann, wenn am Kontrakt eine Rabattgruppe hinterlegt ist und die Option „Individuelle Rabattgruppe“ auf „wie SPA“ eingestellt ist.

In diesem Fall wird über den Steuerparameter global festgelegt, ob individuelle Rabatte aus dem Artikel oder die Rabatte aus dem Kontrakt für die Berechnung herangezogen werden.

Verhalten des Steuerparameters:

| Einstellung | Bedeutung |
| --- | --- |
| Ja (Defautl) | Es wird die am Artikel hinterlegte individuelle Rabattgruppe verwendet.  
 |
| Nein | Auch wenn am Artikel eine Individuelle Rabattgruppe hinterlegt worden ist, so wird diese nicht berücksichtigt. Stattdessen wird der im Kontrakt hinterlegte Rabattgruppe angewendet |
