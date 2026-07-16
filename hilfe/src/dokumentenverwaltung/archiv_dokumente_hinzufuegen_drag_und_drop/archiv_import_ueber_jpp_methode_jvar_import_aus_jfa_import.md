# Archiv-Import über JPP-Methode JVAR_IMPORT aus JFA_Import

<!-- source: https://amic.de/hilfe/_archivimportberjppme.htm -->

Diese Methode wird u.a. im Hinzufügen-Dialog als Basis-Methode des Archivs eingesetzt. Als JPP-Methode steht sie GUI-los auch zum Costumizing bereit.

Diese Methode wird im Rahmen des Integrationstestes amic_test_jarchivexport_tofile verwendet.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Parameter:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Owner</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>Gibt den JVAR-Owner vor, in dem die folgenden dynamischen Parameter gesucht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>$file</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>Dateipfad</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>$delete</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Löschen der Datei nach Import</p>
          <p>Standard 0 = Nein</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fa_mandant</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_kundenummer</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_belegtyptext</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_belegnummer</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_belegreferenz</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_autor</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_betreff</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_kategorie</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_stichwoerter</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_kommentar</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_info_titel</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_belegdatum</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_mndnr</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_barcode</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>fa_klasse</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard ist 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fa_belegklasse</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard ist 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fa_bedienerklasse</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Standard ist die Bedienerklasse des ausführenden Users</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Bei erfolgreicher Archivierung befindet sich systemüblich in

| 5000 | JVARS_LAST_FA_ID |
| --- | --- |
| 5000 | JVARS_LAST_FA_MNDNR |

der Primary-Key des hinzugefügten Archiv-Dokuments der Relation Formulararchiv.
