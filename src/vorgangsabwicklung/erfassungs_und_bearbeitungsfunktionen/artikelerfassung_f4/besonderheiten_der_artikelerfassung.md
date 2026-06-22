# Besonderheiten der Artikelerfassung

<!-- source: https://amic.de/hilfe/besonderheitenderartikelerfass.htm -->

<p class="just-emphasize">Lager, Lagerplatz, Zusatzinformation, Rabatt</p>

Bei Einschaltung weiterer Optionen kann sich der Erfassungsbildschirm folgender­maßen darstellen:

Durch Steuerungsparameter (im Bereich “Vorgangsbearbeitung Warenpos.”) kann eingestellt werden, ob und in welcher Weise eine Lagernummer-Eingabe inner­halb der einzelnen Warenposition (natürlich nur bei Neuerfassung) und eine Lagerplatzzuordnung möglich ist. Es kann zwischen “ohne Lager” (wie bisher), “nur Anzeige”, “änderbar” (nur nach Artikel­nummer-Eingabe mittels Pfeil-nach-oben-Taste) und “Einstieg” (die Lagernummer ist das erste Eingabefeld) gewählt werden.

Die Eingabe der Zusatzinfo wie auch die Skontieingabe wird über die EPA- Steuerung ermöglicht; die Rabattabfrage steuert der Parameter “manuelle Rabatte zu­lässig” im Artikel.

<p class="just-emphasize">Kontrakte, Partien</p>

Artikel mit Partiezwang oder vorliegende Kontrakte für den Kunden und Artikel erfordern manuelle bzw. automatische Zuordnungen. Auf die Abläufe wird im Abschnitt Kontrakte bzw. Partien eingegangen.

<p class="just-emphasize">Anlegen nicht vorhandener Artikel in Standard-Vorgängen</p>

Analog zur Umbuchung kann man auch während der normalen Standardvorgänge (insbesondere wohl sinnvoll im Einkauf und bei Lagerumbuchungen) Artikel in einem Lager anlegen, in dem sie bisher fehlen.

Die Vorgehensweise ist hier aber etwas komplizierter, weil ja noch kein Artikel (im Abgangslager) selektiert worden ist, den man auf das (Zugangs-)Lager kopieren könnte.

Daher muss man zu diesem Zweck (z. B. nach fehlgeschlagener und mit **ESC** abgebrochener Suche des Artikels) **SF11** betätigen. Es öffnet sich ein Minifenster für die Auswahl des zu kopierenden Artikels, ggf. bereits mit einer Auswahlmaske (Itembox) obendrauf. Nach erfolgter Auswahl des Artikels und Bestätigung der Korrekt-Abfrage wird der selektierte Artikel in das zuvor eingestellte Lager kopiert und ist damit bebuchbar.
