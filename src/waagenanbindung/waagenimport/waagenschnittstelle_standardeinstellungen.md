# Waagenschnittstelle – Standardeinstellungen

<!-- source: https://amic.de/hilfe/waagenschnittstellestandardein.htm -->

Durch die Einrichtung der Waage nach untenstehenden Vorgaben wird kann die Standard-Waagenschnittstelle im A.eins mit geringmöglichem Aufwand eingerichtet werden:

| Dateiformat | ASCII | |
| --- | --- | --- |
| Transferdateiname | WAAGE.DAT | |
| Anzahl der Satzarten | 2 | |
| Satzartkennung Cerea | CER | |
| Satzartkennung Faktura | FAK | |
| Datumsformat | JJMMTT | |
| Laufwerk für Import | A:\\ | |
| Konvertierung Sorte nach Artikel in Satzart Cerea | Ja | |
| Default-Lagernummer | 1 | |
| Fehlerakzeptanz LKW mit unbekanntem Kennzeichen | Ja | |
| Default-Steuerschlüssel | 1 | |
| Datenimport wird nur akzeptiert, wenn alle Daten fehlerfrei sind | Ja | |
| interne Zwischendatei | C:\\TEMP\\WAAGE.DAT | |
| Wiegenummer | aus Waage | |
| Default-Mengeneinheit | 1<br>(ME_Nummer für „kg“ in Basis-Datenbank) | |
| | | |
| Aufbau der Importdatei | Position | Länge |
| 
Artikel (Satzart Faktura)

 | 37 | 6 |
| Kunde | 4 | 8 |
| Lieferscheindatum | 20 | 6 |
| Lieferscheinnummer | 12 | 8 |
| Menge | 55 | 6 (Cerea), 5 (Faktura) |
| Qualität 1 | 61 | 4 |
| Qualität 2 | 66 | 6 |
| Qualität 3 | 71 | 4 |
| Qualität 4 | 75 | 4 |
| Qualität 5 | 79 | 2 |
| Qualität 6 | 81 | 3 |
| Qualität 7 | 86 | 4 |
| Satzartkennung | 1 | 3 |
| Sorte | 33 | 2 |
| Steuerschlüssel | 11 | 1 |
| Wiegenummer | 12 | 8 |

Wichtiger Hinweis

Bei Aeins-Versionen bis 4.2.2.181 (21.06.1999) ist nach Beendigung der Konfigurationen das SQL-Script WAAGKORR.SQL per Direktsprung [OSQL] F3 einzuspielen. Es enthält folgendes Statement:

delete from ScriptParamPar WHERE ScriptPId='WaagenImport' and ScriptPPAktiv=0;

commit;
