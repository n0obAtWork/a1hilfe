# Formate

<!-- source: https://amic.de/hilfe/_vorgangsmappeProfile_formate.htm -->

Hier finden Sie die Formate, welche im Streckenerfassungsprofil verwendet werden.

[Streckenmenüzuordnung](./formate.md#formatStreckenmenuezuordnung)

[Partie bei Übermenge](./formate.md#formatPartiebeiUebermenge)

[Lagerauswahl](./formate.md#formatLagerauswahl)

[Lagerbezogene Artikelauswahl](./formate.md#formatLagerbezogeneArtikelauswahl)

[Zeitintervall eines Kontrakts](./formate.md#formatZeitIntervalleinesKontraktes)

[Reporttyp](./formate.md#formatReporttyp)

[Auswertungstyp](./formate.md#formatAuswertungstyp)

### Streckenmenüzuordnung

Mit diesem Format kann die Streckenmenüzuordnung festgelegt werden. (*Formatname „DISPOMENUZ“*)

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Verkauf / Einkauf | Menüzuordnung in Verkauf und Einkauf |
| 1 | Verkauf | Menüzuordnung nur im Verkauf |
| 2 | Einkauf | Menüzuordnung nur im Einkauf |

### Partie bei Übermenge

Mit diesem Format wird festgelegt, welche Partie bei einer Übermenge gezogen wird. (*Formatname „SE_PARTIE“*)

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Erste Partie | Die erste gefundene Partie wird gezogen. |
| 1 | Letzte Partie | Die letzte gefundene Partie wird gezogen. |

### Lagerauswahl

Mit diesem Format wird festgelegt, aus welchem Lager der Artikel gezogen werden soll. (*Formatname „SE_AUSWLAGER“*)

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Standardlager | Der Artikel wird über das Standardlager gezogen. |
| 1 | Kundenlager | Der Artikel wird über das Kundenlager gezogen. |

### Lagerbezogene Artikelauswahl

Mit diesem Format wird festgelegt, wie ein Artikel Lagertechnisch gezogen wird. (*Formatname „SE_AUSWARTIK“*)

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Lagerbezogen | Der Artikel wird hierbei Lagerbezogen ermittelt. |
| 1 | Lagerübergreifend | Der Artikel wird hier Lagerübergreifen ermittelt. |

### Zeitintervall eines Kontrakts

Mit diesem Format wird festgelegt, wie der Zeitraumintervall eines Kontraktes ist. (*Formatname „KTRINTERVALL“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Tag | Die Kontraktzeiträume sind einen Tag lang. |
| 1 | Woche | Die Kontraktzeiträume haben eine Laufzeit von einer Woche. |
| 2 | Monat | Die Kontraktzeiträume haben eine Laufzeit von einem Monat. |
| 3 | Jahr | Die Kontraktzeiträume haben eine Laufzeit von einem Jahr. |

### Reporttyp

Mit diesem Format wird festgelegt, um was für einen Report es sich handelt. *(Formatname „DISPREPORT“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Crystal Report | Es handelt sich hierbei um einen Crystal Report. |
| 1 | AMIC Etikettendruck | Es handelt sich hierbei um einen Crystal Report. |
| 2 | Formulardruck | Es handelt sich hierbei um einen Formulardruck. |

### Auswertungstyp

Mit diesem Format wird festgelegt, um was für einen Auswertungstyp es sich handelt. *(Formatname „DISPAUSWTYP“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Tourübersicht | Nur die Daten der Tour *(aktuelle Zeile)* werden mit in die Transfertabelle übergeben. |
| 1 | Grid 1 | Nur die Daten der ersten Datentabelle werden in die Transfertabelle übergeben. |
| 2 | Grid 2 | Nur die Daten der zweiten Datentabelle werden in die Transfertabelle übergeben. |
| 3 | Grid 3 | Nur die Daten der dritten Datentabelle werden in die Transfertabelle übergeben. |
| 4 | Markierte Zeilen | Nur die markierten Datensätze werden in die Transfertabelle übergeben. |
| 5 | Gesamt | Alle Datensätze werden in die Transfertabelle übergeben. |
