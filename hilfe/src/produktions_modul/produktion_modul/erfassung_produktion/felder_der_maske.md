# Felder der Maske

<!-- source: https://amic.de/hilfe/_felderdermaske.htm -->

![](../../../ImagesExt/image8_266.jpg)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundennummer (Produktionsmaske)</p>
        </td>
        <td>
          <p>Hier kann man als Information den Kunden hinterlegen für den man z.B. ein Produktionsangebot gemacht hat.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Buchungstyp (Produktionsmaske)</p>
        </td>
        <td>
          <table>
            <tbody>
              <tr>
                <th>Buchungstyp</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>Angebot Produktion</td>
                <td>Diese Einstellung verwendet man, wenn man ein Produktionsangebot machen möchte.</td>
              </tr>
              <tr>
                <td>Auftrag Produktion</td>
                <td>Für die Produktionsplanung, Bestellungen und Aufträge.</td>
              </tr>
              <tr>
                <td>Produktion</td>
                <td>Um für die Produktion eine normale Bestandsbuchung durchzuführen.</td>
              </tr>
            </tbody>
          </table>
          <p>Der Buchungstyp kann auch schon im Vorgangskopf im Userfeld „Prod.Buchtyp“ (Nummer 4204) eingetragen werden.</p>
          <p>Eine Vorbelegung für den Buchungstyp ist in den Optionen <strong>[OPT]</strong> unter dem Wert „VorbelegungBuchungsTypProduktion“ einstellbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verwendung</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Plan/Lieferdatum</p>
        </td>
        <td>
          <p>Mit dem Plan/Lieferdatum kann das Lieferdatum (Ausführungsdatum) einer Produktion abweichend vom Vorgangsdatum bestimmt werden. Dieses kann auf der Startseite des Vorgangs angegeben werden, es dient dann allerdings nur zur Vorbelegung für neu erzeugte Produktionen. Wird dort zum Beispiel zunächst unbemerkt ein falsches Datum angegeben, so lässt es sich dieses nach Erfassung einer oder gar mehrerer Produktionen zwar vorgangsseitig ändern. Die bereits erfassten Produktionen behalten aber das ursprüngliche Datum bei. Daher lässt sich per Einrichterparameter <a href="../../../firmenstamm/einrichterparameter/maskentitel_epa_produkt.md">Plan-/Lieferdatum auf Produktionsmaske</a> das Maskenfeld <i>Plan/Lieferdatum </i>aktivieren. Hier kann, auch bei der Belegkorrektur, ein vom Vorgangstamm abweichendes Datum für die aktuelle Produktion angegeben werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Produktnummer</p>
        </td>
        <td>
          <p>Hier wählt man den Artikel aus, den man produzieren will. Es werden alle Artikel in der <strong>F3</strong>-Auswahl angezeigt, die zu dem Lager gehören und eine Rezepturgruppe hinterlegt haben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagerplatz</p>
        </td>
        <td>
          <p>Hier kann man den Lagerplatz angeben</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rezeptur</p>
        </td>
        <td>
          <p>Hier wählt man die Rezeptur an, die man verwenden möchte. Es werden die Mengeneinheit (wenn per <a href="../../../firmenstamm/einrichterparameter/maskentitel_epa_produkt.md">Einrichterparameter</a> so eingestellt) und die Komponenten des Rezeptes in die Maske übernommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Produktmenge / Mengeneinheit</p>
        </td>
        <td>
          <p>Die Mengeneinheit des Produktes wurde automatisch gefüllt mit der Mengeneinheit des Artikels oder der Rezeptur. Gibt man die Produktmenge an, dann wird das Feld Menge der Komponenten entsprechend der Angaben in der Rezeptur im Feld Anteilstyp gefüllt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengenkontrolle zwischen Produkt und Komponenten aktiv</p>
        </td>
        <td>
          <p>Ist die Mengenkontrolle aktiviert, dann wird bei Änderung der Menge einer Komponente die Produktmenge mit angepasst.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
