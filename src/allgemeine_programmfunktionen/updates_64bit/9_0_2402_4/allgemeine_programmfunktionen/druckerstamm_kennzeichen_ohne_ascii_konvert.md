# Druckerstamm: Kennzeichen "Ohne ASCII Konvert."

<!-- source: https://amic.de/hilfe/_90_35876.htm -->

Im Zusammenhang mit dem Feature "Queue / Datei" und dem Druck in eine Datei wurde offenbar das Kennzeichen "Ohne ASCII Konvert." nicht berücksichtigt. Das führte dazu das die Umlaute der üblichen Sonderbehandlung im ASCII-Druck-Umfeld unterlagen, was aber im "Datei-Druck" zu Fehlern führt, da dieser die Umlaute schon richtig erzeugt. Durch den nun funktionierenden Schalter lässt sich die "Sonderbehandlung" abstellen, mit dem Effekt das die Umlaute unverändert und richtig durchgeleitet werden. Zusätzliche Erläuterung sei erwähnt:Wenn z.B. in eine Spool-Datei (Notepad) / auf ein Fax gedruckt wurde, wurden gewisse Zeichen (z.B. Umlaute) nicht korrekt dargestellt. Hierfür ist dieses Kennzeichen eingerichtet. Wird dieses auf "Ja" gestellt, wird die zusätzliche Zeichenkonvertierung ausgeschaltet und auch diese Sonderzeichen werden korrekt dargestellt. Die "Defaulteinstellung" ist "Nein", das Verhalten bleibt wie bisher. Bei normalen Druckern sollte die Voreinstellung "Nein" beibehalten bleiben.

### Releasenote Kategorie:

Ticket: 740321[35876]

Version: 9.0.2402.4

Datum: 06.12.2024

Anwendung: Druckerstamm

Variante: -

Funktion/Report: -

[Weitere Informationen](http://www.amic.de/hilfe/druckerstammdrst_pfleger.htm)

#### Tags:

Releasenote, 9.0.2402.4, 35876, 740321
