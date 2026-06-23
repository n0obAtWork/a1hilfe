# Umsatzsteuer-ID Prüfung

<!-- source: https://amic.de/hilfe/_umsatzsteueridprfung.htm -->

Die Umsatzsteuer-ID (international auch VAT genannt ist für den Geschäftsverkehr mit Kunden und Lieferanten wichtig zu erfassen. Insbesondere beim Handel mit ausländischen Kunden stellt sich die Frage der Überprüfung der ID auf Gültigkeit und Plausibilität.

Dazu bietet das [Bundeszentralamt für Steuern](http://www.bzst.de) einen Internetdienst an. Sowohl mit Hilfe einer Internetseite als auch mit einer Datenschnittstelle für Software kann dort die Existenz und korrekte Zuordnung zu einer Firma erfragt werden. Parallel wird auch ein telefonischer Dienst angeboten. Alle diese Dienste nutzen die gleiche Datenbasis und haben die gleiche Verfügbarkeit. So sind in der Regel beim Ausfall der Datenschnittstelle auch die Internetseite und das Info-Telefon nicht zur Prüfung in der Lage.

Bei der Prüfung wird zunächst die Umsatzsteuer-ID auf ihre Plausibilität (Aufbau, Prüfsumme etc.) geprüft und ob diese überhaupt vergeben wurde. In einem zweiten Schritt werden dann die beim Handelsregister des EU-Mitgliedsstaates hinterlegten Daten mit den in A.eins hinterlegten Anschriftenangaben und dem hinterlegten Firmennamen verglichen.

Dabei werden folgende Angaben verglichen:

- Name und Namenszusatz
- Straße
- Postleitzahl
- Ort

Der Steuerparameter ([**SPA 1011** – **UstID Prüfung Datenprozedur**](../../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/ustid_pruefung_datenprozedur_spa_1011.md)) ermöglicht die Privatisierung der Adressaufbereitung. Als Standardprozedur dient (AMIC_UStIDPruefAdressen).

Somit ist durch die Verkettung weiterer Spalten eine Individualisierung der Adressprüfung realisierbar.

Dieser Vergleich lässt folgende Ergebnisse zu:

Die Daten stimmen überein (A)

Die Daten stimmen nicht überein (B)

Die Daten wurden in A.eins nicht hinterlegt (C)

Die Daten wurden im Register des EU-Mitgliedsstaats hinterlegt (D)

Bitte beachten Sie, dass im Fall einer Nicht-Übereinstimmung oftmals abweichende Schreibweisen hinterlegt sind bzw. Gesellschaftsformen wie z.B. GmbH, AG, A/S(Aktieselskab = Aktiengesellschaft - DK) oder e.U. (eingetragenes Unternehmen - AT) in Ihren Angaben fehlen können. Bitte erfragen Sie die volle Namensangabe bei Ihrem Geschäftspartner. Das Bundeszentralamt für Steuern erteilt Ihnen dazu in der Regel aus Datenschutzgründen keine Auskünfte.

Das Ergebnis der Überprüfung der Umsatzsteuer-ID kann nicht mehr schriftlich vom Bundeszentralamt für Steuern angefragt werden. Jedoch kann die Anfrage archiviert und somit das Ergebnis-XML des Webservices unter der Umsatzsteuer-Id im Archiv verwahrt werden.

Zu jeder Prüfung (außer im Vorgang) wird eine Historie der Prüfungen angelegt, die im Anschluss an die Prüfung oder mit der entsprechenden Funktion angezeigt wird.

<p class="siehe-auch">Siehe auch:</p>

- [Lizenz](./lizenz.md)
- [Prüfung im Vorgang:](./pruefung_im_vorgang.md)
- [Übersicht über Prüfungen des Webservices:](./uebersicht_ueber_pruefungen_des_webservices.md)
