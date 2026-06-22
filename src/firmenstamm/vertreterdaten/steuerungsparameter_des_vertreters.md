# Steuerungsparameter des Vertreters

<!-- source: https://amic.de/hilfe/_steuerungsparameterd.htm -->

Über Steuerungsparameter können Voreinstellungen vorgenommen werden. Sie finden sich in der Parametergruppe „Vertreter / Provision“:

<p class="just-emphasize">Standard Provisionierungsformel</p>

Mit diesem SPA kann eingestellt werden, welcher Typ zur Provisionsberechnung als Standardprovisionsberechnungstyp herangezogen werden soll.

\-1 = Standard Provisionsberechnung

0 = keine Provision

1 = prozentuale Provision vom Nettobetrag

2 = prozentuale Provision vom Bruttobetrag

3 = mengeneinheitsbezogene Provision

4 = gebindeanzahlbezogene Provision

5 = gewichtsbezogene Provision

6 = postenbezogene Provision

7 = pauschale Provision je Vorgang

10 = Staffelung in max. 10 Provisionsberechnungen (siehe unten Provisionsstaffelung)

11 = Überschussprovision

101 = rohgewinnbezogene Provision

<p class="just-emphasize">Offene Posten berücksichtigen</p>

Mit diesem Steuerparameter kann eingestellt werden, ob offene Posten bei der Provisionsberechnung berücksichtigt werden.

**JA =** Markierte Belege werden nur dann provisioniert, wenn sie komplett an die FiBu übergeben wurden und somit keine offenen Posten auf dem Beleg existieren.

**Nein =** Es werden alle markierten Belege provisioniert, die zumindest im Warenbuch eingetragen wurden.
