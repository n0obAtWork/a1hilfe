# Streckenerfassung Report Transportauftrag

<!-- source: https://amic.de/hilfe/streckenerfassungreporttranspo.htm -->

Der Transportauftrag ist die Anweisung an den Spediteur, zu einem bestimmten Zeitpunkt Materialien von einem Lagerort/Lagerplatz zu einem anderen zu transportieren.

Die Adresse an die der Transportauftrag geht ist die Adresse des Spediteurs die für die Strecke angegeben wurde.  
Im Transportauftrag sind eine Lade- und eine Lieferadresse angegeben.

Die Anzeige dieser Adressen erfolgt in einer bestimmten Reihenfolge je nachdem was gefüllt ist:

**Adressfolge Lieferadresse :**

1. abweichende Ziel-Adresse

2. Adresse des VK Lagers, wenn man den Transportauftrag für den EK Ladeschein drucken will und auch ein VK Ladeschein existiert. (VK Lagertyp ungleich Streckenlager)  
    

Lagertyp Streckenlager:

n Versandadresse des Ladescheins

n Versandanschrift des VK Kontraktes

n Adresse des Kunden aus dem VK Ladeschein

n Adresse des VK Kontraktes

 3. Versandadresse des Ladescheins

 4. Versandanschrift des VK Kontraktes

 5. Adresse des VK Kontraktes

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p>1. abweichende Ziel-Adresse</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><i>Es gibt einen VK-Ladeschein zum EK-Ladeschein</i></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p><i>VK Lagertyp ungleich Streckenlager</i></p>
        </td>
        <td>
          <p><i>VK Lagertyp Streckenlager</i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2. Versandadresse des Ladescheins</p>
        </td>
        <td>
          <p>2. Adresse des VK Lagers</p>
        </td>
        <td>
          <p>2. Versandadresse des Ladescheins</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3. Versandanschrift des VK Kontraktes</p>
        </td>
        <td></td>
        <td>
          <p>3. Versandanschrift des VK Kontraktes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4. Adresse des VK Kontraktes</p>
        </td>
        <td></td>
        <td>
          <p>4. Adresse des Kunden aus dem VK Ladeschein</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td>
          <p>5. Adresse des VK Kontraktes</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

**Adressfolge Ladeadresse :**

1. abweichende Herkunfts-Adresse

2. Adresse des EK Lagers, wenn man den Transportauftrag für den VK Ladeschein drucken will und auch ein EK Ladeschein existiert. (EK Lagertyp ungleich Streckenlager)  
    
Lagertyp Streckenlager:

n Versandanschrift des EK Kontraktes

n Adresse des Kunden aus dem EK Ladeschein

n Adresse des EK Kontraktes

 3. Versandanschrift des EK Kontraktes

 4. Adresse des EK Kontraktes

 5. Adresse des Lagers

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p>1. abweichende Herkunfts-Adresse</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td colspan="2">
          <p><i>Es gibt einen EK-Ladeschein zum VK-Ladeschein</i></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p><i>EK Lagertyp ungleich Streckenlager</i></p>
        </td>
        <td>
          <p><i>EK Lagertyp Streckenlager</i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2. Versandanschrift des EK Kontraktes</p>
        </td>
        <td>
          <p>2. Adresse des EK Lagers</p>
        </td>
        <td>
          <p>2. Versandanschrift des EK Kontraktes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3. Adresse des EK Kontraktes</p>
        </td>
        <td></td>
        <td>
          <p>3. Adresse des Kunden aus dem EK Ladeschein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4. Adresse des Lagers</p>
        </td>
        <td></td>
        <td>
          <p>4. Adresse des EK Kontraktes</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Sprachabhängigkeit</p>

Welche Felder gepflegt werden müssen, um die Sprachabhängigkeit nutzen zu können liest man unter  
[Sprache der Reporte](./sprache_der_reporte.md).

Sprachabhängige Textfelder in diesem Report

| Name Druckfeld | Standard Text im Report |
| --- | --- |
| Ueberschrift_Transportauftrag | Transportauftrag Nr.<br> |
| Anrede<br> | Sehr geehrte Damen und Herren, |
| an | an: |
| von | von: |
| nach | nach: |
| Bestaetigung_Transportauftrag<br> | hiermit bestätigen wir den Transportauftrag für folgende Relation:<br> |
| Logistic_Transportauftrag<br> | Die Vorgaben nach GMP+ sind zu beachten und einzuhalten! Für Speditionsgeschäfte gelten die ADSp neueste Fassung.<br> |
| Mfg | Mit freundlichen Grüßen |
| Warenart | Warenart |
| Menge | Menge ca. |
| Frachtrate | Frachtrate per mt |
| Termin | Termin |
| Text_Beladung | (die Be- und Entladung ist mit allen Beteiligten rechtzeitig abzustimmen!) |
| Anliefernummer | Anliefernummer |
| Frachtzahler | Frachtzahler |
| Sitz1 | Sitz1 |
| Sitz2 | Sitz2 |
| Sitz3 | Sitz3 (wird ausgeblendet, wenn das Druckfeld nicht gepflegt wird) |
| Sitz4 | Sitz4 (wird ausgeblendet, wenn das Druckfeld nicht gepflegt wird) |
| HR1 | hr1 |
| HR2 | hr2 (wird ausgeblendet, wenn das Druckfeld nicht gepflegt wird) |
| HR3 | hr3 (wird ausgeblendet, wenn das Druckfeld nicht gepflegt wird) |
| HR4 | hr4 (wird ausgeblendet, wenn das Druckfeld nicht gepflegt wird) |
| Standort | Standort, den |
