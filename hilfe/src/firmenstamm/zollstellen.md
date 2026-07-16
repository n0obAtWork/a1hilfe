# Zollstellen

<!-- source: https://amic.de/hilfe/_zollstellen.htm -->

Hauptmenü > Warenverkauf > Ausfuhrbearbeitung > Zollstellen einspielen

Zollstellen sind Dienststellen des Zolls in Deutschland und der EU, die unterschiedliche Funktionen haben und im Rahmen einer Zollausfuhr eine Rolle spielen.

Als Voraussetzung für die Nutzung der Zollstellen muss der [Steuerparameter 833 – Zolldatenerfassung aktiv](./steuerparameter/lizenzen/zolldatenerfassung_lizenz_spa_833.md) aktiviert werden.

Eine Liste der Zollstellen (Custom Office List) wird von der europäischen Union veröffentlicht und als XML-Datei bereitgestellt.

Die Liste der Zollstellen kann unter [https://ec.europa.eu/taxation_customs/dds2/col/download_data_generic.jsp?Lang=de](https://ec.europa.eu/taxation_customs/dds2/col/download_data_generic.jsp?Lang=de) heruntergeladen werden. Dazu muss die „COL-Generic-YYYYMMDD.zip“-Datei gespeichert und entpackt werden.

In A.eins ist der dazugehörige Einspieler unter Warenverkauf / Atlas Ausfuhrbearbeitung / Zollstellen einspielen zu finden. Um die Zollstellen in das A.eins-System zu importieren, muss im Zollstellen-Einspieler unter dem Feld „Dateiname“ der Pfad zur der entpackten „COL-Generic.xml“ angegeben werden. Mit ***F9*** kann der Import-Vorgang gestartet werden. Dabei werden die vorherigen Daten mit den neuen Zollstellen überschrieben.

Die Liste der Zollstellen ändert sich ungefähr alle 4-6 Wochen. In der Regel ändern sich die Nummern der Zollstellen nicht. Ein regelmäßiges Update ist hier nur für aktive Nutzer der Zollausfuhr zu empfehlen. Da die Zollstellennummer z.B. im Lagerstamm gespeichert wird, ist nicht auszuschließen, dass dort nach dem Update und Erlöschen einer Zollstelle eine ungültige Zahl stehen bleibt. Das fällt spätestens bei der Zollausfuhr auf.

**Hinweis zum Dateinamen:**

*Der Pfad zur XML-Datei, der im Einspieler angegeben wird, ist ein relativer Pfad des Datenbankservers! So wird entweder die heruntergeladene XML-Datei auf dem Datenbankserver oder auf einer für den Datenbankserver zugreifbaren Netzwerkfreigabe abgelegt und als Pfad angegeben.*
