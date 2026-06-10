# Vertretergruppen: Pfleger

<div class="table-wrapper">
  <table>
    <thead>
      <tr>
        <th style="text-align: center" colspan="2">Kopfdaten</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td style="text-align: left">Vertretergruppe</td>
        <td style="text-align: left">Schlüssel für die Zuordnung provisionsrelevanter Buchungen (z. B. Warenverkäufe) zu den Provisionsempfängern. Im Normalfall besteht eine Vertretergruppe nur aus einem Vertreter, der 100 Prozent der Provision erhält.</td>
      </tr>
      <tr>
        <td style="text-align: left">Bezeichnung</td>
        <td style="text-align: left">Bezeichnung für die Vertretergruppe</td>
      </tr>
      <tr>
        <td style="text-align: left">Matchcode</td>
        <td style="text-align: left"></td>
      </tr>
    </tbody>
  </table>
</div>

<details>
  <summary>Register Allgemein</summary>

  | Felder | Beschreibung |
  | :--- | :--- |
  | Einzelprovision | <p>Kennzeichen, ob die Provisionssätze in dieser Vertretergruppe von Vertreter zu Vertreter verschieden sein können. Normalerweise ist die Ermittlung der Provision lediglich von der Vertretergruppe abhängig, also identisch innerhalb einer Gruppe.</p><p>Ja – Es ziehen Provisionsmerkmale aus dem Vertreterstamm</p><p>Nein – Es ziehen Provisionsmerkmal aus der Vertretergruppe</p> |
  | Berechnungsvariante | <p>Es sind verschiedene Berechnungsvarianten für die Provision denkbar:</p><p>1 - Berechnung vor der Verteilung = Die Provisionsermittlung erfolgt vorweg entsprechend der Provisionstabelle, danach wird die Provision anhand des Schlüssels auf die Vertreter verteilt. Bei gleichen Provisionssätzen innerhalb einer Gruppe ist dies sinnvoll.</p><p>2 - Berechnung nach der Verteilung = Zunächst werden die Umsätze anhand des Verteilungsschlüssels auf die Vertreter der Gruppe aufgeteilt, dann wird die Provision je Vertreter anhand seiner Provisionstabelle errechnet. Dies ist relevant, wenn in einer Gruppe unterschiedliche Provisionssätze bestehen.</p> |
  | Anteilsausschöpfung | Summe der Anteile aller Vertreter in einer Gruppe. |
  | Vertreternummer | Nummer des Vertreters in der Vertretergruppe |
</details>

<details>
  <summary>Funktionen:</summary>

  |  | Beschreibung |
  | :--- | :--- |
  | Neu (F8), Speichern (F9) |  |
  | Provisionsmerkmale | Hier ändert man die Provisionsmerkmale der Vertretergruppe, wenn die Einzelprovision im Pfleger auf „Nein“ steht. |
</details>