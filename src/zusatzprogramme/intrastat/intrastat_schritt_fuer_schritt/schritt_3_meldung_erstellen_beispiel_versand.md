# Schritt 3 Meldung erstellen (Beispiel Versand)

<!-- source: https://amic.de/hilfe/Intrastatsfs3.htm -->

<details>
<summary>Schritt 3.1: Beispiel Versand</summary>

Für die Erstellung einer Intrastat-Meldung müssen folgende Voraussetzungen erfüllt sein:

\- Rechnung mit einem EU Kunden wurde erstellt **[REE]**

\- Die Rechnung mit dem EU Kunden wurde in die FiBu übertragen **[REB]**

</details>

 

<details>
<summary>Schritt 3.2: Datei Export</summary>

Mit dem Direktsprung **[INTRA]** navigiert man [Intrastat-Meldung Versendung](../intrastat_auswahllisten/intrastat_meldung_versendung.md) (2. Variante) oder [Intrastat-Meldung Einfuhr](../intrastat_auswahllisten/intrastat_meldung_einfuhr.md) (3. Variante). Hier ruft man nun die Funktion ***„Versand erzeugen / Einfuhr erzeugen“*** **(F9)** auf. Nach der Bestätigung des Exports öffnet sich der Explorer.

(Für die Registrierung wird die Datei mit dem Dateinamen „XGTEST*\-Datum-Uhrzeit*“ abgespeichert Normale Exporte werden mit *Materialnummer-Datum-Uhrzeit* abgespeichert)

</details>

Schritt 3.3: Steuergruppen ausschließen

Mit dem Direktsprung **[STS]** in den Steuerdatenpfleger. Dort mit der Funktion ***Steuergruppen*** **(F6)** in die Auswahlliste der Steuergruppen. Hier die Steuergruppe auswählen, welche bei dem Export nicht berücksichtigt werden soll und diese ***Bearbeiten*** **(F5)**. Nun das Feld *„Intrastat“* auf *„Nein“* setzten und den Datensatz ***Speichern*** **(F9)**.
