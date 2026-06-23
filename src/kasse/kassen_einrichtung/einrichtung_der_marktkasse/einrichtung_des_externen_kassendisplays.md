# Einrichtung des externen Kassendisplays

<!-- source: https://amic.de/hilfe/einrichtungdesexternenkassendi.htm -->

Die Einrichtung erfolgt in der Kassenverwaltung. Jeder logischen Kasse ist ein Kassensystem zugeordnet. Dieses Kassensystem beschreibt die Hardwareeinheit. Technisch gesehen sind alle der Einstellungen dem Kassensystem zugeordnet.

Auf der Registerkarte „Geräte“ findet sich im unteren Bereich ein Rahmen mit der Überschrift [Hardware externes Display](../kassenverwaltung_logische_kasse.md#Log_Kasse_Geraete_ExtDisplay). Hier richten Sie ein Bildschirmdisplay ein.

Auf der Registerkarte „Anzeige“ findet sich eine Tabelle mit Feldern, die zur Anzeige eingerichtet werden können.

Diese Tabelle mit Anzeigefeldern ist einem Anzeigeschema zugeordnet, dass Sie in dem Feld „Schema“ auswählen müssen. Sie können auch mit der Funktion „Neues Anzeigeschema“ ein neues Schema erstellen.

<p class="just-emphasize">Hinweis:</p>

Bitte beachten Sie, dass die Angaben des Schemas stets für alle Anzeigen gelten, die das gleiche Schema verwenden!!!

| Registerkarte Anzeige Name<br>Folgende Namen können eingerichtet werden |
| --- |
| BON | Hier wird der laufende Bon dargestellt |
| LDC | In dieses Feld wird das Währungskennzeichen geschrieben. \*) |
| LDT | Text der Zeilendisplayzeile \*) |
| LDV | Wert der Zeilendisplayzeile \*) |
| LINEDISPLAY | Zeilendisplay – Hier werden Daten der o.g. Werte zusammen dargestellt. |
| SCREEN | Diese Konfiguration dient der Größendimensionierung des Fensters |
| QRCODE | QR-Code für AnyBill, wenn die Lizenz vorhanden ist. Diese Einrichtung ist nur bei entsprechender Lizenz verfügbar. |
| SUMME | Hier wird die Summe des Bons angezeigt. |

(\*) Diese Zeilen werden nicht auf dem ScreenDisplay dargestellt. Die dienen der Füllung und Ausrichtung in einem Feld namens LINEDISPLAY. Wird beispielsweise das Währungskennzeichen (LDC) linksbündig angegeben, so wird dies links neben den Wert (LDV) geschrieben.

Wird kein Zeilendisplay und kein Feld mit dem Namen LINEDISPLAY verwendet, so können diese Felder entfallen.

| Positionen<br>In den nachfolgenden Spalten werden die Positionen der Objekte eingerichtet. Alle Angaben sind in Pixel anzugeben. |
| --- |
| TOP | Position relativ vom oberen Rand |
| LEFT | Position vom linken Rand |
| HEIGHT | Höhe des Objekts |
| WIDTH | Breite des Objekts |

| Feldtyp |
| --- |
| Head | Es wird in der Textbox stets der obere Teil des Textes angezeigt. Bei mehrzeiligen Texten wird der untere Teil abgeschnitten. Der zuvor gegebene Text wird gelöscht. |
| Tail | Es wird in der Textbox stets der untere Teil des Textes angezeigt. Bei mehrzeiligen Texten wird der obere Text abgeschnitten. Der zuvor gegebene Text wird gelöscht. |
| AddUp | Es wird der Text im oberen Teil der Textbox hinzugefügt. Der zuvor gegebene Text bleibt darunter erhalten. |
| AddDn | Es wird der Text im unteren Teil der Textbox hinzugefügt. Der zuvor gegebene Text bleibt darüber erhalten. |
| LineDisplay | Diese Textbox verhält sich wie ein Zeilendisplay, zeigt also Text, Währung und Betrag in zwei Zeilen an. |
| LineDisplayText | Für die Anzeige eines Textes im Zeilendisplay muss dieser Typ eingerichtet sein. |
| LineDisplayValue | Für die Anzeige eines Betrages im Zeilendisplay muss dieser Typ eingerichtet sein. |
| LineDisplayCurrency | Für die Anzeige eines dreistelligen Währungskennzeichens im Zeilendisplay muss dieser Typ eingerichtet sein. |
| Screen | Dieser Feldtyp gibt an, dass hier Angaben zur Größe und der Position des Anzeigefensters eingetragen werden. |

| Ausrichtung |
| --- |
| Links | Die Anzeige erfolgt linksbündig in der Textbox |
| Zentriert | Die Anzeige erfolgt zentriert in der Textbox |
| Rechts | Die Anzeige erfolgt rechtsbündig in der Textbox |

Font

Geben Sie hier den Namen der zu verwendenden Schriftart an. Bitte bedenken Sie, dass Sie für den optischen Eindruck eines Zeilendisplays oder eines ASCII-Bons eine nicht-proportionale Schriftart wählen sollten, bei der alle Zeichen die gleiche Breite haben. Anderenfalls könnte die Anzeige ungewohnt ungeordnet aussehen.

Fontsize

Geben Sie hier die Schriftgröße ein

Text

Hier können Sie einen Text für einen Test bzw. die Initialisierung des Displays eingeben
