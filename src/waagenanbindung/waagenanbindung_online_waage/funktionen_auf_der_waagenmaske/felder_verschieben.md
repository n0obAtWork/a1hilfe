# Felder verschieben

<!-- source: https://amic.de/hilfe/_onlinewaagefelderverschieben.htm -->

Die Funktion Felder Verschieben kann über den Steuerparameter [918](../../../firmenstamm/steuerparameter/waagensteuerung/waagenmasken_mit_dem_widget_navigator_starten_spa_918.md) angeschaltet und ausgeschaltet werden. Im Standard wird die Owaage Maske mit dem Standard-Layout gestartet

Über CF8 kann man die Positionierung der Felder in der Waagemaske festlegen. Um diese Funktion in der OptionBox anwählen zu können, muss man die Maske im Neufall geöffnet haben. Danach erfolgt eine Abfrage, für welches Profil die Positionierung gespeichert werden soll. Das gewählte Profil wird sofort angelegt.

Mit einem Klick auf ein Element kann es verschoben werden. Das Feld wird eingefärbt und es öffnet sich dann ein kleines Fenster, in dem sich Knöpfe zum Navigieren des gewählten Elementes befinden. Ebenso ist es jetzt aber auch möglich, das Element über die Pfeiltasten zu steuern oder mit TAB auf den nächsten Tabreiter zu schieben.

In dem Textfeld „Schrittweite“ kann die Schrittweite beim Navigieren des Feldes angegeben werden. Ist keine Schrittweite angegeben, wird standardmäßig „1“ verwendet. Des Weiteren kann mit SEITE RAUF und SEITE RUNTER die Schrift des Feldes vergrößert oder verkleinert werden. Ebenso kann mit POS1 und ENDE die Breite eines Feldes vergrößert oder verkleinert werden. Diese letzten beiden Funktionen stehen nicht bei allen Feldern zur Verfügung. Mit ESCAPE wird die Positionierung des Elementes abgeschlossen. Wenn man alle Elemente wie gewünscht positioniert hat, kann man mit CF8 die Positionierung speichern. Über die Funktion Felder zurücksetzen werden die Felder auf ihren ursprünglichen Ausgangsort zurückgesetzt.

Mit der Funktion Profil löschen wird das gewählte Profil, d.h. „aktueller Bediener“ oder „alle Bediener“ gelöscht. Dadurch kann man in der Anzeige der Felder auf die höhere Positionierung zurückfallen, da vorrangig die Positionierung für den aktuellen Bediener angezeigt wird, falls diese nicht vorhanden ist, die für alle Bediener oder ansonsten die voreingestellte Positionierung. Die Funktionen in der Optionbox sind beim Felderverschieben nur über die rechte Maustaste erreichbar.
