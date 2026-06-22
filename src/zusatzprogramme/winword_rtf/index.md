# Winword / Rtf

<!-- source: https://amic.de/hilfe/_word2rtf.htm -->

Produktpflege, Wartung, interne Modernisierungen sowie Anpassungen an geltende Sicherheitsmaßnahmen machen es notwendig, die bisherige interne Verarbeitung von Word-Dokumenten (.doc) auf die Verwendung von RTF-Dokumenten (.rtf) zurückzuführen.

Da sich eine Umstellung während des Programm-Updates verbietet (mögliche große Anzahl von Dokumenten (Zeit!), Nacharbeitung von möglichen Problemen) verfährt das Programm so, dass eine Konvertierung automatisch ("on the fly") dann durchgeführt wird, wenn die Daten überhaupt benötigt werden ("on demand"). Auftretende Probleme werden per Benutzeroberfläche und Systemprotokoll kundgetan, und sollten dann mit Unterstützung dieser Anwendung zu beheben sein.

Da die Umstellung ein komplexer Prozess ist, der in mehreren Stufen / Phasen durchgeführt wird, bedarf es einer zentralen Verwaltungsstelle in der ggf. bestimmte Tätigkeiten durch- bzw. nachgeführt werden können. Ohne so ein Werkzeug bliebe auf Systemen nur die Verwendungen mit Werkzeugen wie OSQL, und wenn auf den Clienten noch nicht mal Word installiert ist - was ja nicht sein muss - wird es richtig unangenehm für alle Beteiligten.

Diese Anwendung ist nicht als "Kundenwerkzeug" zu verstehen, sondern mehr als Supporter-Unterstützung vor Ort und der Entwicklung /Qualitätsmanagement zur Abwicklung von umfangreichen Tests.

Hinweis: Datentyp der RTF-Dokumente ist "long varchar", siehe auch [Rich-Text-Format](https://de.wikipedia.org/wiki/Rich_Text_Format). Insbesondere die Durchsuchbarkeit ist dann möglich. (Deswegen werden RTF-Dokumente vorerst nicht in "long binary"-Spalten gespeichert).

Empfehlung: Vermeiden Sie weitestgehend Grafiken, das RTF-Format komprimiert diese gar nicht bis sehr schlecht - was in größeren Dokumentenausmaßen resultiert.

<p class="siehe-auch">Siehe auch:</p>

- [Die Anwendung/Variante Winword / Rtf](./die_anwendung_variante_winword_rtf/index.md)
