# Beispiele:

<!-- source: https://amic.de/hilfe/beispiele.htm -->

Vom Vorgangsdatum abweichendes Bepreisungsdatum

Um die Preisermittlung vom Vorgangsdatum (Rechnungsdatum) unabhängig zu machen, gibt es die Möglichkeit, ein separates Bepreisungsdatum mittels ***UFLD*** einzurichten, das ggf. auf der Hauptmaske des Vorgangs abgefragt wird und bei einer Eingabe die Grundlage für das Suchen nach Listen- und Individualpreisen sowie Rabatten und Zu-/Abschlägen liefert. Wurde kein separates Datum eingegeben, so gilt weiterhin das Vorgangsdatum als Grundlage.

Das Bepreisungsdatum wird aber selbstverständlich bei Umwandlungen an den Folgevorgang weitergereicht, so dass beispielsweise die Eingabe eines Bepreisungsdatums beim Auftrag oder Lieferschein in der Rechnung erhalten bleibt.

<p class="just-emphasize">Bonität</p>

Es ist eine reine Anzeige aus dem Kundenstamm. Änderungsmöglichkeiten bestehen nicht.

<p class="just-emphasize">Steuergruppe</p>

Anzeige der Steuergruppe des Kunden. Änderungen können sinnvoll sein, wenn es sich (im Ausnahmefall!) um eine Rechnung aus dem Ausland handelt, obwohl der Kunde im Inland sitzt. Sinnvoller ist es jedoch, die Standardeinstellung nicht zu verändern, sondern das Kundenkonto zusätzlich mit der anderen Steuereinstellung anzulegen.

<p class="just-emphasize">Fakturiergruppe</p>

Hierbei handelt es sich um ein Auswertungskennzeichen, dessen Bedeutung im Unternehmen selbst festgelegt wird. Entsprechend sinnvoll können hier manuelle Änderungen sein.

<p class="just-emphasize">Zahlungsart</p>

Vorbelegt mit der Standardzahlungsart des Kunden. Für einen konkreten Fall kann man jedoch hiervon abweichen, z.B., um einem Kunden, der normalerweise per Scheck bezahlt eine Nachnahmerechnung zu schicken.

Der Steuerparameter “Zahlungsart maximal wie im Kundenstamm” (Parametergruppe: Vorgangsbearbeitung allg.) hat folgende Bedeutung:

<p class="just-emphasize">“Ja” (Default):</p>

Es kann nur eine kleinere Zahlungsart als vorgeschlagen eingegeben werden (z.B. Kundenstammeintrag 4 kann nur auf 1..3 geändert werden, nicht aber auf 5.

<p class="just-emphasize">“Nein”:</p>

 keine Einschränkung bei der Vergabe

<p class="just-emphasize">Versandart</p>

Vorbelegt aus dem Kundenstamm. Kann hier entsprechend der konkreten Situation überschrieben werden. Das kann allerdings auf viele Abläufe Einfluss haben:

Preisfindung

Frachtwesen

etc.

<p class="just-emphasize">Vertretergruppe</p>

Vorbelegt aus dem Kundenstamm. Kann hier entsprechend der konkreten Situation überschrieben werden.

<p class="just-emphasize">Versandanschrift</p>

Wenn für einen Kunden eine Versandanschrift vorliegt, wird sie automatisch abgefragt:

Bei mehreren Alternativen öffnet sich das bekannte Auswahlfenster und die ge­wünschte Anschrift wird ausgewählt. Man kann aber auch über die Funktion „Manuelle Versandadresse“ eine neue Versandanschrift für diesen Vorgang erfassen.

Nach Auswahl der gewünschten Anschrift wird sie zur Information in das rechte obere Anzeigefenster übernommen.

<p class="just-emphasize">Informelle Anschrift</p>

Hier kann eine zusätzliche Anschrift erfasst werden, die zu informationszwecken auf Formulare gedruckt werden kann. So zum Beispiel ein von der Versand- oder Rechnungsanschrift abweichender Besteller oder eine informationelle Rechnung zu Händen von o.ä. .

<p class="just-emphasize">Plandatum / Lieferdatum</p>

Das hier einzugebende Datum wird je nach Vorgangsklasse als geplantes Lieferdatum (Angebot, Auftrag) oder als Lieferdatum (Lieferschein, Rechnung, Gutschrift) interpretiert. Vorbelegt wird es mit dem Erfassungsdatum.

<p class="just-emphasize">Referenz - Nummer</p>

Die Vorgangsnummer, auf die sich dieser Vorgang bezieht. Sie kann ausgedruckt und ausgewertet werden.

<p class="just-emphasize">Listenpreisklasse</p>

Die Preisklasse, der Kunde zugeordnet ist, wird angezeigt und kann überschrieben werden. Damit wird jedoch evtl. auch automatisch ein anderer Preis vorgeschlagen.

<p class="just-emphasize">Sprache</p>

Die Sprache, die Verwendung finden soll; übersteuert die Standardwerte aus dem Kundenstamm.

<p class="just-emphasize">Wiegenummer</p>

Die Wiegenummer, auf die sich dieser Vorgang bezieht. Sie kann ausgedruckt und ausgewertet werden.

<p class="just-emphasize">Frachtklasse</p>

Hier kann für Frachtberechnungen die aus dem Kundenstamm gelieferte Frachtklasse überschrieben und damit ein anderes Verfahren gewählt werden.

<p class="just-emphasize">Frachtvariante</p>

Hier kann für Frachtberechnungen die aus Kunden- und Artikelstamm gelieferte Frachtvariante überschrieben und damit ein anderes Verfahren gewählt werden.

<p class="just-emphasize">Verkaufsgebiet</p>

Das Verkaufsgebiet, auf die sich dieser Vorgang bezieht. Der Parameter kann ausgedruckt und ausgewertet werden.

<p class="just-emphasize">Gebiet von..., Gebiet nach</p>

Für Entfernungsermittlungen im Rahmen der Frachtberechnung sind hier Eingaben erforderlich.

<p class="just-emphasize">LKW Nr. Motor, LKW Nr. Anhänger</p>

Die LKW - Nummer, auf die sich dieser Vorgang bezieht. Sie kann ausgedruckt und ausgewertet werden.

<p class="just-emphasize">Fahrer</p>

Der Fahrer, auf den sich dieser Vorgang bezieht. Er kann ausgedruckt und ausgewertet werden.

<p class="just-emphasize">Währung Nr., Kurs</p>

Hier kann die aus dem Kundenstamm gelieferte Währung und der Standardkurs für diesen Vorgang geändert bzw. eingetragen werden.

<p class="just-emphasize">Objekt</p>

Zuordnung des Objekts zum Vorgang.

<p class="just-emphasize">Zahlart</p>

Zuordnung der Zahlart zum Vorgang.

<p class="just-emphasize">Weitere Funktionen im Rechnungskopf</p>

Weitere Funktionen können über die Funktions-Box aufgerufen werden. Dies ist sowohl während der Kopferfassung als auch zum Abschluss möglich. Selbst aus der Positionserfassung kann man hierhin wechseln und wieder in sie zurückkehren. Ein Teil der Funktionen ist sicherlich erst zum Abschluss sinnvoll aufzurufen, so z.B. “Gesamtsummen”. Hieran orientiert sich deshalb auch nachfolgende Beschreibung.

Der Wechsel aus dem Erfassungsteil in die Box erfolgt mit (F12) und anschließender Selektion mittels der Cursortasten oder per Mausklick.

![](../../ImagesExt/image8_203.jpg)
