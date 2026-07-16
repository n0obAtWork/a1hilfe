# Aufruf aus der Auswahlliste

<!-- source: https://amic.de/hilfe/aufrufausderauswahlliste.htm -->

In der Auswahlliste muss entweder eine Adressid vorhanden sein oder ersatzweise eine Kundis, deren Hauptadresse angezeigt wird.

Mit diesem JPL-Code

```text
call CS ("GoogleMapsPoint")
```

wird der Browser mit den markierten Adressen geöffnet

Im PAS-Makro wird der Controlstring aufgerufen

```text
sprintf(buf,"^ CS ("GoogleMapsPoint")");
ctrlstring(buf);
```
