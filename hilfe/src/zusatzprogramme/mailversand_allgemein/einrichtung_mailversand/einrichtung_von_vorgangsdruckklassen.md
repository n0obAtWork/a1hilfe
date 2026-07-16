# Einrichtung von Vorgangsdruckklassen

<!-- source: https://amic.de/hilfe/einrichtungvonvorgangsdruckkla.htm -->

Administration > Drucker > Vorgangsdruckklassen

Direktsprung **[VRGD]**

Neben dem Standarddruck ist der Vorgangsdruck über die [Vorgangsdruckklassen](../../../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md) möglich.

Dort kann z.B. eingestellt werden, dass ein Beleg mehrfach und/oder auf verschiedenen Druckern mit verschiedenen Formularen ausgegeben wird. An dieser Stelle muss bei Verwendung von Vorgangsdruckklassen ein Formular angegeben werden, das dem zu versendenden Beleg entspricht.

Bei „Anzahl“ kann die Anzahl der Drucke angeben die im Falle des Formulardrucks tatsächlich gedruckt werden sollen. Im Fall der Einstellung „**statt Rechnungsdruck**“ (Kundenstamm des jeweiligen Kunden) wird einer der Drucke bzw. ggf. der einzige Druck dieses Formulars von dieser Anzahl abgezogen.

Es findet sich unter **„Formulare/Drucker zuordnen“** die Felder Belegversand und Mailtyp. Um den Belegversand für diese Vorgangsdruckklasse zu aktivieren, stellt man den Wert auf „**Ja**“. Bei „Mailtyp“ kann per F3 ein E-Mail-Bereich angeben werden. Beim Formulardruck wird die E-Mail-Adresse des Kunden zu dem angegebenen Mailtyp/E-Mail-Bereich ermittelt und verwendet. Hierbei hat diese Einstellung Vorrang zu einer eventuellen FRZ-Einrichtung.

Will man einen Formulardruck beispielsweise an zwei unterschiedliche Empfänger senden, so kann man sich eine weitere Zeile mit dem gleichen Formular einrichten. Im Feld „Belegversand“ wählt man nun „Exclusiv“ aus. Dieses Formular wird ausschließlich über den Belegversand berücksichtigt. Für diese Zeile findet kein physikalischer Druck statt, egal welcher Wert bei Anzahl angegeben wurde.

Möchte man für eine bestimmte Vorgangsart (z.B. Lieferscheine) grundsätzlich keinen Belegversand durchführen, so muss dies ausdrücklich eingerichtet werden. In so einem Fall muss im Feld „Belegversand“ ein „Nein“ eingetragen werden. Hierbei wird auch nicht auf die FRZ-Einrichtung zurückgegriffen!
