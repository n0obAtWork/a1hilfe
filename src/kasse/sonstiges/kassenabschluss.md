# Kassenabschluss

<!-- source: https://amic.de/hilfe/kassenabschluss.htm -->

1. AcashUeb fügt „hängende“ Geldübergaben oder Abschöpfungen als Geldübernahmen in die noch offene Sitzung ein.

2. Abgebrochenen Beleg (BelegId=0) dieser Kasse bereinigen

3. Nur bei SPA_ABSCHOEPFUNG_AUTOMATISCH:

a. Wechselgeld entnehmen

b. Ggf. Zählung durchführen, bei Unterkasse mit automatischer Abschöpfung wird so getan, als wäre eine Zählung erfolgt.

Nur bei Abschluss mit Zählung bzw. Zählung nn passiert all dieses

- Kassenbericht abgeschlossen setzen
- Bei Unterkasse: Bargeldabschöpfung an Hauptkasse
- Umbuchung des Bargeldes vom Kassenkonto auf das eingerichtete Bargeldkonto
- Umbuchung des Manko vom Bargeldkonto auf das Differenzenkonto bzw. des Überschuss vom Differenzenkonto auf das Bargeldkonto
- Umbuchung der Stornos auf Stornokonto
- Umbuchung der Zahlungsmittel auf die jeweils zugeordneten Konten je nach EPA Einstellung EINZELBUCHUNG als eine eben solche oder als Sammelbuchung je Zahlungsmittel

Nur bei Hauptkassen oder bei Unterkassen, die nicht automatisch an Hauptkassen abschöpfen: Erstellung eines maschinellen Abschöpfungsbeleges über alle unbaren Zahlungsmittel. Dieser Beleg wird nicht im Kassenbericht gebucht, damit die Verteilung der Zahlungsmittel nicht verloren geht.

Ohne Zählung passiert folgendes:

- Prüfung ob Abschluss ohne Zählung erlaubt (EPA)
- Kassenbericht abgeschlossen setzen
- Autoabschöpfung und Autobuchung genau wie oben
