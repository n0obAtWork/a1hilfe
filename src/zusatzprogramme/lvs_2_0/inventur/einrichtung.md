# Einrichtung

<!-- source: https://amic.de/hilfe/_lvs20_Inv_einricht.htm -->

Im [SPA 1045 – Permanente Inventur](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/permanente_inventur_mit_lvs_spa_1045.md) werden zwei Werte festgelegt:

1. Die Anzahl der Tage für eine zusammenhängende Zählung in der permanenten Inventur

2. Die Anzahl von Zeilen in einem einzelnen Inventurbeleg

In der Vorgansunterklasse 5055 im Direktsprung [[FRZ]](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md#Allgemein) wird ein Kreditor festgelegt, der für Inventurbelege verwendet wird. Diese Einstellung gilt ebenso für ungeplante Inventuren, muss also in jedem Fall eingerichtet werden. Dieser Kreditor ist als steuerfrei zu konfigurieren!

Für die permanente Inventur muss ein Nummernkreis für Belege der Vorgangsklasse eingerichtet sein.
