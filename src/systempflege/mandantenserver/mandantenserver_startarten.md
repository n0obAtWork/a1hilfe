# Mandantenserver-Startarten

<!-- source: https://amic.de/hilfe/mandantenserverstartarten.htm -->

| Startart | |
| --- | --- |
| Direktsprung [MS] | Direktstart des Mandantenservers im aktuellen Aeins<br><br> |
| User MAND | Einloggen des User MAND startet direkt in den Mandantenserver<br><br> |
| Event | Aufruf über einzurichtendes Datenbank-Event<br><br> |
| Hauptmenü<br>Hauptmenü \> Systempflege \> Mandantenserver \> Mandantenserver | Start des Mandantenservers im aktuellen Aeins |

<p class="just-emphasize">Folgende Optionen können noch gesetzt werden, welche ein Einfluss auf die Steuerung des Mandenservers haben</p>

| Attribut | Konfiguratuionsquelle | Bedeutung |
| --- | --- | --- |
| ms_obergrenze | Option oder Global Variable<br>ms_obergrenze | Max. Anzahl Aufträge; 0 = Dauerbetrieb. Wird beim Start +1 erhöht, nach jedem Durchlauf dekrementiert, Stop bei ≤ 1.<br><br> |
| ms_shutdown | Option ms_shutdown | Flag: TRUE wenn eine Shutdown-Uhrzeit konfiguriert ist. |
| ms_shutdown_time | Option ms_shutdown | Ziel-Uhrzeit für den zeitgesteuerten Stop. |
| ms_einmal | Option ms_einmal | Einmal-Modus: alle anstehenden Aufträge abarbeiten, dann Maske verlassen (kein Dauerbetrieb). |
