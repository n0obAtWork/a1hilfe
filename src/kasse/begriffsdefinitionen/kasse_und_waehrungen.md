# Kasse und Währungen

<!-- source: https://amic.de/hilfe/kasseundwhrungen.htm -->

<p class="just-emphasize">Begriffsklärung:</p>

<p class="just-emphasize">Kassenwährung:</p>

In den Kasseneinstellungen kann in der Gruppe Kasse eine Kassenwährung hinterlegt werden. Diese Einstellung bewirkt, dass der Bargeldzahlungssatz mit dieser Währung vorbelegt wird und man bei Bezahlung in einer anderen Währung über die Taste F12 explizit eine andere Währung auswählen muss. Diese Vorbelegung gilt auch für die Währung des Rückgeldsatzes. Wenn die Kassenwährung in den Kasseneinstellungen geändert wird, wird automatisch auch die Belegwährung der Standardkunden auf die Kassenwährung gesetzt. (Die Umkehrung gilt nicht!)

<p class="just-emphasize">Belegwährung:</p>

In dieser Währung wird der Beleg erfasst (diese Währung kommt aus dem Kundenstamm und kann bei Vorgängen über UFLD-Felder bzw. an der POS-Kasse über eine Funktion zu Beginn des Vorgangs auf eine beliebige Währung gesetzt werden), d.h. auch die gefundenen Preise, ... verstehen sich in Belegwährung.

<p class="just-emphasize">Buchwährung:</p>

Hierbei handelt es sich um die aktuell gültige Buchwährung. Diese wird in der Regel EURO sein.

<p class="just-emphasize">Bezahlwährung:</p>

Hier handelt es sich um die Währung, in der man den zu zahlenden Betrag bezahlt. Vorbelegt ist diese Währung während der Erfassung mit der Kassenwährung, jedoch hat man über eine entsprechende Funktionstaste die Möglichkeit, sich eine beliebige Währung (gemäß Währungsstamm) auszusuchen.

<p class="just-emphasize">Weitere Informationen</p>

Es ist möglich, auch Zahlungsmittel in unterschiedlichen Währungen zu erfassen. Hierzu ist dann zunächst die passende Währung über den Fremdwährungsbutton auszuwählen. Danach kann man die Währung des Zahlungsmittels auswählen.

EC-Karten können nur über das DTA-Lastschrift-Verfahren übertragen werden, wenn sie in EURO erfasst wurden. Hierfür ist es nötig, dass diese Zahlungsmittel als „Euro“ –Währung gekennzeichnet sind (dabei ererbt sich diese Kennzeichnung aus dem Währungskurztext des Währungsstammes). Wenn dort also nicht Euro eingetragen ist, werden diese Zahlungsmittel nicht weiterverarbeitet.

Bem.:

Im Fuß eines Formulars besteht die Möglichkeit, die einzelnen Zahlungssätze einzeln aufzulisten.

Hierzu stehen folgende Druckpositionen zur Verfügung:

201 ID_KASSE_ZAHLART, dort ist die Zahlungsart des Zahlungssatzes hinterlegt (Bar, Scheck, EC-Karte, Fremdwährung,...)

206 ID_KASSE_ZAHLWAEHRTEXT, dort ist die Währung hinterlegt, in der dieser Zahlungssatz erfasst wurde. (gemäß Währungskurztext im Währungsstamm)

202 ID_KASSE_ZAHLARTBETRAG, hier ist der Betrag in der Währung abgelegt, in der kassiert wurde (korrespondiert mit 206)

Um alle Zahlungssätze auflisten zu können, sollte man defaultmäßig mehrere solcher Sätze anlegen, die sich durch einen aufsteigenden Eintrag in den Parametern im Detail der Druckposition unterscheiden).

Bem.: 

Der im Währungsstamm unter Kurztext für die Währung hinterlegte Text wird auch zur Displayanzeige sowie beim Andruck der Währung im Fuß des Formulars sowie zur Gruppierung nach Währungen im Kassenbericht herangezogen.
