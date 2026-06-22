# Wichtige Tabellen des Kassensystems

<!-- source: https://amic.de/hilfe/wichtigetabellendeskassensyste.htm -->

| Tabelle | Beschreibung | Schlüssel |
| --- | --- | --- |
| AcashBelg | Kassenbelege | BelegKs (Kassennummer)<br>BelegId (interne Belegidentifikation)<br>FilialNummer (Betriebsstättennummer Erzeuger)<br> <br>Bei Barverkäufen ist der Kassenbeleg über BelegId und V_Id mit einem Vorgang verknüpft. |
| AcashBelgZhlg | Zahlungssätze zu Kassenbelegen. Zu einem Kassenbeleg kann es mehrere Zahlungssätze geben. | ZahlKs (Kassennummer)<br>ZahlBelegId (interne Belegidentifikation)<br>FilialNummer (Betriebsstättennummer Erzeuger)<br>ZahlNummer (fortlaufende Nummer)<br> <br>Verknüpfung mit AcashBeleg über BelegKs=ZahlKs und BelegId=ZahlBelegId und die Filialnummern. |
| AcashBelgZami | Zahlungsmittel bei unbarer Zahlung | ZamiIdNr (interne Identifikation)<br> <br>Verknüpfung über AcashBelgZhl.ZahlZamiIdNr |
| AcashBelgKsiz | Kassenbericht. Die Tabelle enthält zusammenfassende Information über einzelne Sitzungen. | KsiKsNr (Kassennummer)<br>KsiASK (sitzungsnummer)<br>FilialNummer (Betriebsstättennummer Erzeuger) |
| AcashFibuLink | Beschreibt die Verbindungen von Kassenbelegen zur Fibu | BelegKs (Kassennummer)<br>BelegId (interne Belegidentifikation)<br>FilialNummer (Betriebsstättennummer Erzeuger)<br>FibuV_Id (Ident. Des zugeh. Fibu-Vorgangs) |
