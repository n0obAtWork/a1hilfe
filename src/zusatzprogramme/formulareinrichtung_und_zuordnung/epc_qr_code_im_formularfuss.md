# EPC-QR-CODE im Formularfuß

<!-- source: https://amic.de/hilfe/_frmpepcqrcode.htm -->

#### Allgemeines

Im Formularfuß (Bereich 902: Abschluss und Fuß letzte Seite) von Standardformularen kann die Druckposition ‚EPC-QR-CODE‘ (Position 45) eingerichtet werden, die aber nur bei Vorgängen der Vorgangsklasse 700 (Rechnung) beim Druck berücksichtigt wird.

Bei vorhandener EPC-QR-CODE-Lizenz wird an der eingerichteten Stelle ein EPC-QR-CODE erzeugt, der durch ein Onlinebanking-Programm oder eine Mobile-Banking-App decodiert werden kann, um mit diesen Daten automatisch ein Online-Überweisungsformular auszufüllen.

Die enthaltenen Daten sind:

- Empfängerbezeichnung der Überweisung
- BIC und IBAN des Empfängerkontos
- Überweisungsbetrag in EUR
- Verwendungszweck

Im ausgelieferten Standard wird die Empfängerbezeichnung dem Feld ‚Auftraggeber DTA‘ des Hausbankstamms entnommen.

Auch die IBAN wird dem Hausbankstamm entnommen.

Der BIC wird aus dem Bankenstamm, der der Hausbank zugeordnet ist entnommen.

Der Überweisungsbetrag ist der Rechnungsbetrag.

Als Verwendungszweck wird ein Text mit dem Aufbau

- ‚Rechnung‘
- Rechnungsnummer
- ‚vom‘
- Belegdatum
- ‚KuNr‘
- Kundennummer

generiert.

Bei Vorhandensein mehrerer Hausbanken kann mit dem Steuerparameter **Hausbanknummer für EPC-QRCODE (SPA1079)** durch Angabe der Hausbanknummer die heranzuziehende Hausbank festgelegt werden. Ist der Wert des Steuerparameters 0 oder mit der dort angegebenen Nummer keine (nicht gelöschte) Hausbank gefunden, so wird die (nicht gelöschte) Hausbank mit der niedrigsten Nummer herangezogen.

#### Einrichtungsdetails

Die [Einrichtung der Position](./formulareinrichtung/einrichtung_f6.md) erfolgt durch Angabe von Zeile und Spalte für die linke obere Ecke des QR-CODEs im Formularbereich Bereich 902 für die Position ‚EPC-QR-CODE‘ (45) und die Angabe des **Namens der Etikettendruck-Definition** ( im Standard: <strong><em>AMIC_EPC_QRCODE </em></strong><em>)</em> in der Spalte **Text**.

Die Größe des QR-CODEs ergibt sich aus der dort hinterlegten Report-Definition.

Grundsätzlich kann die Vorlage ***AMIC_EPC_QRCODE*** direkt verwendet werden. Bei Bedarf kann jedoch eine Privatisierung aus der Vorlage unter Angabe eines neuen Namens in der Anwendung **AMIC-Etikettendruck** vorgenommen werden, um zum Beispiel die Größe der QR-CODE-Grafik zu ändern oder in einer abgeleiteten privatisierten Prozedur die Angaben im Verwendungszweck anzupassen.
