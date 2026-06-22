# Produktion mit Seriennummern

<!-- source: https://amic.de/hilfe/_scanner_prod_seriennummer.htm -->

Bei der Produktion wird die Produktartikelnummer gescannt und oder die dazu gehörige Seriennummer. Anhand des Produktartikels wird dann das Rezept gezogen und entsprechend aufgelöst. Erfasste Seriennummern werden in der Ausprägung des Artikels gespeichert. Bei der Erstellung der Produktion wird dann die Seriennummer dem Produktionsbeleg zugeordnet.

Um die Produktion auf dem Scanner zu starten werden zwei Scancodes benötigt. Diese müssen per Etikettendruck bereitgestellt werden. Dazu wird der Text „PRODSTART“ und „PRODENDE“ im EAN 128 Verschlüsselt ausgedruckt.

Um eine Produktion zu starten wird als erstes der Befehl „**PRODSTART**“ eingescannt. Danach wird der Artikel und oder die dazugehörige Seriennummer erfasst. Da es sich bei dieser Produktion um eine Produktion für Stückartikel handelt muss bei der Menge immer eine 1 eingegeben werden. Wird eine Zahl größer 1 eingegeben, so wird zwar die Produktmenge erhöht aber nicht die Komponentenmenge.

Mit dem Befehl „**PRODENDE**“ wird die Produktion abgeschlossen.

Währen der Erfassung der Produktion kann diese neu gestartet werden, durch zweimaliges Scannen von „**PRODSTART**“

Die erfasste Produktion wird in der [Vorgangsimport](../../../vorgangsabwicklung/vorgangsimport/index.md) [**VIMP**] Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

gespeichert. Von dort aus kann mit der Funktion „***Standardvorgang erzeugen***“ eine Produktion aus den erfassten Daten erstellt werden.

Stornierung einer Position

Eine erfasste Position kann wie folgt storniert werden. Dazu wird der Stornobefehl gescannt. Danach kann entweder per Scannung des Artikels oder der Seriennummer die letzte erfasste Position des Artikels gelöscht werden. Des Weiteren gibt es die Möglichkeit die zu löschende Positionsnummer manuell über die Tastatur einzugeben. Es wird immer die komplette Position gelöscht.

Folgende Itembox Stellt die Daten für die Anzeige auf dem Scanner zusammen.

IB_CE_PRODUKTION
