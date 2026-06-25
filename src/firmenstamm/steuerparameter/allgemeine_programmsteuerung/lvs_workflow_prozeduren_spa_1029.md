# LVS-Workflow-Prozeduren(SPA 1029)

<!-- source: https://amic.de/hilfe/_SPA_1029.htm -->

In diesem Steuerparameter können zweierlei Werte festgelegt werden:

1. LVS-Lokalitäten

Diese Funktion ermittelt aus einem eingegebenen Scanwert eine LVS-Lokalität. Das kann hilfreich sein,. Wenn die Lokalitäten im Barcode lesbar geschlüsselt sind, also z.B. H1-R2-S3-E4 für Halle 1 Regal 2 Stellplatz 3 Ebene 4 und dieser Wert in eine Lokalitätsnummer übersetzt werden soll.

2. LVS-Kommandos

Diese Funktion wird in der Standard-Funktion „AMIC_LVS_GETSCANTYPE“ ausgewertet. Die hier gesetzte Funktion wird zusätzlich zum Standard einen Scantyp ermitteln.

Diese Funktion hat die folgende Signatur:

```text
---<summary>Gibt einen Typ eines gescannten Codes aus </summary>
---<returns>Typ des Scancodes</returns>
---<param name="in_tcpip_adresse">Adresse des Scanners</param>
---<param name="in_Aktionswert">gescannter Wert</param>
---<param name="in_returnwert">Typ aus der Standard-Funktion</param>
create function P_DEMO_GetScanType(
  in in_tcpip_adresse char(40) default '1.1.1.1',
  in in_Aktionswert long varchar default '',
  in in_returnwert long varchar default ''
)
returns varchar(30)
```
