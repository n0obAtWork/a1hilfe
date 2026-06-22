# Schritt für Schritt

<!-- source: https://amic.de/hilfe/_dokumenthilfe_sfs.htm -->

Einrichten eines dynamischen QR-Codes / laden eines Bildes:

<details>
<summary>Schritt 1: Private Prozedur</summary>

Je nach gewünschtem Ergebnis erstellt man sich über **[SQLPP]** -> ***Neu*** **(F8)** eine private Prozedur. Beispiele hierfür sind zu finden unter „[Beispiele für den Bilddruck](./sql_beispiele_fuer_den_bilddruck/index.md)“ und „[QR-Code Beispiele zum dynamischen laden](./tabs/einfuegen/qr_code_beispiele_zum_dynamischen_laden.md)“.

</details>

<details>
<summary>Schritt 2: Formulareinrichtung</summary>

Zuerst in den Formularstamm **[FRM]** und dort einen Datensatz auswählen und ***Ändern*** **(F5)**. Im Formularstamm Pfleger nun in den Tab “*Formularbereiche*” wechseln. Hier einen Bereich auswählen (in dem die Position 464 hinterlegt ist, z.B: 101) und diese ***Position Bearbeiten*** **(F6)**. Hier die Position 464 hinzufügen. In dem Positions Pfleger mit **(ESC)** abspeichern und dann im Formularstamm ***Pfleger Speichern*** **(F9)**.

</details>

<details>
<summary>Schritt 3: Dokumenten Editor im Vorgang</summary>

Zuerst einen Vorgang erfassen **[REE]** und dort einen Kunden auswählen. Nun in die ***Positionen*** **(F5)** und dort eine ***Textzeile*** **(F8)** erfassen. In den Textzeilen ein ***Dokument laden*** **(F9)**.

</details>

<details>
<summary>Schritt 4: Prozedur im Dokument hinterlegen</summary>

Im Dokumenten Editor auf den Tab „*Einfügen*“ wechseln. Hier die Funktion „*Stichcode*“ wählen und „*QR-Code*“ auswählen. Den QR-Code markieren und mit Rechtsklick ***Formatieren***. In dem Pfleger des QR-Codes auf den Tab „*Typ und Farbe*“ wechseln. Unter der Kategorie „*Typ*“ im Feld Text, die aus Schritt 1 erstellte Prozedur hinterlegen.

</details>
