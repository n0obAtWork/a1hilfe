# Herkunft des Preises

<!-- source: https://amic.de/hilfe/_wabewpreiswoherundwabewpreiswohertext.htm -->

Neben dem Preis gibt es ein Kennzeichen, das angibt, aus welcher Quelle der Preis stammt. Als mögliche Quellen seien hier beispielsweise Kontrakte und Preislisten genannt.

Eine manuelle Eingabe des Preises wird als solche erkannt und setzt das Kennzeichen neu.

Auch andere Änderungen des Preises z.B. über ein Makro oder eine Einspielschnittstelle können den Preis setzen. Da die Bezeichnung der Herkunft dieser durchaus dynamisch entwickelten Einspielungen vielfältig sein kann, wird ein eigenes Textfeld in der Datenbank bereitgestellt, in dem bis zu 20 Zeichen für die Preisherkunft gespeichert werden können.

Dieses Kennzeichen wird von der Einspielschnittstelle nach der Preissetzung gesetzt und wird dann in der Erfassungsmaske statt der Bezeichnung „manuelle Eingabe“ angezeigt.

<p class="just-emphasize">Technische Beschreibung:</p>

Das Feld WaBewPreisWoherText ergänzt die bekannte Enumeration des Feldes WaBewPreisWoher durch flexible Beschreibung. Ist das nummerische Kennzeichen auf manuell (99) gesetzt, so kann hier eine zusätzliche Information hinterlegt werden. Diese überschreibt, wenn vorhanden auch auf der Erfassungsmaske die Bezeichnung „manuelle Eingabe“.

Das Kennzeichen muss aus technischen Gründen stets nach dem Setzen von Preis und Menge eingetragen werden und kann mit der ID ID_WABEW_PREISWOHERTEXT gesetzt bzw. abgefragt werden.
