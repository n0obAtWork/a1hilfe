# Ableitung (XML-Beschreibung)

<!-- source: https://amic.de/hilfe/ableitungxmlbeschreibung.htm -->

Die Ermittlung der Dokumente erfolgt auf Basis eines SQL welches durch folgende XML-Beschreibungssprache mitgestaltet werden kann.

| Element | | |
| --- | --- | --- |
| **Description** | Name | Informatorisch der Name der Beschreibung. |
| | RowHeight | Höhe einer Datenzeile in Pixel.<br>(Standard bzw. Vorgabe ist 22) |
| | Version | Informatorische Versionsnummer. |
| **Field** | Name | Name für die Zuordnung eines Sql-Elementes. |
| | Caption | Die Spaltenüberschift. |
| | Sql | Ist ein Sql angegeben dann ist das Ergebnis dieses Sql’s der Spalteninhalt. |
| | WidthDisplay | Opt. vorgebbare Start-Spaltenbreite in Pixel. |
| | Format | Opt. Angabe eines Aeins-Formats für die Darstellung des Wertes. |
| | Mime | Standard: false<br>Stellt den Mime-Typen (fa_mime) als Mini-Icon in der Spalte dar. |
| | Icon | Standard: false<br>Wenn angegeben, dann sollte der zugehörige Wert über Sql einer der folgenden fest vorgegebenen Möglichkeiten sein: |
| | „plus“ | ![add\_16](../../../ImagesExt/image8_937.png "add_16") |
| | „minus“ | ![minus\_16](../../../ImagesExt/image8_938.png "minus_16") |
| | „clip“ | ![paperclip\_16x16](../../../ImagesExt/image8_939.png "paperclip_16x16") |
| | Visible | Standard: true<br>Damit lässt sich also eine Spalte „wegblenden“, der Wert wird aber ermittelt. |
| With | | Der Wert des Elements gibt die With-Erweiterung vor. |
| Limitation | | Der Wert des Elements gibt die Limitierung der Anzahl der Datensätze an.<br> <br>Beispiel: top 50 |
| From | | From-Klausel<br>Standard:<br> from formulararchiv fa |
| Join | | Opt. zusätzliche Join-Klausel |
| Where | | Where Klausel. |
| GroupBy | | GroupBy-Klausel. |

Die Klauseln können mit einem Condition-Attribut dekoriert werden. Der Condition-Ausdruck kann einen Gleichheitsoperator (==) oder einen Ungleichheitsoperator (!=) beinhalten. Es werden damit nur Strings verglichen. Ist eine „Profil-Zuordnung“ dem Archiv-Profil zugeordnet, dann findet ein Colon-Processing statt.

Hierbei gibt es eine Besonderheit wenn die Auswahl-Variable „AUSW_VT“ verwendet wird: In diesem Falle behält sich letztendlich A.eins vor den Inhalt in Abhängigkeit des Volltext-Systemstatus weiterzuleiten.

Neben den XML-Kommentaren ist es zusätzlich noch möglich innerhalb der Klauseln sogenannte

Zeilenende-Kommentare und Sql-Kommentare anzugeben.
