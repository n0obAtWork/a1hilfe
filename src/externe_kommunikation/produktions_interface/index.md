# Produktions-Interface

<!-- source: https://amic.de/hilfe/_ prodInterface.htm -->

Es gibt für die Kommunikation mit Produktionssystemen eine XML-Dateiaustausch-basierende Schnittstelle.

Diese erledigt verschiedene Aufgaben:

6. Übertragung der Produktionsdaten an das Produktionssystem

Hier wird die Komponentenliste mit den Mengen an die Produktion übertragen.

7. Empfang von Materialbedarf

In A.eins wird eine Materialorder erstellt.

8. Empfang von Ware-Fertig-Meldungen

Hier werden in A.eins Ladeträger an der Fertigstellungslokalität der Linie erstellt und beladen. (siehe auch [[PRODL]](../../firmenstamm/produktionslinien.md))

9. Empfang von Verbrauchsmeldungen

Hier wird die verbrauchte Menge vom Ladeträger in der Bereitstellungszone abgebucht. (siehe auch [PRODL])

10. Empfang von Fertigmeldung einer Produktion. Hier werden die Verbräuche und die Produktmenge in der Produktion korrigiert.

Mit Hilfe von Makro 2.0 kann man in C# bequem ein Makro erstellen, dass die Dateien z.B. im Rahmen eines Mandantenserverprozesses erstellt bzw. einliest. Um Pfade in Test- und Livesystem pflegen zu können, empfehlen wir dazu die [Mandantenprofile](../../zusatzprogramme/testmandant/pfleger_fuer_das_mandantenprofil.md) zu verwenden.

<p class="siehe-auch">Siehe auch:</p>

- [Anwendung](./anwendung.md)
