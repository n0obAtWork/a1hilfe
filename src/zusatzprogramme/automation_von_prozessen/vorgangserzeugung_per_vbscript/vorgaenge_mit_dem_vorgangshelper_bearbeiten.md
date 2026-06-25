# Vorgänge mit dem Vorgangshelper bearbeiten

<!-- source: https://amic.de/hilfe/vorgngemitdemvorgangshelperbea.htm -->

Das Grundgerüst des Vorgangshelper sieht wie folgt aus

```text
aeins.jpp_new(hdl, "CVorgangsHelper")
    aeins.jpp_in hdl , "KundNummer", ZielKundenNummer
    aeins.jpp_in hdl , "Klasse", ZielKlassennummer
    aeins.jpp_in hdl , "Unterklasse", ZielunterKlassennummer
    aeins.jpp_in hdl , "NumNummer", ZielNumNummer
    aeins.jpp_ex (hdl, "StartVorgang")
      tu was ….
    aeins.jpp_in hdl , "Speichern", "1"
    aeins.jpp_ex hdl, "BeendeVorgang"
  aeins.jpp_delete hdl
```

Es wird ein neues Objekt vom Typ “ CVorgangsHelper“ instanziiert, vier IN-Parameter an das Objekt übergeben und der Vorgang gestartet. Mit „tu was …“ wird der Vorgang bearbeitet. Anschließend wird der Vorgang gespeichert und beendet (also die Veränderungen übernommen) und das Handle freigegeben.
