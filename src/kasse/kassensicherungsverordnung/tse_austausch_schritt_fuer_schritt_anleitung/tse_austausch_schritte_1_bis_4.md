# TSE-Austausch Schritte 1 bis 4

<!-- source: https://amic.de/hilfe/_kassenSichVsfs1B.htm -->

<details>
<summary>TSE-Austausch Schritt 1 Kassenabschluss durchführen</summary>

Hauptmenü > Barvorgänge > Stammdaten > Kasseneröffnung / Kassenabschluss

Vor jeder Installation der neuen TSE-Version bei einem Update müssen die Kassenabschlüsse aller betreffenden Kassen durchgeführt werden.

Um einen Kassenabschluss der betreffenden Kassen durchzuführen, wie folgt vorgehen:

1. Zu Barvorgänge > Stammdaten > Kasseneröffnung / Kassenabschluss navigieren.

2. Betreffende Kasse auswählen.

3. Auf **Abschluss** klicken oder ***F8*** drücken.

</details>

<details>
<summary>TSE-Austausch Schritt 2 Daten sichern per Export</summary>

Hauptmenü > Barvorgänge > Kassensicherungsverordnung

Direktsprung **[KSVO]**

<p class="just-emphasize">Hinweis!</p>

Die Datensicherung ist wichtig für Ihre Betriebsprüfung und die erstellten Dateien müssen gut aufbewahrt werden.

Der Betriebsprüfer könnte bei der Prüfung nach den Dateien fragen.

Weitere Informationen zum DSFinV-K Export finden Sie unter: [DSFinVK_Export](../../DSFinVK_Export)

<p class="just-emphasize">DSFinV-K Export erzeugen</p>

1. Zum Direktsprung **[KSVO]** navigieren.

2. Mit der Funktion ***Export erzeugen*** die Dateien exportieren.

3. Dateien speichern/ablegen.

<p class="just-emphasize">TAR-Export erzeugen</p>

1. Zum Direktsprung **[KSVO]** navigieren.

2. Mit der Funktion ***Export TAR Zeitraum*** mit Datumseingrenzung den Export erstellen.

3. Dateien speichern/ablegen.

***ODER*** (*ohne Datumseingrenzung):*

1. Zum Direktsprung <strong>[TSE]</strong> navigieren.

2. Datensatz markieren.

3. Dateien ansehen **(F6).**

4. Mit der Funktion ***Export TAR*** die Dateien exportieren.

<p class="just-emphasize">A.Eins beenden</p>

1. A.Eins Client beenden.

</details>

<details>
<summary>TSE-Austausch Schritt 3 Neue A.eins Lizenz (ahoi2.xml) einspielen</summary>

Damit die Lizenz zum TSE-Stick passt, muss zunächst die aktuelle A.eins-Lizenz eingespielt werden.

Dazu wie folgt vorgehen:

***1.*** Im Ordner ***Aeins\\Config*** die aktive **ahoi2.xml** umbenennen z. B. in ***ahoi2_OLD***

So ist sichergestellt, dass sie im Notfall reaktiviert werden kann.

2. Lizenzdatei aus der E-Mail unter den genau folgender gekürzten Bezeichnung: **ahoi2.xml** auf allen Servern und Clients ins Verzeichnis ***Aeins\\Config*** speichern.

</details>

<details>
<summary>TSE-Austausch Schritt 4 TSE-Sticks tauschen und Kassenarbeitsplatz A.eins starten</summary>

Voraussetzungen:

- Freigabe für das Laufwerk, auf dem der TSE-Stick betrieben wird (bei Netzwerkbetrieb).
- Lese- und Schreibberechtigungen für das Laufwerk (bei Netzwerkbetrieb)

Um die TSE-Sticks zu tauschen, wie folgt vorgehen:

1. Alten TSE-Stick am Kassenarbeitsplatz oder Server entnehmen.

2. Neuen TSE-Stick einsetzen.

3. Am Kassenarbeitsplatz A.eins starten

<p class="just-emphasize">Hinweis!</p>

Für die Kassen- und Betriebsprüfung ist der alte TSE-Stick gut aufzubewahren!

[Weiter zu Schritt 5](./tse_austausch_schritt_5_7.md)

[Zurück](./tse_austausch_schritte_1_bis_4.md#KassenSichV_sfs3B)

</details>
