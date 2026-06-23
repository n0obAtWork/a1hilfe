# Produktion

<!-- source: https://amic.de/hilfe/produktion1.htm -->

<p class="just-emphasize">Produktion anlegen</p>

Produktionen können auf zweierlei Weisen angelegt werden:

1. Angabe eines Produkts mit automatischer Auflösung eines Rezepts. (klassisch)

[Produktion](../../../../zusatzprogramme/a_eins_scannersoftware/kundenspezifische_scannermodule/produktion_mit_seriennummern.md) mit Scanner

In diesem Fall geben Sie als einzige Position den Produktartikel mit seiner Menge an.

Im Vorgangstamm lassen Sie das Feld „Importtyp“ bitte auf NULL bzw. setzen eine 0 ein.

2. Angabe eines Produkts und der Komponenten mit Kennungen

In diesem Fall setzen Sie zunächst im Vorgangstamm das Feld „Importtyp“ bitte auf 1.

In der Tabelle ImportvorgPosition muss für das Produkt nun das Feld „ArtikelVariante“ mit 201 gekennzeichnet werden. Die Stücklistenkomponenten werden mit der Artikelvariante 101 gekennzeichnet.

Für Wertartikel gelten entsprechend 102.

**Produktion ändern**

Produktionen können auf zweierlei Weisen geändert werden:

1. Changing

Hier werden nur die im Import gegebenen Komponenten geändert.

Nicht vorhandene Positionen werden hinzugefügt.

Nicht gegebene Positionen werden unverändert bleiben.

Zu diesem Zweck wird der ImportTyp im Vorgangstamm auf 10 gesetzt.

2. Explizit –

Jede Komponente muss gegeben werden. Jede nicht gegebene Komponente wird entfernt.

Nicht vorhandene Positionen werden hinzugefügt.

Zu diesem Zweck wird der ImportTyp im Vorgangstamm auf 11 gesetzt.

Die Zuordnung von Produkt und Komponenten erfolgen wie oben beschrieben über die ArtikelVariante in der ImportVorgPosition.
