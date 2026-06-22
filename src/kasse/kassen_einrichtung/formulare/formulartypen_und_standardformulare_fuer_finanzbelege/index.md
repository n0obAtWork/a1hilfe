# Formulartypen und Standardformulare für Finanzbelege

<!-- source: https://amic.de/hilfe/formulartypenundstandardformul.htm -->

Seit A.eins 7.1 ist der Formulardruck für Finanzbelege teilweise umgestellt. Bis dahin galt:

Wenn kein Formular eingerichtet war, so erfolgte der Druck nach einer fest programmierten Formularsteuerung. Diese griff immer bei Übernahmen, Übergaben und Zählberichten. Jeglicher Wiederholungsdruck erfolgte stets auf diese Art und Weise. Die anderen Belegarten von Finanzbelegen konnten im Erstdruck über Formularsteuerung erstellt werden, sofern ein entsprechender EPA gesetzt (Zahlungsmaske, Formulardruck für …) und die entsprechenden Formulare eingerichtet waren. Diese Formulare (alle vom Formulartyp 201) sind:

51: Ein- und Auszahlungen, Entnahmen

52: Einreichungen

53: Sortenwechsel

54: Zahlungsmeldungen

Andere Formulare konnten nicht zugeordnet werden.

Die Formularsteuerung von Barverkaufsvorgängen erfolgt unabhängig davon analog den Standardvorgängen in der Ware. Die Zuordnung des zu verwendenden Formulars erfolgt hier nach den üblichen Mechanismen VRGD / FRZ

Die Umstellung des Drucks von Finanzbelegen in der Kasse beinhaltet nun den Wegfall der fest programmierten Formularsteuerungen. An ihre Stelle treten vordefinierte Basisformulare (Nummer -51 bis -57), deren Einrichtungen zum Lieferumfang von A.eins gehören und die vom Anwender nicht verändert werden können. Die Basisformulare entsprechen in ihrem Layout wesentlich der bisher bekannten festen Programmierung und sind auf eine übliche Bondruckbreite von 40 Zeichen ausgelegt. Der Anwender kann nun alle Formulare auch für Finanzbelege selbst gestalten und ist dabei auch nicht mehr an bestimmte Formularnummern gebunden. Die vorhandenen Basisformulare können selbstverständlich als Vorlagen benutzt werden.

Die neuen Basisformulare sind auf einen höheren Leistungsumfang ausgelegt, als man dieses von den bisherigen 50er Formularen kennt. Dafür war es erforderlich, neue Formulartypen zu schaffen, die mit dem Formulartyp 201 nicht mehr kompatibel sind. Der Anwender besitzt nun die Wahlfreiheit, welchen Formulartyp er verwenden möchte. Das zu verwendende Formular wird in den Kasseneinstellungen hinterlegt. Die standardmäßige Einstellung bei der A.eins Umstellung ist „wie bisher“, aus Sicht des Anwenders verändern sich nur die internen Abläufe.

| Die Basisformulare und ihre Formulartypen |
| --- |
| variabel | Lastschriftbestätigung | 10 EC Lastschrift | Wie bisher |
| \-50 | Scheck | 49 Scheckdruck | |
| \-51 | Ein-/Auszahlung, Entnahme | 51 Kassenformular | |
| \-52 | Einreichung | 51 Kassenformular | |
| \-53 | Sortenwechsel | 51 Kassenformular | |
| \-54 | Zahlungsmeldungen (mehrfach) | 51 Kassenformular | |
| \-56 | Zählbericht | 50 Kassensturz-Formular | |
| \-57 | Geldübernahme / -übergabe | 51 Kassenformular | |
| | | | |

Im Formularwesen für Vorgänge benutzte kassenspezifische Druckpositionen:

| Variablenname | Druckposition | Bedeutung |
| --- | --- | --- |
| 201 Zahlart Kassenbeleg | 201 | Wie wurde bezahlt (Scheck, Bar, ...) Textersetzung pro Zahlungssatz. Da mehrere Zahlungssätze existieren, sind mehrere Positionen anzulegen, die sich im Detail durch eine fortlaufende Parameternummer unterscheiden. Dabei werden nur die angedruckt, die auch befüllt wurden. (Vorschlag: mindestens 3 Parameter anlegen, mehr schaden nicht!!) |
| 202 Zahlart Betrag | 202 | Korrespondiert zu 201 und liefert den gezahlten Betrag in der gewählten Währung |
| 206 Kassenzahlwährung | 206 | Korrespondiert zu 201 und liefert die gewählte Währung des Zahlungssatzes |
| 203 Zahlbetrag gesamt Kasse | 203 | Gesamt-Zahlungsbetrag des Vorgangs in Belegwährung (bei gewährtem Skonto vermindert um den Skontobetrag) |
| 204 Skontobetrag Kasse | 204 | Skontobetrag des Vorgangs in Belegwährung (wenn Skonto gewährt wurde, steht dieser auch in der Druckposition 202 unter dem ersten Parameter) |
| 205 Kasse Rückgeld | 205 | Rückgeldbetrag des Vorgangs in Belegwährung (wenn Rückgeld rauszugeben ist, steht dieser auch in der Druckposition 202 unter dem letzten Parameter) |
| 1974 Kassennummer | 1974 | Die Kassennummer, an der dieser Vorgang erzeugt wurde |
| 1975 Kassenkonto | 1975 | Das Kassenkonto, das zu der Kasse gehört, die diesen Vorgang erzeugt hat |

<p class="siehe-auch">Siehe auch:</p>

- [Formulartypen](./formulartypen.md)
- [Druckbereiche](./druckbereiche.md)
