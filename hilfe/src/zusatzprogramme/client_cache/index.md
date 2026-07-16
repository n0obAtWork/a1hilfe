# Client-Cache

<!-- source: https://amic.de/hilfe/_clientcache.htm -->

**Cache** (engl. [[kæʃ](http://de.wikipedia.org/wiki/Liste_der_IPA-Zeichen)]; selten auch: [[kaʃ](http://de.wikipedia.org/wiki/Liste_der_IPA-Zeichen)][[1]](http://de.wikipedia.org/wiki/Cache#cite_note-0)) bezeichnet in der [EDV](http://de.wikipedia.org/wiki/Elektronische_Datenverarbeitung) einen schnellen [Puffer](http://de.wikipedia.org/wiki/Puffer_%28Informatik%29)\-[Speicher](http://de.wikipedia.org/wiki/Datenspeicher), der (erneute) Zugriffe auf ein langsames [Hintergrundmedium](http://de.wikipedia.org/wiki/Datenspeicher) oder aufwändige Neuberechnungen zu vermeiden hilft. Inhalte/Daten, die bereits einmal beschafft/berechnet wurden, verbleiben im Cache, so dass sie bei späterem Bedarf schneller zur Verfügung stehen. Auch können Daten, die vermutlich bald benötigt werden, vorab vom Hintergrundmedium abgerufen und vorerst im Cache bereitgestellt werden.

Caches können als Hardware- oder Softwarestruktur ausgebildet sein. In ihnen werden Kopien zwischengespeichert.

*Cache* ist ein [Lehnwort](http://de.wikipedia.org/wiki/Lehnwort) aus dem Englischen. Seinen Ursprung hat es im französischen *cache*, das eigentlich die Bedeutung *Versteck* besitzt.[[2]](http://de.wikipedia.org/wiki/Cache#cite_note-1)[[3]](http://de.wikipedia.org/wiki/Cache#cite_note-2) Der Name verdeutlicht den Umstand, dass dem Verwender in der Regel der Cache und seine Ersatzfunktion für das angesprochene Hintergrundmedium verborgen bleibt. Wer das Hintergrundmedium verwendet, muss Größe oder Funktionsweise des Caches prinzipiell nicht kennen, denn der Cache wird nicht direkt angesprochen. Der Verwender „spricht das Hintergrundmedium an“, und es „antwortet“ stattdessen der Cache – genau auf die Art und Weise, wie auch das Hintergrundmedium geantwortet, also Daten geliefert hätte. Man spricht wegen der Unsichtbarkeit dieser zwischengeschalteten Einheit auch von [Transparenz](http://de.wikipedia.org/wiki/Transparenz_%28Computersystem%29). Praktisch ist er eine gespiegelte Ressource, die stellvertretend für das Original sehr schnell bearbeitet/verwendet wird.

Greifen außer dem Cache-verwendenden Gerät noch weitere auf das Hintergrundmedium zu, so könnte es zu [Inkohärenzen](http://de.wikipedia.org/wiki/Inkoh%C3%A4renz) kommen – um auf ein identisches Datenabbild zugreifen zu können, ist es notwendig, zuvor die Änderungen des Caches in das Hintergrundmedium zu übernehmen. [Cachestrategien](http://de.wikipedia.org/wiki/Cache-Algorithmus) wie [Write-Through](http://de.wikipedia.org/wiki/Cache#Write-Through) oder [Write-Back](http://de.wikipedia.org/wiki/Cache#Write-Back) sind hier praktikabel. Im Extremfall muss ein kompletter „Cache Flush“ erfolgen. Außerdem muss ggf. der Cache informiert werden, dass sich Daten auf dem Hintergrundmedium geändert haben und sein Inhalt nicht mehr gültig ist.

Stellt die Cachelogik dies nicht sicher, so ergibt sich als Nachteil, dass inzwischen im Hintergrundmedium oder im Rechenprogramm erfolgte Änderungen nicht erkannt werden. Bei Verdacht auf Änderungen, oder um sicher zu gehen, dass der aktuelle Stand berücksichtigt wird, muss der Verwender explizit eine Cache-Aktualisierung veranlassen.

Eingefügt aus &lt;[http://de.wikipedia.org/wiki/Cache](http://de.wikipedia.org/wiki/Cache)\>

smx_call ( „Define“ ) an die entsprechenden Fach-Abteilungen delegiert:

| ***Define*** | | ***Besonderes*** |
| --- | --- | --- |
| 21 | CLR_FORMATCACHE | |
| 22 | CLR_FORMCACHE | |
| 23 | CLR_SQLCACHE | |
| 24 | CLR_ITEMCACHE | |
| 33 | CLR_STEUPA_CACHE | |
| 35 | CLR_MAKRO_CACHE | Wird nicht mehr durchgeführt wenn ein Makro in der Ausführung ist! |
| 26 | CLR_ME_CACHE | |
| 250 | CLR_AW_CACHE | |
| 540 | CLR_WORKFLOW_CACHE | |
| 29 | CLR_ADRMASK_CACHE | |
| 334 | CLR_UFLD_CACHE | |
| 365 | CLR_VORGANG_CACHE | |
| 262 | CLR_ALL_CACHES | Wird nur bei Wiedereintritt in die Menü-Maske durchgeführt! |
| 546 | CLR_OPTIONBOX_CACHE | |
| 286 | MENU_CLR_ALL_CACHES | **Ist ausschließlich der Systemprogrammierung vorbehalten!** |

Die Aktualisierung CLR_ALL_CACHES wird nicht unmittelbar bei Anforderung durchgeführt, sondern erst bei Wiedereintritt in das Menü.

Die Cache-Aktualisierungen werden auch an andere A.eins-Clienten durchgereicht. Diese aktualisieren dann die angeforderten Caches beim Wiedereintritt in das Menü.

<p class="siehe-auch">Siehe auch:</p>

- [Cache Informationen](./cache_informationen.md)
