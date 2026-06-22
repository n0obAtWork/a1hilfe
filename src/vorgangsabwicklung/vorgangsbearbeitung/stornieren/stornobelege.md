# Stornobelege

<!-- source: https://amic.de/hilfe/_stornobelege_ware.htm -->

Bereits weiter verarbeitete Belege (und ggf. gedruckte Belege) können mit der Funktion ***Stornobeleg*** ausgebucht werden. Es wird ein Stornobeleg mit der Belegnummer des Originalbeleges erstellt. Stornobelege sind Belege einer Stornovorgangsklasse. Wie für die Vorgangsklassen/-unterklassen der Originalbelege müssen auch für die entsprechenden Stornovorgangsklassen/-unterklassen die notwendigen Einrichtungen vorgenommen werden (Nummernkreiszuordnungen, Formularzuordnungen etc.). Stornobelege werden anschließend wie Originalbelege an die Finanzbuchhaltung übergeben.

Es gibt zwei zusätzliche Parameter für die Stornobelegerstellung:

1. **Kopie erstellen** – Wird eine Stornorechnung mit dieser Einstellung erstellt, so muss die Rechnung nicht erneut erfasst werden. Es wird eine bearbeitbare Kopie des stornierten Beleges erzeugt.

Die Standardeinstellung ist **Nein**

2. **Stornobeleg erstellen**\- Diese Bedingung kennt 3 Fälle:

a. ***Immer*** – Es wird in jedem Fall ein Stornobeleg erstellt.

b. ***Nur wenn Fibu Übertrag*** – In diesem Fall wird nur ein Stornobeleg erstellt, wenn der Beleg in die Fibu übertragen wurde. Die ggf. angeforderte Erstellung einer Kopie erfolgt auch nur in diesem Fall. Anderenfalls werden nur die Druckkennzeichen zurückgesetzt.

c. ***Nur wenn Fibu gebucht*** – In diesem Fall wird nur ein Stornobeleg erstellt, wenn der Beleg in der Fibu gebucht oder ausgeziffert wurde. Die ggf. angeforderte Erstellung einer Kopie erfolgt auch nur in diesem Fall. Anderenfalls wird der Primanota-Eintrag in der Fibu zurückgenommen und die Druckkennzeichen werden zurückgesetzt.

Die Standardeinstellung ist **Immer**

Beide Einstellungen müssen mit jeweils einem Steuerungsparameter **[SPA]** [Ware-Storno mit Quellbeleg-Kopie (SPA656)](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/ware_storno_mit_quellbeleg_kopie_spa_656.md) bzw. [Ware-Storno mit Alternative bezüglich Fibu-Status (SPA657)](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/warestorno_mit_alternative_bez_fibstatus_spa_657.md) freigeschaltet werden.
