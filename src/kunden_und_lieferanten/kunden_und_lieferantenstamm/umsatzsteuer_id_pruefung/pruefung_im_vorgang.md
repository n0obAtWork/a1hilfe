# Prüfung im Vorgang:

<!-- source: https://amic.de/hilfe/prfungimvorgang.htm -->

<p class="just-emphasize">Prüfung</p>

Während der Vorgangserfassung kann in Abhängigkeit der Einstellung im [SPA 1062 – UstId Prüfung im Vorgang](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/ustid_pruefung_im_vorgang_spa_1062.md) die Umsatzsteuer-Id auch asynchron im Vorgang geprüft werden.

<p class="just-emphasize">Auswertung</p>

Die Auswertung dieser Prüfung muss wegen der Vielfältigkeit der daraus abzuleitenden Konsequenten jedoch individualisiert erfolgen.

Dazu kann zum einen aus dem Vorgang ermittelt werden, ob eine Prüfung vorgesehen war:

```sql
select amic_func_bit_test(
V_VorgBits1, 4)
from amic_v_vorgaenge
where v_id = ???;
```

Anschließend kann, sofern eine Prüfung vorgesehen war (Ergebnis 1) das Ergebnis aus der Auftragstabelle abgelesen werden:

```sql
select *
from UmsatzSteuerIdPruefAuftrag PA
join amic_v_vorgaenge vs on vs.v_guid = PA.v_guid and
vs.UstId_Kunde = PA.UstId
where v_id = ???;
```
