# Status einer E-Mail

<!-- source: https://amic.de/hilfe/statuseineremail.htm -->

Das Mailing-Kennzeichen hat folgende Ausprägungen:

| Wert | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 0 | Wartet auf Freigabe | Der Beleg befindet sich in der Warteschleife und soll versendet werden. Die E-Mail kann jetzt manuell Freigegeben/Versendet werden. Die [Standard-Eventmail Funktion](./events_fuer_beleg_mailversand.md) versendet diese E-Mails ebenfalls, sobald das Event startet. |
| 1 | Freigegeben | Der Beleg ist zum Versand freigegeben. Diesen Status haben E-Mails die manuell versendet werden sollen oder von einem [Dienst oder Exe](../../mailversand_allgemein/einrichtung_mailversand/synchron_oder_asynchron/index.md) verschickt werden. |
| 2 | Versendet | Der Beleg wurde erfolgreich versendet. |
| 10 | Zurückgestellt | Dieser Beleg wurde zurückgestellt. Die E-Mail wird erst nach erneuter Freigabe versendet. |
| 95 | Unzustellbar | Der Beleg kann nicht zugestellt werden. |
| 99 | fehlerhaft | Der Beleg konnte auf Grund eines Problems nicht versendet werden. |

Im Fall einer fehlerhaften E-Mail kann der Fehlercode Aufschluss über die Ursache geben. Zusätzlich finden sich Einträge im Fehlerprotokoll.
